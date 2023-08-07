use anyhow::Error as AnyhowError;
use ethers_providers::ProviderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataLoaderError {
    #[error("loader init error {0}")]
    LoaderInitError(String),
    #[error("unsupported chain (id = {0})")]
    UnsupportedChainError(u64),
    #[error("loader load error {0}")]
    LoaderLoadError(String),
    #[error("contracts empty")]
    LoaderContractsEmpty,
    #[error("failed fetch from all fetchers")]
    LoaderFetchersFailed,
    #[error("fetcher data empty")]
    LoaderFetcherDataEmpty,
    #[error("fetcher data all invalid")]
    LoaderFetcherDataInvalid,
    #[error(transparent)]
    ProviderError(#[from] ProviderError),
    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),
}
