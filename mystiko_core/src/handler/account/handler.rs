use crate::{
    Account, AccountColumn, AccountHandler, Database, FromContext, MystikoContext, MystikoError, Wallet, WalletHandler,
    Wallets, WalletsError,
};
use async_trait::async_trait;
use bip32::{ChildNumber, ExtendedPrivateKey, XPrv};
use futures::TryFutureExt;
use mystiko_crypto::crypto::{decrypt_symmetric, encrypt_symmetric};
use mystiko_crypto::error::CryptoError;
use mystiko_protocol::address::ShieldedAddress;
use mystiko_protocol::key::{combined_secret_key, full_public_key, separate_secret_keys};
use mystiko_protocol::types::{EncSk, FullSk, VerifySk};
use mystiko_protos::core::document::v1::Account as ProtoAccount;
use mystiko_protos::core::handler::v1::{CreateAccountOptions, UpdateAccountOptions};
use mystiko_protos::core::v1::MnemonicType;
use mystiko_protos::storage::v1::{ConditionOperator, QueryFilter, SubFilter};
use mystiko_storage::{Document, DocumentColumn, StatementFormatter, Storage, StorageError};
use mystiko_utils::hex::{decode_hex_with_length, encode_hex};
use std::sync::Arc;
use thiserror::Error;

// m/purpose/coin_type/account/key_type/address_index
pub const DEFAULT_KEY_DERIVE_PATH: &str = "m/44'/94085'/0'";

#[derive(Debug)]
pub struct Accounts<F: StatementFormatter, S: Storage> {
    db: Arc<Database<F, S>>,
    wallets: Wallets<F, S>,
}

