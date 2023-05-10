use mystiko_storage::error::StorageError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error(transparent)]
    StorageError(#[from] StorageError),
}
