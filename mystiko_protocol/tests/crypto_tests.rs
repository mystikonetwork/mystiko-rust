extern crate ff;
extern crate mystiko_crypto;
extern crate mystiko_protocol;

use ff::hex;
use mystiko_crypto::crypto::{
    decrypt_asymmetric, decrypt_symmetric, encrypt_asymmetric, encrypt_symmetric,
};
use mystiko_crypto::utils::{random_bytes, random_utf8_string};
use mystiko_protocol::key::{public_key_for_encryption, secret_key_for_encryption};

#[tokio::test]
async fn test_decrypt_symmetric_compatible_with_js() {
    let plain_text = "mystiko is awesome";
    let passsord = "P@ssw0rd";
    let js_cipher_text = "U2FsdGVkX1+zDTOX4UIDYsI1sV9Xg4NUNa/fM+7+mnew4z1zKth3vySvCu+zPkeb";
    let dec_text = decrypt_symmetric(passsord, &js_cipher_text).unwrap();
    assert_eq!(dec_text, plain_text);
}

#[tokio::test]
async fn test_random_encrypt_symmetric() {
    let size = random_bytes(1);
    let plain_text = random_utf8_string(size[0] as usize);
    let size = random_bytes(1);
    let password = random_utf8_string((size[0] % 64 + 1) as usize);
    let cipher_text = encrypt_symmetric(&password, &plain_text).unwrap();
    let dec_text = decrypt_symmetric(&password, &cipher_text).unwrap();
    assert_eq!(plain_text, dec_text);
}

#[tokio::test]
async fn test_decrypt_asymmetric_compatible_with_js() {
    let raw_sk = b"98765432101234567890123456789012";
    let plain_text = b"mystiko is awesome";
    let enc_data_js = hex::decode("cf4fde5e49053dbb80baaf9e13eb2be804096f23c2a1e445ad7dfb6ece6e7e1febdb9559bd0bb2d76ea60e1abc56fe2d2599b01a3a3d29574f08b8143f68b2f6215a593760bdae6fccaea3d6523440cb8fec2c8470bb6f60151d7bbcf48854a0a3f743c17f65ec7dfd01f2bf8d6c9523bd7b2ec39d61fa760c2e12204f296cc51c8ba7513ba8be4b20757c5f4ee4647cca").unwrap();
    let sk = secret_key_for_encryption(raw_sk.as_slice());
    let dec_data = decrypt_asymmetric(&sk, enc_data_js.as_slice()).unwrap();
    assert_eq!(dec_data.as_slice(), plain_text);
}

#[tokio::test]
async fn test_random_encrypt_asymmetric() {
    let raw_sk = random_bytes(32);
    let sk = secret_key_for_encryption(raw_sk.as_slice());
    let pk = public_key_for_encryption(raw_sk.as_slice());
    let plain_text = random_bytes(80);
    let enc_data = encrypt_asymmetric(&pk, plain_text.as_slice()).unwrap();
    let dec_data = decrypt_asymmetric(&sk, enc_data.as_slice()).unwrap();
    assert_eq!(dec_data, plain_text);
}
