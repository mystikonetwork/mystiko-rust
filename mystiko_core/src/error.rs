use crate::{ScannerError, SynchronizerError};
use mystiko_dataloader::DataLoaderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MystikoError {
    #[error(transparent)]
    SynchronizerError(#[from] SynchronizerError),
    #[error(transparent)]
    ScannerError(#[from] ScannerError),
    #[error(transparent)]
    RelayerClientError(#[from] mystiko_relayer_client::error::RelayerClientError),
    #[error(transparent)]
    DataLoaderError(#[from] DataLoaderError),
    #[error("config raised error: {0:?}")]
    ConfigError(#[source] anyhow::Error),
    #[error("failed to migrate database: {0:?}")]
    DatabaseMigrationError(#[source] anyhow::Error),
    #[error("invalid provider url provided: {0:?}")]
    InvalidProviderUrlError(String),
}
