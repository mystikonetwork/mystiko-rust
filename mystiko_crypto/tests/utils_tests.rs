extern crate mystiko_crypto;
extern crate num_bigint;

use num_bigint::BigInt;

use mystiko_crypto::constants::FIELD_SIZE;
use mystiko_crypto::utils::{
    bigint_to_16_bytes, bigint_to_33_bytes, calc_mod, random_bigint, random_bytes,
};

#[tokio::test]
async fn test_mod() {
    let field = FIELD_SIZE.clone();
    assert_eq!(calc_mod(&BigInt::from(-1), &field), field - 1);
}

#[tokio::test]
async fn test_random() {
    let a = random_bigint(0, &FIELD_SIZE);
    assert_eq!(a, BigInt::from(0));

    let b = random_bytes(10);
    assert_eq!(b.len(), 10);
}

#[tokio::test]
async fn test_bigint_to_byes() {
    let a = random_bigint(15, &FIELD_SIZE);
    let a1_bytes = bigint_to_16_bytes(&a);
    let a2_bytes = bigint_to_33_bytes(&a);
    assert_eq!(a1_bytes[0..15], a2_bytes[0..15]);

    let b = random_bigint(32, &FIELD_SIZE);
    assert!(b.le(&FIELD_SIZE));
}
