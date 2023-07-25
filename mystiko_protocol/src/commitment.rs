use crate::address::ShieldedAddress;
use crate::error::ProtocolError;
use crate::types::TxAmount;
use crate::types::{EncSk, RandomSk, DECRYPTED_NOTE_SIZE, RANDOM_SK_SIZE};
use anyhow::Result;
use mystiko_crypto::crypto::{decrypt_asymmetric, encrypt_asymmetric};
use mystiko_crypto::hash::poseidon;
use mystiko_crypto::utils::{biguint_to_32_bytes, random_bytes};
use num_bigint::BigUint;
use num_traits::identities::Zero;

pub type EncryptedNote = Vec<u8>;

#[derive(Debug, Clone)]
pub struct EncryptedData {
    pub sk_enc: EncSk,
    pub encrypted_note: EncryptedNote,
}

fn generate_random_sk() -> RandomSk {
    let sk = random_bytes(RANDOM_SK_SIZE);
    sk.try_into().unwrap()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub random_p: RandomSk,
    pub random_r: RandomSk,
    pub random_s: RandomSk,
    pub amount: TxAmount,
}

impl Note {
    pub fn new(amount: Option<BigUint>, r: Option<(RandomSk, RandomSk, RandomSk)>) -> Self {
        let amount = amount.unwrap_or(BigUint::zero());
        let (random_p, random_r, random_s) = match r {
            Some(r) => (r.0, r.1, r.2),
            None => (generate_random_sk(), generate_random_sk(), generate_random_sk()),
        };

        Self {
            random_p,
            random_r,
            random_s,
            amount,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut note_bytes = vec![];

        note_bytes.extend(&self.random_p);
        note_bytes.extend(&self.random_r);
        note_bytes.extend(&self.random_s);
        note_bytes.extend(biguint_to_32_bytes(&self.amount));
        note_bytes
    }

    pub fn from_vec(note_bytes: Vec<u8>) -> Result<Self, ProtocolError> {
        if note_bytes.len() != DECRYPTED_NOTE_SIZE {
            return Err(ProtocolError::InvalidNoteSize);
        }

        let mut chunks = note_bytes.chunks(RANDOM_SK_SIZE);
        let random_p = chunks.next().unwrap();
        let random_r = chunks.next().unwrap();
        let random_s = chunks.next().unwrap();
        let amount = chunks.next().unwrap();
        let amount = BigUint::from_bytes_le(amount);

        Ok(Self {
            random_p: random_p.try_into().unwrap(),
            random_r: random_r.try_into().unwrap(),
            random_s: random_s.try_into().unwrap(),
            amount,
        })
    }

    pub fn random_p_big(&self) -> BigUint {
        BigUint::from_bytes_le(&self.random_p)
    }

    pub fn random_r_big(&self) -> BigUint {
        BigUint::from_bytes_le(&self.random_r)
    }

    pub fn random_s_big(&self) -> BigUint {
        BigUint::from_bytes_le(&self.random_s)
    }
}

#[derive(Clone, Debug)]
pub struct Commitment {
    pub encrypted_note: EncryptedNote,
    pub note: Note,
    pub shielded_address: ShieldedAddress,
    pub commitment_hash: BigUint,
    pub k: BigUint,
}

impl Commitment {
    pub fn new(
        shielded_address: ShieldedAddress,
        note: Option<Note>,
        encrypted_note: Option<EncryptedData>,
    ) -> Result<Self, ProtocolError> {
        let (pk_verify, pk_enc) = shielded_address.public_key();
        let note = if let Some(n) = encrypted_note {
            let data = decrypt_asymmetric(&n.sk_enc, &n.encrypted_note)?;
            Note::from_vec(data)?
        } else {
            note.unwrap_or(Note::new(None, None))
        };

        let shielded_address = ShieldedAddress::from_public_key(&pk_verify, &pk_enc);
        let encrypted_note = encrypt_asymmetric(&pk_enc, note.to_vec().as_slice())?;
        let pk_big = BigUint::from_bytes_le(&pk_verify);
        let k = poseidon(&[pk_big, note.random_p_big(), note.random_r_big()]);
        let commitment_hash = poseidon(&[k.clone(), note.amount.clone(), note.random_s_big()]);

        let commitment = Self {
            note,
            encrypted_note,
            shielded_address,
            commitment_hash,
            k,
        };
        Ok(commitment)
    }
}
