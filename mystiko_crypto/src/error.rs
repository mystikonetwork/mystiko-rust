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

#[derive(Error, Debug, Clone)]
pub enum FileError {
    #[error("read {0} error {1}")]
    OpenFileError(String, String),
    #[error("read {0} error {1}")]
    ReadFileError(String, String),
    #[error("internal error")]
    InternalError,
}

impl PartialEq for FileError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::OpenFileError(_, _), Self::OpenFileError(_, _))
                | (Self::ReadFileError(_, _), Self::ReadFileError(_, _))
                | (Self::InternalError, Self::InternalError)
        )
    }
}

#[derive(Error, Debug, Clone)]
pub enum ZkpError {
    #[error(transparent)]
    FileError(#[from] FileError),
    #[error("serde json {0} error {1}")]
    SerdeJsonError(String, String),
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
        match (self, other) {
            (Self::FileError(l), Self::FileError(r)) => l == r,
            (Self::SerdeJsonError(_, _), Self::SerdeJsonError(_, _)) => true,
            (Self::AbiParseError(_), Self::AbiParseError(_)) => true,
            (Self::DeserializeProgramError(_), Self::DeserializeProgramError(_)) => true,
            (Self::ComputeWitnessError(_), Self::ComputeWitnessError(_)) => true,
            (Self::ProofError(_), Self::ProofError(_)) => true,
            (Self::VKError(_), Self::VKError(_)) => true,
            (Self::MismatchError(_), Self::MismatchError(_)) => true,
            (Self::NotSupport, Self::NotSupport) => true,
            _ => false,
        }
    }
}

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("data length error")]
    DataLengthError,
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
                | (Self::MacMismatchError, Self::MacMismatchError)
                | (Self::DecryptError(_), Self::DecryptError(_))
                | (Self::InternalError, Self::InternalError)
        )
    }
}
