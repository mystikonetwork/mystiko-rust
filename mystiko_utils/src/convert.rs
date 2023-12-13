use anyhow::Result;
use ethers_core::types::{U128, U256};
use num_bigint::BigUint;
use num_traits::{zero, FromBytes, NumCast, Zero};
use rust_decimal::Decimal;
use std::fmt::Display;
use std::ops::Mul;
use std::str::FromStr;

pub const DEFAULT_NUM_DECIMALS: u32 = 18;

pub fn decimal_to_number<T, S>(decimal: &S, num_decimals: Option<u32>) -> Result<T>
where
    S: Display,
    T: NumCast + Zero,
{
    let decimal = Decimal::from_str(&decimal.to_string())?;
    let divisor = Decimal::from_str(
        &BigUint::from(10u32)
            .pow(num_decimals.unwrap_or(DEFAULT_NUM_DECIMALS))
            .to_string(),
    )?;
    Ok(T::from(decimal / divisor).unwrap_or(zero::<T>()))
}

pub fn number_to_decimal<T>(number: T, num_decimals: Option<u32>) -> Result<Decimal>
where
    T: Display,
{
    let multiplier = Decimal::from_str(
        &BigUint::from(10u32)
            .pow(num_decimals.unwrap_or(DEFAULT_NUM_DECIMALS))
            .to_string(),
    )?;
    let base = Decimal::from_str(&number.to_string())?;
    Ok(base.mul(multiplier))
}

pub fn number_to_u256_decimal<T>(number: T, num_decimals: Option<u32>) -> Result<U256>
where
    T: Display,
{
    let decimal = number_to_decimal(number, num_decimals)?;
    Ok(U256::from_dec_str(&decimal.round().to_string())?)
}

pub fn number_to_biguint_decimal<T>(number: T, num_decimals: Option<u32>) -> Result<BigUint>
where
    T: Display,
{
    let decimal = number_to_decimal(number, num_decimals)?;
    Ok(BigUint::from_str(&decimal.round().to_string())?)
}

pub fn u256_to_biguint(u: &U256) -> BigUint {
    bytes_to_biguint(u256_to_bytes(u))
}

pub fn u256_to_bytes(u: &U256) -> Vec<u8> {
    let mut arr = [0u8; 32];
    u.to_little_endian(&mut arr[..]);
    arr.to_vec()
}

pub fn u256_to_fixed_bytes<const N: usize>(u: &U256) -> [u8; N] {
    let mut arr = [0u8; N];
    u.to_little_endian(&mut arr[..]);
    arr
}

pub fn u256_to_hex_string(u: &U256) -> String {
    format!("0x{:x}", u)
}

pub fn hex_string_to_u256<S>(hex_string: S) -> Result<U256>
where
    S: AsRef<str>,
{
    Ok(U256::from_str(hex_string.as_ref())?)
}

pub fn biguint_to_u256(b: &BigUint) -> U256 {
    bytes_to_u256(biguint_to_bytes(b))
}

pub fn biguint_to_u128(b: &BigUint) -> u128 {
    U128::from_little_endian(&biguint_to_bytes(b)).as_u128()
}

pub fn biguint_to_bytes(b: &BigUint) -> Vec<u8> {
    b.to_bytes_le()
}

pub fn biguint_str_to_bytes<S>(biguint_str: S) -> Result<Vec<u8>>
where
    S: AsRef<str>,
{
    let number = BigUint::from_str(biguint_str.as_ref())?;
    Ok(biguint_to_bytes(&number))
}

pub fn bytes_to_biguint<B>(b: B) -> BigUint
where
    B: AsRef<[u8]>,
{
    BigUint::from_bytes_le(b.as_ref())
}

pub fn bytes_to_u256<B>(b: B) -> U256
where
    B: AsRef<[u8]>,
{
    U256::from_little_endian(b.as_ref())
}

pub fn i128_to_bytes(i: i128) -> Vec<u8> {
    i.to_be_bytes().to_vec()
}

pub fn bytes_to_i128<B>(b: B) -> i128
where
    B: AsRef<[u8]>,
{
    let mut bytes = [0; 16]; // i128 occupies 16 bytes
    bytes.copy_from_slice(&b.as_ref()[..16]); // Copy the first 16 bytes from the input slice
    FromBytes::from_be_bytes(&bytes)
}

pub fn u128_to_bytes(u: u128) -> Vec<u8> {
    u.to_be_bytes().to_vec()
}

pub fn bytes_to_u128<B>(b: B) -> u128
where
    B: AsRef<[u8]>,
{
    let mut bytes = [0; 16];
    bytes.copy_from_slice(&b.as_ref()[..16]);
    FromBytes::from_be_bytes(&bytes)
}
