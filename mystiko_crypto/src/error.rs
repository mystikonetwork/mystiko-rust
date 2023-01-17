use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum MerkleTreeError {
    #[error("merkle tree is full")]
    MerkleTreeIsFull,
    #[error("index out of bounds")]
    IndexOutOfBounds,
    #[error("unknown error")]
    Unknown,
}

#[derive(Error, Debug, PartialEq)]
pub enum SecretShareError {
    #[error("num of shares out of range")]
    SharesOutOfBounds,
    #[error("threshold out of range")]
    ThresholdOutOfBounds,
}
