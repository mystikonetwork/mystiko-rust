use std::io;
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Error, Debug)]
pub enum RelayerClientError {
    #[error("chainId {0} relayer config not found")]
    RelayerConfigNotFoundError(u64),
    #[error("api response with exception (result_code: {code:?}, err_message: {message:?})")]
    ApiResponseError { code: i32, message: String },
    #[error("response content-type is not supported: {0}")]
    UnsupportedContentTypeError(String),
    #[error("wait uuid {0} transaction until confirmed timeout")]
    WaitTransactionTimeout(String),
    #[error("relayer url {0} transaction uuid {1} status is failed")]
    TransactionFailed(String, String),
    #[error("call relayer contract error {0}")]
    CallRelayerContractError(String),
    #[error("create relayer config error")]
    CreateRelayerConfigError,
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    CreateRuntimeError(#[from] io::Error),
    #[error(transparent)]
    FromHexError(#[from] rustc_hex::FromHexError),
    #[error("get or create provider error {0}")]
    GetOrCreateProviderError(String),
    #[error(transparent)]
    JoinError(#[from] JoinError),
}
