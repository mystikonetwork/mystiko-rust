extern crate mystiko_crypto;
extern crate mystiko_protocol;

use mystiko_crypto::error::CryptoError;
use mystiko_protocol::error::ProtocolError;

#[tokio::test]
async fn test_error() {
    let err = ProtocolError::ECCryptoError(CryptoError::InternalError);
    assert!(!matches!(err, ProtocolError::ParameterError));
    assert!(!matches!(err, ProtocolError::InvalidShieldedAddress));
    assert!(!matches!(err, ProtocolError::InvalidNoteSize));
    assert!(!matches!(err, ProtocolError::SecretShareError(_)));
    assert!(!matches!(err, ProtocolError::ZkpError(_)));
    assert!(!matches!(err, ProtocolError::MerkleTreeError(_)));
    assert!(!matches!(err, ProtocolError::SerdeJsonError(_)));
}
