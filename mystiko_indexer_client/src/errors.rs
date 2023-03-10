use reqwest;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Api response with exception (result_code: {code:?}, err_message: {message:?})")]
    ApiResponseError { code: i32, message: String },

    #[error("Http response error (response_code: {code:?}, message: {message:?})")]
    HttpResponseError { code: u16, message: String },

    #[error("Reqwest {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("An unknown error has occurred: `{0}`")]
    UnknowError(String),
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
            (Self::UnknowError(l), Self::UnknowError(r)) => l == r,
            _ => false,
        }
    }
}
