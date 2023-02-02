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
    #[error("generate proof error {0}")]
    GenerateProofError(String),
    #[error("proof error {0}")]
    ProofError(String),
    #[error("vk error {0}")]
    VKError(String),
    #[error("mismatch error {0}")]
    MismatchError(String),
    #[error("verify error {0}")]
    VerifyError(String),
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
            ZkpError::GenerateProofError(_) => ZkpError::GenerateProofError(empty),
            ZkpError::ProofError(_) => ZkpError::ProofError(empty),
            ZkpError::VKError(_) => ZkpError::VKError(empty),
            ZkpError::MismatchError(_) => ZkpError::MismatchError(empty),
            ZkpError::VerifyError(_) => ZkpError::VerifyError(empty),
            ZkpError::NotSupport => ZkpError::NotSupport,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let merkle_err = MerkleTreeError::MerkleTreeIsFull;
        assert_ne!(merkle_err, MerkleTreeError::IndexOutOfBounds);

        let share_err = SecretShareError::ThresholdOutOfBounds;
        assert_ne!(share_err, SecretShareError::SharesOutOfBounds);

        let zkp_err = ZkpError::ReadFileError(String::from(""), String::from(""));
        assert_ne!(zkp_err.name(), ZkpError::NotSupport);
    }
}
