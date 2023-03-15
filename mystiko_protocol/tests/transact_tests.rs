extern crate ethers;
extern crate mystiko_crypto;
extern crate mystiko_protocol;
extern crate num_bigint;
extern crate num_traits;

use std::ops::Sub;

use ethers::core::rand::thread_rng;
use ethers::core::types::U256;
use ethers::signers::{LocalWallet, Signer};
use num_bigint::{BigInt, Sign};
use num_traits::identities::Zero;

use mystiko_crypto::ecies;
use mystiko_crypto::merkle_tree::MerkleTree;
use mystiko_crypto::utils::random_bytes;
use mystiko_protocol::address::ShieldedAddress;
use mystiko_protocol::commitment::{Commitment, EncryptedNote, Note};
use mystiko_protocol::key::{
    public_key_for_encryption, public_key_for_verification, secret_key_for_encryption,
    secret_key_for_verification,
};
use mystiko_protocol::transact::Transaction;
use mystiko_protocol::types::{
    AuditingPk, EncPk, EncSk, RandomSk, TxAmount, VerifyPk, VerifySk, NUM_OF_AUDITORS,
};
use mystiko_protocol::types::{ENC_SK_SIZE, MERKLE_TREE_LEVELS, VERIFY_SK_SIZE};

fn generate_eth_wallet() -> Vec<u8> {
    let wallet = LocalWallet::new(&mut thread_rng());
    let wallet = wallet.with_chain_id(1u64);
    wallet.address().as_bytes().to_vec()
}

fn u256_to_big_int(u: &U256) -> BigInt {
    let mut arr = [0u8; 32];
    u.to_little_endian(&mut arr[..]);
    BigInt::from_bytes_le(Sign::Plus, &arr[..])
}

fn generate_transaction(
    num_inputs: u32,
    num_outputs: u32,
    program_file: String,
    abi_file: String,
    proving_key_file: String,
    generate_auditing_key: Option<bool>,
) -> Transaction {
    let mut in_verify_pks: Vec<VerifyPk> = vec![];
    let mut in_verify_sks: Vec<VerifySk> = vec![];
    let mut in_enc_pks: Vec<EncPk> = vec![];
    let mut in_enc_sks: Vec<EncSk> = vec![];
    let mut in_amounts: Vec<TxAmount> = vec![];
    let mut in_commitments: Vec<BigInt> = vec![];
    let mut in_private_notes: Vec<EncryptedNote> = vec![];

    let in_amount = u256_to_big_int(&ethers::utils::parse_ether("200").unwrap());
    let out_amount = u256_to_big_int(&ethers::utils::parse_ether("50").unwrap());
    let rollup_fee_amount = u256_to_big_int(&ethers::utils::parse_ether("10").unwrap());
    let relayer_fee_amount = u256_to_big_int(&ethers::utils::parse_ether("20").unwrap());

    for i in 0..num_inputs as usize {
        let raw_verify_sk = random_bytes(VERIFY_SK_SIZE);
        let raw_enc_sk = random_bytes(ENC_SK_SIZE);
        in_verify_sks.push(secret_key_for_verification(raw_verify_sk.as_slice()));
        in_verify_pks.push(public_key_for_verification(raw_verify_sk.as_slice()));
        in_enc_sks.push(secret_key_for_encryption(raw_enc_sk.as_slice()));
        in_enc_pks.push(public_key_for_encryption(raw_enc_sk.as_slice()));
        in_amounts.push(in_amount.clone());

        let cm = Commitment::new(
            ShieldedAddress::from_public_key(&in_verify_pks[i], &in_enc_pks[i]),
            Some(Note::new(Some(in_amounts[i].clone()), None)),
            None,
        )
        .unwrap();

        in_commitments.push(cm.commitment_hash.clone());
        in_private_notes.push(cm.encrypted_note);
    }

    let merkle_tree =
        MerkleTree::new(Some(in_commitments.clone()), Some(MERKLE_TREE_LEVELS), None).unwrap();
    let mut path_elements: Vec<Vec<BigInt>> = vec![];
    let mut path_indices: Vec<Vec<usize>> = vec![];
    for i in 0..num_inputs as usize {
        let path = merkle_tree.path(i).unwrap();
        path_elements.push(path.0);
        path_indices.push(path.1);
    }

    let sig_pk = generate_eth_wallet();

    let mut out_verify_pks: Vec<VerifyPk> = vec![];
    let mut out_enc_pks: Vec<EncPk> = vec![];
    let mut rollup_fee_amounts: Vec<TxAmount> = vec![];
    let mut out_amounts: Vec<TxAmount> = vec![];
    let mut out_commitments: Vec<BigInt> = vec![];
    let mut out_random_ps: Vec<RandomSk> = vec![];
    let mut out_random_rs: Vec<RandomSk> = vec![];
    let mut out_random_ss: Vec<RandomSk> = vec![];

    for i in 0..num_outputs as usize {
        let raw_verify_sk = random_bytes(VERIFY_SK_SIZE);
        let raw_enc_sk = random_bytes(ENC_SK_SIZE);
        out_verify_pks.push(public_key_for_verification(raw_verify_sk.as_slice()));
        out_enc_pks.push(public_key_for_encryption(raw_enc_sk.as_slice()));
        out_amounts.push(out_amount.clone());
        rollup_fee_amounts.push(rollup_fee_amount.clone());

        let cm = Commitment::new(
            ShieldedAddress::from_public_key(&out_verify_pks[i], &out_enc_pks[i]),
            Some(Note::new(Some(out_amounts[i].clone()), None)),
            None,
        )
        .unwrap();

        out_commitments.push(cm.commitment_hash.clone());
        out_random_ps.push(cm.note.random_p);
        out_random_rs.push(cm.note.random_r);
        out_random_ss.push(cm.note.random_s);
    }

    let total_in = in_amounts.iter().fold(BigInt::zero(), |acc, x| acc + x);
    let total_out = out_amounts.iter().fold(BigInt::zero(), |acc, x| acc + x);
    let total_rollup_fee = rollup_fee_amounts
        .iter()
        .fold(BigInt::zero(), |acc, x| acc + x);

    let public_amount = total_in
        .sub(total_out)
        .sub(total_rollup_fee)
        .sub(relayer_fee_amount.clone());

    let random_auditing_secret_key = if generate_auditing_key.unwrap_or(false) {
        Some(ecies::generate_secret_key())
    } else {
        None
    };

    let mut auditor_public_keys: Vec<AuditingPk> = vec![];
    for _ in 0..NUM_OF_AUDITORS {
        let pk = ecies::public_key(&ecies::generate_secret_key());
        auditor_public_keys.push(pk);
    }

    Transaction {
        num_inputs,
        num_outputs,
        in_verify_pks,
        in_verify_sks,
        in_enc_pks,
        in_enc_sks,
        in_commitments,
        in_private_notes,
        path_indices,
        path_elements,
        sig_pk,
        tree_root: merkle_tree.root(),
        public_amount,
        relayer_fee_amount,
        rollup_fee_amounts,
        out_verify_pks,
        out_amounts,
        out_commitments,
        out_random_ps,
        out_random_rs,
        out_random_ss,
        program_file,
        abi_file,
        proving_key_file,
        random_auditing_secret_key,
        auditor_public_keys,
    }
}

