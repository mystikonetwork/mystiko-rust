extern crate mystiko_crypto;
extern crate num_bigint;

use num_bigint::BigUint;

use mystiko_crypto::constants::FIELD_SIZE;
use mystiko_crypto::utils::{biguint_to_be_32_bytes, mod_floor, random_biguint, random_bytes};

#[tokio::test]
async fn test_mod() {
    let field = FIELD_SIZE.clone();
    assert_eq!(mod_floor(&BigUint::from(1u32), &field), BigUint::from(1u32));
}

#[tokio::test]
async fn test_random() {
    let a = random_biguint(0, &FIELD_SIZE);
    assert_eq!(a, BigUint::from(0u32));

    let be = biguint_to_be_32_bytes(&a);
    assert_eq!(be.len(), 32);

    let b = random_bytes(10);
    assert_eq!(b.len(), 10);
}
