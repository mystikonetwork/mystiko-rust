use num_bigint::BigInt;

pub const VERIFY_PK_SIZE: usize = 32;
pub const VERIFY_SK_SIZE: usize = 32;
pub const ENC_PK_SIZE: usize = 33;
pub const ENC_SK_SIZE: usize = 32;
pub const RANDOM_SK_SIZE: usize = 16;
pub const AMOUNT_SIZE: usize = 32;
pub const MERKLE_TREE_LEVELS: usize = 20;
pub const NUM_OF_AUDITORS: usize = 5;
pub const AUDITING_THRESHOLD: usize = 3;

lazy_static! {
    pub static ref EC_GROUP_ORDER: BigInt = BigInt::parse_bytes(
        b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        16,
    )
    .unwrap();
}
