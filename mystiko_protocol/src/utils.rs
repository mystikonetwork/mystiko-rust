use crate::types::{RandomSk, SigPk, VerifySk};
use mystiko_crypto::hash::poseidon;
use num_bigint::BigUint;

pub fn compute_nullifier(sk_verify: &VerifySk, random_p: &RandomSk) -> BigUint {
    let sk = BigUint::from_bytes_le(sk_verify);
    let rp = BigUint::from_bytes_le(random_p);
    let nullifier_key = poseidon(&[sk]);
    poseidon(&[rp, nullifier_key])
}

pub fn compute_sig_pk_hash(sig_pk: &SigPk, secret_key: &VerifySk) -> BigUint {
    let pk = BigUint::from_bytes_be(sig_pk);
    let sk = BigUint::from_bytes_le(secret_key);
    poseidon(&[sk, pk])
}
