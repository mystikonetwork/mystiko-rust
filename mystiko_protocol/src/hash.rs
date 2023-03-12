use crate::mystiko_crypto::constants::FIELD_SIZE;
use crate::mystiko_crypto::hash::{hmac_sha512, poseidon_bigint, sha256 as sha256_raw};
use crate::mystiko_crypto::utils::calc_mod;
use num_bigint::BigInt;

pub fn sha256(msg: &[u8]) -> BigInt {
    let hash = sha256_raw(msg);
    calc_mod(hash, &FIELD_SIZE)
}

pub fn poseidon_hash(arr: &[BigInt]) -> BigInt {
    assert!(arr.len() < 7);
    let hash = poseidon_bigint(arr);
    assert!(hash < FIELD_SIZE.clone());
    hash
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
