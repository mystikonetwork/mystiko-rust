use crate::constants::{
    ECIES_IV_LENGTH, ECIES_KEY_LENGTH, ECIES_MAGIC_DATA, KDF_MAGIC_DATA_LENGTH, KDF_SALT_LENGTH,
};
use crate::error::ECCryptoError;
use crate::utils::random_bytes;
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use base64::{engine::general_purpose, Engine as _};
use generic_array::GenericArray;

pub fn encrypt_str(password: &str, plain_text: &str) -> Result<String, ECCryptoError> {
    let salt = random_bytes(KDF_SALT_LENGTH);
    let (key, iv) = password_to_key(password.as_bytes(), salt.as_slice());
    let cipher_text = encrypt(&iv, &key, plain_text.as_bytes());
    let full_encrypted_data = stringify_cipher_data_with_salt(&cipher_text, &salt);
    Ok(general_purpose::STANDARD.encode(full_encrypted_data))
}

pub fn decrypt_str(password: &str, cipher_data: &str) -> Result<String, ECCryptoError> {
    let cipher_data_raw = general_purpose::STANDARD.decode(cipher_data).unwrap();
    let (salt, cipher_text) = parse_salt_from_cipher_data(cipher_data_raw.as_slice());
    let (key, iv) = password_to_key(password.as_bytes(), salt.as_slice());
    let decrypted_data = decrypt(iv.as_slice(), key.as_slice(), cipher_text.as_slice());
    let text = String::from_utf8(decrypted_data).unwrap();
    Ok(text)
}

//
// pub fn encrypt(iv: &[u8], key: &[u8], plain_text: &[u8]) -> Vec<u8> {
//     // todo check encrypt result
//     // openssl::symm::encrypt(Cipher::aes_256_cbc(), key, Some(iv), plain_text).unwrap()
// }
//
// pub fn decrypt(iv: &[u8], key: &[u8], cipher_text: &[u8]) -> Vec<u8> {
//     // todo check decrypt result
//     // openssl::symm::decrypt(Cipher::aes_256_cbc(), key, Some(iv), cipher_text).unwrap()
// }

pub fn encrypt(iv: &[u8], key: &[u8], plain_text: &[u8]) -> Vec<u8> {
    type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;

    let key_g = GenericArray::from_slice(key);
    let iv_g = GenericArray::from_slice(iv);
    Aes256CbcEnc::new(key_g, iv_g).encrypt_padded_vec_mut::<Pkcs7>(plain_text)
}

pub fn decrypt(iv: &[u8], key: &[u8], cipher_text: &[u8]) -> Vec<u8> {
    type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

    let key = GenericArray::from_slice(key);
    let iv = GenericArray::from_slice(iv);
    // todo return  error if failed to decrypt
    Aes256CbcDec::new(key, iv)
        .decrypt_padded_vec_mut::<Pkcs7>(cipher_text)
        .unwrap()
}

// compatible with crypto-js evpkdf compute
pub fn password_to_key(password: &[u8], salt: &[u8]) -> (Vec<u8>, Vec<u8>) {
    // Initial values
    let key_size = ECIES_KEY_LENGTH + ECIES_IV_LENGTH; // in bytes
    let iterations = 1; // for demonstration purposes only

    // Generate key
    let mut derived_key = Vec::new();
    let mut block = Vec::new();
    while derived_key.len() < key_size {
        let mut hasher = md5::Context::new();

        if !block.is_empty() {
            hasher.consume(&block);
        }

        hasher.consume(password);
        hasher.consume(salt);
        block = hasher.compute().to_vec();

        // Iterations
        for _ in 1..iterations {
            let mut hasher = md5::Context::new();
            hasher.consume(block);
            block = hasher.compute().to_vec();
        }

        derived_key.extend(block.clone());
    }

    let (key, iv) = derived_key.split_at(ECIES_KEY_LENGTH);
    (key.to_vec(), iv.to_vec())
}

fn stringify_cipher_data_with_salt(cipher_text: &Vec<u8>, salt: &Vec<u8>) -> Vec<u8> {
    let mut data = ECIES_MAGIC_DATA.clone();
    data.extend(salt);
    data.extend(cipher_text);
    data
}

fn parse_salt_from_cipher_data(cipher_data: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let (magic_data, right) = cipher_data.split_at(KDF_MAGIC_DATA_LENGTH);
    assert_eq!(magic_data.to_vec(), ECIES_MAGIC_DATA.clone());
    let (salt, text) = right.split_at(KDF_SALT_LENGTH);
    (salt.to_vec(), text.to_vec())
}
