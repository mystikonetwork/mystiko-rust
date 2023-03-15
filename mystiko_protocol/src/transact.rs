use crate::commitment::{EncryptedNote, Note};
use crate::error::ProtocolError;
use crate::types::{
    AuditingPk, AuditingSk, EncPk, EncSk, RandomSk, SigPk, TxAmount, VerifyPk, VerifySk,
};
use crate::types::{AUDITING_THRESHOLD, DECRYPTED_NOTE_SIZE, NUM_OF_AUDITORS};
use crate::utils::{serial_number, sig_pk_hash};
use ff::hex;
use mystiko_crypto::constants::FIELD_SIZE;
use mystiko_crypto::crypto::decrypt_asymmetric;
use mystiko_crypto::ecies;
use mystiko_crypto::shamir;
use mystiko_crypto::utils::big_int_to_32_bytes;
use mystiko_crypto::zkp::proof::ZKProof;
use num_bigint::BigInt;
use std::ops::Shr;

fn is_neg(x: &BigInt) -> bool {
    let field_size_half: BigInt = FIELD_SIZE.clone().shr(1);
    x.gt(&field_size_half)
}

fn to_string_slice(v: &[BigInt]) -> Vec<String> {
    v.iter().map(|n| n.to_string()).collect()
}

#[derive(Debug, Clone)]
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

impl Transaction {
    // todo do more check
    fn check(&self) -> Result<(), ProtocolError> {
        assert_eq!(self.num_outputs as usize, self.out_amounts.len());
        assert_eq!(self.num_inputs as usize, self.in_enc_pks.len());
        Ok(())
    }
    //
    // // todo return error
    // pub fn check_tx_call_param(&self) {
    //     // let random_public_key =
    //     //     public_key_from_unpack_point(&self .random_public_key_x, &self .random_public_key_y);
    //     // assert_eq!(
    //     //     random_public_key,
    //     //     ecies::public_key(&u256_to_big_int(&self .random_secret_key))
    //     // );
    //     // assert_eq!(
    //     //     self .random_public_key_x_sign,
    //     //     is_neg(&u256_to_big_int(&self .random_public_key_x))
    //     // );
    //
    //     for i in 0..self .in_commitments.len() {
    //         assert_eq!(
    //             self .auditor_public_key_x_signs[i],
    //             is_neg(&u256_to_big_int(&self .auditor_public_key_xs[i]))
    //         );
    //
    //         let mut points: Vec<shamir::Point> = vec![];
    //
    //         for j in 0..self .encrypted_commitment_shares[i].len() {
    //             let share = u256_to_big_int(&self .encrypted_commitment_shares[i][j]);
    //             let pk = public_key_from_unpack_point(
    //                 &u256_to_big_int(&self .auditor_public_key_xs[j]),
    //                 &u256_to_big_int(&self .auditor_public_key_ys[j]),
    //             );
    //
    //             let decrypted_share =
    //                 ecies::decrypt(&share, &u256_to_big_int(&self .random_secret_key), &pk);
    //             assert_eq!(
    //                 decrypted_share.clone(),
    //                 u256_to_big_int(&self .commitment_shares[i][j])
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
    //         assert_eq!(big_int_to_u256(&raw), self .in_commitments[i]);
    //         assert_eq!(big_int_to_u256(&raw), self .coefficients[i][0]);
    //     }
    // }

