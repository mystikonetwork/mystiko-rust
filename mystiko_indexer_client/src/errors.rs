use reqwest;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("api response with exception (result_code: {code:?}, err_message: {message:?})")]
    ApiResponseError { code: i32, message: String },

    #[error("http response error (response_code: {code:?}, message: {message:?})")]
    HttpResponseError { code: u16, message: String },

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error("custom error: {0}")]
    CustomError(String),
}

impl PartialEq for ClientError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ReqwestError(_), Self::ReqwestError(_)) => true,
            (
                Self::HttpResponseError {
                    code: l_code,
                    message: _,
                },
                Self::HttpResponseError {
                    code: r_code,
                    message: _,
                },
            ) => l_code == r_code,
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
            (Self::CustomError(l), Self::CustomError(r)) => l == r,
            _ => false,
        }
    }
}
