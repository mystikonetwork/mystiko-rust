extern crate mystiko_protocol;

use mystiko_protocol::error::CryptoError;

#[tokio::test]
async fn test_error() {
    let err = CryptoError::AesCbcDecryptError;
    assert_eq!(err, CryptoError::AesCbcDecryptError);
}
