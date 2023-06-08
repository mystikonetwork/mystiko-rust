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
