use crate::commitment::serial_number;
use crate::commitment::sig_pk_hash;
use crate::crypto::decrypt_asymmetric;
use crate::error::ProtocolError;
use crate::types::{
    AuditingPk, AuditingSk, DecryptedNote, EncPk, EncSk, EncryptedNote, RandomSk, SigPk, TxAmount,
    VerifyPk, VerifySk,
};
use crate::types::{AUDITING_THRESHOLD, DECRYPTED_NOTE_SIZE, NUM_OF_AUDITORS};
use crate::utils::u256_to_fixed_bytes;
use crate::utils::{big_int_to_u256, u256_to_big_int};
use ethers::core::types::U256;
use mystiko_abi::zk_transact::transact::TransactCall;
use mystiko_crypto::constants::FIELD_SIZE;
use mystiko_crypto::ecies;
use mystiko_crypto::ecies::public_key_from_unpack_point;
use mystiko_crypto::shamir;
use mystiko_crypto::zkp::prove::prove_by_file;
use mystiko_crypto::zkp::types::ZKProof;
use mystiko_crypto::zkp::verify::verify_by_file;
use num_bigint::BigInt;
use std::ops::Shr;

pub struct Transaction {
    num_inputs: u32,
    num_outputs: u32,
    in_verify_pks: Vec<VerifyPk>,
    in_verify_sks: Vec<VerifySk>,
    in_enc_pks: Vec<EncPk>,
    in_enc_sks: Vec<EncSk>,
    in_commitments: Vec<U256>,
    in_private_notes: Vec<EncryptedNote>,
    path_indices: Vec<Vec<usize>>,
    path_elements: Vec<Vec<U256>>,
    sig_pk: SigPk,
    tree_root: U256,
    public_amount: TxAmount,
    relayer_fee_amount: TxAmount,
    rollup_fee_amounts: Vec<TxAmount>,
    out_verify_pks: Vec<VerifyPk>,
    out_amounts: Vec<TxAmount>,
    out_commitments: Vec<U256>,
    out_random_ps: Vec<RandomSk>,
    out_random_rs: Vec<RandomSk>,
    out_random_ss: Vec<RandomSk>,
    program_file: String,
    abi_file: String,
    proving_key_file: String,
    random_auditing_secret_key: Option<AuditingSk>,
    auditor_public_keys: Vec<AuditingPk>,
}

fn is_neg(x: &BigInt) -> bool {
    let field_size_half: BigInt = FIELD_SIZE.clone().shr(1);
    x.gt(&field_size_half)
}

// todo do more check
fn check_transaction(tx: &Transaction) -> Result<(), ProtocolError> {
    assert_eq!(tx.num_outputs as usize, tx.out_amounts.len());
    assert_eq!(tx.num_inputs as usize, tx.in_enc_pks.len());
    Ok(())
}

fn covert_transact_to_json_value(tx: &TransactCall) -> String {
    let mut array: Vec<serde_json::Value> = vec![serde_json::json!(tx.root)];
    array.push(serde_json::json!(tx.serial_numbers));
    array.push(serde_json::json!(tx.sig_hashes));
    array.push(serde_json::json!(tx.sig_public_key));
    array.push(serde_json::json!(tx.public_amount));
    array.push(serde_json::json!(tx.relayer_fee_amount));
    array.push(serde_json::json!(tx.out_commitments));
    array.push(serde_json::json!(tx.rollup_fee_amounts));
    array.push(serde_json::json!(tx.random_public_key_x_sign));
    array.push(serde_json::json!(tx.random_public_key_y));
    array.push(serde_json::json!(tx.auditor_public_key_x_signs));
    array.push(serde_json::json!(tx.auditor_public_key_ys));
    array.push(serde_json::json!(tx.encrypted_commitment_shares));
    array.push(serde_json::json!(tx.in_commitments));
    array.push(serde_json::json!(tx.in_amount));
    array.push(serde_json::json!(tx.in_random_p));
    array.push(serde_json::json!(tx.in_random_r));
    array.push(serde_json::json!(tx.in_random_s));
    array.push(serde_json::json!(tx.in_secret_key));
    array.push(serde_json::json!(tx.in_public_key));
    array.push(serde_json::json!(tx.in_path_elements));
    array.push(serde_json::json!(tx.in_path_indices));
    array.push(serde_json::json!(tx.out_amount));
    array.push(serde_json::json!(tx.out_random_p));
    array.push(serde_json::json!(tx.out_random_r));
    array.push(serde_json::json!(tx.out_random_s));
    array.push(serde_json::json!(tx.out_public_key));
    array.push(serde_json::json!(tx.random_public_key_x));
    array.push(serde_json::json!(tx.auditor_public_key_xs));
    array.push(serde_json::json!(tx.random_secret_key));
    array.push(serde_json::json!(tx.coefficients));
    array.push(serde_json::json!(tx.commitment_shares));

    serde_json::Value::Array(array).to_string()
}

