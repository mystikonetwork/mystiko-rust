use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("missing required data: {0:?}")]
    MissingDataError(String),
    #[error("data is corrupted: {0:?}")]
    CorruptedDataError(String),
    #[error("failed to execute query on database: {0:?}")]
    DatabaseError(#[source] anyhow::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    ParseBigIntError(#[from] num_bigint::ParseBigIntError),
}
