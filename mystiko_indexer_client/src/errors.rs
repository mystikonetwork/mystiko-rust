use reqwest;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("api response with exception (result_code: {code:?}, err_message: {message:?})")]
    ApiResponseError { code: i32, message: String },

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error("response content-type is not supported: {0}")]
    UnsupportedContentTypeError(String),

    #[error("custom error: {0}")]
    CustomError(String),
}

impl PartialEq for ClientError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ReqwestError(_), Self::ReqwestError(_)) => true,
            (
                Self::ApiResponseError {
                    code: l_code,
                    message: _,
                },
                Self::ApiResponseError {
                    code: r_code,
                    message: _,
                },
            ) => l_code == r_code,
            (Self::UnsupportedContentTypeError(l), Self::UnsupportedContentTypeError(r)) => l == r,
            (Self::CustomError(l), Self::CustomError(r)) => l == r,
            _ => false,
        }
    }
}
