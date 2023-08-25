use crate::error::MystikoError;
use crate::handler::wallet::{to_document_wallet, WalletHandler};
use crate::types::Result;
use bip32::XPrv;
use futures::TryFutureExt;
use mystiko_crypto::crypto::{decrypt_symmetric, encrypt_symmetric};
use mystiko_database::database::Database;
use mystiko_database::document::account::Account;
use mystiko_database::document::account::AccountColumn;
use mystiko_database::document::wallet::Wallet;
use mystiko_protocol::address::ShieldedAddress;
use mystiko_protocol::key::{
    combined_public_key, combined_secret_key, encryption_public_key, separate_secret_keys, verification_public_key,
};
use mystiko_protocol::types::{EncSk, FullSk, VerifySk};
use mystiko_protos::core::document::v1::Account as ProtoAccount;
use mystiko_protos::core::handler::v1::{CreateAccountOptions, UpdateAccountOptions};
use mystiko_protos::storage::v1::{ConditionOperator, QueryFilter, SubFilter};
use mystiko_storage::document::{Document, DocumentColumn};
use mystiko_storage::formatter::types::StatementFormatter;
use mystiko_storage::storage::Storage;
use mystiko_types::AccountStatus;
use mystiko_utils::hex::{decode_hex_with_length, encode_hex};
use std::sync::Arc;

pub const DEFAULT_ACCOUNT_SCAN_SIZE: u32 = 10000;
// m/purpose/coin_type/account/key_type/address_index
pub const DEFAULT_KEY_DERIVE_PATH: &str = "m/44'/94085'/0'";

#[derive(Debug)]
pub struct AccountHandler<F: StatementFormatter, S: Storage> {
    db: Arc<Database<F, S>>,
    wallets: WalletHandler<F, S>,
}

