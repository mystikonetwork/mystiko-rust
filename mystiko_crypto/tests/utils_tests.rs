extern crate mystiko_crypto;
extern crate num_bigint;

use num_bigint::BigInt;

use mystiko_crypto::constants::FIELD_SIZE;
use mystiko_crypto::utils::{bigint_to_be_32_bytes, mod_floor, random_bigint, random_bytes};

#[tokio::test]
async fn test_mod() {
    let field = FIELD_SIZE.clone();
    assert_eq!(mod_floor(&BigInt::from(-1), &field), field - 1);
}

#[tokio::test]
async fn test_random() {
    let a = random_bigint(0, &FIELD_SIZE);
    assert_eq!(a, BigInt::from(0));

    let be = bigint_to_be_32_bytes(&a);
    assert_eq!(be.len(), 32);

    let b = random_bytes(10);
    assert_eq!(b.len(), 10);
}
