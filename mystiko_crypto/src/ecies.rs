use crate::constants::{ECIES_KEY_LENGTH, FIELD_SIZE};
use crate::hash::poseidon_fr;
use crate::utils::{
    babyjubjub_public_key, babyjubjub_unpack_point, calc_mod, fr_to_big_int, random_big_int,
};
use babyjubjub_rs::Point;
use ff::*;
use lazy_static::lazy_static;
use num_bigint::{BigInt, Sign};

pub type Fr = poseidon_rs::Fr;
lazy_static! {
    static ref B8: Point = Point {
        x: Fr::from_str(
            "5299619240641551281634865583518297030282874472190772894086521144482721001553",
        )
        .unwrap(),
        y: Fr::from_str(
            "16950150798460657717958625567821834550301663161624707787222815936182638968203",
        )
        .unwrap(),
    };
}

pub fn generate_secret_key() -> BigInt {
    random_big_int(ECIES_KEY_LENGTH, &FIELD_SIZE)
}

pub fn public_key(secret_key: &BigInt) -> BigInt {
    let pk = B8.mul_scalar(secret_key).compress();
    BigInt::from_bytes_le(Sign::Plus, &pk)
}

pub fn unpack_public_key(public_key: &BigInt) -> (BigInt, BigInt) {
    let point = babyjubjub_unpack_point(public_key);
    (fr_to_big_int(&point.x), fr_to_big_int(&point.y))
}

pub fn public_key_from_unpack_point(x: &BigInt, y: &BigInt) -> BigInt {
    babyjubjub_public_key(x, y)
}

pub fn encrypt(plain: &BigInt, pk: &BigInt, common_sk: &BigInt) -> BigInt {
    let point_pk = babyjubjub_unpack_point(pk);
    let k = point_pk.mul_scalar(common_sk);
    let hm = poseidon_fr(vec![k.x, k.y]);

    calc_mod(plain.clone() + hm, &FIELD_SIZE)
}

pub fn decrypt(encrypted: &BigInt, sk: &BigInt, common_pk: &BigInt) -> BigInt {
    let point_pk = babyjubjub_unpack_point(common_pk);
    let k = point_pk.mul_scalar(sk);
    let hm = poseidon_fr(vec![k.x, k.y]);

    calc_mod(encrypted.clone() - hm, &FIELD_SIZE)
}
