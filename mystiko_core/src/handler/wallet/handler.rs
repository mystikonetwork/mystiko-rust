use crate::{Database, FromContext, MystikoContext, MystikoError, Wallet, WalletHandler};
use async_trait::async_trait;
use bip32::{Language, Mnemonic, KEY_SIZE};
use lazy_static::lazy_static;
use mystiko_crypto::crypto::{decrypt_symmetric, encrypt_symmetric};
use mystiko_crypto::error::CryptoError;
use mystiko_crypto::hash::checksum;
use mystiko_protos::core::document::v1::Wallet as ProtoWallet;
use mystiko_protos::core::handler::v1::CreateWalletOptions;
use mystiko_protos::storage::v1::{ConditionOperator, Order, OrderBy, QueryFilter};
use mystiko_storage::{Document, DocumentColumn, StatementFormatter, Storage, StorageError};
use mystiko_utils::hex::{decode_hex_with_length, encode_hex};
use rand_core::OsRng;
use regex::Regex;
use std::sync::Arc;
use thiserror::Error;

lazy_static! {
    static ref PASSWORD_LOWER_REGEX: Regex = Regex::new(r"[a-z]").unwrap();
    static ref PASSWORD_UPPER_REGEX: Regex = Regex::new(r"[A-Z]").unwrap();
    static ref PASSWORD_DIGIT_REGEX: Regex = Regex::new(r"\d").unwrap();
    static ref PASSWORD_SPECIAL_REGEX: Regex = Regex::new(r"[#?!@$%^&*-]").unwrap();
}

const PASSWORD_HINT: &str = "\
    the password must contain \
    at least one upper case letter, \
    one lower case letter, \
    one number digit, \
    one special character in [#?!@$%^&*-], \
    and the length should be as least 8";

#[derive(Debug)]
pub struct Wallets<F: StatementFormatter, S: Storage> {
    db: Arc<Database<F, S>>,
}

#[derive(Debug, Error)]
pub enum WalletsError {
    #[error(transparent)]
    StorageError(#[from] StorageError),
    #[error(transparent)]
    CryptoError(#[from] CryptoError),
    #[error(transparent)]
    HexStringError(#[from] rustc_hex::FromHexError),
    #[error(transparent)]
    MnemonicError(#[from] bip32::Error),
    #[error("invalid password: {0:?}")]
    InvalidPasswordError(String),
    #[error("password is wrong")]
    MismatchedPasswordError,
    #[error("no existing wallet found")]
    NoExistingWalletError,
}

type Result<T> = std::result::Result<T, WalletsError>;

#[async_trait]
impl<F, S> WalletHandler<ProtoWallet, CreateWalletOptions> for Wallets<F, S>
where
    F: StatementFormatter,
    S: Storage,
{
    type Error = WalletsError;

    async fn current(&self) -> Result<Option<ProtoWallet>> {
        self.current_document()
            .await
            .map(|doc| doc.map(Wallet::document_into_proto))
    }

    async fn check_current(&self) -> Result<ProtoWallet> {
        self.check_document_current().await.map(Wallet::document_into_proto)
    }

    async fn create(&self, options: &CreateWalletOptions) -> Result<ProtoWallet> {
        validate_password(&options.password)?;
        let mnemonic = if let Some(mnemonic_words) = &options.mnemonic_phrase {
            Mnemonic::new(mnemonic_words, Language::English)?
        } else {
            Mnemonic::random(OsRng, Language::English)
        };
        let encrypted_entropy = encrypt_symmetric(&options.password, &encode_hex(mnemonic.entropy()))?;
        let hashed_password = checksum(&options.password, None);
        let wallet = Wallet {
            hashed_password,
            encrypted_entropy,
            account_nonce: 0,
        };
        let wallet = self
            .db
            .wallets
            .insert(&wallet)
            .await
            .map_err(WalletsError::StorageError)?;
        log::info!("successfully created a wallet(id = \"{}\")", wallet.id);
        Ok(Wallet::document_into_proto(wallet))
    }

    async fn check_password(&self, password: &str) -> Result<ProtoWallet> {
        self.check_document_password(password)
            .await
            .map(Wallet::document_into_proto)
    }

    async fn update_password(&self, old_password: &str, new_password: &str) -> Result<ProtoWallet> {
        let mut wallet = self.check_document_password(old_password).await?;
        validate_password(new_password)?;
        let entropy_string = decrypt_symmetric(old_password, &wallet.data.encrypted_entropy)?;
        wallet.data.encrypted_entropy = encrypt_symmetric(new_password, &entropy_string)?;
        wallet.data.hashed_password = checksum(new_password, None);
        wallet = self
            .db
            .wallets
            .update(&wallet)
            .await
            .map_err(WalletsError::StorageError)?;
        log::info!(
            "successfully updated the password of the wallet(id = \"{}\")",
            wallet.id
        );
        Ok(Wallet::document_into_proto(wallet))
    }

    async fn export_mnemonic(&self, password: &str) -> Result<Mnemonic> {
        let wallet = self.check_password(password).await?;
        let entropy_string = decrypt_symmetric(password, &wallet.encrypted_entropy)?;
        let entropy: [u8; KEY_SIZE] = decode_hex_with_length(entropy_string)?;
        Ok(Mnemonic::from_entropy(entropy, Language::English))
    }

    async fn export_mnemonic_phrase(&self, password: &str) -> Result<String> {
        Ok(self.export_mnemonic(password).await?.phrase().to_string())
    }
}

#[async_trait]
impl<F, S> FromContext<F, S> for Wallets<F, S>
where
    F: StatementFormatter,
    S: Storage,
{
    async fn from_context(context: &MystikoContext<F, S>) -> std::result::Result<Self, MystikoError> {
        Ok(Self::new(context.db.clone()))
    }
}

impl<F, S> Wallets<F, S>
where
    F: StatementFormatter,
    S: Storage,
{
    pub fn new(db: Arc<Database<F, S>>) -> Self {
        Self { db }
    }
    pub(crate) async fn current_document(&self) -> Result<Option<Document<Wallet>>> {
        let filter = QueryFilter::builder()
            .conditions_operator(ConditionOperator::And)
            .order_by(
                OrderBy::builder()
                    .columns(vec![DocumentColumn::Id.to_string()])
                    .order(Order::Desc)
                    .build(),
            )
            .build();
        self.db
            .wallets
            .find_one(filter)
            .await
            .map_err(WalletsError::StorageError)
    }

    pub(crate) async fn check_document_current(&self) -> Result<Document<Wallet>> {
        if let Some(wallet) = self.current_document().await? {
            Ok(wallet)
        } else {
            Err(WalletsError::NoExistingWalletError)
        }
    }

    pub(crate) async fn check_document_password(&self, password: &str) -> Result<Document<Wallet>> {
        let wallet = self.check_document_current().await?;
        let hashed_password = checksum(password, None);
        if wallet.data.hashed_password == hashed_password {
            Ok(wallet)
        } else {
            Err(WalletsError::MismatchedPasswordError)
        }
    }
}

fn validate_password(password: &str) -> Result<()> {
    if password.len() >= 8
        && PASSWORD_LOWER_REGEX.is_match(password)
        && PASSWORD_UPPER_REGEX.is_match(password)
        && PASSWORD_DIGIT_REGEX.is_match(password)
        && PASSWORD_SPECIAL_REGEX.is_match(password)
    {
        Ok(())
    } else {
        Err(WalletsError::InvalidPasswordError(PASSWORD_HINT.to_string()))
    }
}
