extern crate mystiko_crypto;
extern crate mystiko_protocol;
extern crate num_bigint;

use num_bigint::BigInt;

use mystiko_crypto::merkle_tree::MerkleTree;
use mystiko_protocol::rollup::{zk_prove_rollup, Rollup};
use mystiko_protocol::verify::zk_verify;

const FILE_PATH: &str = "./../mystiko-circuits/dist/zokrates/dev";

#[tokio::test]
async fn test_rollup1() {
    let in_initial_elements = [BigInt::from(100), BigInt::from(200), BigInt::from(300)].to_vec();
    let tree = MerkleTree::new(Some(in_initial_elements), None, None).unwrap();

    let mut rollup = Rollup {
        tree,
        new_leaves: vec![BigInt::from(1u32)],
        program_file: (FILE_PATH.to_owned() + "/Rollup1.program").to_string(),
        abi_file: (FILE_PATH.to_owned() + "/Rollup1.abi.json").to_string(),
        proving_key_file: (FILE_PATH.to_owned() + "/Rollup1.pkey").to_string(),
    };
    let proof = zk_prove_rollup(&mut rollup).unwrap();
    let verify = zk_verify(proof, &(FILE_PATH.to_owned() + "/Rollup1.vkey")).unwrap();
    assert!(verify);
}

#[tokio::test]
async fn test_rollup2() {
    let in_initial_elements = [BigInt::from(100), BigInt::from(200)].to_vec();
    let tree = MerkleTree::new(Some(in_initial_elements), None, None).unwrap();

    let mut rollup = Rollup {
        tree,
        new_leaves: vec![BigInt::from(1u32), BigInt::from(2u32)],
        program_file: (FILE_PATH.to_owned() + "/Rollup2.program").to_string(),
        abi_file: (FILE_PATH.to_owned() + "/Rollup2.abi.json").to_string(),
        proving_key_file: (FILE_PATH.to_owned() + "/Rollup2.pkey").to_string(),
    };
    let proof = zk_prove_rollup(&mut rollup).unwrap();
    let verify = zk_verify(proof, &(FILE_PATH.to_owned() + "/Rollup2.vkey")).unwrap();
    assert!(verify);
}

#[tokio::test]
#[ignore]
async fn test_rollup4() {
    let in_initial_elements = [
        BigInt::from(100),
        BigInt::from(200),
        BigInt::from(300),
        BigInt::from(400),
    ]
    .to_vec();
    let tree = MerkleTree::new(Some(in_initial_elements), None, None).unwrap();

    let mut rollup = Rollup {
        tree,
        new_leaves: vec![
            BigInt::from(1u32),
            BigInt::from(2u32),
            BigInt::from(3u32),
            BigInt::from(4u32),
        ],
        program_file: (FILE_PATH.to_owned() + "/Rollup4.program").to_string(),
        abi_file: (FILE_PATH.to_owned() + "/Rollup4.abi.json").to_string(),
        proving_key_file: (FILE_PATH.to_owned() + "/Rollup4.pkey").to_string(),
    };
    let proof = zk_prove_rollup(&mut rollup).unwrap();
    let verify = zk_verify(proof, &(FILE_PATH.to_owned() + "/Rollup4.vkey")).unwrap();
    assert!(verify);
}

#[tokio::test]
#[ignore]
async fn test_rollup8() {
    let in_initial_elements = [
        BigInt::from(100),
        BigInt::from(200),
        BigInt::from(300),
        BigInt::from(400),
        BigInt::from(500),
        BigInt::from(600),
        BigInt::from(700),
        BigInt::from(800),
    ]
    .to_vec();
    let tree = MerkleTree::new(Some(in_initial_elements), None, None).unwrap();

    let mut rollup = Rollup {
        tree,
        new_leaves: vec![
            BigInt::from(1u32),
            BigInt::from(2u32),
            BigInt::from(3u32),
            BigInt::from(4u32),
            BigInt::from(5u32),
            BigInt::from(6u32),
            BigInt::from(7u32),
            BigInt::from(8u32),
        ],
        program_file: (FILE_PATH.to_owned() + "/Rollup8.program").to_string(),
        abi_file: (FILE_PATH.to_owned() + "/Rollup8.abi.json").to_string(),
        proving_key_file: (FILE_PATH.to_owned() + "/Rollup8.pkey").to_string(),
    };
    let proof = zk_prove_rollup(&mut rollup).unwrap();
    let verify = zk_verify(proof, &(FILE_PATH.to_owned() + "/Rollup8.vkey")).unwrap();
    assert!(verify);
}

#[tokio::test]
#[ignore]
async fn test_rollup16() {
    let in_initial_elements = [
        BigInt::from(100),
        BigInt::from(200),
        BigInt::from(300),
        BigInt::from(400),
        BigInt::from(500),
        BigInt::from(600),
        BigInt::from(700),
        BigInt::from(800),
        BigInt::from(900),
        BigInt::from(1000),
        BigInt::from(1100),
        BigInt::from(1200),
        BigInt::from(1300),
        BigInt::from(1400),
        BigInt::from(1500),
        BigInt::from(1600),
    ]
    .to_vec();
    let tree = MerkleTree::new(Some(in_initial_elements), None, None).unwrap();

    let mut rollup = Rollup {
        tree,
        new_leaves: vec![
            BigInt::from(1u32),
            BigInt::from(2u32),
            BigInt::from(3u32),
            BigInt::from(4u32),
            BigInt::from(5u32),
            BigInt::from(6u32),
            BigInt::from(7u32),
            BigInt::from(8u32),
            BigInt::from(9u32),
            BigInt::from(10u32),
            BigInt::from(11u32),
            BigInt::from(12u32),
            BigInt::from(13u32),
            BigInt::from(14u32),
            BigInt::from(15u32),
            BigInt::from(16u32),
        ],
        program_file: (FILE_PATH.to_owned() + "/Rollup16.program").to_string(),
        abi_file: (FILE_PATH.to_owned() + "/Rollup16.abi.json").to_string(),
        proving_key_file: (FILE_PATH.to_owned() + "/Rollup16.pkey").to_string(),
    };
    let proof = zk_prove_rollup(&mut rollup).unwrap();
    let verify = zk_verify(proof, &(FILE_PATH.to_owned() + "/Rollup16.vkey")).unwrap();
    assert!(verify);
}