// todo return error
pub fn check_tx_call_param(tx: &TransactCall) {
    let random_public_key = public_key_from_unpack_point(
        &u256_to_big_int(&tx.random_public_key_x),
        &u256_to_big_int(&tx.random_public_key_y),
    );
    assert_eq!(
        random_public_key,
        ecies::public_key(&u256_to_big_int(&tx.random_secret_key))
    );
    assert_eq!(
        tx.random_public_key_x_sign,
        is_neg(&u256_to_big_int(&tx.random_public_key_x))
    );

    for i in 0..tx.in_commitments.len() {
        assert_eq!(
            tx.auditor_public_key_x_signs[i],
            is_neg(&u256_to_big_int(&tx.auditor_public_key_xs[i]))
        );

        let mut points: Vec<shamir::Point> = vec![];

        for j in 0..tx.encrypted_commitment_shares[i].len() {
            let share = u256_to_big_int(&tx.encrypted_commitment_shares[i][j]);
            let pk = public_key_from_unpack_point(
                &u256_to_big_int(&tx.auditor_public_key_xs[j]),
                &u256_to_big_int(&tx.auditor_public_key_ys[j]),
            );

            let decrypted_share =
                ecies::decrypt(&share, &u256_to_big_int(&tx.random_secret_key), &pk);
            assert_eq!(
                decrypted_share.clone(),
                u256_to_big_int(&tx.commitment_shares[i][j])
            );

            points.push(shamir::Point {
                x: BigInt::from(j + 1),
                y: decrypted_share,
            });
        }
        assert_eq!(points.len(), 5);
        let (sub_points, _) = points.split_at(3);
        let raw = shamir::recover(sub_points.to_vec(), None);
        assert_eq!(big_int_to_u256(&raw), tx.in_commitments[i]);
        assert_eq!(big_int_to_u256(&raw), tx.coefficients[i][0]);
    }
}

