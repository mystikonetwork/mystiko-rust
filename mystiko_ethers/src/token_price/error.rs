use serde_json::Error as SerdeJsonError;
use thiserror::Error;

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

impl PartialEq for TokenPriceError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::FileError(_), Self::FileError(_))
                | (Self::SerdeJsonError(_), Self::SerdeJsonError(_))
                | (Self::ApiKeyNotConfigure, Self::ApiKeyNotConfigure)
                | (Self::TokenNotSupport, Self::TokenNotSupport)
                | (Self::ReqwestError(_), Self::ReqwestError(_))
                | (Self::ResponseError(_), Self::ResponseError(_))
                | (Self::InternalError, Self::InternalError)
        )
    }
}
