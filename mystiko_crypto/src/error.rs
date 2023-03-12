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

#[derive(Error, Debug, PartialEq)]
pub enum ZkpError {
    #[error("read {0} error {1}")]
    ReadFileError(String, String),
    #[error("parse {0} error {1}")]
    ParseError(String, String),
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

impl ZkpError {
    pub fn name(&self) -> ZkpError {
        let empty = String::from("");
        match self {
            ZkpError::ReadFileError(_, _) => ZkpError::ReadFileError(empty.clone(), empty),
            ZkpError::ParseError(_, _) => ZkpError::ParseError(empty.clone(), empty),
            ZkpError::DeserializeProgramError(_) => ZkpError::DeserializeProgramError(empty),
            ZkpError::ComputeWitnessError(_) => ZkpError::ComputeWitnessError(empty),
            ZkpError::ProofError(_) => ZkpError::ProofError(empty),
            ZkpError::VKError(_) => ZkpError::VKError(empty),
            ZkpError::MismatchError(_) => ZkpError::MismatchError(empty),
            ZkpError::NotSupport => ZkpError::NotSupport,
        }
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum ECCryptoError {
    #[error("eccrypto data length error")]
    ECCryptoDataLengthError,
    #[error("eccrypto mac mismatch error")]
    ECCryptoMacMismatchError,
    #[error("internal error")]
    InternalError,
}
