use anyhow::Error as AnyhowError;
use ethers_providers::ProviderError;
use mystiko_server_utils::token_price::error::TokenPriceError;
use mystiko_server_utils::tx_manager::error::TxManagerError;
use mystiko_storage::error::StorageError;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;

pub type Result<T> = anyhow::Result<T, RollerError>;

#[derive(Error, Debug)]
pub enum RollerError {
    #[error("read file error {0}")]
    FileError(String),
    #[error(transparent)]
    SerdeJsonError(#[from] SerdeJsonError),
    #[error("log level error {0}")]
    InitLogError(String),
    #[error("load {0} config error")]
    LoadConfigError(String),
    #[error("evn {0} not config")]
    EnvNotConfig(String),
    #[error("commitment queue slow")]
    CommitmentQueueSlow,
    #[error("no provider")]
    NoProvider,
    #[error("no provider")]
    NoIndexer,
    #[error("invalid commitment hash")]
    InvalidCommitmentHash,
    #[error("invalid call data {0}")]
    InvalidCallData(String),
    #[error("rpc call error {0}")]
    RpcCallError(String),
    #[error("contract call error {0}")]
    ContractCallError(String),
    #[error(transparent)]
    DatabaseError(#[from] StorageError),
    #[error(transparent)]
    ProviderError(#[from] ProviderError),
    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),
    #[error(transparent)]
    TokenPriceError(#[from] TokenPriceError),
    #[error(transparent)]
    TxManagerError(#[from] TxManagerError),
}

impl PartialEq for RollerError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::FileError(_), Self::FileError(_))
                | (Self::SerdeJsonError(_), Self::SerdeJsonError(_))
                | (Self::InitLogError(_), Self::InitLogError(_))
                | (Self::LoadConfigError(_), Self::LoadConfigError(_))
                | (Self::EnvNotConfig(_), Self::EnvNotConfig(_))
                | (Self::CommitmentQueueSlow, Self::CommitmentQueueSlow)
                | (Self::NoProvider, Self::NoProvider)
                | (Self::NoIndexer, Self::NoIndexer)
                | (Self::InvalidCommitmentHash, Self::InvalidCommitmentHash)
                | (Self::ProviderError(_), Self::ProviderError(_))
                | (Self::AnyhowError(_), Self::AnyhowError(_))
        )
    }
}
