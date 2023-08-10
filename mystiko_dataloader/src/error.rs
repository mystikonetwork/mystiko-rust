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
    #[error(transparent)]
    ValidatorError(#[from] DataValidatorError),
    #[error("unsupported chain (id = {0})")]
    UnsupportedChainError(u64),
}

#[derive(Error, Debug)]
pub enum DataValidatorError {
    #[error("validator init error {0}")]
    ValidatorBuildError(String),
    #[error("validator data to be validated is empty")]
    ValidatorEmptyValidateDataError,
    #[error("validator validate error {0}")]
    ValidatorValidateError(String),
    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),
}
