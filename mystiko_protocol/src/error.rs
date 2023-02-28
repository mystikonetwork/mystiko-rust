use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CryptoError {
    #[error("aes cbc decrypt error")]
    AesCbcDecryptError,
}

#[derive(Error, Debug, PartialEq)]
pub enum ProtocolError {
    #[error("parameter error")]
    ParameterError,
    #[error("crypto error {0}")]
    CryptoError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let protocol_err = CryptoError::AesCbcDecryptError;
        assert_eq!(protocol_err, CryptoError::AesCbcDecryptError);
    }
}
