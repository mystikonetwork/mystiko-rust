use crate::types::{RandomSk, SigPk, VerifySk};
use mystiko_crypto::hash::poseidon;
use num_bigint::{BigInt, Sign};

pub fn compute_serial_number(sk_verify: &VerifySk, random_p: &RandomSk) -> BigInt {
    let sk = BigInt::from_bytes_le(Sign::Plus, sk_verify);
    let rp = BigInt::from_bytes_le(Sign::Plus, random_p);
    let nullifier_key = poseidon(&[sk]);
    poseidon(&[rp, nullifier_key])
}

pub fn compute_sig_pk_hash(sig_pk: &SigPk, secret_key: &VerifySk) -> BigInt {
    let pk = BigInt::from_bytes_be(Sign::Plus, sig_pk);
    let sk = BigInt::from_bytes_le(Sign::Plus, secret_key);
    poseidon(&[sk, pk])
}
