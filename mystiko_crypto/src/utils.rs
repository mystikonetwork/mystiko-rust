use ff::*;
use num_bigint::BigInt;
use poseidon_rs::{Fr, Poseidon};

pub fn fr_to_big_int(fr: &Fr) -> BigInt {
    BigInt::parse_bytes(to_hex(fr).as_bytes(), 16).unwrap()
}

pub fn poseidon_hash(arr: Vec<Fr>) -> BigInt {
    let poseidon = Poseidon::new();
    let ph = poseidon.hash(arr).unwrap();
    fr_to_big_int(&ph)
}