pub fn zk_prove_transaction(tx: &Transaction) -> Result<ZKProof, ProtocolError> {
    // todo add logs
    check_transaction(tx)?;

    let mut in_random_p = vec![];
    let mut in_random_r = vec![];
    let mut in_random_s = vec![];
    let mut in_amount = vec![];
    let mut serial_numbers = vec![];
    let mut sig_hashes = vec![];

    for i in 0..tx.num_inputs as usize {
        let note = decrypt_asymmetric(
            &u256_to_fixed_bytes(&tx.in_enc_sks[i]),
            tx.in_private_notes[i].as_slice(),
        )
        .unwrap();
        assert_eq!(note.len(), DECRYPTED_NOTE_SIZE);
        let note = DecryptedNote::from_vec(note);
        in_random_p.push(note.random_p);
        in_random_r.push(note.random_r);
        in_random_s.push(note.random_s);
        in_amount.push(note.amount);
        serial_numbers.push(serial_number(&tx.in_verify_sks[i], &note.random_p));
        sig_hashes.push(sig_pk_hash(&tx.sig_pk, &tx.in_verify_sks[i]));
    }

    let random_auditing_sk = if let Some(key) = &tx.random_auditing_secret_key {
        u256_to_big_int(key)
    } else {
        ecies::generate_secret_key()
    };

    let random_auditing_pk = ecies::public_key(&random_auditing_sk);
    // todo unpack pk from secret key
    let unpacked_random_auditing_pk = ecies::unpack_public_key(&random_auditing_pk);
    let mut auditor_public_key_x_signs = [false; NUM_OF_AUDITORS as usize];
    let mut auditor_public_key_xs = [U256::zero(); NUM_OF_AUDITORS as usize];
    let mut auditor_public_key_ys = [U256::zero(); NUM_OF_AUDITORS as usize];
    for i in 0..tx.auditor_public_keys.len() {
        let pk = u256_to_big_int(&tx.auditor_public_keys[i]);
        let (unpacked_key_x, unpacked_key_y) = ecies::unpack_public_key(&pk);
        auditor_public_key_x_signs[i] = is_neg(&unpacked_key_x);
        auditor_public_key_xs[i] = big_int_to_u256(&unpacked_key_x);
        auditor_public_key_ys[i] = big_int_to_u256(&unpacked_key_y);
    }

    let mut coefficients = vec![];
    let mut commitment_shares = vec![];
    let mut encrypted_commitment_shares = vec![];
    for i in 0..tx.in_commitments.len() {
        let ss = shamir::split(
            u256_to_big_int(&tx.in_commitments[i]),
            NUM_OF_AUDITORS,
            AUDITING_THRESHOLD,
            None,
        )
        .unwrap();

        let cos: Vec<U256> = ss.coefficients.iter().map(big_int_to_u256).collect();
        let cos: [U256; AUDITING_THRESHOLD as usize] = cos.try_into().unwrap();
        coefficients.push(cos);

        let p_ys: Vec<U256> = ss.shares.iter().map(|p| big_int_to_u256(&p.y)).collect();
        let p_ys: [U256; NUM_OF_AUDITORS as usize] = p_ys.try_into().unwrap();
        commitment_shares.push(p_ys);

        let mut encrypted_shares = vec![];
        for j in 0..ss.shares.len() {
            let pk = u256_to_big_int(&tx.auditor_public_keys[j]);
            let encrypted_share = ecies::encrypt(&ss.shares[j].y, &pk, &random_auditing_sk);
            encrypted_shares.push(big_int_to_u256(&encrypted_share));
        }
        let encrypted_shares: [U256; NUM_OF_AUDITORS as usize] =
            encrypted_shares.try_into().unwrap();
        encrypted_commitment_shares.push(encrypted_shares);
    }

    // let in_path_indices = tx.path_indices.iter().map(|p| p != 0).collect();
    let in_path_indices = tx
        .path_indices
        .iter()
        .map(|p| p.iter().map(|&x| x != 0).collect())
        .collect();

    let tx_param = TransactCall {
        root: tx.tree_root,
        serial_numbers,
        sig_hashes,
        sig_public_key: U256::from_big_endian(&tx.sig_pk),
        public_amount: tx.public_amount,
        relayer_fee_amount: tx.relayer_fee_amount,
        out_commitments: tx.out_commitments.clone(),
        rollup_fee_amounts: tx.rollup_fee_amounts.clone(),
        random_public_key_x_sign: is_neg(&unpacked_random_auditing_pk.0),
        random_public_key_y: big_int_to_u256(&unpacked_random_auditing_pk.1),
        auditor_public_key_x_signs,
        auditor_public_key_ys,
        encrypted_commitment_shares,
        in_commitments: tx.in_commitments.clone(),
        in_amount,
        in_random_p,
        in_random_r,
        in_random_s,
        in_secret_key: tx.in_verify_sks.clone(),
        in_public_key: tx.in_verify_pks.clone(),
        in_path_elements: tx.path_elements.clone(),
        in_path_indices,
        out_amount: tx.out_amounts.clone(),
        out_random_p: tx.out_random_ps.clone(),
        out_random_r: tx.out_random_rs.clone(),
        out_random_s: tx.out_random_ss.clone(),
        out_public_key: tx.out_verify_pks.clone(),
        random_public_key_x: big_int_to_u256(&unpacked_random_auditing_pk.0),
        auditor_public_key_xs,
        random_secret_key: big_int_to_u256(&random_auditing_sk),
        coefficients,
        commitment_shares,
    };

    check_tx_call_param(&tx_param);

    let tx_param = covert_transact_to_json_value(&tx_param);
    let proof = prove_by_file(
        &tx.program_file,
        &tx.abi_file,
        &tx.proving_key_file,
        &tx_param,
    )
    .unwrap();

    Ok(proof)
}

