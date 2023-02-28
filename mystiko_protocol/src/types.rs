use crate::utils::{u128_to_fixed_bytes, u256_to_fixed_bytes};
use ethers::core::types::{U128, U256};
use mystiko_crypto::utils::random_bytes;

pub const VERIFY_PK_SIZE: usize = 32;
pub const VERIFY_SK_SIZE: usize = 32;
pub const ENC_PK_SIZE: usize = 33;
pub const ENC_SK_SIZE: usize = 32;
pub const AUDITOR_PK_SIZE: usize = 32;
pub const AUDITOR_SK_SIZE: usize = 32;
pub const RANDOM_SK_SIZE: usize = 16;
pub const AMOUNT_SIZE: usize = 32;
pub const DECRYPTED_NOTE_SIZE: usize = RANDOM_SK_SIZE * 3 + AMOUNT_SIZE;

pub const MERKLE_TREE_LEVELS: u32 = 20;
pub const NUM_OF_AUDITORS: u32 = 5;
pub const AUDITING_THRESHOLD: u32 = 3;

pub type VerifyPk = U256;
pub type VerifySk = U256;
pub type EncPk = [u8; ENC_PK_SIZE];
pub type EncSk = U256;
pub type SigPk = Vec<u8>;
pub type RandomSk = U128;
pub type TxAmount = U256;
pub type AuditingSk = U256;
pub type AuditingPk = U256;
pub type EncryptedShares = Vec<U256>;

pub fn generate_random_sk() -> RandomSk {
    let sk = random_bytes(RANDOM_SK_SIZE);
    let mut bytes = [0u8; RANDOM_SK_SIZE];
    bytes.copy_from_slice(&sk[..]);
    U128::from_little_endian(&bytes[..])
}

#[derive(Debug, Clone)]
pub struct RandomSecrets {
    pub random_p: RandomSk,
    pub random_r: RandomSk,
    pub random_s: RandomSk,
}

impl RandomSecrets {
    pub fn generate() -> Self {
        Self {
            random_p: generate_random_sk(),
            random_r: generate_random_sk(),
            random_s: generate_random_sk(),
        }
    }
}

pub type EncryptedNote = Vec<u8>;

pub struct EncryptedData {
    pub sk_enc: EncSk,
    pub encrypted_note: EncryptedNote,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DecryptedNote {
    pub random_p: RandomSk,
    pub random_r: RandomSk,
    pub random_s: RandomSk,
    pub amount: TxAmount,
}

impl DecryptedNote {
    pub fn to_vec(&self) -> Vec<u8> {
        let mut n = vec![];

        n.extend(u128_to_fixed_bytes(&self.random_p));
        n.extend(u128_to_fixed_bytes(&self.random_r));
        n.extend(u128_to_fixed_bytes(&self.random_s));
        n.extend(u256_to_fixed_bytes(&self.amount));
        n
    }

    pub fn from_vec(n: Vec<u8>) -> Self {
        assert_eq!(n.len(), DECRYPTED_NOTE_SIZE);

        let mut chunks = n.chunks(RANDOM_SK_SIZE);
        let random_p = chunks.next().unwrap();
        let random_p = U128::from_little_endian(random_p);
        let random_r = chunks.next().unwrap();
        let random_r = U128::from_little_endian(random_r);
        let random_s = chunks.next().unwrap();
        let random_s = U128::from_little_endian(random_s);
        let amount = chunks.next().unwrap();
        let amount = U256::from_little_endian(amount);

        Self {
            random_p,
            random_r,
            random_s,
            amount,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypted_note() {
        let r = RandomSecrets::generate();
        let amount = U256::from(10);

        let note = DecryptedNote {
            random_p: r.random_p,
            random_r: r.random_r,
            random_s: r.random_s,
            amount,
        };

        let enc_v = note.to_vec();
        let note_s = DecryptedNote::from_vec(enc_v);
        assert_eq!(note, note_s);
    }
}
