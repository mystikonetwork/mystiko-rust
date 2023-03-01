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

fn big_int_to_fixed_bytes(num: &BigInt) -> [u8; 32] {
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

pub fn babyjubjub_unpack_point(key: &BigInt) -> Point {
    let arr = big_int_to_fixed_bytes(key);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::FIELD_SIZE;

    #[test]
    fn test_mod() {
        let field = FIELD_SIZE.clone();
        assert_eq!(calc_mod(BigInt::from(-1), &field), field - 1);
    }

    #[test]
    fn test_random() {
        let a = random_big_int(0, &FIELD_SIZE);
        assert_eq!(a, BigInt::from(0));

        let b = random_bytes(10);
        assert_eq!(b.len(), 10);
    }
}
