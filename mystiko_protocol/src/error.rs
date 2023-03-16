use mystiko_crypto::error::{ECCryptoError, MerkleTreeError, SecretShareError, ZkpError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("parameter error")]
    ParameterError,
    #[error(transparent)]
    ECCryptoError(#[from] ECCryptoError),
    #[error(transparent)]
    SecretShareError(#[from] SecretShareError),
    #[error(transparent)]
    ZkpError(#[from] ZkpError),
    #[error(transparent)]
    MerkleTreeError(#[from] MerkleTreeError),
}

impl PartialEq for ProtocolError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ParameterError, Self::ParameterError) => true,
            (Self::ECCryptoError(l), Self::ECCryptoError(r)) => l == r,
            (Self::SecretShareError(l), Self::SecretShareError(r)) => l == r,
            (Self::ZkpError(l), Self::ZkpError(r)) => l == r,
            (Self::MerkleTreeError(l), Self::MerkleTreeError(r)) => l == r,
            _ => false,
        }
    }
}
