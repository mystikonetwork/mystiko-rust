// compatible with nodejs eccrypto
use crate::aes_cbc;
use crate::constants::{
    ECIES_EPHEMERAL_PK_LENGTH, ECIES_IV_LENGTH, ECIES_MAC_LENGTH, ECIES_META_LENGTH,
};
use crate::error::ECCryptoError;
use crate::utils::random_bytes;
use elliptic_curve::sec1::{FromEncodedPoint, ToEncodedPoint};
use hmac::{Hmac, Mac};
use k256::{AffinePoint, EncodedPoint, PublicKey, SecretKey};
use rand_core::OsRng;
use sha2::{Digest, Sha256, Sha512};
use std::cmp::min;

#[derive(Debug)]
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

    pub fn from_vec(v: &Vec<u8>) -> Result<Self, ECCryptoError> {
        Self::from_bytes(v.as_slice())
    }

    pub fn from_bytes(v: &[u8]) -> Result<Self, ECCryptoError> {
        if v.len() <= ECIES_META_LENGTH {
            return Err(ECCryptoError::ECCryptoDataLengthError);
        }

        let (iv, right) = v.split_at(ECIES_IV_LENGTH);
        let (ephemeral_public_key, right) = right.split_at(ECIES_EPHEMERAL_PK_LENGTH);
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
    let sk_a = SecretKey::random(OsRng);
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
    let secret_key_b = SecretKey::from_be_bytes(sk_b).unwrap();

    let shared = elliptic_curve::ecdh::diffie_hellman(
        secret_key_b.to_nonzero_scalar(),
        public_key_a.as_affine(),
    );

    let shared_bytes = shared.raw_secret_bytes();
    let shared_bytes = shared_bytes.as_ref() as &[u8];
    shared_bytes.to_vec()
}

pub fn sha512_hash(msg: &[u8]) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(msg);
    hasher.finalize().to_vec()
}

pub fn hmac_sha256(key: &[u8], msg: &[u8]) -> Vec<u8> {
    type HmacSha256 = Hmac<Sha256>;
    let mut hmac = HmacSha256::new_from_slice(key).unwrap();
    hmac.update(msg);
    hmac.finalize().into_bytes().to_vec()
}

pub fn encrypt(public_key_bytes: &[u8], plain_data: &[u8]) -> Vec<u8> {
    let (ephemeral_public_key, shared_bytes) = encrypt_derive_shared_secret(public_key_bytes);

    let shared_hash = sha512_hash(shared_bytes.as_slice());
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
    ec_data.to_vec()
}

pub fn equal_const_time(b1: &Vec<u8>, b2: &Vec<u8>) -> bool {
    if b1.len() != b2.len() {
        return false;
    }

    let mut res = 0;
    for i in 0..min(b1.len(), b2.len()) {
        res |= b1[i] ^ b2[i];
    }

    res == 0
}

pub fn decrypt(secret_key_bytes: &[u8], cipher_data: &[u8]) -> Vec<u8> {
    // todo check unwrap
    let ec_data = ECCryptoData::from_bytes(cipher_data).unwrap();
    let pk = uncompressed_public_key_to_compressed(ec_data.ephemeral_public_key.as_slice());
    let shared_bytes = decrypt_derive_shared_secret(pk.as_slice(), secret_key_bytes);

    let shared_hash = sha512_hash(shared_bytes.as_slice());
    let (encryption_key, mac_key) = shared_hash.split_at(32);
    let mut data_to_mac = ec_data.iv.clone();
    data_to_mac.extend(ec_data.ephemeral_public_key.clone());
    data_to_mac.extend(ec_data.cipher_text.clone());
    let real_mac = hmac_sha256(mac_key, data_to_mac.as_slice());
    // todo change to result check
    assert!(equal_const_time(&real_mac, &ec_data.mac));
    aes_cbc::decrypt(&ec_data.iv, encryption_key, ec_data.cipher_text.as_slice())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrypt_compatible_with_js() {
        let sk = SecretKey::from_be_bytes(b"98765432101234567890123456789012").unwrap();
        let text = b"mystiko is awesome";

        let js_dec_data: &[u8] = &[
            0x91, 0x93, 0x06, 0xFC, 0x77, 0xF9, 0xAA, 0x65, 0xFA, 0xF9, 0x77, 0x30, 0xDF, 0x13,
            0x35, 0x82, 0x04, 0xDA, 0xE8, 0x38, 0x3F, 0x49, 0xCF, 0x70, 0x50, 0x86, 0x0B, 0x85,
            0x13, 0x3A, 0x03, 0x22, 0x25, 0xB7, 0xFA, 0x28, 0x7F, 0x6E, 0xFE, 0xCD, 0xAD, 0xDB,
            0xC5, 0x6F, 0x3C, 0xBF, 0x08, 0x04, 0x29, 0xAA, 0x01, 0xAC, 0xDA, 0x08, 0x3C, 0xAA,
            0xB3, 0xC4, 0x41, 0xD6, 0xA5, 0x0B, 0x98, 0x6F, 0xD4, 0x50, 0xF1, 0xC7, 0xCC, 0x3B,
            0x76, 0x00, 0xB8, 0x47, 0x41, 0x64, 0x5E, 0x59, 0x15, 0x58, 0xFA, 0xF8, 0x9F, 0x3B,
            0x96, 0xFF, 0xD2, 0xBC, 0x96, 0x50, 0x24, 0x99, 0x6E, 0x7F, 0x4E, 0x28, 0xEE, 0x06,
            0xC0, 0x32, 0xF4, 0x10, 0xD7, 0xC1, 0xC4, 0x4F, 0x87, 0x83, 0xDA, 0x1B, 0x48, 0xB1,
            0x74, 0x0D, 0x8C, 0x1C, 0x18, 0xD7, 0x9C, 0x29, 0x4A, 0xE3, 0xFC, 0xCF, 0x68, 0xF9,
            0x68, 0x6A, 0x1C, 0xC4, 0x41, 0x3E, 0xCA, 0x2C, 0x0E, 0xDD, 0x34, 0x18, 0xAB, 0xE7,
            0x97, 0x67, 0x1B, 0x6A, 0x97,
        ];
        let dec_text = decrypt(sk.to_be_bytes().to_vec().as_slice(), &js_dec_data.to_vec());
        assert_eq!(text, dec_text.as_slice());
    }

    #[test]
    fn test_random_data() {
        let sk = SecretKey::random(OsRng);
        let pk = sk.public_key();
        let pk = public_key_to_vec(&pk, true);

        let text = random_bytes(80);
        let data = encrypt(pk.as_slice(), text.as_slice());
        let dec_text = decrypt(sk.to_be_bytes().to_vec().as_slice(), &data);
        assert_eq!(text, dec_text);
    }

    // #[test]
    // fn test_random_ecies() {
    //     use ecies::utils::generate_keypair;
    //
    //     let text = random_bytes(80);
    //     let (sk, pk) = generate_keypair();
    //     let (sk, pk) = (&sk.serialize(), &pk.serialize());
    //
    //     let msg = text.as_slice();
    //     let enc = ecies::encrypt(pk, msg).unwrap();
    //
    //     let now = Local::now();
    //     let start = now.timestamp_millis();
    //
    //     let dec = ecies::decrypt(sk, enc.as_slice()).unwrap();
    //
    //     let now = Local::now();
    //     let end = now.timestamp_millis();
    //     println!("spend {:?}", end - start);
    //
    //     assert_eq!(text, dec);
    // }
}
