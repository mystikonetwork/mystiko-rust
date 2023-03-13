use crate::error::ProtocolError;
use mystiko_crypto::zkp::types::ZKProof;
use mystiko_crypto::zkp::verify::verify_by_file;

pub async fn zk_verify(proof: ZKProof, verify_key_file: &str) -> Result<bool, ProtocolError> {
    verify_by_file(proof, verify_key_file)
        .await
        .map_err(|e| ProtocolError::CryptoError(e.to_string()))
}
