use anyhow::Error as AnyhowError;
use ethers_providers::ProviderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataLoaderError {
    #[error("loader init error {0}")]
    LoaderInitError(String),
    #[error("unsupported chain (id = {0})")]
    UnsupportedChainError(u64),
    #[error("loader run error {0}")]
    LoaderRunError(String),
    #[error(transparent)]
    ProviderError(#[from] ProviderError),
    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),
}
