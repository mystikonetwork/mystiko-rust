extern crate mystiko_crypto;
extern crate mystiko_protocol;

use mystiko_crypto::error::ECCryptoError;
use mystiko_protocol::error::ProtocolError;

#[tokio::test]
async fn test_error() {
    let err = ProtocolError::ECCryptoError(ECCryptoError::InternalError);
    assert_ne!(err, ProtocolError::ParameterError);
}
