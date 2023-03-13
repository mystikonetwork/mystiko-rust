extern crate mystiko_crypto;

use mystiko_crypto::error::{ECCryptoError, MerkleTreeError, SecretShareError, ZkpError};

#[tokio::test]
async fn test_error() {
    let merkle_err = MerkleTreeError::MerkleTreeIsFull;
    assert_ne!(merkle_err, MerkleTreeError::IndexOutOfBounds);

    let share_err = SecretShareError::ThresholdOutOfBounds;
    assert_ne!(share_err, SecretShareError::SharesOutOfBounds);

    let zkp_err = ZkpError::ReadFileError(String::from(""), String::from(""));
    assert_ne!(zkp_err.name(), ZkpError::NotSupport);

    let ec_err = ECCryptoError::ECCryptoDataLengthError;
    assert_ne!(ec_err, ECCryptoError::InternalError);
}
