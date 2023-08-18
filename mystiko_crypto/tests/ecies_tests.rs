extern crate mystiko_crypto;
extern crate num_bigint;

use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::ops::Shr;

use mystiko_crypto::constants::FIELD_SIZE;
use mystiko_crypto::ecies::{
    decrypt, encrypt, generate_secret_key, public_key, public_key_from_unpack_point, unpack_public_key,
};
use mystiko_crypto::utils::{biguint_to_32_bytes, random_biguint};

#[tokio::test]
async fn test_secret_key() {
    let sk = BigUint::parse_bytes(
        b"17271648533819761767633660408073145085934772589775836550317652488597130541763",
        10,
    )
    .unwrap();
    let sk = biguint_to_32_bytes(&sk);
    let pk = public_key(&sk);
    assert_eq!(
        BigUint::from_bytes_le(&pk),
        BigUint::parse_bytes(
            b"72444700469954344414033902054315551824029723235242170438854670892932808883061",
            10,
        )
        .unwrap()
    );

    let unpacked_pk = unpack_public_key(&pk);
    assert_eq!(
        BigUint::from_bytes_le(&unpacked_pk.0),
        BigUint::parse_bytes(
            b"17698851190026478217268086792453479467089177242109235834022425894878167006166",
            10,
        )
        .unwrap()
    );
    assert_eq!(
        BigUint::from_bytes_le(&unpacked_pk.1),
        BigUint::parse_bytes(
            b"14548655851296246702248409549971597897394730902421888419125878888976244063093",
            10,
        )
        .unwrap()
    );

    let sk2 = BigUint::parse_bytes(
        b"10159867704475093819611390305399872840803137048112391803348825378506064827917",
        10,
    )
    .unwrap();
    let sk2 = biguint_to_32_bytes(&sk2);
    let pk2 = public_key(&sk2);
    // let unpacked_pk = unpack_public_key(&pk);
    assert_eq!(
        BigUint::from_bytes_le(&pk2),
        BigUint::parse_bytes(
            b"144953317550107391240674677905376978673879922040003637731432436387597190873",
            10,
        )
        .unwrap()
    );

    let unpacked_pk2 = unpack_public_key(&pk2);
    assert_eq!(
        BigUint::from_bytes_le(&unpacked_pk2.0),
        BigUint::parse_bytes(
            b"909244511446444536038804174950319430779653247671679866159305631824459185121",
            10,
        )
        .unwrap()
    );
    assert_eq!(
        BigUint::from_bytes_le(&unpacked_pk2.1),
        BigUint::parse_bytes(
            b"144953317550107391240674677905376978673879922040003637731432436387597190873",
            10,
        )
        .unwrap()
    );
}

#[tokio::test]
async fn test_unpack_public_key() {
    for _ in 0..10 {
        let common_sk = generate_secret_key();
        let common_pk = public_key(&common_sk);
        let (x, y) = unpack_public_key(&common_pk);
        let pk = public_key_from_unpack_point(&x, &y);
        assert_eq!(pk, common_pk);
    }
}

#[tokio::test]
async fn test_encrypt() {
    let common_sk = generate_secret_key();
    let common_pk = public_key(&common_sk);
    let sk = generate_secret_key();
    let pk = public_key(&sk);

    let message = random_biguint(32, &FIELD_SIZE);
    let encrypted = encrypt(&message, &pk, &common_sk);
    let decrypted = decrypt(&encrypted, &sk, &common_pk);
    assert_eq!(message, decrypted);

    let message = BigUint::zero();
    let encrypted = encrypt(&message, &pk, &common_sk);
    let decrypted = decrypt(&encrypted, &sk, &common_pk);
    assert_eq!(message, decrypted);

    let message = FIELD_SIZE.clone() - BigUint::one();
    let encrypted = encrypt(&message, &pk, &common_sk);
    let decrypted = decrypt(&encrypted, &sk, &common_pk);
    assert_eq!(message, decrypted);

    let message = FIELD_SIZE.clone().shr(1u32);
    let encrypted = encrypt(&message, &pk, &common_sk);
    let decrypted = decrypt(&encrypted, &sk, &common_pk);
    assert_eq!(message, decrypted);
}
