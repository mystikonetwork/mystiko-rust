extern crate ff;
extern crate mystiko_crypto;

use ff::hex;

use mystiko_crypto::aes_cbc::{decrypt, encrypt, password_to_key};

#[tokio::test]
async fn test_key_gen_compatible_with_js() {
    let password = b"P@ssw0rd";
    let salt = b"01234567";
    let expect_key = hex::decode("8bd31ea8bc0a7143450a689625f28bb17d452bff17416cc8a4b010f1f114014e").unwrap();
    let expect_iv = hex::decode("f596e8897b7e9c2f4d856b0e052f318c").unwrap();
    let (key, iv) = password_to_key(password, salt);
    assert_eq!(key, expect_key);
    assert_eq!(iv, expect_iv);
}

#[tokio::test]
async fn test_aes_cbc_encrypt_compatible_with_js() {
    let plain_data = b"mystiko is awesome";
    let iv = hex::decode("02d9be5378539da18157bdd6ec814c77").unwrap();
    let key = hex::decode("d8ae99571b83ecef3974b8596d17fd1c46d4798e2afdd32f32345bfa8224f3dc").unwrap();
    let expect_cipher_txt = hex::decode("fe515f9acd6711ccfc8287110296d14a739f9d6f663a923d72b281485f9cc699").unwrap();

    let cipher_txt = encrypt(&iv.to_vec(), &key.to_vec(), plain_data.to_vec().as_slice());
    assert_eq!(cipher_txt.as_slice(), expect_cipher_txt);

    let dec_plain_text = decrypt(&iv.to_vec(), &key, cipher_txt.to_vec().as_slice()).unwrap();
    assert_eq!(dec_plain_text, plain_data);
}
