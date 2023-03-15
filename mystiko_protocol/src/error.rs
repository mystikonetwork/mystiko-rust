use mystiko_crypto::error::ECCryptoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("parameter error")]
    ParameterError,
    #[error(transparent)]
    ECCryptoError(#[from] ECCryptoError),
}

impl PartialEq for ProtocolError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ParameterError, Self::ParameterError) => true,
            (Self::ECCryptoError(l), Self::ECCryptoError(r)) => l == r,
            _ => false,
        }
    }
}
