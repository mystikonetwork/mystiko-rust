use crate::handler::HandlerError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataMergeError {
    #[error("data merge start block error")]
    StartBlockError,
    #[error("data merge target block error expected {0} actual {1}")]
    TargetBlockError(u64, u64),
    #[error("data merge commitment data len error expected {0} actual {1}")]
    CommitmentDataLenError(usize, usize),
    #[error("data merge commitment hash mismatch error")]
    CommitmentHashMismatchError,
    #[error("data merge commitment hash collision error")]
    CommitmentHashCollisionError,
    #[error("data merge leaf index is none error")]
    LeafIndexIsNoneError,
    #[error("data merge leaf index mismatch error one is {0} another is {1}")]
    LeafIndexMismatchError(u64, u64),
    #[error(transparent)]
    HandlerError(#[from] HandlerError),
}
