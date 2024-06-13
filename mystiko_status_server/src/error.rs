use tokio::task::JoinError;

#[derive(thiserror::Error, Debug)]
pub enum StatusServerError {
    #[error("failed to send status server's start signal")]
    SendStatusServerStartSignalError,
    #[error("failed to send status server's stop signal")]
    SendStatusServerStopSignalError,
    #[error(transparent)]
    AddrParseError(#[from] std::net::AddrParseError),
    #[error(transparent)]
    HttpError(#[from] http::Error),
    #[error(transparent)]
    HyperError(#[from] hyper::Error),
    #[error(transparent)]
    JoinError(#[from] JoinError),
}
