use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ProtocolError {
    #[error("aes cbc decrypt error")]
    AesCbcDecryptError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let protocol_err = ProtocolError::AesCbcDecryptError;
        assert_eq!(protocol_err, ProtocolError::AesCbcDecryptError);
    }
}
