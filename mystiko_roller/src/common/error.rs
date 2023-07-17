use anyhow::Error as AnyhowError;
use ethers_core::abi::Error as AbiError;
use ethers_providers::ProviderError;
use mehcode_config::ConfigError;
use mystiko_crypto::error::MerkleTreeError;
use mystiko_protocol::error::ProtocolError;
use mystiko_server_utils::token_price::error::TokenPriceError;
use mystiko_server_utils::tx_manager::error::TxManagerError;
use mystiko_storage::error::StorageError;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;
use tokio::task::JoinError;

pub type Result<T> = anyhow::Result<T, RollerError>;

#[derive(Error, Debug)]
pub enum RollerError {
    #[error(transparent)]
    ConfigError(#[from] ConfigError),
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
    #[error("provider not exist {0}")]
    NoProvider(String),
    #[error("indexer not exist")]
    NoIndexer,
    #[error("chain explorer not exist")]
    NoChainExplorer,
    #[error("circuits not exist")]
    CircuitNotFound,
    #[error("invalid commitment hash")]
    InvalidCommitmentHash,
    #[error("invalid event logs {0}")]
    InvalidEventLogs(String),
    #[error("invalid call data {0}")]
    InvalidCallData(String),
    #[error("rpc call error {0}")]
    RpcCallError(String),
    #[error("contract call error {0}")]
    ContractCallError(String),
    #[error(transparent)]
    AbiError(#[from] AbiError),
    #[error(transparent)]
    DatabaseError(#[from] StorageError),
    #[error(transparent)]
    ProviderError(#[from] ProviderError),
    #[error(transparent)]
    TokenPriceError(#[from] TokenPriceError),
    #[error(transparent)]
    TxManagerError(#[from] TxManagerError),
    #[error(transparent)]
    MerkleTreeError(#[from] MerkleTreeError),
    #[error(transparent)]
    ProtocolError(#[from] ProtocolError),
    #[error(transparent)]
    JoinError(#[from] JoinError),
    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),
    #[error(transparent)]
    ReqwestError(#[from] ReqwestError),
    #[error("explorer error {0}")]
    ExplorerError(String),
    #[error("commitment missing")]
    CommitmentMissing,
    #[error("new runtime error {0}")]
    RuntimeError(String),
}
