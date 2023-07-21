use crate::constants::FIELD_SIZE;
use crate::utils::fr_to_bigint;
use crate::utils::mod_floor;
use blake2::Blake2b512;
use ff::PrimeField;
use hmac::{Hmac, Mac};
use num_bigint::BigUint;
use poseidon_rs::{Fr, Poseidon};
use sha2::{Sha256, Sha512};
use sha3::{Digest, Keccak256};

lazy_static! {
    static ref G_POSEIDON_INSTANCE: Poseidon = Poseidon::new();
}

pub fn poseidon_bigint(arr: &[BigUint]) -> BigUint {
    let arr_fr: Vec<Fr> = arr.iter().map(|n| Fr::from_str(&n.to_string()).unwrap()).collect();

    poseidon_fr(arr_fr.as_slice())
}

pub fn poseidon(arr: &[BigUint]) -> BigUint {
    assert!(arr.len() < 7);
    let hash = poseidon_bigint(arr);
    assert!(hash < FIELD_SIZE.clone());
    hash
}

pub fn poseidon_fr(arr: &[Fr]) -> BigUint {
    let ph = G_POSEIDON_INSTANCE.hash(arr.to_vec()).unwrap();
    fr_to_bigint(&ph)
}

pub fn sha512(msg: &[u8]) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(msg);
    hasher.finalize().to_vec()
}

pub fn sha256(msg: &[u8]) -> BigUint {
    let mut hasher = Sha256::new();
    hasher.update(msg);
    let result = hasher.finalize();
    BigUint::from_bytes_be(&result)
}

pub fn sha256_mod(msg: &[u8]) -> BigUint {
    let hash = sha256(msg);
    mod_floor(&hash, &FIELD_SIZE)
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

pub fn checksum(data: &str, salt: Option<&str>) -> String {
    let salt_str = match salt {
        Some(a) => {
            if a.is_empty() {
                "mystiko"
            } else {
                a
            }
        }
        _ => "mystiko",
    };

    let h = hmac_sha512(salt_str.as_bytes(), data.as_bytes());
    ff::hex::encode(h)
}

pub fn blake2b_512(msg: &str) -> Vec<u8> {
    let mut hasher = Blake2b512::new();
    hasher.update(msg);
    hasher.finalize().to_vec()
}

pub fn keccak256(msg: &[u8]) -> [u8; 32] {
    Keccak256::digest(msg).into()
}
