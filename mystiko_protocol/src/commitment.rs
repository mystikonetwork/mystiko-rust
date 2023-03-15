use crate::address::ShieldedAddress;
use crate::error::ProtocolError;
use crate::types::TxAmount;
use crate::types::{EncSk, RandomSk, DECRYPTED_NOTE_SIZE, RANDOM_SK_SIZE};
use mystiko_crypto::crypto::{decrypt_asymmetric, encrypt_asymmetric};
use mystiko_crypto::hash::poseidon;
use mystiko_crypto::utils::{
    big_int_to_16_bytes, big_int_to_32_bytes, big_int_to_33_bytes, random_bytes,
};
use num_bigint::{BigInt, Sign};
use num_traits::identities::Zero;

pub type EncryptedNote = Vec<u8>;

#[derive(Debug, Clone)]
pub struct EncryptedData {
    pub sk_enc: EncSk,
    pub encrypted_note: EncryptedNote,
}

fn generate_random_sk() -> RandomSk {
    let sk = random_bytes(RANDOM_SK_SIZE);
    let mut bytes = [0u8; RANDOM_SK_SIZE];
    bytes.copy_from_slice(&sk[..]);
    BigInt::from_bytes_le(Sign::Plus, &bytes[..])
}

#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub random_p: RandomSk,
    pub random_r: RandomSk,
    pub random_s: RandomSk,
    pub amount: TxAmount,
}

impl Note {
    pub fn new(amount: Option<BigInt>, r: Option<(RandomSk, RandomSk, RandomSk)>) -> Self {
        let amount = amount.unwrap_or(BigInt::zero());
        let (random_p, random_r, random_s) = match r {
            Some(r) => (r.0, r.1, r.2),
            None => (
                generate_random_sk(),
                generate_random_sk(),
                generate_random_sk(),
            ),
        };

        Self {
            random_p,
            random_r,
            random_s,
            amount,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut n = vec![];

        n.extend(big_int_to_16_bytes(&self.random_p));
        n.extend(big_int_to_16_bytes(&self.random_r));
        n.extend(big_int_to_16_bytes(&self.random_s));
        n.extend(big_int_to_32_bytes(&self.amount));
        n
    }

    pub fn from_vec(n: Vec<u8>) -> Self {
        assert_eq!(n.len(), DECRYPTED_NOTE_SIZE);

        let mut chunks = n.chunks(RANDOM_SK_SIZE);
        let random_p = chunks.next().unwrap();
        let random_p = BigInt::from_bytes_le(Sign::Plus, random_p);
        let random_r = chunks.next().unwrap();
        let random_r = BigInt::from_bytes_le(Sign::Plus, random_r);
        let random_s = chunks.next().unwrap();
        let random_s = BigInt::from_bytes_le(Sign::Plus, random_s);
        let amount = chunks.next().unwrap();
        let amount = BigInt::from_bytes_le(Sign::Plus, amount);

        Self {
            random_p,
            random_r,
            random_s,
            amount,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Commitment {
    pub encrypted_note: EncryptedNote,
    pub note: Note,
    pub shielded_address: ShieldedAddress,
    pub commitment_hash: BigInt,
    pub k: BigInt,
}

impl Commitment {
    pub fn new(
        shielded_address: ShieldedAddress,
        note: Option<Note>,
        encrypted_note: Option<EncryptedData>,
    ) -> Result<Self, ProtocolError> {
        let (pk_verify, pk_enc) = shielded_address.public_key();
        let note = if let Some(n) = encrypted_note {
            decrypt_asymmetric(&big_int_to_32_bytes(&n.sk_enc), &n.encrypted_note)
                .map(Note::from_vec)
        } else {
            Ok(note.unwrap_or(Note::new(None, None)))
        }?;

        let shielded_address = ShieldedAddress::from_public_key(&pk_verify, &pk_enc);
        // todo check encrypt result
        let encrypted_note =
            encrypt_asymmetric(&big_int_to_33_bytes(&pk_enc), note.to_vec().as_slice()).unwrap();
        let k = poseidon(&[pk_verify, note.random_p.clone(), note.random_r.clone()]);
        let commitment_hash = poseidon(&[k.clone(), note.amount.clone(), note.random_s.clone()]);

        let s = Self {
            note,
            encrypted_note,
            shielded_address,
            commitment_hash,
            k,
        };
        Ok(s)
    }
}
