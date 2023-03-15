extern crate mystiko_crypto;
extern crate num_bigint;

use num_bigint::BigInt;

use mystiko_crypto::constants::FIELD_SIZE;
use mystiko_crypto::utils::{
    big_int_to_16_bytes, big_int_to_33_bytes, calc_mod, random_big_int, random_bytes,
};

#[tokio::test]
async fn test_mod() {
    let field = FIELD_SIZE.clone();
    assert_eq!(calc_mod(&BigInt::from(-1), &field), field - 1);
}

#[tokio::test]
async fn test_random() {
    let a = random_big_int(0, &FIELD_SIZE);
    assert_eq!(a, BigInt::from(0));

    let b = random_bytes(10);
    assert_eq!(b.len(), 10);
}

#[tokio::test]
async fn test_big_int_to_byes() {
    let a = random_big_int(15, &FIELD_SIZE);
    let a_bytes = big_int_to_16_bytes(&a);
    let b_bytes = big_int_to_33_bytes(&a);
    assert_eq!(a_bytes[0..15], b_bytes[0..15]);
}
