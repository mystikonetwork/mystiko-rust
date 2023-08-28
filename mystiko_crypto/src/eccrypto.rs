// compatible with nodejs eccrypto
use crate::aes_cbc;
use crate::constants::{ECIES_IV_LENGTH, ECIES_MAC_LENGTH, ECIES_META_LENGTH, ECIES_UNCOMPRESSED_PK_LENGTH};
use crate::error::CryptoError;
use crate::hash::{hmac_sha256, sha512};
use crate::utils::random_bytes;
use anyhow::Result;
use elliptic_curve::sec1::{FromEncodedPoint, ToEncodedPoint};
use k256::{AffinePoint, EncodedPoint, PublicKey, SecretKey};
use rand_core::OsRng;
use std::cmp::min;

#[derive(Debug, Clone)]
pub struct ECCryptoData {
    iv: Vec<u8>,
    ephemeral_public_key: Vec<u8>,
    mac: Vec<u8>,
    cipher_text: Vec<u8>,
}

impl ECCryptoData {
    pub fn to_vec(&self) -> Vec<u8> {
        let mut v = self.iv.clone();
        v.extend(self.ephemeral_public_key.clone());
        v.extend(self.mac.clone());
        v.extend(self.cipher_text.clone());
        v
    }

    pub fn from_bytes(v: &[u8]) -> Result<Self, CryptoError> {
        if v.len() <= ECIES_META_LENGTH {
            return Err(CryptoError::DataLengthError);
        }

        let (iv, right) = v.split_at(ECIES_IV_LENGTH);
        let (ephemeral_public_key, right) = right.split_at(ECIES_UNCOMPRESSED_PK_LENGTH);
        let (mac, cipher_text) = right.split_at(ECIES_MAC_LENGTH);

        Ok(Self {
            iv: iv.to_vec(),
            ephemeral_public_key: ephemeral_public_key.to_vec(),
            mac: mac.to_vec(),
            cipher_text: cipher_text.to_vec(),
        })
    }
}

pub fn public_key_to_vec(pk: &PublicKey, compress: bool) -> Vec<u8> {
    let pk = pk.as_affine().to_encoded_point(compress);
    pk.as_bytes().to_vec()
}

fn uncompressed_public_key_to_compressed(pk: &[u8]) -> Vec<u8> {
    let encoded = EncodedPoint::from_bytes(pk).unwrap();
    let point: AffinePoint = AffinePoint::from_encoded_point(&encoded).unwrap();
    let res = point.to_encoded_point(true);
    res.as_bytes().to_vec()
}

fn encrypt_derive_shared_secret(public_key_bytes: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut rng = OsRng;
    let sk_a = SecretKey::random(&mut rng);
    let pk_a = sk_a.public_key();
    let pk_a = public_key_to_vec(&pk_a, false);

    let pk_b = PublicKey::from_sec1_bytes(public_key_bytes).unwrap();
    let shared = elliptic_curve::ecdh::diffie_hellman(sk_a.to_nonzero_scalar(), pk_b.as_affine());
    let shared_bytes = shared.raw_secret_bytes();
    let shared_bytes = shared_bytes.as_ref() as &[u8];

    (pk_a, shared_bytes.to_vec())
}

fn decrypt_derive_shared_secret(pk_a: &[u8], sk_b: &[u8]) -> Vec<u8> {
    // todo check if failed
    let public_key_a = PublicKey::from_sec1_bytes(pk_a).unwrap();
    let secret_key_b = SecretKey::from_slice(sk_b).unwrap();

    let shared = elliptic_curve::ecdh::diffie_hellman(secret_key_b.to_nonzero_scalar(), public_key_a.as_affine());

    let shared_bytes = shared.raw_secret_bytes();
    let shared_bytes = shared_bytes.as_ref() as &[u8];
    shared_bytes.to_vec()
}

pub fn encrypt(public_key_bytes: &[u8], plain_data: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if public_key_bytes.len() != 33 {
        return Err(CryptoError::KeyLengthError);
    }

    let (ephemeral_public_key, shared_bytes) = encrypt_derive_shared_secret(public_key_bytes);

    let shared_hash = sha512(shared_bytes.as_slice());
    let (encryption_key, mac_key) = shared_hash.split_at(32);

    let iv = random_bytes(16);
    let mut data_to_mac = iv.clone();
    let cipher_text = aes_cbc::encrypt(&iv, encryption_key, plain_data);
    data_to_mac.extend(ephemeral_public_key.clone());
    data_to_mac.extend(cipher_text.clone());

    let mac = hmac_sha256(mac_key, data_to_mac.as_slice());

    let ec_data = ECCryptoData {
        iv,
        ephemeral_public_key,
        cipher_text,
        mac,
    };
    Ok(ec_data.to_vec())
}

pub fn equal_const_time(b1: &[u8], b2: &[u8]) -> bool {
    if b1.len() != b2.len() {
        return false;
    }

    let mut res = 0;
    for i in 0..min(b1.len(), b2.len()) {
        res |= b1[i] ^ b2[i];
    }

    res == 0
}

pub fn decrypt(secret_key_bytes: &[u8], cipher_data: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let ec_data = ECCryptoData::from_bytes(cipher_data)?;
    let pk = uncompressed_public_key_to_compressed(ec_data.ephemeral_public_key.as_slice());
    let shared_bytes = decrypt_derive_shared_secret(pk.as_slice(), secret_key_bytes);

    let shared_hash = sha512(shared_bytes.as_slice());
    let (encryption_key, mac_key) = shared_hash.split_at(32);
    let mut data_to_mac = ec_data.iv.clone();
    data_to_mac.extend(ec_data.ephemeral_public_key.clone());
    data_to_mac.extend(ec_data.cipher_text.clone());
    let real_mac = hmac_sha256(mac_key, data_to_mac.as_slice());
    if !equal_const_time(real_mac.as_slice(), ec_data.mac.as_slice()) {
        return Err(CryptoError::MacMismatchError);
    }
    aes_cbc::decrypt(&ec_data.iv, encryption_key, ec_data.cipher_text.as_slice())
}
