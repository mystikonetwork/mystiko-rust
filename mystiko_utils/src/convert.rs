use anyhow::Result;
use ethers_core::types::U256;
use num_bigint::{BigInt, Sign};
use num_traits::{zero, NumCast, Zero};
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
        &BigInt::from(10)
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
        &BigInt::from(10)
            .pow(num_decimals.unwrap_or(DEFAULT_NUM_DECIMALS))
            .to_string(),
    )?;
    let base = Decimal::from_str(&number.to_string())?;
    Ok(base.mul(multiplier))
}

pub fn u256_to_big_int(u: &U256) -> BigInt {
    let mut arr = [0u8; 32];
    u.to_little_endian(&mut arr[..]);
    bytes_to_big_int(&arr)
}

pub fn big_int_to_u256(b: &BigInt) -> U256 {
    U256::from_little_endian(&big_int_to_bytes(b))
}

pub fn big_int_to_bytes(b: &BigInt) -> Vec<u8> {
    b.to_bytes_le().1
}

pub fn bytes_to_big_int(b: &[u8]) -> BigInt {
    BigInt::from_bytes_le(Sign::Plus, b)
}
