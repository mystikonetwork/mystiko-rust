use ff::*;
use num_bigint::{BigInt, RandBigInt, Sign, ToBigInt};
use num_integer::Integer;
use poseidon_rs::{Fr, Poseidon};

pub fn fr_to_big_int(fr: &Fr) -> BigInt {
    BigInt::parse_bytes(to_hex(fr).as_bytes(), 16).unwrap()
}

pub fn poseidon_hash(arr: Vec<Fr>) -> BigInt {
    let poseidon = Poseidon::new();
    let ph = poseidon.hash(arr).unwrap();
    fr_to_big_int(&ph)
}

pub fn calc_mod(a_number: BigInt, prime: &BigInt) -> BigInt {
    a_number.mod_floor(prime)
}

pub fn random(size: usize, prime: &BigInt) -> BigInt {
    assert!(size < 1024);

    let mut rng = rand::thread_rng();
    let sk_raw = rng.gen_biguint(1024).to_bigint().unwrap();
    let (_, sk_raw_bytes) = sk_raw.to_bytes_le();
    let sk_raw_bytes = sk_raw_bytes[..size].to_vec();
    let random_bigint = BigInt::from_bytes_le(Sign::Plus, &sk_raw_bytes);

    random_bigint.mod_floor(prime)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mystiko_utils::constants::FIELD_SIZE;

    #[test]
    fn test_mod() {
        let field = FIELD_SIZE.clone();
        assert_eq!(calc_mod(BigInt::from(-1), &field), field - 1);
    }
}