    pub async fn prove(&self) -> Result<ZKProof, ProtocolError> {
        // todo add logs
        self.check()?;

        let mut in_random_p = vec![];
        let mut in_random_r = vec![];
        let mut in_random_s = vec![];
        let mut in_amount = vec![];
        let mut serial_numbers = vec![];
        let mut sig_hashes = vec![];

        for i in 0..self.num_inputs as usize {
            let note = decrypt_asymmetric(
                &big_int_to_32_bytes(&self.in_enc_sks[i]),
                self.in_private_notes[i].as_slice(),
            )
            .unwrap();
            assert_eq!(note.len(), DECRYPTED_NOTE_SIZE);
            let note = Note::from_vec(note);
            in_random_p.push(note.random_p.clone());
            in_random_r.push(note.random_r.clone());
            in_random_s.push(note.random_s.clone());
            in_amount.push(note.amount);
            serial_numbers.push(serial_number(&self.in_verify_sks[i], &note.random_p));
            sig_hashes.push(sig_pk_hash(&self.sig_pk, &self.in_verify_sks[i]));
        }

        let random_auditing_sk = if let Some(key) = self.random_auditing_secret_key.clone() {
            key
        } else {
            ecies::generate_secret_key()
        };

        let random_auditing_pk = ecies::public_key(&random_auditing_sk);
        let unpacked_random_auditing_pk = ecies::unpack_public_key(&random_auditing_pk);
        let mut auditor_public_key_x_signs = vec![];
        let mut auditor_public_key_xs = vec![];
        let mut auditor_public_key_ys = vec![];
        for i in 0..self.auditor_public_keys.len() {
            let pk = &self.auditor_public_keys[i];
            let (unpacked_key_x, unpacked_key_y) = ecies::unpack_public_key(pk);
            auditor_public_key_x_signs.push(is_neg(&unpacked_key_x));
            auditor_public_key_xs.push(unpacked_key_x);
            auditor_public_key_ys.push(unpacked_key_y);
        }

        let mut coefficients = vec![];
        let mut commitment_shares = vec![];
        let mut encrypted_commitment_shares = vec![];
        for i in 0..self.in_commitments.len() {
            let ss = shamir::split(
                self.in_commitments[i].clone(),
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
                let pk = self.auditor_public_keys[j].clone();
                let encrypted_share = ecies::encrypt(&ss.shares[j].y, &pk, &random_auditing_sk);
                encrypted_shares.push(encrypted_share.to_string());
            }
            let encrypted_shares: [String; NUM_OF_AUDITORS as usize] =
                encrypted_shares.try_into().unwrap();
            encrypted_commitment_shares.push(encrypted_shares);
        }

        let in_path_indices: Vec<_> = self
            .path_indices
            .clone()
            .iter_mut()
            .map(|p| p.iter_mut().map(|&mut x| x != 0).collect::<Vec<_>>())
            .collect();

        let mut array: Vec<serde_json::Value> = vec![serde_json::json!(self.tree_root.to_string())];
        array.push(serde_json::json!(to_string_slice(&serial_numbers)));
        array.push(serde_json::json!(to_string_slice(&sig_hashes)));
        array.push(serde_json::json!(hex::encode(self.sig_pk.clone())));
        array.push(serde_json::json!(self.public_amount.to_string()));
        array.push(serde_json::json!(self.relayer_fee_amount.to_string()));
        array.push(serde_json::json!(to_string_slice(&self.out_commitments)));
        array.push(serde_json::json!(to_string_slice(&self.rollup_fee_amounts)));
        array.push(serde_json::json!(is_neg(&unpacked_random_auditing_pk.0)));
        array.push(serde_json::json!(unpacked_random_auditing_pk.1.to_string()));
        array.push(serde_json::json!(auditor_public_key_x_signs));
        array.push(serde_json::json!(to_string_slice(&auditor_public_key_ys)));
        array.push(serde_json::json!(encrypted_commitment_shares));
        array.push(serde_json::json!(to_string_slice(&self.in_commitments)));
        array.push(serde_json::json!(to_string_slice(&in_amount)));
        array.push(serde_json::json!(to_string_slice(&in_random_p)));
        array.push(serde_json::json!(to_string_slice(&in_random_r)));
        array.push(serde_json::json!(to_string_slice(&in_random_s)));
        array.push(serde_json::json!(to_string_slice(&self.in_verify_sks)));
        array.push(serde_json::json!(to_string_slice(&self.in_verify_pks)));
        array.push(serde_json::json!(self
            .path_elements
            .iter()
            .map(|n| to_string_slice(n))
            .collect::<Vec<_>>()));
        array.push(serde_json::json!(in_path_indices));
        array.push(serde_json::json!(to_string_slice(&self.out_amounts)));
        array.push(serde_json::json!(to_string_slice(&self.out_random_ps)));
        array.push(serde_json::json!(to_string_slice(&self.out_random_rs)));
        array.push(serde_json::json!(to_string_slice(&self.out_random_ss)));
        array.push(serde_json::json!(to_string_slice(&self.out_verify_pks)));
        array.push(serde_json::json!(unpacked_random_auditing_pk.0.to_string()));
        array.push(serde_json::json!(to_string_slice(&auditor_public_key_xs)));
        array.push(serde_json::json!(random_auditing_sk.to_string()));
        array.push(serde_json::json!(coefficients));
        array.push(serde_json::json!(commitment_shares));
        let tx_param = serde_json::Value::Array(array).to_string();
        let proof = ZKProof::generate_with_file(
            &self.program_file,
            &self.abi_file,
            &self.proving_key_file,
            &tx_param,
        )
        .await
        .unwrap();

        Ok(proof)
    }
}
