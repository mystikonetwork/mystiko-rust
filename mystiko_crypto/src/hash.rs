use crate::utils::fr_to_big_int;
use ff::PrimeField;
use hmac::{Hmac, Mac};
use num_bigint::{BigInt, Sign};
use poseidon_rs::{Fr, Poseidon};
use sha2::{Sha256, Sha512};
use sha3::{Digest, Keccak256};

pub fn poseidon_bigint(arr: &[BigInt]) -> BigInt {
    let mut arr_fr = vec![];
    for n in arr {
        let n_fr = Fr::from_str(&n.to_string()).unwrap();
        arr_fr.push(n_fr);
    }

    poseidon_fr(arr_fr)
}

pub fn poseidon_fr(arr: Vec<Fr>) -> BigInt {
    let poseidon = Poseidon::new();
    let ph = poseidon.hash(arr).unwrap();
    fr_to_big_int(&ph)
}

pub fn sha512(msg: &[u8]) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(msg);
    hasher.finalize().to_vec()
}

pub fn sha256(msg: &[u8]) -> BigInt {
    let mut hasher = Sha256::new();
    hasher.update(msg);
    let result = hasher.finalize();
    BigInt::from_bytes_be(Sign::Plus, &result)
}

pub fn hmac_sha256(key: &[u8], msg: &[u8]) -> Vec<u8> {
    type HmacSha256 = Hmac<Sha256>;
    let mut hmac = HmacSha256::new_from_slice(key).unwrap();
    hmac.update(msg);
    hmac.finalize().into_bytes().to_vec()
}

pub fn hmac_sha512(key: &[u8], msg: &[u8]) -> Vec<u8> {
    type HmacSha512 = Hmac<Sha512>;
    let mut hmac = HmacSha512::new_from_slice(key).unwrap();
    hmac.update(msg);
    hmac.finalize().into_bytes().to_vec()
}

pub fn keccak256(msg: &[u8]) -> [u8; 32] {
    Keccak256::digest(msg).into()
}
