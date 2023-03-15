extern crate mystiko_crypto;
extern crate num_bigint;

use num_bigint::BigInt;

use mystiko_crypto::constants::FIELD_SIZE;
use mystiko_crypto::utils::{calc_mod, random_big_int, random_bytes};

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
