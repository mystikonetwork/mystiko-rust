extern crate mystiko_crypto;

use mystiko_crypto::error::{FileError, ZkpError};
use mystiko_crypto::file::load_file;
use mystiko_crypto::zkp::proof::ZKProof;
//
// type ZokTaggedProof = TaggedProof<Bn128Field, G16>;
// type ZokratesSystemProof = ZksProof<Bn128Field, G16>;
//
// fn zks_proof_to_json(proof: ZokratesSystemProof) -> String {
//     serde_json::to_string_pretty(&ZokTaggedProof::new(proof.proof, proof.inputs)).unwrap()
// }

//
// #[tokio::test]
// async fn test_zks_serialize() {
//     let proof = load_file("./tests/files/zkp/proof.zokrates.json")
//         .await
//         .unwrap();
//     let proof: serde_json::Value = serde_json::from_slice(&proof).unwrap();
//     let proof_zk = json_to_zks_proof(proof.to_string()).unwrap();
//     zks_proof_to_json(proof_zk);
// }

#[tokio::test]
async fn test_prove_and_verify() {
    let arr = ("1", "0", "1");
    let args = serde_json::to_string(&arr).unwrap();

    let prog = load_file("./tests/files/zkp/program").await.unwrap();
    let abi_spec = load_file("./tests/files/zkp/abi.json").await.unwrap();
    let pk = load_file("./tests/files/zkp/proving.key").await.unwrap();
    let vk = load_file("./tests/files/zkp/verification.key")
        .await
        .unwrap();

    let proof =
        ZKProof::generate(prog.as_slice(), abi_spec.as_slice(), pk.as_slice(), &args).unwrap();
    let result = proof.verify(vk.as_slice()).unwrap();
    assert!(result);

    let _ = proof.to_json_string();
}

#[tokio::test]
async fn test_prove_and_verify_by_file() {
    let arr = ("3", "2", "5");
    let args = serde_json::to_string(&arr).unwrap();
    let proof = ZKProof::generate_with_file(
        "./tests/files/zkp/program",
        "./tests/files/zkp/abi.json",
        "./tests/files/zkp/proving.key",
        &args,
    )
    .await
    .unwrap();

    let result = proof
        .verify_with_file("./tests/files/zkp/verification.key")
        .await
        .unwrap();
    assert!(result);

    let mut modify_proof = proof.clone();
    modify_proof.inputs[2] =
        "0x0000000000000000000000000000000000000000000000000000000000000004".to_string();
    let result = modify_proof
        .verify_with_file("./tests/files/zkp/verification.key")
        .await
        .unwrap();
    assert!(!result);
}

#[tokio::test]
async fn test_prove_by_file() {
    let arr = ("1", "0", "1");
    let args = serde_json::to_string(&arr).unwrap();
    let proof = ZKProof::generate_with_file(
        "./xxx/program",
        "./tests/files/zkp/abi.json",
        "./tests/files/zkp/proving.key",
        &args,
    )
    .await;
    let expect_err =
        ZkpError::FileError(FileError::OpenFileError(String::from(""), String::from("")));
    assert_eq!(proof.err().unwrap(), expect_err);

    let proof = ZKProof::generate_with_file(
        "./tests/files/zkp/program",
        "./xxx/abi.json",
        "./tests/files/zkp/proving.key",
        &args,
    )
    .await;
    assert_eq!(proof.err().unwrap(), expect_err);

    let proof = ZKProof::generate_with_file(
        "./tests/files/zkp/program",
        "./tests/files/zkp/abi.json",
        "./xxx/proving.key",
        &args,
    )
    .await;
    assert_eq!(proof.err().unwrap(), expect_err);

    let proof = ZKProof::generate_with_file(
        "./tests/files/zkp/wrong/program",
        "./tests/files/zkp/abi.json",
        "./tests/files/zkp/proving.key",
        &args,
    )
    .await;
    assert_eq!(
        proof.err().unwrap(),
        ZkpError::DeserializeProgramError(String::from(""))
    );
}

#[tokio::test]
async fn test_prove() {
    let arr = ("1", "0", "1");
    let args = serde_json::to_string(&arr).unwrap();
    let prog = load_file("./tests/files/zkp/wrong/program").await.unwrap();
    let abi_spec = load_file("./tests/files/zkp/abi.json").await.unwrap();
    let pk = load_file("./tests/files/zkp/proving.key").await.unwrap();

    let proof = ZKProof::generate(prog.as_slice(), abi_spec.as_slice(), pk.as_slice(), &args);
    assert_eq!(
        proof.err().unwrap(),
        ZkpError::DeserializeProgramError(String::from(""))
    );
}

#[tokio::test]
async fn test_verify_by_file() {
    let proof = load_file("./tests/files/zkp/proof.json").await.unwrap();
    let proof: serde_json::Value = serde_json::from_slice(&proof).unwrap();
    let proof = ZKProof::from_json_string(&proof.to_string()).unwrap();
    let result = proof
        .verify_with_file("./tests/files/zkp/wrong/verification_error.key")
        .await;
    assert_eq!(
        result.err().unwrap(),
        ZkpError::SerdeJsonError(String::from(""), String::from(""))
    );

    let result = proof
        .verify_with_file("./tests/files/zkp/wrong/verification_missing_curve.key")
        .await;
    assert_eq!(result.err().unwrap(), ZkpError::VKError(String::from("")));

    let result = proof
        .verify_with_file("./tests/files/zkp/wrong/verification_missing_scheme.key")
        .await;
    assert_eq!(result.err().unwrap(), ZkpError::VKError(String::from("")));

    let result = proof
        .verify_with_file("./tests/files/zkp/wrong/verification_gm17.key")
        .await;
    assert_eq!(
        result.err().unwrap(),
        ZkpError::MismatchError(String::from(""))
    );

    let result = proof
        .verify_with_file("./tests/files/zkp/wrong/verification_bls12_381.key")
        .await;
    assert_eq!(
        result.err().unwrap(),
        ZkpError::MismatchError(String::from(""))
    );
}

#[tokio::test]
async fn test_verify() {
    let proof = load_file("./tests/files/zkp/proof.json").await.unwrap();
    let proof: serde_json::Value = serde_json::from_reader(proof.as_slice()).unwrap();
    let proof = ZKProof::from_json_string(&proof.to_string()).unwrap();
    let vk = load_file("./tests/files/zkp/wrong/verification_error.key")
        .await
        .unwrap();

    let result = proof.verify(vk.as_slice());
    assert_eq!(
        result.err().unwrap(),
        ZkpError::SerdeJsonError(String::from(""), String::from(""))
    );
}