impl<F, S> AccountHandler<F, S>
where
    F: StatementFormatter,
    S: Storage,
{
    pub fn new(db: Arc<Database<F, S>>) -> Self {
        Self {
            db: db.clone(),
            wallets: WalletHandler::new(db),
        }
    }

    pub async fn create(&self, options: &CreateAccountOptions) -> Result<ProtoAccount> {
        let mut wallet = to_document_wallet(self.wallets.check_password(&options.wallet_password).await?);
        let (raw_account, account_nonce) = self.create_raw_account(&wallet, options).await?;
        let account = self.insert_raw_account(&mut wallet, raw_account, account_nonce).await?;
        log::info!(
            "successfully created an account(id = \"{}\", name = \"{}\")",
            &account.id,
            &account.name
        );
        Ok(account)
    }

    pub async fn count<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<u64> {
        let filter = self.wrap_filter(Some(filter)).await?;
        self.db.accounts.count(filter).await.map_err(MystikoError::StorageError)
    }

    pub async fn count_all(&self) -> Result<u64> {
        self.count(
            QueryFilter::builder()
                .conditions_operator(ConditionOperator::And)
                .build(),
        )
        .await
    }

    pub async fn find<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<Vec<ProtoAccount>> {
        let filter = self.wrap_filter(Some(filter)).await?;
        Ok(self
            .db
            .accounts
            .find(filter)
            .await
            .map_err(MystikoError::StorageError)?
            .iter()
            .map(|document| to_proto_account(document.clone()))
            .collect())
    }

    pub async fn find_all(&self) -> Result<Vec<ProtoAccount>> {
        self.find(
            QueryFilter::builder()
                .conditions_operator(ConditionOperator::And)
                .build(),
        )
        .await
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<ProtoAccount>> {
        self.find_one_by_identifier(id, DocumentColumn::Id).await
    }

    pub async fn find_by_shielded_address(&self, shielded_address: &str) -> Result<Option<ProtoAccount>> {
        self.find_one_by_identifier(shielded_address, AccountColumn::ShieldedAddress)
            .await
    }

    pub async fn find_by_public_key(&self, shielded_address: &str) -> Result<Option<ProtoAccount>> {
        self.find_one_by_identifier(shielded_address, AccountColumn::PublicKey)
            .await
    }

    pub async fn update_by_id(&self, id: &str, options: &UpdateAccountOptions) -> Result<ProtoAccount> {
        self.update_by_identifier(id, DocumentColumn::Id, options).await
    }

    pub async fn update_by_shielded_address(
        &self,
        shielded_address: &str,
        options: &UpdateAccountOptions,
    ) -> Result<ProtoAccount> {
        self.update_by_identifier(shielded_address, AccountColumn::ShieldedAddress, options)
            .await
    }

    pub async fn update_by_public_key(&self, public_key: &str, options: &UpdateAccountOptions) -> Result<ProtoAccount> {
        self.update_by_identifier(public_key, AccountColumn::PublicKey, options)
            .await
    }

    pub async fn update_encryption(
        &self,
        old_wallet_password: &str,
        new_wallet_password: &str,
    ) -> Result<Vec<ProtoAccount>> {
        let wallet = self.wallets.check_password(old_wallet_password).await?;
        let mut accounts = self.find_all().await?;
        for account in accounts.iter_mut() {
            let secret_key = decrypt_symmetric(old_wallet_password, &account.encrypted_secret_key)?;
            account.encrypted_secret_key = encrypt_symmetric(new_wallet_password, &secret_key)?;
        }
        let accounts = self
            .db
            .accounts
            .update_batch(
                &accounts
                    .iter()
                    .map(|account| to_document_account(account.clone()))
                    .collect::<Vec<Document<Account>>>(),
            )
            .await
            .map_err(MystikoError::StorageError)?;
        log::info!(
            "successfully updated the encryption of all accounts from wallet(id = \"{}\")",
            &wallet.id
        );
        Ok(accounts
            .iter()
            .map(|account| to_proto_account(account.clone()))
            .collect())
    }

    pub async fn export_secret_key_by_id(&self, wallet_password: &str, id: &str) -> Result<String> {
        self.export_secret_key_by_identifier(wallet_password, id, DocumentColumn::Id)
            .await
    }

    pub async fn export_secret_key_by_public_key(&self, wallet_password: &str, public_key: &str) -> Result<String> {
        self.export_secret_key_by_identifier(wallet_password, public_key, AccountColumn::PublicKey)
            .await
    }

    pub async fn export_secret_key_by_shielded_address(
        &self,
        wallet_password: &str,
        shielded_address: &str,
    ) -> Result<String> {
        self.export_secret_key_by_identifier(wallet_password, shielded_address, AccountColumn::ShieldedAddress)
            .await
    }

    async fn wrap_filter<Q: Into<QueryFilter>>(&self, filter: Option<Q>) -> Result<QueryFilter> {
        let wallet = self.wallets.check_current().await?;
        let mut filter: QueryFilter = match filter {
            Some(filter) => filter.into(),
            None => QueryFilter::builder()
                .conditions_operator(ConditionOperator::And)
                .build(),
        };
        filter
            .conditions
            .push(SubFilter::equal(AccountColumn::WalletId, wallet.id).into());
        Ok(filter)
    }

    async fn find_one_by_identifier<T: ToString>(
        &self,
        identifier: &str,
        field_name: T,
    ) -> Result<Option<ProtoAccount>> {
        let filter = SubFilter::equal(field_name, identifier);
        let wrapped_filter = self.wrap_filter(Some(filter)).await?;

        Ok(self
            .db
            .accounts
            .find_one(wrapped_filter)
            .await
            .map_err(MystikoError::StorageError)?
            .map(to_proto_account))
    }

    async fn update_by_identifier<T: ToString>(
        &self,
        identifier: &str,
        field_name: T,
        options: &UpdateAccountOptions,
    ) -> Result<ProtoAccount> {
        self.wallets.check_password(&options.wallet_password).await?;
        let field_name_str = field_name.to_string();
        if let Some(account) = self.find_one_by_identifier(identifier, field_name).await? {
            let mut account = to_document_account(account);
            let mut has_update = false;
            if let Some(new_name) = &options.name {
                if !new_name.is_empty() && new_name != &account.data.name {
                    account.data.name = new_name.clone();
                    has_update = true;
                }
            }
            if let Some(new_scan_size) = options.scan_size {
                if new_scan_size > 0 && new_scan_size != account.data.scan_size {
                    account.data.scan_size = new_scan_size;
                    has_update = true;
                }
            }
            if let Some(new_status) = &options.status {
                let account_status: AccountStatus = (*new_status).into();
                if account_status != account.data.status {
                    account.data.status = account_status;
                    has_update = true;
                }
            }
            if has_update {
                let updated_account = self
                    .db
                    .accounts
                    .update(&account)
                    .map_err(MystikoError::StorageError)
                    .await?;
                log::info!(
                    "successfully updated an account(id = \"{}\") with options: {:?}",
                    &updated_account.id,
                    options
                );
                Ok(to_proto_account(updated_account))
            } else {
                Ok(to_proto_account(account))
            }
        } else {
            Err(MystikoError::NoSuchAccountError(field_name_str, identifier.to_string()))
        }
    }

    async fn export_secret_key_by_identifier<T: ToString>(
        &self,
        wallet_password: &str,
        identifier: &str,
        field_name: T,
    ) -> Result<String> {
        self.wallets.check_password(wallet_password).await?;
        let field_name_str = field_name.to_string();
        if let Some(account) = self.find_one_by_identifier(identifier, field_name).await? {
            Ok(decrypt_symmetric(wallet_password, &account.encrypted_secret_key)?)
        } else {
            Err(MystikoError::NoSuchAccountError(field_name_str, identifier.to_string()))
        }
    }

    async fn default_account_name(&self) -> Result<String> {
        let count = self.count_all().await?;
        Ok(format!("Account {}", count + 1))
    }

    async fn generate_secret_key(
        &self,
        wallet: &Document<Wallet>,
        wallet_password: &str,
    ) -> Result<(VerifySk, EncSk, u32)> {
        let mnemonic_words = self.wallets.export_mnemonic(wallet_password).await?;
        let master_seed = mnemonic_words.to_seed("");
        let sk_verify_path = format!("{}/{}/{}", DEFAULT_KEY_DERIVE_PATH, 0, wallet.data.account_nonce);
        let sk_enc_path = format!("{}/{}/{}", DEFAULT_KEY_DERIVE_PATH, 1, wallet.data.account_nonce);
        let sk_verify = XPrv::derive_from_path(&master_seed, &sk_verify_path.parse()?)?;
        let sk_enc = XPrv::derive_from_path(&master_seed, &sk_enc_path.parse()?)?;
        Ok((sk_verify.to_bytes(), sk_enc.to_bytes(), wallet.data.account_nonce + 1))
    }

    async fn create_raw_account(
        &self,
        wallet: &Document<Wallet>,
        options: &CreateAccountOptions,
    ) -> Result<(Account, u32)> {
        let (sk_verify, sk_enc, account_nonce) = if let Some(secret_key) = &options.secret_key {
            let secret_key_bytes: FullSk = decode_hex_with_length(secret_key)?;
            let (verify_sk, enc_sk) = separate_secret_keys(&secret_key_bytes);
            (verify_sk, enc_sk, wallet.data.account_nonce)
        } else {
            self.generate_secret_key(wallet, &options.wallet_password).await?
        };
        let pk_verify = verification_public_key(&sk_verify);
        let pk_enc = encryption_public_key(&sk_enc);
        let full_pk = combined_public_key(&pk_verify, &pk_enc);
        let full_sk = combined_secret_key(&sk_verify, &sk_enc);
        let full_sk_str = encode_hex(&full_sk);
        let encrypted_sk = encrypt_symmetric(&options.wallet_password, &full_sk_str)?;
        let shielded_address = ShieldedAddress::from_public_key(&pk_verify, &pk_enc);
        let account_name: String = if let Some(given_name) = &options.name {
            if given_name.is_empty() {
                self.default_account_name().await?
            } else {
                given_name.clone()
            }
        } else {
            self.default_account_name().await?
        };
        let scan_size: u32 = if let Some(scan_size) = &options.scan_size {
            if *scan_size > 0 {
                *scan_size
            } else {
                DEFAULT_ACCOUNT_SCAN_SIZE
            }
        } else {
            DEFAULT_ACCOUNT_SCAN_SIZE
        };
        let account = Account {
            name: account_name,
            shielded_address: shielded_address.address(),
            public_key: encode_hex(&full_pk),
            encrypted_secret_key: encrypted_sk,
            status: AccountStatus::Created,
            scan_size,
            wallet_id: wallet.id.clone(),
        };
        Ok((account, account_nonce))
    }

    async fn insert_raw_account(
        &self,
        wallet: &mut Document<Wallet>,
        account: Account,
        account_nonce: u32,
    ) -> Result<ProtoAccount> {
        if let Some(existing_account) = self.find_by_shielded_address(&account.shielded_address).await? {
            Ok(existing_account)
        } else {
            if wallet.data.account_nonce != account_nonce {
                wallet.data.account_nonce = account_nonce;
                self.db
                    .wallets
                    .update(wallet)
                    .await
                    .map_err(MystikoError::StorageError)?;
            }
            Ok(to_proto_account(
                self.db
                    .accounts
                    .insert(&account)
                    .await
                    .map_err(MystikoError::StorageError)?,
            ))
        }
    }
}

fn to_proto_account(document: Document<Account>) -> ProtoAccount {
    ProtoAccount::builder()
        .id(document.id)
        .created_at(document.created_at)
        .updated_at(document.updated_at)
        .name(document.data.name)
        .shielded_address(document.data.shielded_address)
        .public_key(document.data.public_key)
        .encrypted_secret_key(document.data.encrypted_secret_key)
        .scan_size(document.data.scan_size)
        .wallet_id(document.data.wallet_id)
        .status(Into::<i32>::into(document.data.status))
        .build()
}

fn to_document_account(proto: ProtoAccount) -> Document<Account> {
    Document::new(
        proto.id,
        proto.created_at,
        proto.updated_at,
        Account {
            name: proto.name,
            shielded_address: proto.shielded_address,
            public_key: proto.public_key,
            encrypted_secret_key: proto.encrypted_secret_key,
            status: proto.status.into(),
            scan_size: proto.scan_size,
            wallet_id: proto.wallet_id,
        },
    )
}
