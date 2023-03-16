extern crate mystiko_crypto;
extern crate mystiko_protocol;
extern crate num_bigint;

use mystiko_protocol::key::{
    combined_public_key, combined_secret_key, encryption_public_key, encryption_secret_key,
    separate_public_keys, separate_secret_keys, verification_public_key, verification_secret_key,
};
use mystiko_protocol::types::{ENC_PK_SIZE, ENC_SK_SIZE, VERIFY_PK_SIZE, VERIFY_SK_SIZE};

#[tokio::test]
async fn test_secret_key_for_verification() {
    // todo do random key test
    let raw_key = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0,
        1, 2,
    ];
    let expect_vk = [
        0, 134, 214, 195, 195, 39, 41, 91, 119, 1, 111, 223, 51, 69, 90, 161, 81, 86, 103, 62, 118,
        211, 151, 234, 51, 125, 197, 58, 86, 95, 32, 15,
    ];

    let vk = verification_secret_key(&raw_key);
    assert_eq!(vk, expect_vk);
}

#[tokio::test]
async fn test_public_key_for_verification() {
    let raw_key = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0,
        1, 2,
    ];
    let expect_sk = [
        35, 67, 78, 191, 181, 140, 63, 160, 165, 110, 204, 134, 242, 240, 250, 194, 198, 217, 57,
        5, 136, 71, 203, 25, 247, 30, 120, 219, 84, 207, 18, 27,
    ];

    let vk = verification_public_key(&raw_key);
    assert_eq!(vk, expect_sk);
}

#[tokio::test]
async fn test_secret_key_for_encryption() {
    let raw_key = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0,
        1, 2,
    ];
    let sk = encryption_secret_key(&raw_key);
    assert_eq!(sk, raw_key);
}

#[tokio::test]
async fn test_public_key_for_encryption() {
    let raw_key = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0,
        1, 2,
    ];
    let expect_pk = [
        2, 177, 145, 47, 171, 168, 15, 206, 205, 42, 100, 197, 116, 254, 254, 66, 44, 97, 16, 96,
        1, 236, 88, 138, 241, 189, 157, 117, 72, 184, 16, 100, 203,
    ];
    let pk = encryption_public_key(&raw_key);
    assert_eq!(pk, expect_pk);
}

#[tokio::test]
async fn test_full_and_separate_public_key() {
    let vk: [u8; VERIFY_PK_SIZE] = [
        0, 134, 214, 195, 195, 39, 41, 91, 119, 1, 111, 223, 51, 69, 90, 161, 81, 86, 103, 62, 118,
        211, 151, 234, 51, 125, 197, 58, 86, 95, 32, 15,
    ];
    let ek: [u8; ENC_PK_SIZE] = [
        2, 177, 145, 47, 171, 168, 15, 206, 205, 42, 100, 197, 116, 254, 254, 66, 44, 97, 16, 96,
        1, 236, 88, 138, 241, 189, 157, 117, 72, 184, 16, 100, 203,
    ];

    let expect_combine: [u8; VERIFY_PK_SIZE + ENC_PK_SIZE] = [
        0, 134, 214, 195, 195, 39, 41, 91, 119, 1, 111, 223, 51, 69, 90, 161, 81, 86, 103, 62, 118,
        211, 151, 234, 51, 125, 197, 58, 86, 95, 32, 15, 2, 177, 145, 47, 171, 168, 15, 206, 205,
        42, 100, 197, 116, 254, 254, 66, 44, 97, 16, 96, 1, 236, 88, 138, 241, 189, 157, 117, 72,
        184, 16, 100, 203,
    ];

    let combine = combined_public_key(&vk, &ek);
    assert_eq!(combine, expect_combine);

    let (vk_s, ek_s) = separate_public_keys(&combine);
    assert_eq!(vk, vk_s);
    assert_eq!(ek, ek_s);
}

#[tokio::test]
async fn test_full_and_separate_secret_key() {
    let vk: [u8; VERIFY_SK_SIZE] = [
        0, 134, 214, 195, 195, 39, 41, 91, 119, 1, 111, 223, 51, 69, 90, 161, 81, 86, 103, 62, 118,
        211, 151, 234, 51, 125, 197, 58, 86, 95, 32, 15,
    ];
    let ek: [u8; ENC_SK_SIZE] = [
        2, 177, 145, 47, 171, 168, 15, 206, 205, 42, 100, 197, 116, 254, 254, 66, 44, 97, 16, 96,
        1, 236, 88, 138, 241, 189, 157, 117, 72, 184, 16, 0,
    ];

    let expect_combine: [u8; VERIFY_SK_SIZE + ENC_SK_SIZE] = [
        0, 134, 214, 195, 195, 39, 41, 91, 119, 1, 111, 223, 51, 69, 90, 161, 81, 86, 103, 62, 118,
        211, 151, 234, 51, 125, 197, 58, 86, 95, 32, 15, 2, 177, 145, 47, 171, 168, 15, 206, 205,
        42, 100, 197, 116, 254, 254, 66, 44, 97, 16, 96, 1, 236, 88, 138, 241, 189, 157, 117, 72,
        184, 16, 0,
    ];
    let combine = combined_secret_key(&vk, &ek);
    assert_eq!(combine, expect_combine);

    let (vk_s, ek_s) = separate_secret_keys(&combine);
    assert_eq!(vk, vk_s);
    assert_eq!(ek, ek_s);
}
