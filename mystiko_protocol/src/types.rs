use mystiko_crypto::utils::{big_int_to_16_bytes, big_int_to_32_bytes, random_bytes};
use num_bigint::{BigInt, Sign};

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

pub type VerifyPk = BigInt;
pub type VerifySk = BigInt;
pub type EncPk = [u8; ENC_PK_SIZE];
pub type EncSk = BigInt;
pub type SigPk = Vec<u8>;
pub type RandomSk = BigInt;
pub type TxAmount = BigInt;
pub type AuditingSk = BigInt;
pub type AuditingPk = BigInt;
pub type EncryptedShares = Vec<BigInt>;

pub fn generate_random_sk() -> RandomSk {
    let sk = random_bytes(RANDOM_SK_SIZE);
    let mut bytes = [0u8; RANDOM_SK_SIZE];
    bytes.copy_from_slice(&sk[..]);
    BigInt::from_bytes_le(Sign::Plus, &bytes[..])
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
