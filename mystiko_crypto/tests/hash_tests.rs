extern crate mystiko_crypto;
extern crate num_bigint;

use mystiko_crypto::constants::FIELD_SIZE;
use mystiko_crypto::hash::{blake2b_512, checksum, keccak256, poseidon, poseidon_bigint, sha256_mod};
use mystiko_crypto::utils::{random_biguint, random_bytes};
use num_bigint::BigUint;

#[tokio::test]
async fn test_keccak256() {
    let msg = b"mystiko is ansome";
    let expect = [
        177, 73, 3, 90, 48, 15, 126, 207, 23, 0, 195, 9, 161, 60, 15, 218, 149, 186, 80, 255, 219, 169, 248, 92, 210,
        82, 54, 254, 36, 133, 94, 49,
    ];
    let k1 = keccak256(msg);
    assert_eq!(k1, expect);
}

#[tokio::test]
async fn test_sha256_mod() {
    // compatible test with js
    let data1 = b"baad";
    let hash_js = BigUint::parse_bytes(
        b"21386729550194668992953605105751046526972920509576226227329140564650153218040",
        10,
    )
    .unwrap();
    let hash1 = sha256_mod(data1);
    assert_eq!(hash1, hash_js);

    let data2 = b"effe";
    let hash2 = sha256_mod(data2);
    assert_ne!(hash1, hash2);

    let data3 = b"baad";
    let hash3 = sha256_mod(data3);
    assert_eq!(hash1, hash3);
}

#[tokio::test]
async fn test_poseidon_compatible_with_js() {
    let b1 = BigUint::from(1u32);
    let b2 = BigUint::from(2u32);
    let expect_hash = BigUint::parse_bytes(
        b"7853200120776062878684798364095072458815029376092732009249414926327459813530",
        10,
    )
    .unwrap();
    let hash = poseidon(&[b1, b2]);
    assert_eq!(hash, expect_hash);
}

#[tokio::test]
async fn test_poseidon_bigint() {
    let size = random_bytes(1);
    let b1 = random_biguint(size[0] as usize, &FIELD_SIZE);
    let size = random_bytes(1);
    let b2 = random_biguint(size[0] as usize, &FIELD_SIZE);
    poseidon_bigint(&[b1, b2]);
}

#[tokio::test]
async fn test_check_sum() {
    let hash1 = checksum("hello world", None);
    let hash2 = checksum("Mystiko is awesome", Some(""));
    let hash3 = checksum("Mystiko is awesome", Some("P@ssw0rd"));
    let hash4 = checksum("hello world", None);
    assert_ne!(hash1, hash2);
    assert_ne!(hash2, hash3);
    assert_ne!(hash3, hash4);
    assert_eq!(hash4, hash1);
    assert_eq!(hash3.as_str(), "03b41505aa26437d94831f9bfd24afd4e7eaf33d6aaf368d0c77545ad2a958024222badb4d84a17f84ff15297e16199dabc88b417ce764624ed5a2443681afcd");
    assert_eq!(hash2.as_str(), "8b9fb4d5f890ea83d09f63af9dee5ba8a53a9f5dedeb9bfd0e6ed135d5dca7abbc75d455fe0ee46040828834186543e008401238aeaaab1039f3a5ab37bb1c97");
}

#[tokio::test]
async fn test_blake2_512() {
    let hash1 = blake2b_512("hello world");
    let hash2 = blake2b_512("Mystiko is awesome");
    assert_ne!(hash1, hash2);
}
