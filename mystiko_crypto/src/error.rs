use serde_json::Error as SerdeJsonError;
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

#[derive(Error, Debug)]
pub enum ZkpError {
    #[error(transparent)]
    SerdeJsonError(#[from] SerdeJsonError),
    #[error("abi parse error {0}")]
    AbiParseError(String),
    #[error("deserialize program error {0}")]
    DeserializeProgramError(String),
    #[error("compute witness error {0}")]
    ComputeWitnessError(String),
    #[error("proof error {0}")]
    ProofError(String),
    #[error("vk error {0}")]
    VKError(String),
    #[error("mismatch error {0}")]
    MismatchError(String),
    #[error("Not support")]
    NotSupport,
}

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("data length error")]
    DataLengthError,
    #[error("key length error")]
    KeyLengthError,
    #[error("mac mismatch error")]
    MacMismatchError,
    #[error("decrypt error {0}")]
    DecryptError(String),
    #[error("internal error")]
    InternalError,
}
