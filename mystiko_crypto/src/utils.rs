use babyjubjub_rs::{decompress_point, Point};
use ff::*;
use num_bigint::{BigInt, RandBigInt, Sign};
use num_integer::Integer;
use poseidon_rs::Fr;
use rand::{distributions::Alphanumeric, Rng};
use std::cmp::min;

pub fn fr_to_big_int(fr: &Fr) -> BigInt {
    BigInt::parse_bytes(to_hex(fr).as_bytes(), 16).unwrap()
}

pub fn big_int_to_be_32_bytes(num: &BigInt) -> [u8; 32] {
    let (_, y_bytes) = num.to_bytes_be();
    if y_bytes.len() >= 32 {
        y_bytes[..32].try_into().unwrap()
    } else {
        let mut arr: [u8; 32] = [0; 32];
        let len = min(y_bytes.len(), arr.len());
        arr[(32 - len)..].copy_from_slice(&y_bytes[..len]);
        arr
    }
}

pub fn big_int_to_32_bytes(num: &BigInt) -> [u8; 32] {
    let (_, y_bytes) = num.to_bytes_le();
    if y_bytes.len() >= 32 {
        y_bytes[..32].try_into().unwrap()
    } else {
        let mut arr: [u8; 32] = [0; 32];
        let len = min(y_bytes.len(), arr.len());
        arr[..len].copy_from_slice(&y_bytes[..len]);
        arr
    }
}

pub fn big_int_to_16_bytes(num: &BigInt) -> [u8; 16] {
    let (_, y_bytes) = num.to_bytes_le();
    if y_bytes.len() >= 16 {
        y_bytes[..16].try_into().unwrap()
    } else {
        let mut arr: [u8; 16] = [0; 16];
        let len = min(y_bytes.len(), arr.len());
        arr[..len].copy_from_slice(&y_bytes[..len]);
        arr
    }
}

pub fn babyjubjub_unpack_point(key: &BigInt) -> Point {
    let arr = big_int_to_32_bytes(key);
    decompress_point(arr).unwrap()
}

pub fn babyjubjub_public_key(x: &BigInt, y: &BigInt) -> BigInt {
    let point = Point {
        x: Fr::from_str(&x.to_string()).unwrap(),
        y: Fr::from_str(&y.to_string()).unwrap(),
    };
    let pc = point.compress();
    BigInt::from_bytes_le(Sign::Plus, &pc)
}

pub fn calc_mod(a_number: BigInt, prime: &BigInt) -> BigInt {
    a_number.mod_floor(prime)
}

pub fn random_big_int(size: usize, prime: &BigInt) -> BigInt {
    assert!(size < 256);

    let mut rng = rand::thread_rng();
    let sk_raw = rng.gen_biguint(256 * 8).to_bytes_le();
    let sk_raw_bytes = sk_raw[..size].to_vec();
    let random_bigint = BigInt::from_bytes_le(Sign::Plus, &sk_raw_bytes);

    random_bigint.mod_floor(prime)
}

pub fn random_bytes(size: usize) -> Vec<u8> {
    assert!(size < 1024);

    let mut rng = rand::thread_rng();
    let mut sk_raw = rng.gen_biguint(1024).to_bytes_le();
    sk_raw.drain(size..);
    sk_raw
}

pub fn random_utf8_string(size: usize) -> String {
    let data: Vec<u8> = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .collect();
    let utf_chars: Vec<char> = data
        .into_iter()
        .filter_map(|b| std::char::from_u32(b as u32))
        .filter(|c| c.is_ascii())
        .collect();
    utf_chars.into_iter().collect()
}
