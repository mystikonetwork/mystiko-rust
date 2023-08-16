use ethers_core::types::U256;
use mystiko_utils::convert::{
    biguint_str_to_bytes, bytes_to_i128, bytes_to_u128, decimal_to_number, i128_to_bytes, number_to_decimal,
    u128_to_bytes,
};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use rust_decimal::Decimal;
use std::str::FromStr;

#[test]
fn test_decimal_to_number() {
    assert_eq!(
        decimal_to_number::<u32, String>(&String::from("1000000000000000000"), None).unwrap(),
        1u32
    );
    assert_eq!(decimal_to_number::<f64, u32>(&1, Some(4)).unwrap(), 0.0001);
    assert_eq!(
        decimal_to_number::<f64, BigUint>(&BigUint::from(10u32), Some(3)).unwrap(),
        0.01
    );
    assert_eq!(
        decimal_to_number::<f32, Decimal>(&Decimal::from(-100), Some(5)).unwrap(),
        -0.001
    );
}

#[test]
fn test_number_to_decimal() {
    assert!(number_to_decimal(0, None).unwrap().is_zero());
    assert_eq!(number_to_decimal(2, Some(4)).unwrap().to_i32().unwrap(), 20000);
    assert_eq!(number_to_decimal(0.1, Some(2)).unwrap().to_i32().unwrap(), 10);
    assert_eq!(number_to_decimal(0.01, Some(4)).unwrap().to_i32().unwrap(), 100);
    assert_eq!(number_to_decimal(-0.01, Some(5)).unwrap().to_i32().unwrap(), -1000);
    assert_eq!(number_to_decimal(1e-18, None).unwrap().to_i32().unwrap(), 1);
}

#[test]
fn test_u256_to_big_int() {
    assert_eq!(
        mystiko_utils::convert::u256_to_biguint(&U256::from_dec_str("123456789").unwrap()),
        BigUint::from(123456789u32)
    );
}

#[test]
fn test_big_int_to_u256() {
    assert_eq!(
        mystiko_utils::convert::biguint_to_u256(&BigUint::from(123456789u32)),
        U256::from_dec_str("123456789").unwrap()
    );
}

#[test]
fn test_biguint_str_to_bytes() {
    assert_eq!(
        BigUint::from_bytes_le(&biguint_str_to_bytes("123456789").unwrap()),
        BigUint::from_str("123456789").unwrap()
    );
}

#[test]
fn test_i128_convert() {
    let value1 = 12345i128;
    let bytes = i128_to_bytes(value1);
    let value2 = bytes_to_i128(&bytes);
    assert_eq!(value1, value2);
}

#[test]
fn test_u128_convert() {
    let value1 = 12345u128;
    let bytes = u128_to_bytes(value1);
    let value2 = bytes_to_u128(&bytes);
    assert_eq!(value1, value2);
}