#[derive(Debug, Error)]
pub enum AccountsError {
    #[error(transparent)]
    StorageError(#[from] StorageError),
    #[error(transparent)]
    CryptoError(#[from] CryptoError),
    #[error(transparent)]
    MnemonicError(#[from] bip32::Error),
    #[error(transparent)]
    HexStringError(#[from] rustc_hex::FromHexError),
    #[error(transparent)]
    WalletsError(#[from] WalletsError),
    #[error(transparent)]
    ProtocolKeyError(#[from] mystiko_protocol::error::ProtocolKeyError),
    #[error("no such account where {0:?} = {1:?}")]
    NoSuchAccountError(String, String),
}

type Result<T> = std::result::Result<T, AccountsError>;

#[async_trait]
impl<F, S> AccountHandler<ProtoAccount, CreateAccountOptions, UpdateAccountOptions> for Accounts<F, S>
where
    F: StatementFormatter,
    S: Storage,
{
    type Error = AccountsError;

    async fn create(&self, options: &CreateAccountOptions) -> Result<ProtoAccount> {
        let mut wallet = self.wallets.check_document_password(&options.wallet_password).await?;
        let (raw_account, account_nonce) = self.create_raw_account(&wallet, options).await?;
        let account = self.insert_raw_account(&mut wallet, raw_account, account_nonce).await?;
        log::info!(
            "successfully created an account(id = \"{}\", name = \"{}\")",
            &account.id,
            &account.name
        );
        Ok(account)
    }

    async fn find<Q>(&self, filter: Q) -> Result<Vec<ProtoAccount>>
    where
        Q: Into<QueryFilter> + Send + Sync,
    {
        Ok(self
            .find_documents(filter)
            .await?
            .into_iter()
            .map(Account::document_into_proto)
            .collect())
    }

    async fn find_all(&self) -> Result<Vec<ProtoAccount>> {
        Ok(self
            .find_all_documents()
            .await?
            .into_iter()
            .map(Account::document_into_proto)
            .collect())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<ProtoAccount>> {
        self.find_one_by_identifier(id, DocumentColumn::Id).await
    }

    async fn find_by_shielded_address(&self, shielded_address: &str) -> Result<Option<ProtoAccount>> {
        self.find_one_by_identifier(shielded_address, AccountColumn::ShieldedAddress)
            .await
    }

    async fn find_by_public_key(&self, public_key: &str) -> Result<Option<ProtoAccount>> {
        self.find_one_by_identifier(public_key, AccountColumn::PublicKey).await
    }

    async fn count<Q>(&self, filter: Q) -> Result<u64>
    where
        Q: Into<QueryFilter> + Send + Sync,
    {
        let filter = self.wrap_filter(Some(filter)).await?;
        self.db
            .accounts
            .count(filter)
            .await
            .map_err(AccountsError::StorageError)
    }

    async fn count_all(&self) -> Result<u64> {
        self.count(
            QueryFilter::builder()
                .conditions_operator(ConditionOperator::And)
                .build(),
        )
        .await
    }

    async fn update_by_id(&self, id: &str, options: &UpdateAccountOptions) -> Result<ProtoAccount> {
        self.update_by_identifier(id, DocumentColumn::Id, options).await
    }

    async fn update_by_shielded_address(
        &self,
        shielded_address: &str,
        options: &UpdateAccountOptions,
    ) -> Result<ProtoAccount> {
        self.update_by_identifier(shielded_address, AccountColumn::ShieldedAddress, options)
            .await
    }

    async fn update_by_public_key(&self, public_key: &str, options: &UpdateAccountOptions) -> Result<ProtoAccount> {
        self.update_by_identifier(public_key, AccountColumn::PublicKey, options)
            .await
    }

    async fn update_encryption(
        &self,
        old_wallet_password: &str,
        new_wallet_password: &str,
    ) -> Result<Vec<ProtoAccount>> {
        let wallet = self.wallets.check_password(new_wallet_password).await?;
        let mut accounts = self.find_all_documents().await?;
        for account in accounts.iter_mut() {
            let secret_key = decrypt_symmetric(old_wallet_password, &account.data.encrypted_secret_key)?;
            let full_sk: FullSk = decode_hex_with_length(secret_key.clone())?;
            let full_pk = full_public_key(&full_sk)?;
            let full_pk_str = encode_hex(full_pk);
            if account.data.public_key != full_pk_str {
                return Err(WalletsError::MismatchedPasswordError.into());
            }
            account.data.encrypted_secret_key = encrypt_symmetric(new_wallet_password, &secret_key)?;
        }
        let accounts = self
            .db
            .accounts
            .update_batch(&accounts)
            .await
            .map_err(AccountsError::StorageError)?;
        log::info!(
            "successfully updated the encryption of all accounts from wallet(id = \"{}\")",
            &wallet.id
        );
        Ok(accounts.into_iter().map(Account::document_into_proto).collect())
    }

    async fn export_secret_key_by_id(&self, wallet_password: &str, id: &str) -> Result<String> {
        self.export_secret_key_by_identifier(wallet_password, id, DocumentColumn::Id)
            .await
    }

    async fn export_secret_key_by_public_key(&self, wallet_password: &str, public_key: &str) -> Result<String> {
        self.export_secret_key_by_identifier(wallet_password, public_key, AccountColumn::PublicKey)
            .await
    }

    async fn export_secret_key_by_shielded_address(
        &self,
        wallet_password: &str,
        shielded_address: &str,
    ) -> Result<String> {
        self.export_secret_key_by_identifier(wallet_password, shielded_address, AccountColumn::ShieldedAddress)
            .await
    }
}

#[async_trait]
impl<F, S> FromContext<F, S> for Accounts<F, S>
where
    F: StatementFormatter,
    S: Storage,
{
    async fn from_context(context: &MystikoContext<F, S>) -> std::result::Result<Self, MystikoError> {
        Ok(Self::new(context.db.clone()))
    }
}

impl<F, S> Accounts<F, S>
where
    F: StatementFormatter,
    S: Storage,
{
    pub fn new(db: Arc<Database<F, S>>) -> Self {
        Self {
            db: db.clone(),
            wallets: Wallets::new(db),
        }
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

    async fn find_documents<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<Vec<Document<Account>>> {
        let filter = self.wrap_filter(Some(filter)).await?;
        Ok(self
            .db
            .accounts
            .find(filter)
            .await
            .map_err(AccountsError::StorageError)?
            .into_iter()
            .collect())
    }

    pub async fn find_all_documents(&self) -> Result<Vec<Document<Account>>> {
        self.find_documents(
            QueryFilter::builder()
                .conditions_operator(ConditionOperator::And)
                .build(),
        )
        .await
    }

    async fn find_one_by_identifier<T: ToString>(
        &self,
        identifier: &str,
        field_name: T,
    ) -> Result<Option<ProtoAccount>> {
        self.find_one_document_by_identifier(identifier, field_name)
            .await
            .map(|doc| doc.map(Account::document_into_proto))
    }

    async fn find_one_document_by_identifier<T: ToString>(
        &self,
        identifier: &str,
        field_name: T,
    ) -> Result<Option<Document<Account>>> {
        let filter = SubFilter::equal(field_name, identifier);
        let wrapped_filter = self.wrap_filter(Some(filter)).await?;
        self.db
            .accounts
            .find_one(wrapped_filter)
            .await
            .map_err(AccountsError::StorageError)
    }

    async fn update_by_identifier<T: ToString>(
        &self,
        identifier: &str,
        field_name: T,
        options: &UpdateAccountOptions,
    ) -> Result<ProtoAccount> {
        self.wallets.check_password(&options.wallet_password).await?;
        let field_name_str = field_name.to_string();
        if let Some(mut account) = self.find_one_document_by_identifier(identifier, field_name).await? {
            let mut has_update = false;
            if let Some(new_name) = &options.name {
                if !new_name.is_empty() && new_name != &account.data.name {
                    account.data.name.clone_from(new_name);
                    has_update = true;
                }
            }
            if has_update {
                let updated_account = self
                    .db
                    .accounts
                    .update(&account)
                    .map_err(AccountsError::StorageError)
                    .await?;
                log::info!(
                    "successfully updated an account(id = \"{}\") with options: {:?}",
                    &updated_account.id,
                    options
                );
                Ok(Account::document_into_proto(updated_account))
            } else {
                Ok(Account::document_into_proto(account))
            }
        } else {
            Err(AccountsError::NoSuchAccountError(
                field_name_str,
                identifier.to_string(),
            ))
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
            Err(AccountsError::NoSuchAccountError(
                field_name_str,
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
        if wallet.data.mnemonic_type == MnemonicType::Web as i32 {
            self.generate_web_secret_key(wallet, wallet_password).await
        } else {
            self.generate_rust_secret_key(wallet, wallet_password).await
        }
    }

    async fn generate_web_secret_key(
        &self,
        wallet: &Document<Wallet>,
        wallet_password: &str,
    ) -> Result<(VerifySk, EncSk, u32)> {
        let mnemonic = self.wallets.export_mnemonic(wallet_password).await?;
        let master_seed = mnemonic.to_entropy();
        let x_prv: XPrv = ExtendedPrivateKey::new(master_seed).unwrap();
        let account_nonce = wallet.data.account_nonce;
        let sk_verify = if account_nonce == 0 {
            x_prv.private_key().to_bytes()
        } else {
            x_prv
                .derive_child(ChildNumber::from(account_nonce))?
                .private_key()
                .to_bytes()
        };
        let sk_enc = x_prv
            .derive_child(ChildNumber::from(account_nonce + 1))?
            .private_key()
            .to_bytes();

        Ok((sk_verify.into(), sk_enc.into(), account_nonce + 2))
    }

    async fn generate_rust_secret_key(
        &self,
        wallet: &Document<Wallet>,
        wallet_password: &str,
    ) -> Result<(VerifySk, EncSk, u32)> {
        let mnemonic_words = self.wallets.export_mnemonic(wallet_password).await?;
        let master_seed = mnemonic_words.to_seed("");
        let sk_verify_path = format!("{}/{}/{}", DEFAULT_KEY_DERIVE_PATH, 0, wallet.data.account_nonce);
        let sk_enc_path = format!("{}/{}/{}", DEFAULT_KEY_DERIVE_PATH, 1, wallet.data.account_nonce);
        let sk_verify = XPrv::derive_from_path(master_seed, &sk_verify_path.parse()?)?;
        let sk_enc = XPrv::derive_from_path(master_seed, &sk_enc_path.parse()?)?;
        Ok((sk_verify.to_bytes(), sk_enc.to_bytes(), wallet.data.account_nonce + 1))
    }

    async fn create_raw_account(
        &self,
        wallet: &Document<Wallet>,
        options: &CreateAccountOptions,
    ) -> Result<(Account, u32)> {
        let (sk_verify, sk_enc, account_nonce) = if let Some(secret_key) = &options.secret_key {
            let secret_key_bytes: FullSk = decode_hex_with_length(secret_key)?;
            let (verify_sk, enc_sk) = separate_secret_keys(&secret_key_bytes)?;
            (verify_sk, enc_sk, wallet.data.account_nonce)
        } else {
            self.generate_secret_key(wallet, &options.wallet_password).await?
        };
        let full_sk = combined_secret_key(&sk_verify, &sk_enc);
        let full_pk = full_public_key(&full_sk)?;
        let full_sk_str = encode_hex(full_sk);
        let encrypted_sk = encrypt_symmetric(&options.wallet_password, &full_sk_str)?;
        let shielded_address = ShieldedAddress::from_full_public_key(&full_pk);
        let account_name: String = if let Some(given_name) = &options.name {
            if given_name.is_empty() {
                self.default_account_name().await?
            } else {
                given_name.clone()
            }
        } else {
            self.default_account_name().await?
        };
        let account = Account {
            name: account_name,
            shielded_address: shielded_address.address(),
            public_key: encode_hex(full_pk),
            encrypted_secret_key: encrypted_sk,
            wallet_id: wallet.id.clone(),
            scanned_to_id: None,
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
                    .map_err(AccountsError::StorageError)?;
            }
            Ok(Account::document_into_proto(
                self.db
                    .accounts
                    .insert(&account)
                    .await
                    .map_err(AccountsError::StorageError)?,
            ))
        }
    }
}
