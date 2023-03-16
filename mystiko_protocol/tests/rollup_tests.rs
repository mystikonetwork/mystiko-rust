extern crate mystiko_crypto;
extern crate mystiko_protocol;
extern crate num_bigint;

use num_bigint::BigInt;

use mystiko_crypto::merkle_tree::MerkleTree;
use mystiko_protocol::rollup::Rollup;

const FILE_PATH: &str = "./../mystiko-circuits/dist/zokrates/dev";

#[tokio::test]
async fn test_rollup1() {
    let in_initial_elements = vec![BigInt::from(100), BigInt::from(200), BigInt::from(300)];
    let tree = MerkleTree::new(Some(in_initial_elements), None, None).unwrap();
    let new_leaves = vec![BigInt::from(1u32)];
    let program_path = FILE_PATH.to_owned() + "/Rollup1.program";
    let abi_path = FILE_PATH.to_owned() + "/Rollup1.abi.json";
    let pkey_path = FILE_PATH.to_owned() + "/Rollup1.pkey";

    let rollup = Rollup::new(tree, new_leaves, program_path, abi_path, pkey_path);

    let proof = rollup.prove().await.unwrap();
    let verify = proof
        .verify_with_file(&(FILE_PATH.to_owned() + "/Rollup1.vkey"))
        .await
        .unwrap();
    assert!(verify);
    let _ = rollup.clone();
}

#[tokio::test]
#[ignore]
async fn test_rollup2() {
    let in_initial_elements = vec![BigInt::from(100), BigInt::from(200)];
    let tree = MerkleTree::new(Some(in_initial_elements), None, None).unwrap();
    let new_leaves = vec![BigInt::from(1u32), BigInt::from(2u32)];
    let program_path = FILE_PATH.to_owned() + "/Rollup2.program";
    let abi_path = FILE_PATH.to_owned() + "/Rollup2.abi.json";
    let pkey_path = FILE_PATH.to_owned() + "/Rollup2.pkey";

    let rollup = Rollup::new(tree, new_leaves, program_path, abi_path, pkey_path);
    let proof = rollup.prove().await.unwrap();
    let verify = proof
        .verify_with_file(&(FILE_PATH.to_owned() + "/Rollup2.vkey"))
        .await
        .unwrap();
    assert!(verify);
}

#[tokio::test]
#[ignore]
async fn test_rollup4() {
    let in_initial_elements = (100..=400)
        .step_by(100)
        .map(BigInt::from)
        .collect::<Vec<BigInt>>();
    let new_leaves = (1..=4).map(BigInt::from).collect::<Vec<BigInt>>();
    let tree = MerkleTree::new(Some(in_initial_elements), None, None).unwrap();
    let program_path = FILE_PATH.to_owned() + "/Rollup4.program";
    let abi_path = FILE_PATH.to_owned() + "/Rollup4.abi.json";
    let pkey_path = FILE_PATH.to_owned() + "/Rollup4.pkey";

    let rollup = Rollup::new(tree, new_leaves, program_path, abi_path, pkey_path);
    let proof = rollup.prove().await.unwrap();
    let verify = proof
        .verify_with_file(&(FILE_PATH.to_owned() + "/Rollup4.vkey"))
        .await
        .unwrap();
    assert!(verify);
}

#[tokio::test]
#[ignore]
async fn test_rollup8() {
    let in_initial_elements = (100..=800)
        .step_by(100)
        .map(BigInt::from)
        .collect::<Vec<BigInt>>();
    let new_leaves = (1..=8).map(BigInt::from).collect::<Vec<BigInt>>();
    let tree = MerkleTree::new(Some(in_initial_elements), None, None).unwrap();
    let program_path = FILE_PATH.to_owned() + "/Rollup8.program";
    let abi_path = FILE_PATH.to_owned() + "/Rollup8.abi.json";
    let pkey_path = FILE_PATH.to_owned() + "/Rollup8.pkey";

    let rollup = Rollup::new(tree, new_leaves, program_path, abi_path, pkey_path);
    let proof = rollup.prove().await.unwrap();
    let verify = proof
        .verify_with_file(&(FILE_PATH.to_owned() + "/Rollup8.vkey"))
        .await
        .unwrap();
    assert!(verify);
}

#[tokio::test]
#[ignore]
async fn test_rollup16() {
    let in_initial_elements = (100..=1600)
        .step_by(100)
        .map(BigInt::from)
        .collect::<Vec<BigInt>>();
    let new_leaves = (1..=16).map(BigInt::from).collect::<Vec<BigInt>>();
    let tree = MerkleTree::new(Some(in_initial_elements), None, None).unwrap();
    let program_path = FILE_PATH.to_owned() + "/Rollup16.program";
    let abi_path = FILE_PATH.to_owned() + "/Rollup16.abi.json";
    let pkey_path = FILE_PATH.to_owned() + "/Rollup16.pkey";

    let rollup = Rollup::new(tree, new_leaves, program_path, abi_path, pkey_path);
    let proof = rollup.prove().await.unwrap();
    let verify = proof
        .verify_with_file(&(FILE_PATH.to_owned() + "/Rollup16.vkey"))
        .await
        .unwrap();
    assert!(verify);
}
