use crate::fetcher::FetcherError;
use crate::handler::HandlerError;
use crate::validator::ValidatorError;
use anyhow::Error as AnyhowError;
use ethers_providers::ProviderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataLoaderError {
    #[error("no contracts to load data from")]
    LoaderNoContractsError,
    #[error("no fetcher to load data from")]
    LoaderNoFetchersError,
    #[error("failed to query loaded block from all fetchers")]
    QueryLoadedBlocksError,
    #[error("query of fetcher(name = {0}) loaded block timed out after (1) ms")]
    QueryLoadedBlocksTimeoutError(String, u64),
    #[error("fetch of fetcher(name = {0}) timed out after {1} ms)")]
    FetchTimeoutError(String, u64),
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
    HandlerError(#[from] HandlerError),
    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),
    #[error("unsupported chain (id = {0})")]
    UnsupportedChainError(u64),
}
