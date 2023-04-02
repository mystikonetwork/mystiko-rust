use anyhow::Result;
use bip32::{Language, Mnemonic};
use futures::lock::Mutex;
use hex::FromHexError;
use lazy_static::lazy_static;
use mystiko_crypto::crypto::{decrypt_symmetric, encrypt_symmetric};
use mystiko_crypto::error::CryptoError;
use mystiko_crypto::hash::checksum;
use mystiko_database::database::Database;
use mystiko_database::document::wallet::Wallet;
use mystiko_storage::document::{Document, DocumentRawData, DOCUMENT_CREATED_AT_FIELD};
use mystiko_storage::filter::{Order, QueryFilterBuilder};
use mystiko_storage::formatter::StatementFormatter;
use mystiko_storage::storage::Storage;
use rand_core::OsRng;
use regex::Regex;
use std::sync::Arc;
use thiserror::Error;
use typed_builder::TypedBuilder;

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

pub struct WalletHandler<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    db: Arc<Mutex<Database<F, R, S>>>,
}

#[derive(Error, Debug)]
pub enum WalletError {
    #[error(transparent)]
    CryptoError(#[from] CryptoError),
    #[error(transparent)]
    MnemonicError(#[from] bip32::Error),
    #[error(transparent)]
    HexStringError(#[from] FromHexError),
    #[error("entropy is invalid")]
    InvalidEntropyError,
    #[error("database raised error: {0:?}")]
    DatabaseError(#[source] anyhow::Error),
    #[error("invalid password: {0:?}")]
    InvalidPasswordError(String),
    #[error("password is wrong")]
    MismatchedPasswordError,
    #[error("no existing wallet found")]
    NoExistingWalletError,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct WalletCreationOptions {
    pub password: String,
    #[builder(default, setter(strip_option))]
    pub mnemonic_phrase: Option<String>,
}

impl<F, R, S> WalletHandler<F, R, S>
where
    F: StatementFormatter,
    R: DocumentRawData,
    S: Storage<R>,
{
    pub fn new(db: Arc<Mutex<Database<F, R, S>>>) -> Self {
        Self { db }
    }

    pub async fn current(&mut self) -> Result<Option<Document<Wallet>>, WalletError> {
        let filter = QueryFilterBuilder::new()
            .order_by(vec![DOCUMENT_CREATED_AT_FIELD.to_string()], Order::DESC)
            .build();
        self.db
            .lock()
            .await
            .wallets
            .find_one(filter)
            .await
            .map_err(WalletError::DatabaseError)
    }

    pub async fn create(
        &mut self,
        options: &WalletCreationOptions,
    ) -> Result<Document<Wallet>, WalletError> {
        validate_password(&options.password)?;
        let mnemonic = if let Some(mnemonic_words) = &options.mnemonic_phrase {
            Mnemonic::new(mnemonic_words, Language::English)?
        } else {
            Mnemonic::random(OsRng, Language::English)
        };
        let encrypted_entropy =
            encrypt_symmetric(&options.password, &hex::encode(mnemonic.entropy()))?;
        let hashed_password = checksum(&options.password, None);
        let wallet = Wallet {
            hashed_password,
            encrypted_entropy,
            account_nonce: 0,
        };
        let wallet = self
            .db
            .lock()
            .await
            .wallets
            .insert(&wallet)
            .await
            .map_err(WalletError::DatabaseError)?;
        log::info!("successfully created a wallet(id = {})", wallet.id);
        Ok(wallet)
    }

    pub async fn check_current(&mut self) -> Result<Document<Wallet>, WalletError> {
        if let Some(wallet) = self.current().await? {
            Ok(wallet)
        } else {
            Err(WalletError::NoExistingWalletError)
        }
    }

    pub async fn check_password(
        &mut self,
        password: &str,
    ) -> Result<Document<Wallet>, WalletError> {
        let wallet = self.check_current().await?;
        let hashed_password = checksum(password, None);
        if wallet.data.hashed_password == hashed_password {
            Ok(wallet)
        } else {
            Err(WalletError::MismatchedPasswordError)
        }
    }

    pub async fn update_password(
        &mut self,
        old_password: &str,
        new_password: &str,
    ) -> Result<Document<Wallet>, WalletError> {
        let mut wallet = self.check_password(old_password).await?;
        validate_password(new_password)?;
        let entropy_string = decrypt_symmetric(old_password, &wallet.data.encrypted_entropy)?;
        wallet.data.encrypted_entropy = encrypt_symmetric(new_password, &entropy_string)?;
        wallet.data.hashed_password = checksum(new_password, None);
        wallet = self
            .db
            .lock()
            .await
            .wallets
            .update(&wallet)
            .await
            .map_err(WalletError::DatabaseError)?;
        log::info!(
            "successfully updated the password of the wallet(id = {})",
            wallet.id
        );
        Ok(wallet)
    }

    pub async fn export_mnemonic_phrase(&mut self, password: &str) -> Result<String, WalletError> {
        let wallet = self.check_password(password).await?;
        let entropy_string = decrypt_symmetric(password, &wallet.data.encrypted_entropy)?;
        let entropy_bytes = hex::decode(entropy_string)?;
        match entropy_bytes.try_into() {
            Ok(entropy) => {
                let mnemonic = Mnemonic::from_entropy(entropy, Language::English);
                Ok(mnemonic.phrase().to_string())
            }
            Err(_) => Err(WalletError::InvalidEntropyError),
        }
    }
}

fn validate_password(password: &str) -> Result<(), WalletError> {
    if password.len() >= 8
        && PASSWORD_LOWER_REGEX.is_match(password)
        && PASSWORD_UPPER_REGEX.is_match(password)
        && PASSWORD_DIGIT_REGEX.is_match(password)
        && PASSWORD_SPECIAL_REGEX.is_match(password)
    {
        Ok(())
    } else {
        Err(WalletError::InvalidPasswordError(PASSWORD_HINT.to_string()))
    }
}
