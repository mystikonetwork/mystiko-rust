use mystiko_crypto::error::CryptoError;
use mystiko_storage::StorageError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MystikoError {
    #[error("config raised error: {0:?}")]
    ConfigError(#[source] anyhow::Error),
    #[error("failed to migrate database: {0:?}")]
    DatabaseMigrationError(#[source] anyhow::Error),
    #[error(transparent)]
    CryptoError(#[from] CryptoError),
    #[error(transparent)]
    MnemonicError(#[from] bip32::Error),
    #[error(transparent)]
    HexStringError(#[from] rustc_hex::FromHexError),
    #[error(transparent)]
    StorageError(#[from] StorageError),
    #[error("invalid password: {0:?}")]
    InvalidPasswordError(String),
    #[error("password is wrong")]
    MismatchedPasswordError,
    #[error("no existing wallet found")]
    NoExistingWalletError,
    #[error("no such account where {0:?} = {1:?}")]
    NoSuchAccountError(String, String),
    #[error("invalid provider url provided: {0:?}")]
    InvalidProviderUrlError(String),
}
