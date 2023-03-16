use num_bigint::BigInt;

pub const VERIFY_PK_SIZE: usize = 32;
pub const VERIFY_SK_SIZE: usize = 32;
pub const ENC_PK_SIZE: usize = 33;
pub const ENC_SK_SIZE: usize = 32;
pub const FULL_PK_SIZE: usize = VERIFY_PK_SIZE + ENC_PK_SIZE;
pub const FULL_SK_SIZE: usize = VERIFY_SK_SIZE + ENC_SK_SIZE;
pub const AUDITOR_PK_SIZE: usize = 32;
pub const AUDITOR_SK_SIZE: usize = 32;
pub const RANDOM_SK_SIZE: usize = 16;
pub const SIG_PK_SIZE: usize = 20;
pub const AMOUNT_SIZE: usize = 32;
pub const DECRYPTED_NOTE_SIZE: usize = RANDOM_SK_SIZE * 3 + AMOUNT_SIZE;

pub const MERKLE_TREE_LEVELS: u32 = 20;
pub const NUM_OF_AUDITORS: u32 = 5;
pub const AUDITING_THRESHOLD: u32 = 3;

pub type VerifyPk = [u8; VERIFY_PK_SIZE];
pub type VerifySk = [u8; VERIFY_SK_SIZE];
pub type EncPk = [u8; ENC_PK_SIZE];
pub type EncSk = [u8; ENC_SK_SIZE];
pub type FullPk = [u8; FULL_PK_SIZE];
pub type FullSk = [u8; FULL_SK_SIZE];
pub type SigPk = [u8; SIG_PK_SIZE];
pub type RandomSk = [u8; RANDOM_SK_SIZE];
pub type TxAmount = BigInt;
pub type AuditingSk = [u8; AUDITOR_SK_SIZE];
pub type AuditingPk = [u8; AUDITOR_PK_SIZE];
pub type EncryptedShares = Vec<BigInt>;
