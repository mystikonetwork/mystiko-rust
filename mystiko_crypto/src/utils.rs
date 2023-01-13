use ff::*;
use num_bigint::BigUint;
use poseidon_rs::{Fr, Poseidon};

pub fn fr_to_big_uint(fr: &Fr) -> BigUint {
    BigUint::parse_bytes(to_hex(fr).as_bytes(), 16).unwrap()
}

pub fn poseidon_hash(arr: Vec<Fr>) -> BigUint {
    let poseidon = Poseidon::new();
    let ph = poseidon.hash(arr).unwrap();
    fr_to_big_uint(&ph)
}
