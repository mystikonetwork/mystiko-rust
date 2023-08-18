use crate::handler::HandlerError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataMergeError {
    #[error("start block error")]
    StartBlockError,
    #[error("target block error expected {0} actual {1}")]
    TargetBlockError(u64, u64),
    #[error("commitment data len error expected {0} actual {1}")]
    CommitmentDataLenError(usize, usize),
    #[error("commitment hash mismatch error")]
    CommitmentHashMismatchError,
    #[error("commitment hash collision error")]
    CommitmentHashCollisionError,
    #[error("leaf index is none error")]
    LeafIndexIsNoneError,
    #[error("leaf index mismatch error one is {0} another is {1}")]
    LeafIndexMismatchError(u64, u64),
    #[error(transparent)]
    HandlerError(#[from] HandlerError),
}
