extern crate mystiko_crypto;

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

//
// #[tokio::test]
// async fn test_zks_serialize() {
//     let proof = read_file_bytes("./tests/files/zkp/proof.zokrates.json")
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

    let prog = read_file_bytes("./tests/files/zkp/program").await.unwrap();
    let abi_spec = read_file_bytes("./tests/files/zkp/abi.json").await.unwrap();
    let pk = read_file_bytes("./tests/files/zkp/proving.key")
        .await
        .unwrap();
    let vk = read_file_bytes("./tests/files/zkp/verification.key")
        .await
        .unwrap();

    let proof =
        ZKProof::generate(prog.as_slice(), abi_spec.as_slice(), pk.as_slice(), &args).unwrap();
    let result = proof.verify(vk.as_slice()).unwrap();
    assert!(result);

    let _ = proof.to_json_string();
}

#[tokio::test]
async fn test_prove() {
    let arr = ("1", "0", "1");
    let args = serde_json::to_string(&arr).unwrap();
    let prog = read_file_bytes("./tests/files/zkp/wrong/program")
        .await
        .unwrap();
    let abi_spec = read_file_bytes("./tests/files/zkp/abi.json").await.unwrap();
    let pk = read_file_bytes("./tests/files/zkp/proving.key")
        .await
        .unwrap();

    let proof = ZKProof::generate(prog.as_slice(), abi_spec.as_slice(), pk.as_slice(), &args);
    assert_eq!(
        proof.err().unwrap(),
        ZkpError::DeserializeProgramError(String::from(""))
    );
}

#[tokio::test]
async fn test_verify() {
    let proof = read_file_bytes("./tests/files/zkp/proof.json")
        .await
        .unwrap();
    let proof: serde_json::Value = serde_json::from_reader(proof.as_slice()).unwrap();
    let proof = ZKProof::from_json_string(&proof.to_string()).unwrap();
    let vk = read_file_bytes("./tests/files/zkp/wrong/verification_error.key")
        .await
        .unwrap();

    let result = proof.verify(vk.as_slice());
    assert_eq!(
        result.err().unwrap(),
        ZkpError::SerdeJsonError(String::from(""), String::from(""))
    );
}
