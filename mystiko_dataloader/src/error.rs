use crate::fetcher::FetcherError;
use crate::handler::HandlerError;
use crate::validator::ValidatorError;
use anyhow::Error as AnyhowError;
use ethers_providers::ProviderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataLoaderError {
    #[error("no contracts to be loaded")]
    LoaderNoContractsError,
    #[error("failed to query loaded block from all fetchers")]
    QueryLoadedBlocksError,
    #[error("query of fetcher(index = {0}) loaded block timed out after (1) ms")]
    QueryLoadedBlocksTimeoutError(usize, u64),
    #[error("fetch of fetcher(index = {0}) timed out after {1} ms)")]
    FetchTimeoutError(usize, u64),
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
