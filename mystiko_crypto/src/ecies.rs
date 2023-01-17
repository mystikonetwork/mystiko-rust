use crate::utils::fr_to_big_int;
use crate::utils::poseidon_hash;
use babyjubjub_rs::{decompress_point, Point};
use ff::*;
use lazy_static::lazy_static;
use mystiko_utils::constants::FIELD_SIZE;
use num_bigint::{BigInt, RandBigInt, Sign, ToBigInt};
use num_integer::Integer;
use std::cmp::min;

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

pub fn generate_secret_key() -> BigInt {
    let mut rng = rand::thread_rng();
    let sk_raw = rng.gen_biguint(1024).to_bigint().unwrap();
    let (_, sk_raw_bytes) = sk_raw.to_bytes_le();
    let sk_raw_bytes = sk_raw_bytes[..ECIES_KEY_LEN].to_vec();
    let random_bigint = BigInt::from_bytes_le(Sign::Plus, &sk_raw_bytes);

    random_bigint.mod_floor(&FIELD_SIZE)
}

pub fn public_key(secret_key: &BigInt) -> BigInt {
    let pk = B8.mul_scalar(secret_key).compress();
    BigInt::from_bytes_le(Sign::Plus, &pk)
}

fn public_key_to_arr(num: &BigInt) -> [u8; ECIES_KEY_LEN] {
    let (_, y_bytes) = num.to_bytes_le();
    let mut arr: [u8; ECIES_KEY_LEN] = [0; ECIES_KEY_LEN];
    let len = min(y_bytes.len(), arr.len());
    arr[..len].copy_from_slice(&y_bytes[..len]);
    arr
}

fn unpack_public_key_point(public_key: &BigInt) -> Point {
    let arr = public_key_to_arr(public_key);
    decompress_point(arr).unwrap()
}

pub fn unpack_public_key(public_key: &BigInt) -> (BigInt, BigInt) {
    let point = unpack_public_key_point(public_key);
    (fr_to_big_int(&point.x), fr_to_big_int(&point.y))
}

pub fn encrypt(plain: BigInt, pk: BigInt, common_sk: BigInt) -> BigInt {
    let point_pk = unpack_public_key_point(&pk);
    let k = point_pk.mul_scalar(&common_sk);
    let hm = poseidon_hash(vec![k.x, k.y]);

    calc_mod(plain + hm)
}

pub fn decrypt(encrypted: BigInt, sk: BigInt, common_pk: BigInt) -> BigInt {
    let point_pk = unpack_public_key_point(&common_pk);
    let k = point_pk.mul_scalar(&sk);
    let hm = poseidon_hash(vec![k.x, k.y]);

    calc_mod(encrypted - hm)
}

fn calc_mod(a_number: BigInt) -> BigInt {
    a_number.mod_floor(&FIELD_SIZE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod() {
        let field = FIELD_SIZE.clone();
        assert_eq!(calc_mod(BigInt::from(-1)), field - 1);
    }

    #[test]
    fn test_secret_key() {
        let sk = BigInt::parse_bytes(
            b"17271648533819761767633660408073145085934772589775836550317652488597130541763",
            10,
        )
        .unwrap();

        let pk = public_key(&sk);
        // let unpacked_pk = unpack_public_key(&pk);
        assert_eq!(
            pk,
            BigInt::parse_bytes(
                b"72444700469954344414033902054315551824029723235242170438854670892932808883061",
                10
            )
            .unwrap()
        );

        let unpacked_pk = unpack_public_key(&pk);
        assert_eq!(
            unpacked_pk.0,
            BigInt::parse_bytes(
                b"17698851190026478217268086792453479467089177242109235834022425894878167006166",
                10
            )
            .unwrap()
        );
        assert_eq!(
            unpacked_pk.1,
            BigInt::parse_bytes(
                b"14548655851296246702248409549971597897394730902421888419125878888976244063093",
                10
            )
            .unwrap()
        );

        let sk2 = BigInt::parse_bytes(
            b"10159867704475093819611390305399872840803137048112391803348825378506064827917",
            10,
        )
        .unwrap();

        let pk2 = public_key(&sk2);
        // let unpacked_pk = unpack_public_key(&pk);
        assert_eq!(
            pk2,
            BigInt::parse_bytes(
                b"144953317550107391240674677905376978673879922040003637731432436387597190873",
                10
            )
            .unwrap()
        );

        let unpacked_pk2 = unpack_public_key(&pk2);
        assert_eq!(
            unpacked_pk2.0,
            BigInt::parse_bytes(
                b"909244511446444536038804174950319430779653247671679866159305631824459185121",
                10
            )
            .unwrap()
        );
        assert_eq!(
            unpacked_pk2.1,
            BigInt::parse_bytes(
                b"144953317550107391240674677905376978673879922040003637731432436387597190873",
                10
            )
            .unwrap()
        );
    }

    #[test]
    fn test_unpack_public_key() {
        for i in 0..10 {
            let common_sk = generate_secret_key();
            let common_pk = public_key(&common_sk);
            let (_, _) = unpack_public_key(&common_pk);
        }
    }

    #[test]
    fn test_encrypt() {
        let common_sk = generate_secret_key();
        let common_pk = public_key(&common_sk);
        let sk = generate_secret_key();
        let pk = public_key(&sk);
        let message = FIELD_SIZE.clone();
        let message = message - BigInt::from(10u32);

        let encrypted = encrypt(message.clone(), pk, common_sk);
        let decrypted = decrypt(encrypted.clone(), sk, common_pk);
        assert_eq!(message, decrypted);
    }
}
