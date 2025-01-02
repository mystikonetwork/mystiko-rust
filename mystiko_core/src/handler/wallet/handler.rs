use crate::{Database, FromContext, MystikoContext, MystikoError, Wallet, WalletHandler};
use async_trait::async_trait;
use bip39::Mnemonic;
use itertools::Itertools;
use lazy_static::lazy_static;
use mystiko_crypto::crypto::{decrypt_symmetric, encrypt_symmetric};
use mystiko_crypto::error::CryptoError;
use mystiko_crypto::hash::checksum;
use mystiko_protos::core::document::v1::Wallet as ProtoWallet;
use mystiko_protos::core::handler::v1::CreateWalletOptions;
use mystiko_protos::core::v1::MnemonicType;
use mystiko_protos::storage::v1::{ConditionOperator, Order, OrderBy, QueryFilter};
use mystiko_storage::{Document, DocumentColumn, StatementFormatter, Storage, StorageError};
use mystiko_utils::hex::{decode_hex, encode_hex};
use regex::Regex;
use std::str::FromStr;
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

const MNEMONIC_WEB_PHRASE_WORD_COUNT: usize = 12;
const MNEMONIC_RUST_PHRASE_WORD_COUNT: usize = 24;

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
    MnemonicError(#[from] bip39::Error),
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
        let (mnemonic, mnemonic_type) = if let Some(options) = &options.mnemonic {
            let mnemonic = Mnemonic::from_str(&options.mnemonic_phrase)?;
            let word_count = mnemonic.words().collect_vec().len();
            let mnemonic_type = if options.mnemonic_type == MnemonicType::Web as i32 {
                if word_count != MNEMONIC_WEB_PHRASE_WORD_COUNT {
                    return Err(WalletsError::MnemonicError(bip39::Error::BadWordCount(word_count)));
                }
                MnemonicType::Web as i32
            } else {
                if word_count != MNEMONIC_RUST_PHRASE_WORD_COUNT {
                    return Err(WalletsError::MnemonicError(bip39::Error::BadWordCount(word_count)));
                }
                MnemonicType::Rust as i32
            };
            (mnemonic, mnemonic_type)
        } else {
            let mnemonic = Mnemonic::generate(MNEMONIC_RUST_PHRASE_WORD_COUNT)?;
            (mnemonic, MnemonicType::Rust as i32)
        };
        let encrypted_entropy = encrypt_symmetric(&options.password, &encode_hex(mnemonic.to_entropy()))?;
        let hashed_password = checksum(&options.password, None)?;
        let wallet = Wallet {
            mnemonic_type,
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
        wallet.data.hashed_password = checksum(new_password, None)?;
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
        let entropy = decode_hex(entropy_string)?;
        Ok(Mnemonic::from_entropy(&entropy)?)
    }

    async fn export_mnemonic_phrase(&self, password: &str) -> Result<String> {
        let mnemonic = self.export_mnemonic(password).await?;
        let mnemonic_phrase = mnemonic.words().collect::<Vec<&str>>().join(" ");
        Ok(mnemonic_phrase)
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
        let hashed_password = checksum(password, None)?;
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
