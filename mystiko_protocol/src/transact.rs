use crate::commitment::{EncryptedNote, Note};
use crate::error::ProtocolError;
use crate::types::{AuditingPk, AuditingSk, EncPk, EncSk, RandomSk, SigPk, TxAmount, VerifyPk, VerifySk};
use crate::types::{AUDITING_THRESHOLD, DECRYPTED_NOTE_SIZE, NUM_OF_AUDITORS};
use crate::utils::{compute_nullifier, compute_sig_pk_hash};
use anyhow::Result;
use ff::hex;
use mystiko_crypto::constants::FIELD_SIZE;
use mystiko_crypto::crypto::decrypt_asymmetric;
use mystiko_crypto::ecies;
use mystiko_crypto::shamir;
use mystiko_crypto::zkp::proof::ZKProof;
use num_bigint::BigUint;
use std::ops::Shr;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct Transaction {
    pub num_inputs: u32,
    pub num_outputs: u32,
    pub in_verify_pks: Vec<VerifyPk>,
    pub in_verify_sks: Vec<VerifySk>,
    pub in_enc_pks: Vec<EncPk>,
    pub in_enc_sks: Vec<EncSk>,
    pub in_commitments: Vec<BigUint>,
    pub in_private_notes: Vec<EncryptedNote>,
    pub path_indices: Vec<Vec<usize>>,
    pub path_elements: Vec<Vec<BigUint>>,
    pub sig_pk: SigPk,
    pub tree_root: BigUint,
    pub public_amount: TxAmount,
    pub relayer_fee_amount: TxAmount,
    pub rollup_fee_amounts: Vec<TxAmount>,
    pub out_verify_pks: Vec<VerifyPk>,
    pub out_amounts: Vec<TxAmount>,
    pub out_commitments: Vec<BigUint>,
    pub out_random_ps: Vec<RandomSk>,
    pub out_random_rs: Vec<RandomSk>,
    pub out_random_ss: Vec<RandomSk>,
    pub program: Vec<u8>,
    pub abi: Vec<u8>,
    pub proving_key: Vec<u8>,
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

    pub fn prove(&self) -> Result<ZKProof, ProtocolError> {
        // todo add logs
        self.check()?;

        let mut in_random_p = vec![];
        let mut in_random_r = vec![];
        let mut in_random_s = vec![];
        let mut in_amount = vec![];
        let mut nullifiers = vec![];
        let mut sig_hashes = vec![];

        for i in 0..self.num_inputs as usize {
            let note = decrypt_asymmetric(&self.in_enc_sks[i], self.in_private_notes[i].as_slice())?;
            assert_eq!(note.len(), DECRYPTED_NOTE_SIZE);
            let note = Note::from_vec(note)?;
            in_random_p.push(note.random_p);
            in_random_r.push(note.random_r);
            in_random_s.push(note.random_s);
            in_amount.push(note.amount);
            nullifiers.push(compute_nullifier(&self.in_verify_sks[i], &note.random_p));
            sig_hashes.push(compute_sig_pk_hash(&self.sig_pk, &self.in_verify_sks[i]));
        }

        let random_auditing_sk = if let Some(key) = self.random_auditing_secret_key {
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
            let s_shares = shamir::split(
                self.in_commitments[i].clone(),
                NUM_OF_AUDITORS,
                AUDITING_THRESHOLD,
                None,
            )?;

            let cos = s_shares.coefficients.clone();
            let cos = bigint_slice_to_strings(&cos).to_owned();
            coefficients.push(cos.clone());

            let p_ys: Vec<String> = s_shares.shares.iter().map(|p| p.y.to_string()).collect();
            let p_ys: [String; NUM_OF_AUDITORS as usize] = p_ys.try_into().unwrap();
            commitment_shares.push(p_ys);

            let mut encrypted_shares = vec![];
            for j in 0..s_shares.shares.len() {
                let pk = self.auditor_public_keys[j];
                let encrypted_share = ecies::encrypt(&s_shares.shares[j].y, &pk, &random_auditing_sk);
                encrypted_shares.push(encrypted_share.to_string());
            }
            let encrypted_shares: [String; NUM_OF_AUDITORS as usize] = encrypted_shares.try_into().unwrap();
            encrypted_commitment_shares.push(encrypted_shares);
        }

        let in_path_indices: Vec<_> = self
            .path_indices
            .clone()
            .iter_mut()
            .map(|p| p.iter_mut().map(|&mut x| x != 0).collect::<Vec<_>>())
            .collect();

        let mut array: Vec<serde_json::Value> = vec![serde_json::json!(self.tree_root.to_string())];
        array.push(serde_json::json!(bigint_slice_to_strings(&nullifiers)));
        array.push(serde_json::json!(bigint_slice_to_strings(&sig_hashes)));
        array.push(serde_json::json!(hex::encode(self.sig_pk)));
        array.push(serde_json::json!(self.public_amount.to_string()));
        array.push(serde_json::json!(self.relayer_fee_amount.to_string()));
        array.push(serde_json::json!(bigint_slice_to_strings(&self.out_commitments)));
        array.push(serde_json::json!(bigint_slice_to_strings(&self.rollup_fee_amounts)));
        array.push(serde_json::json!(is_neg(&unpacked_random_auditing_pk.0)));
        array.push(serde_json::json!(BigUint::from_bytes_le(
            &unpacked_random_auditing_pk.1
        )
        .to_string()));
        array.push(serde_json::json!(auditor_public_key_x_signs));
        array.push(serde_json::json!(bytes_to_strings(&auditor_public_key_ys)));
        array.push(serde_json::json!(encrypted_commitment_shares));
        array.push(serde_json::json!(bigint_slice_to_strings(&self.in_commitments)));
        array.push(serde_json::json!(bigint_slice_to_strings(&in_amount)));
        array.push(serde_json::json!(bytes_to_strings(&in_random_p)));
        array.push(serde_json::json!(bytes_to_strings(&in_random_r)));
        array.push(serde_json::json!(bytes_to_strings(&in_random_s)));
        array.push(serde_json::json!(bytes_to_strings(&self.in_verify_sks)));
        array.push(serde_json::json!(bytes_to_strings(&self.in_verify_pks)));
        array.push(serde_json::json!(self
            .path_elements
            .iter()
            .map(|n| bigint_slice_to_strings(n))
            .collect::<Vec<_>>()));
        array.push(serde_json::json!(in_path_indices));
        array.push(serde_json::json!(bigint_slice_to_strings(&self.out_amounts)));
        array.push(serde_json::json!(bytes_to_strings(&self.out_random_ps)));
        array.push(serde_json::json!(bytes_to_strings(&self.out_random_rs)));
        array.push(serde_json::json!(bytes_to_strings(&self.out_random_ss)));
        array.push(serde_json::json!(bytes_to_strings(&self.out_verify_pks)));
        array.push(serde_json::json!(BigUint::from_bytes_le(
            &unpacked_random_auditing_pk.0
        )
        .to_string()));
        array.push(serde_json::json!(bytes_to_strings(&auditor_public_key_xs)));
        array.push(serde_json::json!(
            BigUint::from_bytes_le(&random_auditing_sk).to_string()
        ));
        array.push(serde_json::json!(coefficients));
        array.push(serde_json::json!(commitment_shares));
        let tx_param = serde_json::Value::Array(array).to_string();

        let proof = ZKProof::generate(
            self.program.as_slice(),
            self.abi.as_slice(),
            self.proving_key.as_slice(),
            &tx_param,
        )?;

        Ok(proof)
    }
}

fn is_neg(key: &[u8]) -> bool {
    let key_big_int = BigUint::from_bytes_le(key);
    let field_size_half: BigUint = FIELD_SIZE.clone().shr(1);
    key_big_int.gt(&field_size_half)
}

fn bigint_slice_to_strings(v: &[BigUint]) -> Vec<String> {
    v.iter().map(|n| n.to_string()).collect()
}

fn bytes_to_strings<T: AsRef<[u8]>>(v: &[T]) -> Vec<String> {
    v.iter()
        .map(|n| BigUint::from_bytes_le(n.as_ref()).to_string())
        .collect()
}
