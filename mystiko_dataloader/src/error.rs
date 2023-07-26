use anyhow::Error as AnyhowError;
use thiserror::Error;

pub type Result<T> = anyhow::Result<T, DataloaderError>;

#[derive(Error, Debug)]
pub enum DataloaderError {
    #[error("loader init error {0}")]
    LoaderInitError(String),
    #[error("loader run error {0}")]
    LoaderRunError(String),
    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),
}
