extern crate mystiko_crypto;
extern crate num_bigint;

use num_bigint::BigUint;

use mystiko_crypto::constants::FIELD_SIZE;
use mystiko_crypto::error::SecretShareError;
use mystiko_crypto::shamir::{recover, split};
use mystiko_crypto::utils::{mod_floor, random_biguint};
use rand::prelude::SliceRandom;

#[tokio::test]
async fn test_secret_sharing() {
    let secret = random_biguint(32, &FIELD_SIZE);
    let result = split(secret.clone(), 0, 17, None);
    assert_eq!(result.err().unwrap(), SecretShareError::SharesOutOfBounds);

    let result = split(secret, 5, 7, None);
    assert_eq!(result.err().unwrap(), SecretShareError::ThresholdOutOfBounds);
}

#[tokio::test]
async fn test_secret_sharing1() {
    let secret = random_biguint(32, &FIELD_SIZE);
    let ss = split(secret.clone(), 30, 17, None).unwrap();
    let mut shares = ss.shares;
    shares.shuffle(&mut rand::thread_rng());
    let es = shares[0..17].to_vec();
    let recovered_secret = recover(es, None);
    assert_eq!(secret, recovered_secret);
}

#[tokio::test]
async fn test_secret_sharing2() {
    let mut secret = random_biguint(32, &FIELD_SIZE);
    let field: BigUint = BigUint::parse_bytes(b"58995116542422524421248775517049", 10).unwrap();
    secret = mod_floor(&secret, &field);

    let ss1 = split(secret.clone(), 5, 3, Some(field.clone())).unwrap();
    let ss = ss1;
    let mut shares = ss.shares;
    shares.shuffle(&mut rand::thread_rng());
    let es = shares[0..3].to_vec();
    let filed = Some(field);
    let recovered_secret = recover(es, filed);
    assert_eq!(secret, recovered_secret);
}
