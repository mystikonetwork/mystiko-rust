use mystiko_utils::convert::{decimal_to_number, number_to_decimal};
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use rust_decimal::Decimal;

#[test]
fn test_decimal_to_number() {
    assert_eq!(
        decimal_to_number::<u32, String>(&String::from("1000000000000000000"), None).unwrap(),
        1u32
    );
    assert_eq!(decimal_to_number::<f64, u32>(&1, Some(4)).unwrap(), 0.0001);
    assert_eq!(
        decimal_to_number::<f64, BigInt>(&BigInt::from(-10), Some(3)).unwrap(),
        -0.01
    );
    assert_eq!(
        decimal_to_number::<f32, Decimal>(&Decimal::from(-100), Some(5)).unwrap(),
        -0.001
    );
}

#[test]
fn test_number_to_decimal() {
    assert!(number_to_decimal(0, None).unwrap().is_zero());
    assert_eq!(
        number_to_decimal(2, Some(4)).unwrap().to_i32().unwrap(),
        20000
    );
    assert_eq!(
        number_to_decimal(0.1, Some(2)).unwrap().to_i32().unwrap(),
        10
    );
    assert_eq!(
        number_to_decimal(0.01, Some(4)).unwrap().to_i32().unwrap(),
        100
    );
    assert_eq!(
        number_to_decimal(-0.01, Some(5)).unwrap().to_i32().unwrap(),
        -1000
    );
    assert_eq!(number_to_decimal(1e-18, None).unwrap().to_i32().unwrap(), 1);
}
