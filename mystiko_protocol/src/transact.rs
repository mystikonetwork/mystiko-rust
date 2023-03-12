use crate::commitment::serial_number;
use crate::commitment::sig_pk_hash;
use crate::crypto::decrypt_asymmetric;
use crate::error::ProtocolError;
use crate::types::{
    AuditingPk, AuditingSk, DecryptedNote, EncPk, EncSk, EncryptedNote, RandomSk, SigPk, TxAmount,
    VerifyPk, VerifySk,
};
use crate::types::{AUDITING_THRESHOLD, DECRYPTED_NOTE_SIZE, NUM_OF_AUDITORS};
use crate::utils::to_string_slice;
use ff::hex;
use mystiko_crypto::constants::FIELD_SIZE;
use mystiko_crypto::ecies;
use mystiko_crypto::shamir;
use mystiko_crypto::utils::big_int_to_32_bytes;
use mystiko_crypto::zkp::prove::prove_by_file;
use mystiko_crypto::zkp::types::ZKProof;
use num_bigint::BigInt;
use std::ops::Shr;

pub struct Transaction {
    pub num_inputs: u32,
    pub num_outputs: u32,
    pub in_verify_pks: Vec<VerifyPk>,
    pub in_verify_sks: Vec<VerifySk>,
    pub in_enc_pks: Vec<EncPk>,
    pub in_enc_sks: Vec<EncSk>,
    pub in_commitments: Vec<BigInt>,
    pub in_private_notes: Vec<EncryptedNote>,
    pub path_indices: Vec<Vec<usize>>,
    pub path_elements: Vec<Vec<BigInt>>,
    pub sig_pk: SigPk,
    pub tree_root: BigInt,
    pub public_amount: TxAmount,
    pub relayer_fee_amount: TxAmount,
    pub rollup_fee_amounts: Vec<TxAmount>,
    pub out_verify_pks: Vec<VerifyPk>,
    pub out_amounts: Vec<TxAmount>,
    pub out_commitments: Vec<BigInt>,
    pub out_random_ps: Vec<RandomSk>,
    pub out_random_rs: Vec<RandomSk>,
    pub out_random_ss: Vec<RandomSk>,
    pub program_file: String,
    pub abi_file: String,
    pub proving_key_file: String,
    pub random_auditing_secret_key: Option<AuditingSk>,
    pub auditor_public_keys: Vec<AuditingPk>,
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
//
// // todo return error
// pub fn check_tx_call_param(tx: &TransactCall) {
//     // let random_public_key =
//     //     public_key_from_unpack_point(&tx.random_public_key_x, &tx.random_public_key_y);
//     // assert_eq!(
//     //     random_public_key,
//     //     ecies::public_key(&u256_to_big_int(&tx.random_secret_key))
//     // );
//     // assert_eq!(
//     //     tx.random_public_key_x_sign,
//     //     is_neg(&u256_to_big_int(&tx.random_public_key_x))
//     // );
//
//     for i in 0..tx.in_commitments.len() {
//         assert_eq!(
//             tx.auditor_public_key_x_signs[i],
//             is_neg(&u256_to_big_int(&tx.auditor_public_key_xs[i]))
//         );
//
//         let mut points: Vec<shamir::Point> = vec![];
//
//         for j in 0..tx.encrypted_commitment_shares[i].len() {
//             let share = u256_to_big_int(&tx.encrypted_commitment_shares[i][j]);
//             let pk = public_key_from_unpack_point(
//                 &u256_to_big_int(&tx.auditor_public_key_xs[j]),
//                 &u256_to_big_int(&tx.auditor_public_key_ys[j]),
//             );
//
//             let decrypted_share =
//                 ecies::decrypt(&share, &u256_to_big_int(&tx.random_secret_key), &pk);
//             assert_eq!(
//                 decrypted_share.clone(),
//                 u256_to_big_int(&tx.commitment_shares[i][j])
//             );
//
//             points.push(shamir::Point {
//                 x: BigInt::from(j + 1),
//                 y: decrypted_share,
//             });
//         }
//         assert_eq!(points.len(), 5);
//         let (sub_points, _) = points.split_at(3);
//         let raw = shamir::recover(sub_points.to_vec(), None);
//         assert_eq!(big_int_to_u256(&raw), tx.in_commitments[i]);
//         assert_eq!(big_int_to_u256(&raw), tx.coefficients[i][0]);
//     }
// }

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
            &big_int_to_32_bytes(&tx.in_enc_sks[i]),
            tx.in_private_notes[i].as_slice(),
        )
        .unwrap();
        assert_eq!(note.len(), DECRYPTED_NOTE_SIZE);
        let note = DecryptedNote::from_vec(note);
        in_random_p.push(note.random_p.clone());
        in_random_r.push(note.random_r.clone());
        in_random_s.push(note.random_s.clone());
        in_amount.push(note.amount);
        serial_numbers.push(serial_number(
            tx.in_verify_sks[i].clone(),
            note.random_p.clone(),
        ));
        sig_hashes.push(sig_pk_hash(&tx.sig_pk, tx.in_verify_sks[i].clone()));
    }

    let random_auditing_sk = if let Some(key) = tx.random_auditing_secret_key.clone() {
        key
    } else {
        ecies::generate_secret_key()
    };

    let random_auditing_pk = ecies::public_key(&random_auditing_sk);
    // todo unpack pk from secret key
    let unpacked_random_auditing_pk = ecies::unpack_public_key(&random_auditing_pk);
    let mut auditor_public_key_x_signs = vec![];
    let mut auditor_public_key_xs = vec![];
    let mut auditor_public_key_ys = vec![];
    for i in 0..tx.auditor_public_keys.len() {
        let pk = &tx.auditor_public_keys[i];
        let (unpacked_key_x, unpacked_key_y) = ecies::unpack_public_key(pk);
        auditor_public_key_x_signs.push(is_neg(&unpacked_key_x));
        auditor_public_key_xs.push(unpacked_key_x);
        auditor_public_key_ys.push(unpacked_key_y);
    }

    let mut coefficients = vec![];
    let mut commitment_shares = vec![];
    let mut encrypted_commitment_shares = vec![];
    for i in 0..tx.in_commitments.len() {
        let ss = shamir::split(
            tx.in_commitments[i].clone(),
            NUM_OF_AUDITORS,
            AUDITING_THRESHOLD,
            None,
        )
        .unwrap();

        let cos = ss.coefficients.clone();
        let cos = to_string_slice(&cos).to_owned();
        coefficients.push(cos.clone());

        let p_ys: Vec<String> = ss.shares.iter().map(|p| p.y.to_string()).collect();
        let p_ys: [String; NUM_OF_AUDITORS as usize] = p_ys.try_into().unwrap();
        commitment_shares.push(p_ys);

        let mut encrypted_shares = vec![];
        for j in 0..ss.shares.len() {
            let pk = tx.auditor_public_keys[j].clone();
            let encrypted_share = ecies::encrypt(&ss.shares[j].y, &pk, &random_auditing_sk);
            encrypted_shares.push(encrypted_share.to_string());
        }
        let encrypted_shares: [String; NUM_OF_AUDITORS as usize] =
            encrypted_shares.try_into().unwrap();
        encrypted_commitment_shares.push(encrypted_shares);
    }

    let in_path_indices: Vec<_> = tx
        .path_indices
        .clone()
        .iter_mut()
        .map(|p| p.iter_mut().map(|&mut x| x != 0).collect::<Vec<_>>())
        .collect();

    let mut array: Vec<serde_json::Value> = vec![serde_json::json!(tx.tree_root.to_string())];
    array.push(serde_json::json!(to_string_slice(&serial_numbers)));
    array.push(serde_json::json!(to_string_slice(&sig_hashes)));
    array.push(serde_json::json!(hex::encode(tx.sig_pk.clone())));
    array.push(serde_json::json!(tx.public_amount.to_string()));
    array.push(serde_json::json!(tx.relayer_fee_amount.to_string()));
    array.push(serde_json::json!(to_string_slice(&tx.out_commitments)));
    array.push(serde_json::json!(to_string_slice(&tx.rollup_fee_amounts)));
    array.push(serde_json::json!(is_neg(&unpacked_random_auditing_pk.0)));
    array.push(serde_json::json!(unpacked_random_auditing_pk.1.to_string()));
    array.push(serde_json::json!(auditor_public_key_x_signs));
    array.push(serde_json::json!(to_string_slice(&auditor_public_key_ys)));
    array.push(serde_json::json!(encrypted_commitment_shares));
    array.push(serde_json::json!(to_string_slice(&tx.in_commitments)));
    array.push(serde_json::json!(to_string_slice(&in_amount)));
    array.push(serde_json::json!(to_string_slice(&in_random_p)));
    array.push(serde_json::json!(to_string_slice(&in_random_r)));
    array.push(serde_json::json!(to_string_slice(&in_random_s)));
    array.push(serde_json::json!(to_string_slice(&tx.in_verify_sks)));
    array.push(serde_json::json!(to_string_slice(&tx.in_verify_pks)));
    array.push(serde_json::json!(tx
        .path_elements
        .iter()
        .map(|n| to_string_slice(n))
        .collect::<Vec<_>>()));
    array.push(serde_json::json!(in_path_indices));
    array.push(serde_json::json!(to_string_slice(&tx.out_amounts)));
    array.push(serde_json::json!(to_string_slice(&tx.out_random_ps)));
    array.push(serde_json::json!(to_string_slice(&tx.out_random_rs)));
    array.push(serde_json::json!(to_string_slice(&tx.out_random_ss)));
    array.push(serde_json::json!(to_string_slice(&tx.out_verify_pks)));
    array.push(serde_json::json!(unpacked_random_auditing_pk.0.to_string()));
    array.push(serde_json::json!(to_string_slice(&auditor_public_key_xs)));
    array.push(serde_json::json!(random_auditing_sk.to_string()));
    array.push(serde_json::json!(coefficients));
    array.push(serde_json::json!(commitment_shares));
    let tx_param = serde_json::Value::Array(array).to_string();
    let proof = prove_by_file(
        &tx.program_file,
        &tx.abi_file,
        &tx.proving_key_file,
        &tx_param,
    )
    .unwrap();

    Ok(proof)
}
