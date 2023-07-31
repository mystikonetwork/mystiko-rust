use anyhow::Error as AnyhowError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataLoaderError {
    #[error("loader init error {0}")]
    LoaderInitError(String),
    #[error("loader run error {0}")]
    LoaderRunError(String),
    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),
}
