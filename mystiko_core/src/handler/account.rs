use crate::error::MystikoError;
use crate::handler::wallet::WalletHandler;
use crate::types::Result;
use bip32::XPrv;
use futures::TryFutureExt;
use mystiko_crypto::crypto::{decrypt_symmetric, encrypt_symmetric};
use mystiko_database::database::Database;
use mystiko_database::document::account::{
    Account, PUBLIC_KEY_FIELD_NAME, SHIELDED_ADDRESS_FIELD_NAME, WALLET_ID_FIELD_NAME,
};
use mystiko_database::document::wallet::Wallet;
use mystiko_protocol::address::ShieldedAddress;
use mystiko_protocol::key::{
    combined_public_key, combined_secret_key, encryption_public_key, separate_secret_keys,
    verification_public_key,
};
use mystiko_protocol::types::{EncSk, FullSk, VerifySk};
use mystiko_storage::document::{Document, DocumentRawData, DOCUMENT_ID_FIELD};
use mystiko_storage::filter::{Condition, QueryFilter, QueryFilterBuilder, SubFilter};
use mystiko_storage::formatter::StatementFormatter;
use mystiko_storage::storage::Storage;
use mystiko_types::AccountStatus;
use mystiko_utils::hex::{decode_hex_with_length, encode_hex};
use std::sync::Arc;
use typed_builder::TypedBuilder;

pub const DEFAULT_ACCOUNT_SCAN_SIZE: u32 = 10000;
// m/purpose/coin_type/account/key_type/address_index
pub const DEFAULT_KEY_DERIVE_PATH: &str = "m/44'/94085'/0'";

