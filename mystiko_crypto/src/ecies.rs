use crate::constants::{ECIES_KEY_LENGTH, FIELD_SIZE};
use crate::hash::poseidon_fr;
use crate::utils::bigint_to_32_bytes;
use crate::utils::fr_to_bytes;
use crate::utils::{babyjubjub_public_key, babyjubjub_unpack_point, mod_floor, random_bigint};
use babyjubjub_rs::Point;
use ff::*;
use lazy_static::lazy_static;
use num_bigint::{BigInt, Sign};

pub type Fr = poseidon_rs::Fr;
lazy_static! {
    static ref B8: Point = Point {
        x: Fr::from_str("5299619240641551281634865583518297030282874472190772894086521144482721001553",).unwrap(),
        y: Fr::from_str("16950150798460657717958625567821834550301663161624707787222815936182638968203",).unwrap(),
    };
}

pub fn generate_secret_key() -> [u8; 32] {
    let s = random_bigint(ECIES_KEY_LENGTH, &FIELD_SIZE);
    bigint_to_32_bytes(&s)
}

pub fn public_key(secret_key: &[u8]) -> [u8; 32] {
    let sk = BigInt::from_bytes_le(Sign::Plus, secret_key);
    B8.mul_scalar(&sk).compress()
}

pub fn unpack_public_key(public_key: &[u8]) -> ([u8; 32], [u8; 32]) {
    let point = babyjubjub_unpack_point(public_key);
    (fr_to_bytes(&point.x), fr_to_bytes(&point.y))
}

pub fn public_key_from_unpack_point(x: &[u8], y: &[u8]) -> [u8; 32] {
    babyjubjub_public_key(x, y)
}

pub fn encrypt(plain: &BigInt, pk: &[u8], common_sk: &[u8]) -> BigInt {
    let point_pk = babyjubjub_unpack_point(pk);
    let sk = BigInt::from_bytes_le(Sign::Plus, common_sk);
    let k = point_pk.mul_scalar(&sk);
    let hm = poseidon_fr(&[k.x, k.y]);

    mod_floor(&(plain.clone() + hm), &FIELD_SIZE)
}

pub fn decrypt(encrypted: &BigInt, sk: &[u8], common_pk: &[u8]) -> BigInt {
    let point_pk = babyjubjub_unpack_point(common_pk);
    let point_sk = BigInt::from_bytes_le(Sign::Plus, sk);
    let k = point_pk.mul_scalar(&point_sk);
    let hm = poseidon_fr(&[k.x, k.y]);

    mod_floor(&(encrypted.clone() - hm), &FIELD_SIZE)
}
