use tokio::task::JoinError;

#[derive(thiserror::Error, Debug)]
pub enum SchedulerError {
    #[error("failed to send sleep interrupt")]
    SendSleepInterruptError,
    #[error("failed to send start signal")]
    SendStartSignalError,
    #[error("failed to send stop signal")]
    SendStopSignalError,
    #[cfg(feature = "status")]
    #[error("failed to send status server's start signal")]
    SendStatusServerStartSignalError,
    #[cfg(feature = "status")]
    #[error("failed to send status server's stop signal")]
    SendStatusServerStopSignalError,
    #[error(transparent)]
    JoinError(#[from] JoinError),
    #[cfg(feature = "signal")]
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[cfg(feature = "status")]
    #[error(transparent)]
    AddrParseError(#[from] std::net::AddrParseError),
    #[cfg(feature = "status")]
    #[error(transparent)]
    HttpError(#[from] http::Error),
    #[cfg(feature = "status")]
    #[error(transparent)]
    HyperError(#[from] hyper::Error),
}
