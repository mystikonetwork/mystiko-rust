extern crate mystiko_crypto;

use mystiko_crypto::error::{CryptoError, MerkleTreeError, SecretShareError, ZkpError};

#[tokio::test]
async fn test_error() {
    let merkle_err = MerkleTreeError::MerkleTreeIsFull;
    assert_ne!(merkle_err, MerkleTreeError::IndexOutOfBounds);

    let share_err = SecretShareError::ThresholdOutOfBounds;
    assert_ne!(share_err, SecretShareError::SharesOutOfBounds);

    let zkp_err = ZkpError::ComputeWitnessError(String::from(""));
    assert_ne!(zkp_err, ZkpError::NotSupport);
    let zkp_err = ZkpError::ProofError(String::from(""));
    assert_ne!(zkp_err, ZkpError::NotSupport);
    let zkp_err = ZkpError::NotSupport;
    assert_eq!(zkp_err, ZkpError::NotSupport);

    let ec_err = CryptoError::DataLengthError;
    assert_ne!(ec_err, CryptoError::InternalError);
}
