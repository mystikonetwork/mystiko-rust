use crate::types::{RandomSk, SigPk, VerifySk};
use mystiko_crypto::hash::poseidon;
use num_bigint::{BigInt, Sign};

pub fn serial_number(sk_verify: &VerifySk, random_p: &RandomSk) -> BigInt {
    let nullifier_key = poseidon(&[sk_verify.clone()]);
    poseidon(&[random_p.clone(), nullifier_key])
}

pub fn sig_pk_hash(sig_pk: &SigPk, secret_key: &VerifySk) -> BigInt {
    let pk = BigInt::from_bytes_be(Sign::Plus, sig_pk);
    poseidon(&[secret_key.clone(), pk])
}
