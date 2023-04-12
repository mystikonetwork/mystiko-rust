use hex::FromHexError;
use mystiko_crypto::error::CryptoError;
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
    HexStringError(#[from] FromHexError),
    #[error("database raised error: {0:?}")]
    DatabaseError(#[source] anyhow::Error),
    #[error("invalid password: {0:?}")]
    InvalidPasswordError(String),
    #[error("password is wrong")]
    MismatchedPasswordError,
    #[error("no existing wallet found")]
    NoExistingWalletError,
    #[error("no such account where {0:?} = {1:?}")]
    NoSuchAccountError(String, String),
}
