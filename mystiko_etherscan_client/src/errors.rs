use thiserror::Error;

#[derive(Error, Debug)]
pub enum EtherScanError {
    #[error("unknown error: {0}")]
    UnknownError(String),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error("missing current block: {0:?})")]
    MissingCurrentBlock(String),

    #[error("unexpected content-type returned: {0:?}")]
    UnexpectedContentTypeError(String),

    #[error("etherscan request failed: {0:?}")]
    ResponseError(String),

    #[error("unsupported chain id: {0:?}")]
    UnsupportedChainIdError(u64)
}