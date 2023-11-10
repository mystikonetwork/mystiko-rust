use crate::{AccountHandlerError, WalletHandlerError};
use mystiko_protocol::error::ProtocolError;
use mystiko_storage::StorageError;
use rustc_hex::FromHexError;
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Debug, Error)]
pub enum ScannerError {
    #[error("commitment={0} encrypted note is empty")]
    CommitmentEncryptedNoteIsNone(String),
    #[error("no account found")]
    NoAccountFound,
    #[error("chain config not found for chain id={0}")]
    ChainConfigNotFoundError(u64),
    #[error("asset config not found for chain id={0} and asset symbol={1}")]
    AssetConfigNotFoundError(u64, String),
    #[error("chain id={0} commitment id={1} missing amount error ")]
    MissingAmountError(u64, String),
    #[error("internal error: {0}")]
    InternalError(String),
    #[error(transparent)]
    AccountHandlerError(#[from] AccountHandlerError),
    #[error(transparent)]
    WalletHandlerError(#[from] WalletHandlerError),
    #[error(transparent)]
    StorageError(#[from] StorageError),
    #[error(transparent)]
    JoinError(#[from] JoinError),
    #[error(transparent)]
    ProtocolError(#[from] ProtocolError),
    #[error(transparent)]
    FromHexError(#[from] FromHexError),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}
