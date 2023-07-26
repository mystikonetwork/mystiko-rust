use thiserror::Error;

#[derive(Error, Debug)]
pub enum EtherFetcherError {
    #[error("custom error err_message: {0}")]
    CustomError(String),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

impl PartialEq for EtherFetcherError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ReqwestError(_), Self::ReqwestError(_)) => true,
            (Self::CustomError(l), Self::CustomError(r)) => l == r,
            _ => false,
        }
    }
}
