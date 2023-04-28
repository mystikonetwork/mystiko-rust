use ethers_core::types::U256;
use std::ops::Mul;

pub fn u256_to_f64(v: U256, decimal: u32) -> f64 {
    assert!(decimal <= 18);
    let divisor = 10_u64.pow(decimal) as f64;
    v.as_u128() as f64 / divisor
}

pub fn f64_to_u256(v: f64, decimal: u32) -> U256 {
    assert!(decimal <= 18);
    let scaled = v.mul(10_u64.pow(decimal) as f64).round() as u64;
    U256::from(scaled)
}

pub fn calc_token_precision(price: f64, decimal: u32, swap_precision: u32) -> U256 {
    let mut len = swap_precision;
    if price >= 1.0 {
        len += price.floor().to_string().len() as u32;
        if len >= decimal {
            len = 0;
        } else {
            len = decimal - len;
        }
    } else if len >= decimal {
        len = 0;
    } else {
        len = decimal - len;
    }

    U256::from(10).pow(U256::from(len))
}
