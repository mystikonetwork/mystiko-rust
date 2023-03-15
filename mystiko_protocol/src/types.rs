use num_bigint::BigInt;

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
pub type EncPk = BigInt;
pub type EncSk = BigInt;
pub type SigPk = Vec<u8>;
pub type RandomSk = BigInt;
pub type TxAmount = BigInt;
pub type AuditingSk = BigInt;
pub type AuditingPk = BigInt;
pub type EncryptedShares = Vec<BigInt>;
