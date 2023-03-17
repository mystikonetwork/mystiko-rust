use mystiko_crypto::error::{CryptoError, MerkleTreeError, SecretShareError, ZkpError};
use serde_json::Error as SerdeJsonError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("parameter error")]
    ParameterError,
    #[error("invalid shielded address")]
    InvalidShieldedAddress,
    #[error("invalid note size")]
    InvalidNoteSize,
    #[error(transparent)]
    ECCryptoError(#[from] CryptoError),
    #[error(transparent)]
    SecretShareError(#[from] SecretShareError),
    #[error(transparent)]
    ZkpError(#[from] ZkpError),
    #[error(transparent)]
    MerkleTreeError(#[from] MerkleTreeError),
    #[error(transparent)]
    SerdeJsonError(#[from] SerdeJsonError),
}

impl PartialEq for ProtocolError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ParameterError, Self::ParameterError) => true,
            (Self::InvalidShieldedAddress, Self::InvalidShieldedAddress) => true,
            (Self::ECCryptoError(l), Self::ECCryptoError(r)) => l == r,
            (Self::SecretShareError(l), Self::SecretShareError(r)) => l == r,
            (Self::ZkpError(l), Self::ZkpError(r)) => l == r,
            (Self::MerkleTreeError(l), Self::MerkleTreeError(r)) => l == r,
            (Self::SerdeJsonError(_), Self::SerdeJsonError(_)) => true,
            _ => false,
        }
    }
}
