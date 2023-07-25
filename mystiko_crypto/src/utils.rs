use babyjubjub_rs::{decompress_point, Point};
use ff::*;
use num_bigint::BigUint;
use num_integer::Integer;
use poseidon_rs::Fr;
use rand::{distributions::Alphanumeric, Rng, RngCore};
use std::cmp::min;
use std::convert::TryInto;

pub fn fr_to_bytes(fr: &Fr) -> [u8; 32] {
    let b = fr_to_bigint(fr);
    biguint_to_32_bytes(&b)
}

pub fn fr_to_bigint(fr: &Fr) -> BigUint {
    BigUint::parse_bytes(to_hex(fr).as_bytes(), 16).unwrap()
}

pub fn biguint_to_be_32_bytes(num: &BigUint) -> [u8; 32] {
    let y_bytes = num.to_bytes_be();
    if y_bytes.len() >= 32 {
        y_bytes[..32].try_into().unwrap()
    } else {
        let mut arr: [u8; 32] = [0; 32];
        let len = min(y_bytes.len(), arr.len());
        arr[(32 - len)..].copy_from_slice(&y_bytes[..len]);
        arr
    }
}

pub fn biguint_to_32_bytes(num: &BigUint) -> [u8; 32] {
    let y_bytes = num.to_bytes_le();
    if y_bytes.len() >= 32 {
        y_bytes[..32].try_into().unwrap()
    } else {
        let mut arr: [u8; 32] = [0; 32];
        let len = min(y_bytes.len(), arr.len());
        arr[..len].copy_from_slice(&y_bytes[..len]);
        arr
    }
}

pub fn babyjubjub_unpack_point(key: &[u8]) -> Point {
    decompress_point(key.try_into().unwrap()).unwrap()
}

pub fn babyjubjub_public_key(x: &[u8], y: &[u8]) -> [u8; 32] {
    let x_bigint = BigUint::from_bytes_le(x);
    let y_bigint = BigUint::from_bytes_le(y);

    let point = Point {
        x: Fr::from_str(&x_bigint.to_string()).unwrap(),
        y: Fr::from_str(&y_bigint.to_string()).unwrap(),
    };
    point.compress()
}

pub fn mod_floor(a_number: &BigUint, prime: &BigUint) -> BigUint {
    a_number.mod_floor(prime)
}

pub fn random_biguint(size: usize, prime: &BigUint) -> BigUint {
    let bytes = random_bytes(size);
    let random_bigint = BigUint::from_bytes_le(&bytes);
    mod_floor(&random_bigint, prime)
}

pub fn random_bytes(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut bytes = vec![0u8; size];
    rng.fill_bytes(&mut bytes);
    bytes
}

pub fn random_utf8_string(size: usize) -> String {
    let data: Vec<u8> = rand::thread_rng().sample_iter(&Alphanumeric).take(size).collect();
    let utf_chars: Vec<char> = data
        .into_iter()
        .filter_map(|b| std::char::from_u32(b as u32))
        .filter(|c| c.is_ascii())
        .collect();
    utf_chars.into_iter().collect()
}
