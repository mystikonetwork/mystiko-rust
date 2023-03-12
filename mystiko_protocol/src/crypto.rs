use mystiko_crypto::error::ECCryptoError;
use mystiko_crypto::{aes_cbc, eccrypto};

pub fn encrypt_asymmetric(public_key: &[u8], plain_data: &[u8]) -> Result<Vec<u8>, ECCryptoError> {
    eccrypto::encrypt(public_key, plain_data)
}

// todo check decrypt result from caller
pub fn decrypt_asymmetric(secret_key: &[u8], cipher_data: &[u8]) -> Result<Vec<u8>, ECCryptoError> {
    eccrypto::decrypt(secret_key, cipher_data)
}

pub fn encrypt_symmetric(password: &str, plain_text: &str) -> String {
    aes_cbc::encrypt_str(password, plain_text).unwrap()
}

pub fn decrypt_symmetric(password: &str, cipher_text: &str) -> String {
    aes_cbc::decrypt_str(password, cipher_text).unwrap()
}
