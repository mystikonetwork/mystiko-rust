use crate::error::CryptoError;
use crate::{aes_cbc, eccrypto};
use anyhow::Result;

pub fn encrypt_asymmetric(public_key: &[u8], plain_data: &[u8]) -> Result<Vec<u8>, CryptoError> {
    eccrypto::encrypt(public_key, plain_data)
}

pub fn decrypt_asymmetric(secret_key: &[u8], cipher_data: &[u8]) -> Result<Vec<u8>, CryptoError> {
    eccrypto::decrypt(secret_key, cipher_data)
}

pub fn encrypt_symmetric(password: &str, plain_text: &str) -> Result<String, CryptoError> {
    aes_cbc::encrypt_str(password, plain_text)
}

pub fn decrypt_symmetric(password: &str, cipher_text: &str) -> Result<String, CryptoError> {
    aes_cbc::decrypt_str(password, cipher_text)
}
