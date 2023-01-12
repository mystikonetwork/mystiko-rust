use crate::utils::poseidon_hash;
use crate::utils::{big_uint_to_arr, fr_to_big_uint};
use babyjubjub_rs::{decompress_point, PrivateKey};
use mystiko_utils::constants::FIELD_SIZE;
use num_bigint::{BigUint, RandBigInt, ToBigInt, ToBigUint};

const KEY_LEN: usize = 32;

pub fn generate_secret_key() -> BigUint {
    let mut rng = rand::thread_rng();
    let sk_raw = rng.gen_biguint(1024).to_biguint().unwrap();
    let sk_raw_bytes = sk_raw.to_bytes_be()[..KEY_LEN].to_vec();
    let random_uint = BigUint::from_bytes_be(&sk_raw_bytes);

    random_uint.modpow(&BigUint::from(1u32), &FIELD_SIZE);
    random_uint
}

pub fn public_key(secret_key: BigUint) -> BigUint {
    let pk = PrivateKey::import(secret_key.to_bytes_be()).unwrap();
    let pk_compress = pk.public().compress();
    BigUint::from_bytes_be(&pk_compress)
}

pub fn unpack_public_key(public_key: BigUint) -> (BigUint, BigUint) {
    let point = decompress_point(big_uint_to_arr(&public_key)).unwrap();
    (fr_to_big_uint(&point.x), fr_to_big_uint(&point.y))
}

pub fn encrypt(plain: BigUint, pk: BigUint, common_sk: BigUint) -> BigUint {
    let point_pk = decompress_point(big_uint_to_arr(&pk)).unwrap();

    let k = point_pk.mul_scalar(&common_sk.to_bigint().unwrap());
    let hm = poseidon_hash(vec![k.x, k.y]);

    e_mod(plain + hm)
}

pub fn decrypt(encrypted: BigUint, sk: BigUint, common_pk: BigUint) -> BigUint {
    let point_pk = decompress_point(big_uint_to_arr(&common_pk)).unwrap();

    let k = point_pk.mul_scalar(&sk.to_bigint().unwrap());
    let hm = poseidon_hash(vec![k.x, k.y]);

    e_mod(encrypted - hm)
}

fn e_mod(a_number: BigUint) -> BigUint {
    // todo nodejs a_number can less than 0
    a_number.modpow(&BigUint::from(1u32), &FIELD_SIZE)
}
