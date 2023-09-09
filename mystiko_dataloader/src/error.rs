use crate::fetcher::FetcherError;
use crate::handler::HandlerError;
use crate::validator::rule::RuleValidatorError;
use crate::validator::ValidatorError;
use anyhow::Error as AnyhowError;
use ethers_providers::ProviderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataLoaderError {
    #[error("not supported fetcher type {0}")]
    FetcherTypeError(i32),
    #[error("not supported validator type {0}")]
    ValidatorTypeError(i32),
    #[error("not supported rule validator checker type {0}")]
    RuleValidatorCheckerTypeError(i32),
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
    FetcherError(#[from] FetcherError),
    #[error(transparent)]
    ValidatorError(#[from] ValidatorError),
    #[error(transparent)]
    RuleValidatorError(#[from] RuleValidatorError),
    #[error(transparent)]
    HandlerError(#[from] HandlerError),
    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),
    #[error("unsupported chain (id = {0})")]
    UnsupportedChainError(u64),
}