#[derive(Debug)]
pub struct AccountHandler<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    db: Arc<Database<F, R, S>>,
    wallets: WalletHandler<F, R, S>,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct CreateAccountOptions {
    pub wallet_password: String,
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    #[builder(default, setter(strip_option))]
    pub scan_size: Option<u32>,
    #[builder(default, setter(strip_option))]
    pub secret_key: Option<String>,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct UpdateAccountOptions {
    pub wallet_password: String,
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    #[builder(default, setter(strip_option))]
    pub scan_size: Option<u32>,
    #[builder(default, setter(strip_option))]
    pub status: Option<AccountStatus>,
}

impl<F, R, S> AccountHandler<F, R, S>
where
    F: StatementFormatter,
    R: DocumentRawData,
    S: Storage<R>,
{
    pub fn new(db: Arc<Database<F, R, S>>) -> Self {
        Self {
            db: db.clone(),
            wallets: WalletHandler::new(db),
        }
    }

    pub async fn create(&self, options: &CreateAccountOptions) -> Result<Document<Account>> {
        let mut wallet = self
            .wallets
            .check_password(&options.wallet_password)
            .await?;
        let (raw_account, account_nonce) = self.create_raw_account(&wallet, options).await?;
        let account = self
            .insert_raw_account(&mut wallet, raw_account, account_nonce)
            .await?;
        log::info!(
            "successfully created an account(id = \"{}\", name = \"{}\")",
            &account.id,
            &account.data.name
        );
        Ok(account)
    }

    pub async fn count(&self, filter: QueryFilter) -> Result<u64> {
        let filter = self.wrap_filter(Some(filter)).await?;
        self.db
            .accounts
            .count(filter)
            .await
            .map_err(MystikoError::DatabaseError)
    }

    pub async fn count_all(&self) -> Result<u64> {
        self.count(QueryFilterBuilder::new().build()).await
    }

    pub async fn find(&self, filter: QueryFilter) -> Result<Vec<Document<Account>>> {
        let filter = self.wrap_filter(Some(filter)).await?;
        self.db
            .accounts
            .find(filter)
            .await
            .map_err(MystikoError::DatabaseError)
    }

    pub async fn find_all(&self) -> Result<Vec<Document<Account>>> {
        self.find(QueryFilterBuilder::new().build()).await
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Document<Account>>> {
        self.find_one_by_identifier(id, DOCUMENT_ID_FIELD).await
    }

    pub async fn find_by_shielded_address(
        &self,
        shielded_address: &str,
    ) -> Result<Option<Document<Account>>> {
        self.find_one_by_identifier(shielded_address, SHIELDED_ADDRESS_FIELD_NAME)
            .await
    }

    pub async fn find_by_public_key(
        &self,
        shielded_address: &str,
    ) -> Result<Option<Document<Account>>> {
        self.find_one_by_identifier(shielded_address, PUBLIC_KEY_FIELD_NAME)
            .await
    }

    pub async fn update_by_id(
        &self,
        id: &str,
        options: &UpdateAccountOptions,
    ) -> Result<Document<Account>> {
        self.update_by_identifier(id, DOCUMENT_ID_FIELD, options)
            .await
    }

    pub async fn update_by_shielded_address(
        &self,
        shielded_address: &str,
        options: &UpdateAccountOptions,
    ) -> Result<Document<Account>> {
        self.update_by_identifier(shielded_address, SHIELDED_ADDRESS_FIELD_NAME, options)
            .await
    }

    pub async fn update_by_public_key(
        &self,
        public_key: &str,
        options: &UpdateAccountOptions,
    ) -> Result<Document<Account>> {
        self.update_by_identifier(public_key, PUBLIC_KEY_FIELD_NAME, options)
            .await
    }

    pub async fn update_encryption(
        &self,
        old_wallet_password: &str,
        new_wallet_password: &str,
    ) -> Result<Vec<Document<Account>>> {
        let wallet = self.wallets.check_password(old_wallet_password).await?;
        let mut accounts = self.find_all().await?;
        for account in accounts.iter_mut() {
            let secret_key =
                decrypt_symmetric(old_wallet_password, &account.data.encrypted_secret_key)?;
            account.data.encrypted_secret_key =
                encrypt_symmetric(new_wallet_password, &secret_key)?;
        }
        let accounts = self
            .db
            .accounts
            .update_batch(&accounts)
            .await
            .map_err(MystikoError::DatabaseError)?;
        log::info!(
            "successfully updated the encryption of all accounts from wallet(id = \"{}\")",
            &wallet.id
        );
        Ok(accounts)
    }

    pub async fn export_secret_key_by_id(&self, wallet_password: &str, id: &str) -> Result<String> {
        self.export_secret_key_by_identifier(wallet_password, id, DOCUMENT_ID_FIELD)
            .await
    }

    pub async fn export_secret_key_by_public_key(
        &self,
        wallet_password: &str,
        public_key: &str,
    ) -> Result<String> {
        self.export_secret_key_by_identifier(wallet_password, public_key, PUBLIC_KEY_FIELD_NAME)
            .await
    }

    pub async fn export_secret_key_by_shielded_address(
        &self,
        wallet_password: &str,
        shielded_address: &str,
    ) -> Result<String> {
        self.export_secret_key_by_identifier(
            wallet_password,
            shielded_address,
            SHIELDED_ADDRESS_FIELD_NAME,
        )
        .await
    }

    async fn wrap_filter(&self, filter: Option<QueryFilter>) -> Result<QueryFilter> {
        let wallet = self.wallets.check_current().await?;
        let mut filter = filter.unwrap_or(QueryFilterBuilder::new().build());
        filter.conditions.push(Condition::FILTER(SubFilter::Equal(
            WALLET_ID_FIELD_NAME.to_string(),
            wallet.id,
        )));
        Ok(filter)
    }

    async fn find_one_by_identifier(
        &self,
        identifier: &str,
        field_name: &str,
    ) -> Result<Option<Document<Account>>> {
        let filter = QueryFilterBuilder::new()
            .filter(Condition::FILTER(SubFilter::Equal(
                field_name.to_string(),
                identifier.to_string(),
            )))
            .build();
        let wrapped_filter = self.wrap_filter(Some(filter)).await?;
        self.db
            .accounts
            .find_one(wrapped_filter)
            .await
            .map_err(MystikoError::DatabaseError)
    }

    async fn update_by_identifier(
        &self,
        identifier: &str,
        field_name: &str,
        options: &UpdateAccountOptions,
    ) -> Result<Document<Account>> {
        self.wallets
            .check_password(&options.wallet_password)
            .await?;
        if let Some(mut account) = self.find_one_by_identifier(identifier, field_name).await? {
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
                if new_status != &account.data.status {
                    account.data.status = new_status.clone();
                    has_update = true;
                }
            }
            if has_update {
                let updated_account = self
                    .db
                    .accounts
                    .update(&account)
                    .map_err(MystikoError::DatabaseError)
                    .await?;
                log::info!(
                    "successfully updated an account(id = \"{}\") with options: {:?}",
                    &updated_account.id,
                    options
                );
                Ok(updated_account)
            } else {
                Ok(account)
            }
        } else {
            Err(MystikoError::NoSuchAccountError(
                field_name.to_string(),
                identifier.to_string(),
            ))
        }
    }

    async fn export_secret_key_by_identifier(
        &self,
        wallet_password: &str,
        identifier: &str,
        field_name: &str,
    ) -> Result<String> {
        self.wallets.check_password(wallet_password).await?;
        if let Some(account) = self.find_one_by_identifier(identifier, field_name).await? {
            Ok(decrypt_symmetric(
                wallet_password,
                &account.data.encrypted_secret_key,
            )?)
        } else {
            Err(MystikoError::NoSuchAccountError(
                field_name.to_string(),
                identifier.to_string(),
            ))
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
        let sk_verify_path = format!(
            "{}/{}/{}",
            DEFAULT_KEY_DERIVE_PATH, 0, wallet.data.account_nonce
        );
        let sk_enc_path = format!(
            "{}/{}/{}",
            DEFAULT_KEY_DERIVE_PATH, 1, wallet.data.account_nonce
        );
        let sk_verify = XPrv::derive_from_path(&master_seed, &sk_verify_path.parse()?)?;
        let sk_enc = XPrv::derive_from_path(&master_seed, &sk_enc_path.parse()?)?;
        Ok((
            sk_verify.to_bytes(),
            sk_enc.to_bytes(),
            wallet.data.account_nonce + 1,
        ))
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
            self.generate_secret_key(wallet, &options.wallet_password)
                .await?
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
    ) -> Result<Document<Account>> {
        if let Some(existing_account) = self
            .find_by_shielded_address(&account.shielded_address)
            .await?
        {
            Ok(existing_account)
        } else {
            if wallet.data.account_nonce != account_nonce {
                wallet.data.account_nonce = account_nonce;
                self.db
                    .wallets
                    .update(wallet)
                    .await
                    .map_err(MystikoError::DatabaseError)?;
            }
            self.db
                .accounts
                .insert(&account)
                .await
                .map_err(MystikoError::DatabaseError)
        }
    }
}
