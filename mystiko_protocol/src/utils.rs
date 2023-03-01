use ethers::core::types::{U128, U256};
use num_bigint::{BigInt, Sign};

pub fn big_int_to_u256(b: &BigInt) -> U256 {
    U256::from_little_endian(b.to_bytes_le().1.as_slice())
}

pub fn u128_to_big_int(u: &U128) -> BigInt {
    let mut arr = [0u8; 16];
    u.to_little_endian(&mut arr[..]);
    BigInt::from_bytes_le(Sign::Plus, &arr[..])
}

pub fn u256_to_big_int(u: &U256) -> BigInt {
    let mut arr = [0u8; 32];
    u.to_little_endian(&mut arr[..]);
    BigInt::from_bytes_le(Sign::Plus, &arr[..])
}

pub fn u256_to_fixed_bytes(u: &U256) -> [u8; 32] {
    let mut arr = [0u8; 32];
    u.to_little_endian(&mut arr[..]);
    arr
}

pub fn u256_to_big_endian_fixed_bytes(u: &U256) -> [u8; 32] {
    let mut arr = [0u8; 32];
    u.to_big_endian(&mut arr[..]);
    arr
}

pub fn u128_to_fixed_bytes(u: &U128) -> [u8; 16] {
    let mut arr = [0u8; 16];
    u.to_little_endian(&mut arr[..]);
    arr
}
