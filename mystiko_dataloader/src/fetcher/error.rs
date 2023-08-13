use anyhow::Error as AnyhowError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetcherError {
    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),
    #[error("fetcher contract with error: {0}")]
    FetchContractResultError(String),
}