const FILE_PATH: &str = "./../mystiko-circuits/dist/zokrates/dev";

#[tokio::test]
#[ignore]
async fn test_transaction1x0() {
    let tx = generate_transaction(
        1u32,
        0u32,
        (FILE_PATH.to_owned() + "/Transaction1x0.program").to_string(),
        (FILE_PATH.to_owned() + "/Transaction1x0.abi.json").to_string(),
        (FILE_PATH.to_owned() + "/Transaction1x0.pkey").to_string(),
        None,
    );

    let proof = tx.prove().await.unwrap();
    let verify = proof
        .verify_with_file(&(FILE_PATH.to_owned() + "/Transaction1x0.vkey"))
        .await
        .unwrap();
    assert!(verify);
    let _ = tx.clone();
}

#[tokio::test]
#[ignore]
async fn test_transaction1x1() {
    let tx = generate_transaction(
        1u32,
        1u32,
        (FILE_PATH.to_owned() + "/Transaction1x1.program").to_string(),
        (FILE_PATH.to_owned() + "/Transaction1x1.abi.json").to_string(),
        (FILE_PATH.to_owned() + "/Transaction1x1.pkey").to_string(),
        Some(true),
    );

    let proof = tx.prove().await.unwrap();
    let verify = proof
        .verify_with_file(&(FILE_PATH.to_owned() + "/Transaction1x1.vkey"))
        .await
        .unwrap();
    assert!(verify);
}

#[tokio::test]
#[ignore]
async fn test_transaction1x2() {
    let tx = generate_transaction(
        1u32,
        2u32,
        (FILE_PATH.to_owned() + "/Transaction1x2.program").to_string(),
        (FILE_PATH.to_owned() + "/Transaction1x2.abi.json").to_string(),
        (FILE_PATH.to_owned() + "/Transaction1x2.pkey").to_string(),
        Some(true),
    );

    let proof = tx.prove().await.unwrap();
    let verify = proof
        .verify_with_file(&(FILE_PATH.to_owned() + "/Transaction1x2.vkey"))
        .await
        .unwrap();
    assert!(verify);
}

#[tokio::test]
#[ignore]
async fn test_transaction2x0() {
    let tx = generate_transaction(
        2u32,
        0u32,
        (FILE_PATH.to_owned() + "/Transaction2x0.program").to_string(),
        (FILE_PATH.to_owned() + "/Transaction2x0.abi.json").to_string(),
        (FILE_PATH.to_owned() + "/Transaction2x0.pkey").to_string(),
        Some(true),
    );

    let proof = tx.prove().await.unwrap();
    let verify = proof
        .verify_with_file(&(FILE_PATH.to_owned() + "/Transaction2x0.vkey"))
        .await
        .unwrap();
    assert!(verify);
}

#[tokio::test]
#[ignore]
async fn test_transaction2x1() {
    let tx = generate_transaction(
        2u32,
        1u32,
        (FILE_PATH.to_owned() + "/Transaction2x1.program").to_string(),
        (FILE_PATH.to_owned() + "/Transaction2x1.abi.json").to_string(),
        (FILE_PATH.to_owned() + "/Transaction2x1.pkey").to_string(),
        Some(true),
    );

    let proof = tx.prove().await.unwrap();
    let verify = proof
        .verify_with_file(&(FILE_PATH.to_owned() + "/Transaction2x1.vkey"))
        .await
        .unwrap();
    assert!(verify);
}

#[tokio::test]
async fn test_transaction2x2() {
    let tx = generate_transaction(
        2u32,
        2u32,
        (FILE_PATH.to_owned() + "/Transaction2x2.program").to_string(),
        (FILE_PATH.to_owned() + "/Transaction2x2.abi.json").to_string(),
        (FILE_PATH.to_owned() + "/Transaction2x2.pkey").to_string(),
        Some(true),
    );

    let proof = tx.prove().await.unwrap();
    let verify = proof
        .verify_with_file(&(FILE_PATH.to_owned() + "/Transaction2x2.vkey"))
        .await
        .unwrap();
    assert!(verify);
}
