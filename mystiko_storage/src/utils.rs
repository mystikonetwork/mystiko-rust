use std::num::ParseIntError;

type Result<T> = anyhow::Result<T, ParseIntError>;

pub fn i64_to_comparable_string(number: i64) -> String {
    format!("{:016x}", number.wrapping_add(i64::MAX).wrapping_add(1))
}

pub fn comparable_string_to_i64(string: &str) -> Result<i64> {
    let number = u64::from_str_radix(string, 16)?;
    Ok(number.wrapping_sub(i64::MAX as u64).wrapping_sub(1) as i64)
}

pub fn u64_to_comparable_string(number: u64) -> String {
    format!("{:016x}", number)
}

pub fn comparable_string_to_u64(string: &str) -> Result<u64> {
    u64::from_str_radix(string, 16)
}

pub fn isize_to_comparable_string(number: isize) -> String {
    format!("{:016x}", number.wrapping_add(isize::MAX).wrapping_add(1))
}

pub fn comparable_string_to_isize(string: &str) -> Result<isize> {
    Ok(usize::from_str_radix(string, 16)?
        .wrapping_sub(isize::MAX as usize)
        .wrapping_sub(1) as isize)
}

pub fn usize_to_comparable_string(number: usize) -> String {
    format!("{:016x}", number)
}

pub fn comparable_string_to_usize(string: &str) -> Result<usize> {
    usize::from_str_radix(string, 16)
}

pub fn i128_to_comparable_string(number: i128) -> String {
    format!("{:032x}", number.wrapping_add(i128::MAX).wrapping_add(1))
}

pub fn comparable_string_to_i128(string: &str) -> Result<i128> {
    Ok(u128::from_str_radix(string, 16)?
        .wrapping_sub(i128::MAX as u128)
        .wrapping_sub(1) as i128)
}

pub fn u128_to_comparable_string(number: u128) -> String {
    format!("{:032x}", number)
}

pub fn comparable_string_to_u128(string: &str) -> Result<u128> {
    u128::from_str_radix(string, 16)
}
