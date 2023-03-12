use crate::crypto::{decrypt_asymmetric, encrypt_asymmetric};
use crate::error::ProtocolError;
use crate::hash::poseidon_hash;
use crate::types::{DecryptedNote, EncryptedData, EncryptedNote, RandomSecrets};
use crate::types::{EncPk, RandomSk, SigPk, TxAmount, VerifyPk, VerifySk};
use crate::wallet::{public_key_from_shielded_address, shielded_address};
use mystiko_crypto::utils::big_int_to_32_bytes;
use num_bigint::{BigInt, Sign};
use num_traits::identities::Zero;

pub fn serial_number(sk_verify: VerifySk, random_p: RandomSk) -> BigInt {
    let nullifier_key = poseidon_hash(&[sk_verify]);
    poseidon_hash(&[random_p, nullifier_key])
}

pub fn sig_pk_hash(sig_pk: &SigPk, secret_key: VerifySk) -> BigInt {
    let pk = BigInt::from_bytes_be(Sign::Plus, sig_pk);
    poseidon_hash(&[secret_key, pk])
}

pub enum PublicKeys {
    String(String),
    Object { pk_verify: VerifyPk, pk_enc: EncPk },
}

pub struct CommitmentInput {
    pub public_keys: PublicKeys,
    pub amount: Option<TxAmount>,
    pub random_secrets: Option<RandomSecrets>,
    pub encrypted_note: Option<EncryptedData>,
}

pub struct CommitmentOutput {
    pub encrypted_note: EncryptedNote,
    pub shielded_address: String,
    pub commitment_hash: BigInt,
    pub amount: TxAmount,
    pub random_p: RandomSk,
    pub random_r: RandomSk,
    pub random_s: RandomSk,
    pub k: BigInt,
}

pub fn build_commitment(input: CommitmentInput) -> Result<CommitmentOutput, ProtocolError> {
    let (pk_verify, pk_enc) = match input.public_keys {
        PublicKeys::String(s) => public_key_from_shielded_address(s),
        PublicKeys::Object { pk_verify, pk_enc } => (pk_verify, pk_enc),
    };

    let note = if let Some(n) = input.encrypted_note {
        decrypt_asymmetric(&big_int_to_32_bytes(&n.sk_enc), &n.encrypted_note)
            .map(DecryptedNote::from_vec)
            .map_err(|e| ProtocolError::CryptoError(e.to_string()))?
    } else {
        let amount = input.amount.unwrap_or(BigInt::zero());
        let (random_p, random_r, random_s) = match input.random_secrets {
            Some(r) => (r.random_p, r.random_r, r.random_s),
            None => {
                let r = RandomSecrets::generate();
                (r.random_p, r.random_r, r.random_s)
            }
        };
        DecryptedNote {
            random_p,
            random_r,
            random_s,
            amount,
        }
    };

    let k = poseidon_hash(&[
        pk_verify.clone(),
        note.random_p.clone(),
        note.random_r.clone(),
    ]);
    let commitment_hash = poseidon_hash(&[k.clone(), note.amount.clone(), note.random_s.clone()]);
    // todo check encrypt result
    let encrypted_note = encrypt_asymmetric(&pk_enc, note.to_vec().as_slice()).unwrap();
    let shielded_address = shielded_address(&pk_verify, &pk_enc);
    Ok(CommitmentOutput {
        encrypted_note,
        shielded_address,
        amount: note.amount,
        commitment_hash,
        random_p: note.random_p,
        random_r: note.random_r,
        random_s: note.random_s,
        k,
    })
}
