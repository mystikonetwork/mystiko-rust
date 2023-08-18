use crate::validator::rule::checker::RuleCheckError;
use crate::validator::rule::merger::DataMergeError;
use thiserror::Error;

pub type Result<T> = anyhow::Result<T, RuleValidatorError>;

#[derive(Error, Debug)]
pub enum RuleValidatorError {
    #[error(transparent)]
    DataMergeError(#[from] DataMergeError),
    #[error(transparent)]
    RuleCheckError(#[from] RuleCheckError),
}
