use ff::hex;
use num_bigint::BigUint;

pub const KDF_MAGIC_DATA_LENGTH: usize = 8;
pub const KDF_SALT_LENGTH: usize = 8;

pub const ECIES_KEY_LENGTH: usize = 32;
pub const ECIES_IV_LENGTH: usize = 16;
pub const ECIES_UNCOMPRESSED_PK_LENGTH: usize = 65;
pub const ECIES_MAC_LENGTH: usize = 32;
pub const ECIES_META_LENGTH: usize = ECIES_IV_LENGTH + ECIES_UNCOMPRESSED_PK_LENGTH + ECIES_MAC_LENGTH;

lazy_static! {
    pub static ref ECIES_MAGIC_DATA: Vec<u8> = hex::decode("53616c7465645f5f").unwrap();
    pub static ref FIELD_SIZE: BigUint = BigUint::parse_bytes(
        b"21888242871839275222246405745257275088548364400416034343698204186575808495617",
        10,
    )
    .unwrap();
}
