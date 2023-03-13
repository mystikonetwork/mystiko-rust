extern crate mystiko_crypto;

use mystiko_crypto::error::ZkpError;
use mystiko_crypto::utils::{create_file_reader, load_file};
use mystiko_crypto::zkp::prove::{prove, prove_by_file};
use mystiko_crypto::zkp::types::{json_to_zks_proof, ZKProof};
use mystiko_crypto::zkp::verify::{verify, verify_by_file};
use zokrates_field::Bn128Field;
use zokrates_proof_systems::TaggedProof;
use zokrates_proof_systems::{Proof as ZksProof, G16};

type ZokTaggedProof = TaggedProof<Bn128Field, G16>;
type ZokratesSystemProof = ZksProof<Bn128Field, G16>;

fn zks_proof_to_json(proof: ZokratesSystemProof) -> String {
    serde_json::to_string_pretty(&ZokTaggedProof::new(proof.proof, proof.inputs)).unwrap()
}

#[tokio::test]
async fn test_zks_serialize() {
    let proof = create_file_reader("./tests/files/zkp/proof.zokrates.json").unwrap();
    let proof: serde_json::Value = serde_json::from_reader(proof).unwrap();
    let proof_zk = json_to_zks_proof(proof.to_string()).unwrap();
    zks_proof_to_json(proof_zk);
}

#[tokio::test]
async fn test_prove_and_verify() {
    let arr = ("1", "0", "1");
    let args = serde_json::to_string(&arr).unwrap();

    let prog = load_file("./tests/files/zkp/program").unwrap();
    let abi_spec = load_file("./tests/files/zkp/abi.json").unwrap();
    let pk = load_file("./tests/files/zkp/proving.key").unwrap();
    let vk = load_file("./tests/files/zkp/verification.key").unwrap();

    let proof = prove(prog.as_slice(), abi_spec.as_slice(), pk.as_slice(), &args).unwrap();
    let result = verify(proof, vk.as_slice()).unwrap();
    assert!(result);
}

#[tokio::test]
async fn test_prove_and_verify_by_file() {
    let arr = ("3", "2", "5");
    let args = serde_json::to_string(&arr).unwrap();
    let proof = prove_by_file(
        "./tests/files/zkp/program",
        "./tests/files/zkp/abi.json",
        "./tests/files/zkp/proving.key",
        &args,
    )
    .unwrap();

    let result = verify_by_file(proof.clone(), "./tests/files/zkp/verification.key").unwrap();
    assert!(result);

    let mut modify_proof = proof.clone();
    modify_proof.inputs[2] =
        "0x0000000000000000000000000000000000000000000000000000000000000004".to_string();
    let result = verify_by_file(modify_proof, "./tests/files/zkp/verification.key").unwrap();
    assert!(!result);
}

#[tokio::test]
async fn test_prove_by_file() {
    let arr = ("1", "0", "1");
    let args = serde_json::to_string(&arr).unwrap();
    let witness = prove_by_file(
        "./xxx/program",
        "./tests/files/zkp/abi.json",
        "./tests/files/zkp/proving.key",
        &args,
    );
    assert_eq!(
        witness.err().unwrap().name(),
        ZkpError::ReadFileError(String::from(""), String::from(""))
    );

    let witness = prove_by_file(
        "./tests/files/zkp/program",
        "./xxx/abi.json",
        "./tests/files/zkp/proving.key",
        &args,
    );
    assert_eq!(
        witness.err().unwrap().name(),
        ZkpError::ReadFileError(String::from(""), String::from(""))
    );

    let witness = prove_by_file(
        "./tests/files/zkp/program",
        "./tests/files/zkp/abi.json",
        "./xxx/proving.key",
        &args,
    );
    assert_eq!(
        witness.err().unwrap().name(),
        ZkpError::ReadFileError(String::from(""), String::from(""))
    );

    let witness = prove_by_file(
        "./tests/files/zkp/wrong/program",
        "./tests/files/zkp/abi.json",
        "./tests/files/zkp/proving.key",
        &args,
    );
    assert_eq!(
        witness.err().unwrap().name(),
        ZkpError::DeserializeProgramError(String::from(""))
    );
}

#[tokio::test]
async fn test_prove() {
    let arr = ("1", "0", "1");
    let args = serde_json::to_string(&arr).unwrap();
    let prog = load_file("./tests/files/zkp/wrong/program").unwrap();
    let abi_spec = load_file("./tests/files/zkp/abi.json").unwrap();
    let pk = load_file("./tests/files/zkp/proving.key").unwrap();

    let witness = prove(prog.as_slice(), abi_spec.as_slice(), pk.as_slice(), &args);
    assert_eq!(
        witness.err().unwrap().name(),
        ZkpError::DeserializeProgramError(String::from(""))
    );
}

#[tokio::test]
async fn test_verify_by_file() {
    let proof = create_file_reader("./tests/files/zkp/proof.json").unwrap();
    let proof: serde_json::Value = serde_json::from_reader(proof).unwrap();
    let proof = ZKProof::from_json_string(proof.to_string()).unwrap();
    let result = verify_by_file(
        proof.clone(),
        "./tests/files/zkp/wrong/verification_error.key",
    );
    assert_eq!(
        result.err().unwrap().name(),
        ZkpError::ParseError(String::from(""), String::from(""))
    );

    let result = verify_by_file(
        proof.clone(),
        "./tests/files/zkp/wrong/verification_missing_curve.key",
    );
    assert_eq!(
        result.err().unwrap().name(),
        ZkpError::VKError(String::from(""))
    );

    let result = verify_by_file(
        proof.clone(),
        "./tests/files/zkp/wrong/verification_missing_scheme.key",
    );
    assert_eq!(
        result.err().unwrap().name(),
        ZkpError::VKError(String::from(""))
    );

    let result = verify_by_file(
        proof.clone(),
        "./tests/files/zkp/wrong/verification_gm17.key",
    );
    assert_eq!(
        result.err().unwrap().name(),
        ZkpError::MismatchError(String::from(""))
    );

    let result = verify_by_file(
        proof.clone(),
        "./tests/files/zkp/wrong/verification_bls12_381.key",
    );
    assert_eq!(
        result.err().unwrap().name(),
        ZkpError::MismatchError(String::from(""))
    );
}

#[tokio::test]
async fn test_verify() {
    let proof = create_file_reader("./tests/files/zkp/proof.json").unwrap();
    let proof: serde_json::Value = serde_json::from_reader(proof).unwrap();
    let proof = ZKProof::from_json_string(proof.to_string()).unwrap();
    let vk = load_file("./tests/files/zkp/wrong/verification_error.key").unwrap();

    let result = verify(proof, vk.as_slice());
    assert_eq!(
        result.err().unwrap().name(),
        ZkpError::ParseError(String::from(""), String::from(""))
    );
}
