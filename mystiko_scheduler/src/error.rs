use tokio::task::JoinError;

#[derive(thiserror::Error, Debug)]
pub enum SchedulerError {
    #[error("failed to send sleep interrupt")]
    SendSleepInterruptError,
    #[error("failed to send start signal")]
    SendStartSignalError,
    #[error("failed to send stop signal")]
    SendStopSignalError,
    #[error(transparent)]
    JoinError(#[from] JoinError),
    #[cfg(feature = "signal")]
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[cfg(feature = "status")]
    #[error(transparent)]
    StatusServerError(#[from] mystiko_status_server::StatusServerError),
}