pub fn zk_verify(proof: ZKProof, verify_key_file: &str) -> Result<bool, ProtocolError> {
    verify_by_file(proof, verify_key_file).map_err(|e| ProtocolError::CryptoError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commitment::{build_commitment, CommitmentInput, PublicKeys};
    use crate::types::RandomSecrets;
    use crate::types::{ENC_SK_SIZE, MERKLE_TREE_LEVELS, VERIFY_SK_SIZE};
    use crate::wallet::{
        public_key_for_encryption, public_key_for_verification, secret_key_for_encryption,
        secret_key_for_verification,
    };
    use ethers::core::rand::thread_rng;
    use ethers::signers::{LocalWallet, Signer};
    use mystiko_crypto::merkle_tree::MerkleTree;
    use mystiko_crypto::utils::random_bytes;

    fn generate_eth_wallet() -> Vec<u8> {
        let wallet = LocalWallet::new(&mut thread_rng());
        let wallet = wallet.with_chain_id(1u64);
        wallet.address().as_bytes().to_vec()
    }

    fn generate_transaction(
        num_inputs: u32,
        num_outputs: u32,
        program_file: String,
        abi_file: String,
        proving_key_file: String,
    ) -> Transaction {
        let mut in_verify_pks: Vec<VerifyPk> = vec![];
        let mut in_verify_sks: Vec<VerifySk> = vec![];
        let mut in_enc_pks: Vec<EncPk> = vec![];
        let mut in_enc_sks: Vec<EncSk> = vec![];
        let mut in_amounts: Vec<TxAmount> = vec![];
        let mut in_commitments: Vec<U256> = vec![];
        let mut in_commitments_bigint: Vec<BigInt> = vec![];
        let mut in_private_notes: Vec<EncryptedNote> = vec![];

        let in_amount = ethers::utils::parse_ether("200").unwrap();
        let out_amount = ethers::utils::parse_ether("50").unwrap();
        let rollup_fee_amount = ethers::utils::parse_ether("10").unwrap();
        let relayer_fee_amount = ethers::utils::parse_ether("20").unwrap();

        for i in 0..num_inputs as usize {
            let raw_verify_sk = random_bytes(VERIFY_SK_SIZE);
            let raw_enc_sk = random_bytes(ENC_SK_SIZE);
            in_verify_sks.push(secret_key_for_verification(raw_verify_sk.as_slice()));
            in_verify_pks.push(public_key_for_verification(raw_verify_sk.as_slice()));
            in_enc_sks.push(secret_key_for_encryption(raw_enc_sk.as_slice()));
            in_enc_pks.push(public_key_for_encryption(raw_enc_sk.as_slice()));
            in_amounts.push(in_amount);

            let commitment_input = CommitmentInput {
                public_keys: PublicKeys::Object {
                    pk_verify: in_verify_pks[i],
                    pk_enc: in_enc_pks[i],
                },
                amount: Some(in_amounts[i]),
                random_secrets: None,
                encrypted_note: None,
            };

            let commitment_output = build_commitment(commitment_input).unwrap();
            in_commitments_bigint.push(commitment_output.commitment_hash.clone());
            in_commitments.push(big_int_to_u256(&commitment_output.commitment_hash));
            in_private_notes.push(commitment_output.encrypted_note);
        }

        let merkle_tree =
            MerkleTree::new(Some(in_commitments_bigint), Some(MERKLE_TREE_LEVELS), None).unwrap();
        let mut path_elements: Vec<Vec<U256>> = vec![];
        let mut path_indices: Vec<Vec<usize>> = vec![];
        for i in 0..num_inputs as usize {
            let path = merkle_tree.path(i).unwrap();
            let elements = path.0.iter().map(|b| big_int_to_u256(&b)).collect();
            path_elements.push(elements);
            path_indices.push(path.1);
        }

        let sig_pk = generate_eth_wallet();

        let mut out_verify_pks: Vec<VerifyPk> = vec![];
        let mut out_enc_pks: Vec<EncPk> = vec![];
        let mut rollup_fee_amounts: Vec<TxAmount> = vec![];
        let mut out_amounts: Vec<TxAmount> = vec![];
        let mut out_commitments: Vec<U256> = vec![];
        let mut out_random_ps: Vec<RandomSk> = vec![];
        let mut out_random_rs: Vec<RandomSk> = vec![];
        let mut out_random_ss: Vec<RandomSk> = vec![];

        for i in 0..num_outputs as usize {
            let raw_verify_sk = random_bytes(VERIFY_SK_SIZE);
            let raw_enc_sk = random_bytes(ENC_SK_SIZE);
            out_verify_pks.push(public_key_for_verification(raw_verify_sk.as_slice()));
            out_enc_pks.push(public_key_for_encryption(raw_enc_sk.as_slice()));
            out_amounts.push(out_amount);
            rollup_fee_amounts.push(rollup_fee_amount);

            let commitment_input = CommitmentInput {
                public_keys: PublicKeys::Object {
                    pk_verify: out_verify_pks[i],
                    pk_enc: out_enc_pks[i],
                },
                amount: Some(out_amounts[i]),
                random_secrets: Some(RandomSecrets::generate()),
                encrypted_note: None,
            };

            let commitment_output = build_commitment(commitment_input).unwrap();
            out_commitments.push(big_int_to_u256(&commitment_output.commitment_hash));
            out_random_ps.push(commitment_output.random_p);
            out_random_rs.push(commitment_output.random_r);
            out_random_ss.push(commitment_output.random_s);
        }

        let total_in = in_amounts
            .iter()
            .fold(U256::zero(), |acc, x| acc.saturating_add(*x));
        let total_out = out_amounts
            .iter()
            .fold(U256::zero(), |acc, x| acc.saturating_add(*x));
        let total_rollup_fee = rollup_fee_amounts
            .iter()
            .fold(U256::zero(), |acc, x| acc.saturating_add(*x));

        let public_amount: U256 = total_in
            .saturating_sub(total_out)
            .saturating_sub(total_rollup_fee)
            .saturating_sub(relayer_fee_amount);

        let random_auditing_secret_key = big_int_to_u256(&ecies::generate_secret_key());
        let mut auditor_public_keys: Vec<AuditingPk> = vec![];
        for _ in 0..NUM_OF_AUDITORS {
            let pk = ecies::public_key(&ecies::generate_secret_key());
            auditor_public_keys.push(big_int_to_u256(&pk));
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
            tree_root: big_int_to_u256(&merkle_tree.root()),
            public_amount,
            relayer_fee_amount: relayer_fee_amount,
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
            random_auditing_secret_key: Some(random_auditing_secret_key),
            auditor_public_keys,
        }
    }

    const FILE_PATH: &str = "./../mystiko-circuits/dist/zokrates/dev";

    #[test]
    fn test_is_neg() {
        assert!(!is_neg(&(BigInt::from(1))));
        assert!(!is_neg(&(FIELD_SIZE.clone() / 2 - 1)));
        assert!(is_neg(&(FIELD_SIZE.clone() / 2 + 1)));
        assert!(is_neg(&(FIELD_SIZE.clone() + 1)));
    }

    #[test]
    fn test_transaction1x0() {
        let tx = generate_transaction(
            1u32,
            0u32,
            (FILE_PATH.to_owned() + "/Transaction1x0.program").to_string(),
            (FILE_PATH.to_owned() + "/Transaction1x0.abi.json").to_string(),
            (FILE_PATH.to_owned() + "/Transaction1x0.pkey").to_string(),
        );

        let proof = zk_prove_transaction(&tx).unwrap();
        let verify = zk_verify(proof, &(FILE_PATH.to_owned() + "/Transaction1x0.vkey")).unwrap();
        assert!(verify);
    }

    #[test]
    fn test_transaction1x1() {
        let tx = generate_transaction(
            1u32,
            1u32,
            (FILE_PATH.to_owned() + "/Transaction1x1.program").to_string(),
            (FILE_PATH.to_owned() + "/Transaction1x1.abi.json").to_string(),
            (FILE_PATH.to_owned() + "/Transaction1x1.pkey").to_string(),
        );

        let proof = zk_prove_transaction(&tx).unwrap();
        let verify = zk_verify(proof, &(FILE_PATH.to_owned() + "/Transaction1x1.vkey")).unwrap();
        assert!(verify);
    }

    #[test]
    fn test_transaction1x2() {
        let tx = generate_transaction(
            1u32,
            2u32,
            (FILE_PATH.to_owned() + "/Transaction1x2.program").to_string(),
            (FILE_PATH.to_owned() + "/Transaction1x2.abi.json").to_string(),
            (FILE_PATH.to_owned() + "/Transaction1x2.pkey").to_string(),
        );

        let proof = zk_prove_transaction(&tx).unwrap();
        let verify = zk_verify(proof, &(FILE_PATH.to_owned() + "/Transaction1x2.vkey")).unwrap();
        assert!(verify);
    }

    #[test]
    fn test_transaction2x0() {
        let tx = generate_transaction(
            2u32,
            0u32,
            (FILE_PATH.to_owned() + "/Transaction2x0.program").to_string(),
            (FILE_PATH.to_owned() + "/Transaction2x0.abi.json").to_string(),
            (FILE_PATH.to_owned() + "/Transaction2x0.pkey").to_string(),
        );

        let proof = zk_prove_transaction(&tx).unwrap();
        let verify = zk_verify(proof, &(FILE_PATH.to_owned() + "/Transaction2x0.vkey")).unwrap();
        assert!(verify);
    }

    #[test]
    fn test_transaction2x1() {
        let tx = generate_transaction(
            2u32,
            1u32,
            (FILE_PATH.to_owned() + "/Transaction2x1.program").to_string(),
            (FILE_PATH.to_owned() + "/Transaction2x1.abi.json").to_string(),
            (FILE_PATH.to_owned() + "/Transaction2x1.pkey").to_string(),
        );

        let proof = zk_prove_transaction(&tx).unwrap();
        let verify = zk_verify(proof, &(FILE_PATH.to_owned() + "/Transaction2x1.vkey")).unwrap();
        assert!(verify);
    }

    #[test]
    fn test_transaction2x2() {
        let tx = generate_transaction(
            2u32,
            2u32,
            (FILE_PATH.to_owned() + "/Transaction2x2.program").to_string(),
            (FILE_PATH.to_owned() + "/Transaction2x2.abi.json").to_string(),
            (FILE_PATH.to_owned() + "/Transaction2x2.pkey").to_string(),
        );

        let proof = zk_prove_transaction(&tx).unwrap();
        let verify = zk_verify(proof, &(FILE_PATH.to_owned() + "/Transaction2x2.vkey")).unwrap();
        assert!(verify);
    }
}
