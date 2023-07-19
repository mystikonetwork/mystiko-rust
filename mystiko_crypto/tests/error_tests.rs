extern crate mystiko_crypto;

use mystiko_crypto::error::{CryptoError, MerkleTreeError, SecretShareError, ZkpError};

#[tokio::test]
async fn test_error() {
    let merkle_err = MerkleTreeError::MerkleTreeIsFull;
    assert!(!matches!(merkle_err, MerkleTreeError::IndexOutOfBounds));
    assert!(!matches!(merkle_err, MerkleTreeError::Unknown));

    let secret_share_err = SecretShareError::SharesOutOfBounds;
    assert!(!matches!(secret_share_err, SecretShareError::ThresholdOutOfBounds));

    let zkp_err = ZkpError::NotSupport;
    assert!(!matches!(zkp_err, ZkpError::SerdeJsonError(_)));
    assert!(!matches!(zkp_err, ZkpError::AbiParseError(_)));
    assert!(!matches!(zkp_err, ZkpError::DeserializeProgramError(_)));
    assert!(!matches!(zkp_err, ZkpError::ComputeWitnessError(_)));
    assert!(!matches!(zkp_err, ZkpError::ProofError(_)));
    assert!(!matches!(zkp_err, ZkpError::VKError(_)));
    assert!(!matches!(zkp_err, ZkpError::MismatchError(_)));

    let crypto_err = CryptoError::DataLengthError;
    assert!(!matches!(crypto_err, CryptoError::KeyLengthError));
    assert!(!matches!(crypto_err, CryptoError::MacMismatchError));
    assert!(!matches!(crypto_err, CryptoError::DecryptError(_)));
    assert!(!matches!(crypto_err, CryptoError::InternalError));
}
