use anyhow::Error as AnyhowError;
use mystiko_storage::StorageError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error(transparent)]
    StorageError(#[from] StorageError),
    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),
}
