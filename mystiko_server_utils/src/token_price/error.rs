use serde_json::Error as SerdeJsonError;
use thiserror::Error;

pub type Result<T> = anyhow::Result<T, TokenPriceError>;

#[derive(Error, Debug)]
pub enum TokenPriceError {
    #[error("read file error {0}")]
    FileError(String),
    #[error(transparent)]
    SerdeJsonError(#[from] SerdeJsonError),
    #[error("env api key not configure")]
    ApiKeyNotConfigure,
    #[error("token not support")]
    TokenNotSupport,
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("server response error {0}")]
    ResponseError(u64),
    #[error("internal error")]
    InternalError,
}
