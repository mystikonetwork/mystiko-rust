use crate::utils::fr_to_big_uint;
use crate::utils::poseidon_hash;
use babyjubjub_rs::{decompress_point, Point};
use ff::*;
use lazy_static::lazy_static;
use mystiko_utils::constants::FIELD_SIZE;
use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt, ToBigUint};

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

const ECIES_KEY_LEN: usize = 32;

fn big_uint_to_arr(num: &BigUint) -> [u8; ECIES_KEY_LEN] {
    let bytes = num.to_bytes_be();
    let mut arr: [u8; ECIES_KEY_LEN] = [0; ECIES_KEY_LEN];
    arr[ECIES_KEY_LEN - bytes.len()..].copy_from_slice(&bytes);
    arr
}

pub fn generate_secret_key() -> BigUint {
    let mut rng = rand::thread_rng();
    let sk_raw = rng.gen_biguint(1024).to_biguint().unwrap();
    let sk_raw_bytes = sk_raw.to_bytes_be()[..ECIES_KEY_LEN].to_vec();
    let random_uint = BigUint::from_bytes_be(&sk_raw_bytes);

    random_uint.modpow(&BigUint::from(1u32), &FIELD_SIZE);
    random_uint
}

pub fn public_key(secret_key: &BigUint) -> BigUint {
    let pk = B8.mul_scalar(&secret_key.to_bigint().unwrap()).compress();
    BigUint::from_bytes_le(&pk)
}

fn unpack_public_key_point(public_key: &BigUint) -> Point {
    let mut arr = big_uint_to_arr(public_key);
    arr.reverse();
    decompress_point(arr).unwrap()
}

pub fn unpack_public_key(public_key: &BigUint) -> (BigUint, BigUint) {
    let point = unpack_public_key_point(public_key);
    (fr_to_big_uint(&point.x), fr_to_big_uint(&point.y))
}

pub fn encrypt(plain: BigUint, pk: BigUint, common_sk: BigUint) -> BigUint {
    let point_pk = unpack_public_key_point(&pk);
    let k = point_pk.mul_scalar(&common_sk.to_bigint().unwrap());
    let hm = poseidon_hash(vec![k.x, k.y]);

    (plain + hm).modpow(&BigUint::from(1u32), &FIELD_SIZE)
}

pub fn decrypt(encrypted: BigUint, sk: BigUint, common_pk: BigUint) -> BigUint {
    let point_pk = unpack_public_key_point(&common_pk);
    let k = point_pk.mul_scalar(&sk.to_bigint().unwrap());
    let hm = poseidon_hash(vec![k.x, k.y]);

    let a = encrypted.to_bigint().unwrap() - hm.to_bigint().unwrap();
    calc_mod(a).to_biguint().unwrap()
}

fn calc_mod(a_number: BigInt) -> BigUint {
    let field_size = FIELD_SIZE.clone().to_bigint().unwrap();
    a_number
        .modpow(&BigInt::from(1u32), &field_size)
        .to_biguint()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_key() {
        let sk = BigUint::parse_bytes(
            b"17271648533819761767633660408073145085934772589775836550317652488597130541763",
            10,
        )
        .unwrap();

        let pk = public_key(&sk);
        // let unpacked_pk = unpack_public_key(&pk);
        assert_eq!(
            pk,
            BigUint::parse_bytes(
                b"72444700469954344414033902054315551824029723235242170438854670892932808883061",
                10
            )
            .unwrap()
        );

        let unpacked_pk = unpack_public_key(&pk);
        assert_eq!(
            unpacked_pk.0,
            BigUint::parse_bytes(
                b"17698851190026478217268086792453479467089177242109235834022425894878167006166",
                10
            )
            .unwrap()
        );
        assert_eq!(
            unpacked_pk.1,
            BigUint::parse_bytes(
                b"14548655851296246702248409549971597897394730902421888419125878888976244063093",
                10
            )
            .unwrap()
        );
    }

    #[test]
    fn test_encrypt() {
        let common_sk = generate_secret_key();
        let common_pk = public_key(&common_sk);
        let sk = generate_secret_key();
        let pk = public_key(&sk);
        let message = FIELD_SIZE.clone();
        let message = message - BigUint::from(10u32);

        let encrypted = encrypt(message.clone(), pk, common_sk);
        let decrypted = decrypt(encrypted.clone(), sk, common_pk);
        assert_eq!(message, decrypted);
    }
}
