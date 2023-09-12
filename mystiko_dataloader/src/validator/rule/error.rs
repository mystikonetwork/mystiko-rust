use crate::validator::rule::checker::RuleCheckError;
use crate::validator::rule::merger::DataMergeError;
use thiserror::Error;

pub type RuleValidatorResult<T> = anyhow::Result<T, RuleValidatorError>;

#[derive(Error, Debug)]
pub enum RuleValidatorError {
    #[error("rule validator checker type {0} error")]
    RuleValidatorCheckerTypeError(i32),
    #[error("invalid validate concurrency error")]
    InvalidValidateConcurrencyError,
    #[error("data to be validated is empty error")]
    EmptyValidateDataError,
    #[error("chain {0} not found error")]
    ChainNotFoundError(u64),
    #[error(transparent)]
    DataMergeError(#[from] DataMergeError),
    #[error(transparent)]
    RuleCheckError(#[from] RuleCheckError),
}
