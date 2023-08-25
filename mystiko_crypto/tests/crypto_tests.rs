extern crate ff;
extern crate k256;
extern crate mystiko_crypto;

use k256::SecretKey;
use mystiko_crypto::crypto::{decrypt_asymmetric, decrypt_symmetric, encrypt_asymmetric, encrypt_symmetric};
use mystiko_crypto::eccrypto::public_key_to_vec;
use mystiko_crypto::error::CryptoError;
use mystiko_crypto::utils::{random_bytes, random_utf8_string};
use rand_core::OsRng;

#[tokio::test]
async fn test_decrypt_symmetric_compatible_with_js() {
    let plain_text = "mystiko is awesome";
    let passsord = "P@ssw0rd";
    let js_cipher_text = "U2FsdGVkX1+zDTOX4UIDYsI1sV9Xg4NUNa/fM+7+mnew4z1zKth3vySvCu+zPkeb";
    let dec_text = decrypt_symmetric(passsord, js_cipher_text).unwrap();
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

    let cipher_text_wrong = "%$##";
    let dec_text = decrypt_symmetric(&password, cipher_text_wrong);
    assert!(matches!(dec_text.err().unwrap(), CryptoError::DecryptError(_)));

    let password = random_utf8_string((size[0] % 64 + 1) as usize);
    let dec_text = decrypt_symmetric(&password, &cipher_text);
    assert!(matches!(dec_text.err().unwrap(), CryptoError::DecryptError(_)));
}

#[tokio::test]
async fn test_random_encrypt_asymmetric() {
    let mut rng = OsRng;
    let sk = SecretKey::random(&mut rng);
    let pk = sk.public_key();
    let pk = public_key_to_vec(&pk, true);

    let text = random_bytes(80);
    let data = encrypt_asymmetric(pk.as_slice(), text.as_slice()).unwrap();
    let dec_text = decrypt_asymmetric(sk.to_bytes().to_vec().as_slice(), &data).unwrap();
    assert_eq!(text, dec_text);
}
