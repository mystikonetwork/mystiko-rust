use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("wrong column type: actual {0:?} vs expected {1:?}")]
    WrongColumnTypeError(String, String),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("missing required column: {0:?})")]
    MissingRequiredColumnError(String),
}
