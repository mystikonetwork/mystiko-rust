use ff::*;
use num_bigint::BigUint;
use poseidon_rs::{Fr, Poseidon};

pub fn fr_to_big_uint(fr: &Fr) -> BigUint {
    let ph_str = fr.into_repr().to_string();
    let hex_string = ph_str.trim_start_matches("0x");
    let hex_bytes = hex::decode(hex_string).unwrap();
    BigUint::parse_bytes(&hex_bytes, 16).unwrap()
}

pub fn big_uint_to_arr(num: &BigUint) -> [u8; 32] {
    let bytes = num.to_bytes_be();
    let mut arr: [u8; 32] = [0; 32];
    arr[32 - bytes.len()..].copy_from_slice(&bytes);
    arr
}

pub fn poseidon_hash(arr: Vec<Fr>) -> BigUint {
    let poseidon = Poseidon::new();
    let ph = poseidon.hash(arr).unwrap();
    fr_to_big_uint(&ph)
}
