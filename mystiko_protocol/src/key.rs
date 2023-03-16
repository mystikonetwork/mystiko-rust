use crate::types::{EncPk, EncSk, FullPk, FullSk, VerifyPk, VerifySk};
use crate::types::{ENC_PK_SIZE, ENC_SK_SIZE, VERIFY_PK_SIZE, VERIFY_SK_SIZE};
use babyjubjub_rs::PrivateKey;
use k256::SecretKey;
use mystiko_crypto::constants::FIELD_SIZE;
use mystiko_crypto::eccrypto::public_key_to_vec;
use mystiko_crypto::utils::{bigint_to_32_bytes, fr_to_bytes};

pub fn secret_key_for_verification(raw_secret_key: &[u8]) -> VerifySk {
    assert_eq!(raw_secret_key.len(), VERIFY_SK_SIZE);
    let pk = PrivateKey::import(raw_secret_key.to_vec()).unwrap();
    let sk = pk.scalar_key();
    assert!(sk < FIELD_SIZE.clone());
    bigint_to_32_bytes(&sk)
}

pub fn public_key_for_verification(raw_secret_key: &[u8]) -> VerifyPk {
    assert_eq!(raw_secret_key.len(), VERIFY_SK_SIZE);
    let pk = PrivateKey::import(raw_secret_key.to_vec()).unwrap();
    let point = pk.public();
    fr_to_bytes(&point.x)
}

pub fn secret_key_for_encryption(raw_secret_key: &[u8]) -> EncSk {
    assert_eq!(raw_secret_key.len(), ENC_SK_SIZE);
    raw_secret_key.try_into().unwrap()
}

pub fn public_key_for_encryption(raw_secret_key: &[u8]) -> EncPk {
    assert_eq!(raw_secret_key.len(), ENC_SK_SIZE);
    let secret_key = SecretKey::from_be_bytes(raw_secret_key).unwrap();
    let public_key = secret_key.public_key();

    let public_key_vec = public_key_to_vec(&public_key, true);
    assert_eq!(public_key_vec.len(), ENC_PK_SIZE);
    public_key_vec.as_slice().try_into().unwrap()
}

pub fn full_secret_key(sk_verify: &[u8], sk_enc: &[u8]) -> FullSk {
    let mut combined = [0u8; VERIFY_SK_SIZE + ENC_SK_SIZE];
    combined[..VERIFY_SK_SIZE].copy_from_slice(sk_verify);
    combined[VERIFY_SK_SIZE..].copy_from_slice(sk_enc);
    combined
}

pub fn full_public_key(pk_verify: &[u8], pk_enc: &[u8]) -> FullPk {
    let mut combined = [0u8; VERIFY_PK_SIZE + ENC_PK_SIZE];
    combined[..VERIFY_PK_SIZE].copy_from_slice(pk_verify);
    combined[VERIFY_PK_SIZE..].copy_from_slice(pk_enc);
    combined
}

pub fn separated_secret_keys(full_sk: &FullSk) -> (VerifySk, EncSk) {
    let (v_sk, e_sk) = full_sk.split_at(VERIFY_SK_SIZE);
    (v_sk.try_into().unwrap(), e_sk.try_into().unwrap())
}

pub fn separated_public_keys(full_pk: &FullPk) -> (VerifyPk, EncPk) {
    let (v_pk, e_pk) = full_pk.split_at(VERIFY_PK_SIZE);
    (v_pk.try_into().unwrap(), e_pk.try_into().unwrap())
}
