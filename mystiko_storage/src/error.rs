use mystiko_protos::error::ProtosError;
use num_bigint::ParseBigIntError;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("wrong column type: actual {0:?} vs expected {1:?}")]
    WrongColumnTypeError(String, String),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("missing required column: {0:?})")]
    MissingRequiredColumnError(String),
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
}
