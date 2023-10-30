use mystiko_protos::error::ProtosError;
use num_bigint::ParseBigIntError;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("wrong column type: actual {0:?} vs expected {1:?}")]
    WrongColumnTypeError(String, String),
    #[error("column {0:?} has wrong value type, expected: {1:?} vs actual: {2:?}")]
    WrongColumnValueTypeError(String, String, String),
    #[error("no such column {0:?}")]
    NoSuchColumnError(String),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("missing required column: {0:?})")]
    MissingRequiredColumnError(String),
    #[error("set null to required column: {0:?})")]
    SetNullToRequiredColumnError(String),
    #[error("failed to execute query on database: {0:?}")]
    DatabaseError(#[source] anyhow::Error),
    #[error("data is corrupted: {0:?}")]
    CorruptedDataError(String),
    #[error("missing required data: {0:?}")]
    MissingDataError(String),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error(transparent)]
    ParseBigIntError(#[from] ParseBigIntError),
    #[error(transparent)]
    ProtosError(#[from] ProtosError),
    #[error("unsupported column type: {0:?}")]
    UnsupportedColumnTypeError(String),
    #[error("unsupported operator: {0:?}")]
    UnsupportedOperator(String),
}
