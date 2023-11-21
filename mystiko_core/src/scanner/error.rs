use crate::{AccountsError, WalletsError};
use mystiko_crypto::error::CryptoError;
use mystiko_protocol::error::ProtocolError;
use mystiko_storage::StorageError;
use rustc_hex::FromHexError;
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Debug, Error)]
pub enum ScannerError {
    #[error("no such accounts")]
    NoSuchAccountError,
    #[error("no such chain={0} contract={1} config")]
    NoSuchContractConfigError(u64, String),
    #[error("commitment is empty")]
    CommitmentEmptyError,
    #[error(transparent)]
    AccountHandlerError(#[from] AccountsError),
    #[error(transparent)]
    WalletHandlerError(#[from] WalletsError),
    #[error(transparent)]
    CryptoError(#[from] CryptoError),
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
