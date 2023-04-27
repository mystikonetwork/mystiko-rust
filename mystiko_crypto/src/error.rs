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

impl PartialEq for ZkpError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::SerdeJsonError(_), Self::SerdeJsonError(_))
                | (Self::AbiParseError(_), Self::AbiParseError(_))
                | (Self::DeserializeProgramError(_), Self::DeserializeProgramError(_))
                | (Self::ComputeWitnessError(_), Self::ComputeWitnessError(_))
                | (Self::ProofError(_), Self::ProofError(_))
                | (Self::VKError(_), Self::VKError(_))
                | (Self::MismatchError(_), Self::MismatchError(_))
                | (Self::NotSupport, Self::NotSupport)
        )
    }
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

impl PartialEq for CryptoError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::DataLengthError, Self::DataLengthError)
                | (Self::KeyLengthError, Self::KeyLengthError)
                | (Self::MacMismatchError, Self::MacMismatchError)
                | (Self::DecryptError(_), Self::DecryptError(_))
                | (Self::InternalError, Self::InternalError)
        )
    }
}
