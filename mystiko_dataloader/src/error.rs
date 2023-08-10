use anyhow::Error as AnyhowError;
use ethers_providers::ProviderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataLoaderError {
    #[error("loader init error {0}")]
    LoaderBuildError(String),
    #[error("loader load error {0}")]
    LoaderLoadError(String),
    #[error("no contracts to be loaded")]
    LoaderNoContractsError,
    #[error("failed to fetch data from all fetchers")]
    LoaderFetchersExhaustedError,
    #[error("data to be validated is empty")]
    LoaderEmptyValidateDataError,
    #[error("data to be handled is empty")]
    LoaderEmptyHandleDataError,
    #[error(transparent)]
    ProviderError(#[from] ProviderError),
    #[error(transparent)]
    FetcherError(#[from] AnyhowError),
    #[error("unsupported chain (id = {0})")]
    UnsupportedChainError(u64),
    #[error("fetcher params validate error: {0}")]
    FetcherParamsError(String),
    #[error("fetcher assemble data error: {0}")]
    FetcherAssembleDataError(String),
}
