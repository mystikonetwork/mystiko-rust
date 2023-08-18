use std::num::TryFromIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtosError {
    #[error("column value is none")]
    ColumnValueNoneError(),
    #[error("wrong column type: actual {0:?} vs expected {1:?}")]
    WrongColumnTypeError(String, String),
    #[error(transparent)]
    TryFromIntError(#[from] TryFromIntError),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
}
