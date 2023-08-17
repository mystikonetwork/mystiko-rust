use crate::handler::HandlerError;
use thiserror::Error;

pub type DataMergeResult<T> = anyhow::Result<T, DataMergeError>;

#[derive(Error, Debug)]
pub enum DataMergeError {
    #[error("invalid start block")]
    InvalidStartBlock,
    #[error("target block error expected {0} actual {1}")]
    TargetBlockError(u64, u64),
    #[error("invalid commitment data len expected {0} actual {1}")]
    InvalidCommitmentDataLen(usize, usize),
    #[error("commitment hash mismatch")]
    CommitmentHashMismatch,
    #[error("commitment hash collision")]
    CommitmentHashCollision,
    #[error("leaf index is none")]
    LeafIndexIsNone,
    #[error("leaf index mismatch one is {0} another is {1}")]
    LeafIndexMismatch(u64, u64),
    #[error(transparent)]
    HandlerError(#[from] HandlerError),
}
