extern crate mystiko_crypto;
extern crate zokrates_proof_systems;

use mystiko_crypto::error::ZkpError;
use mystiko_crypto::zkp::proof::ZKProof;
use mystiko_fs::read_file_bytes;
//
// type ZokTaggedProof = TaggedProof<Bn128Field, G16>;
// type ZokratesSystemProof = ZksProof<Bn128Field, G16>;
//
// fn zks_proof_to_json(proof: ZokratesSystemProof) -> String {
//     serde_json::to_string_pretty(&ZokTaggedProof::new(proof.proof, proof.inputs)).unwrap()
// }

#[tokio::test]
async fn test_proof() {
    let proof = read_file_bytes("./tests/files/zkp/proof.json").await.unwrap();
    let proof: serde_json::Value = serde_json::from_reader(proof.as_slice()).unwrap();
    let proof = ZKProof::from_json_string(&proof.to_string()).unwrap();

    let proof_str = proof.to_json_string();
    let proof2 = ZKProof::from_json_string(&proof_str).unwrap();
    let tag = proof2.to_tagged_proof();
    let proof3 = ZKProof::from_tagged_proof(&tag);
    assert_eq!(proof, proof2);
    assert_eq!(proof, proof3);
}

#[tokio::test]
async fn test_prove_and_verify() {
    let arr = ("1", "0", "1");
    let args = serde_json::to_string(&arr).unwrap();

    let prog = read_file_bytes("./tests/files/zkp/program").await.unwrap();
    let abi_spec = read_file_bytes("./tests/files/zkp/abi.json").await.unwrap();
    let pk = read_file_bytes("./tests/files/zkp/proving.key").await.unwrap();
    let vk = read_file_bytes("./tests/files/zkp/verification.key").await.unwrap();

    let proof = ZKProof::generate(prog.as_slice(), abi_spec.as_slice(), pk.as_slice(), &args).unwrap();
    let result = proof.verify(vk.as_slice()).unwrap();
    assert!(result);
}

#[tokio::test]
async fn test_from_json_string_error() {
    let proof = ZKProof::from_json_string("test");
    assert!(matches!(proof.err().unwrap(), ZkpError::SerdeJsonError(_)));

    let arr = ("1", "0", "1");
    let args = serde_json::to_string(&arr).unwrap();
    let proof = ZKProof::from_json_string(&args);
    assert!(matches!(proof.err().unwrap(), ZkpError::ProofError(_)));
}

#[tokio::test]
async fn test_prove_error() {
    let arr = ("1", "0", "1");
    let args = serde_json::to_string(&arr).unwrap();
    let prog = read_file_bytes("./tests/files/zkp/wrong/program").await.unwrap();
    let abi_spec = read_file_bytes("./tests/files/zkp/abi.json").await.unwrap();
    let pk = read_file_bytes("./tests/files/zkp/proving.key").await.unwrap();

    let proof = ZKProof::generate(prog.as_slice(), abi_spec.as_slice(), pk.as_slice(), &args);
    assert!(matches!(proof.err().unwrap(), ZkpError::DeserializeProgramError(_)));

    let arr = ("1", "0");
    let args = serde_json::to_string(&arr).unwrap();
    let prog = read_file_bytes("./tests/files/zkp/program").await.unwrap();
    let abi_spec = read_file_bytes("./tests/files/zkp/abi.json").await.unwrap();
    let pk = read_file_bytes("./tests/files/zkp/proving.key").await.unwrap();
    let proof = ZKProof::generate(prog.as_slice(), abi_spec.as_slice(), pk.as_slice(), &args);
    assert!(matches!(proof.err().unwrap(), ZkpError::AbiParseError(_)));
}

#[tokio::test]
async fn test_verify_error() {
    let proof = read_file_bytes("./tests/files/zkp/proof.json").await.unwrap();
    let proof: serde_json::Value = serde_json::from_reader(proof.as_slice()).unwrap();
    let proof = ZKProof::from_json_string(&proof.to_string()).unwrap();
    let vk = read_file_bytes("./tests/files/zkp/wrong/verification_error.key")
        .await
        .unwrap();

    let result = proof.verify(vk.as_slice());
    assert!(matches!(result.err().unwrap(), ZkpError::SerdeJsonError(_)));

    let vk = read_file_bytes("./tests/files/zkp/wrong/verification_missing_curve.key")
        .await
        .unwrap();

    let result = proof.verify(vk.as_slice());
    assert!(matches!(result.err().unwrap(), ZkpError::VKError(_)));

    let vk = read_file_bytes("./tests/files/zkp/wrong/verification_missing_scheme.key")
        .await
        .unwrap();

    let result = proof.verify(vk.as_slice());
    assert!(matches!(result.err().unwrap(), ZkpError::VKError(_)));

    let vk = read_file_bytes("./tests/files/zkp/wrong/verification_bls12_381.key")
        .await
        .unwrap();

    let result = proof.verify(vk.as_slice());
    assert!(matches!(result.err().unwrap(), ZkpError::MismatchError(_)));

    let vk = read_file_bytes("./tests/files/zkp/wrong/verification_gm17.key")
        .await
        .unwrap();

    let result = proof.verify(vk.as_slice());
    assert!(matches!(result.err().unwrap(), ZkpError::MismatchError(_)));
}
