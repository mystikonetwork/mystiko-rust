use crate::types::{EncPk, EncSk, FullPk, FullSk, VerifyPk, VerifySk};
use crate::types::{ENC_PK_SIZE, ENC_SK_SIZE, VERIFY_PK_SIZE, VERIFY_SK_SIZE};
use babyjubjub_rs::PrivateKey;
use k256::SecretKey;
use mystiko_crypto::constants::FIELD_SIZE;
use mystiko_crypto::eccrypto::public_key_to_vec;
use mystiko_crypto::utils::{biguint_to_32_bytes, fr_to_bytes};

pub fn verification_secret_key(raw_secret_key: &VerifySk) -> VerifySk {
    let pk = PrivateKey::import(raw_secret_key.to_vec()).unwrap();
    let sk = pk.scalar_key().to_biguint().unwrap();
    assert!(sk < FIELD_SIZE.clone());
    biguint_to_32_bytes(&sk)
}

pub fn verification_public_key(raw_secret_key: &VerifySk) -> VerifyPk {
    let pk = PrivateKey::import(raw_secret_key.to_vec()).unwrap();
    let point = pk.public();
    fr_to_bytes(&point.x)
}

pub fn encryption_secret_key(raw_secret_key: &EncSk) -> EncSk {
    *raw_secret_key
}

pub fn encryption_public_key(raw_secret_key: &EncSk) -> EncPk {
    let secret_key = SecretKey::from_slice(raw_secret_key).unwrap();
    let public_key = secret_key.public_key();

    let public_key_vec = public_key_to_vec(&public_key, true);
    assert_eq!(public_key_vec.len(), ENC_PK_SIZE);
    public_key_vec.as_slice().try_into().unwrap()
}

pub fn combined_secret_key(sk_verify: &VerifySk, sk_enc: &EncSk) -> FullSk {
    let mut combined = [0u8; VERIFY_SK_SIZE + ENC_SK_SIZE];
    combined[..VERIFY_SK_SIZE].copy_from_slice(sk_verify);
    combined[VERIFY_SK_SIZE..].copy_from_slice(sk_enc);
    combined
}

pub fn combined_public_key(pk_verify: &VerifyPk, pk_enc: &EncPk) -> FullPk {
    let mut combined = [0u8; VERIFY_PK_SIZE + ENC_PK_SIZE];
    combined[..VERIFY_PK_SIZE].copy_from_slice(pk_verify);
    combined[VERIFY_PK_SIZE..].copy_from_slice(pk_enc);
    combined
}

pub fn separate_secret_keys(full_sk: &FullSk) -> (VerifySk, EncSk) {
    let (v_sk, e_sk) = full_sk.split_at(VERIFY_SK_SIZE);
    (v_sk.try_into().unwrap(), e_sk.try_into().unwrap())
}

pub fn separate_public_keys(full_pk: &FullPk) -> (VerifyPk, EncPk) {
    let (v_pk, e_pk) = full_pk.split_at(VERIFY_PK_SIZE);
    (v_pk.try_into().unwrap(), e_pk.try_into().unwrap())
}

pub fn full_public_key(full_sk: &FullSk) -> FullPk {
    let (v_sk, e_sk) = separate_secret_keys(full_sk);
    let v_pk = verification_public_key(&v_sk);
    let e_pk = encryption_public_key(&e_sk);
    combined_public_key(&v_pk, &e_pk)
}
