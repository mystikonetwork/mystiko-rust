use thiserror::Error;

#[derive(Error, Debug)]
pub enum MerkleTreeError {
    #[error("merkle tree is full")]
    MerkleTreeIsFull,
    #[error("index out of bounds")]
    IndexOutOfBounds,
    #[error("unknown error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum SecretShareError {
    #[error("num of shares out of range")]
    SharesOutOfBounds,
    #[error("threshold out of range")]
    ThresholdOutOfBounds,
}

#[derive(Error, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let merkle_err = MerkleTreeError::Unknown;
        format!("{:?}", merkle_err);

        let share_err = SecretShareError::ThresholdOutOfBounds;
        format!("{:?}", share_err);

        let zkp_err = ZkpError::NotSupport;
        format!("{:?}", zkp_err);
    }
}
