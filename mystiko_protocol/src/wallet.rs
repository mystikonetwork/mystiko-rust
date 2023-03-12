use crate::types::{ENC_PK_SIZE, ENC_SK_SIZE, VERIFY_PK_SIZE, VERIFY_SK_SIZE};
use babyjubjub_rs::PrivateKey;
use bs58;
use k256::SecretKey;
use mystiko_crypto::constants::FIELD_SIZE;
use mystiko_crypto::eccrypto::public_key_to_vec;
use mystiko_crypto::utils::{big_int_to_32_bytes, fr_to_big_int};
use num_bigint::{BigInt, Sign};

pub fn secret_key_for_verification(raw_secret_key: &[u8]) -> BigInt {
    assert_eq!(raw_secret_key.len(), VERIFY_SK_SIZE);
    let pk = PrivateKey::import(raw_secret_key.to_vec()).unwrap();
    let sk = pk.scalar_key();
    assert!(sk < FIELD_SIZE.clone());
    sk
}

pub fn public_key_for_verification(raw_secret_key: &[u8]) -> BigInt {
    assert_eq!(raw_secret_key.len(), VERIFY_SK_SIZE);
    let pk = PrivateKey::import(raw_secret_key.to_vec()).unwrap();
    let point = pk.public();
    let pk_x = fr_to_big_int(&point.x);
    assert!(pk_x < FIELD_SIZE.clone());
    pk_x
}

pub fn secret_key_for_encryption(raw_secret_key: &[u8]) -> BigInt {
    assert_eq!(raw_secret_key.len(), ENC_SK_SIZE);
    // BigInt::from_little_endian(raw_secret_key)
    BigInt::from_bytes_le(Sign::Plus, raw_secret_key)
}

pub fn public_key_for_encryption(raw_secret_key: &[u8]) -> [u8; ENC_PK_SIZE] {
    assert_eq!(raw_secret_key.len(), ENC_SK_SIZE);
    let secret_key = SecretKey::from_be_bytes(raw_secret_key).unwrap();
    let public_key = secret_key.public_key();

    let public_key_vec = public_key_to_vec(&public_key, true);
    assert_eq!(public_key_vec.len(), ENC_PK_SIZE);
    let mut arr = [0; ENC_PK_SIZE];
    arr.copy_from_slice(public_key_vec.as_slice());
    arr
}

pub fn full_public_key(
    pk_verify: &BigInt,
    pk_enc: &[u8; ENC_PK_SIZE],
) -> [u8; VERIFY_PK_SIZE + ENC_PK_SIZE] {
    let pk = big_int_to_32_bytes(pk_verify);
    let mut combined = [0u8; VERIFY_PK_SIZE + ENC_PK_SIZE];
    combined[..VERIFY_PK_SIZE].copy_from_slice(&pk);
    combined[VERIFY_PK_SIZE..].copy_from_slice(pk_enc);
    combined
}

pub fn full_secret_key(sk_verify: &BigInt, sk_enc: &BigInt) -> [u8; VERIFY_SK_SIZE + ENC_SK_SIZE] {
    let sk_v = big_int_to_32_bytes(sk_verify);
    let sk_e = big_int_to_32_bytes(sk_enc);

    let mut combined = [0u8; VERIFY_SK_SIZE + ENC_SK_SIZE];
    combined[..VERIFY_SK_SIZE].copy_from_slice(&sk_v);
    combined[VERIFY_SK_SIZE..].copy_from_slice(&sk_e);
    combined
}

pub fn separated_public_keys(long_pk: &[u8; VERIFY_PK_SIZE + ENC_PK_SIZE]) -> (BigInt, &[u8]) {
    let (v_pk, e_pk) = long_pk.split_at(VERIFY_PK_SIZE);
    (BigInt::from_bytes_le(Sign::Plus, v_pk), e_pk)
}

pub fn separated_secret_keys(long_sk: &[u8; VERIFY_SK_SIZE + ENC_SK_SIZE]) -> (BigInt, BigInt) {
    let (v_sk, e_sk) = long_sk.split_at(VERIFY_SK_SIZE);
    (
        BigInt::from_bytes_le(Sign::Plus, v_sk),
        BigInt::from_bytes_le(Sign::Plus, e_sk),
    )
}

pub fn shielded_address(pk_verify: &BigInt, pk_enc: &[u8; ENC_PK_SIZE]) -> String {
    bs58::encode(full_public_key(pk_verify, pk_enc)).into_string()
}

pub fn is_shielded_address(addr: &str) -> bool {
    match bs58::decode(addr).into_vec() {
        Err(_) => false,
        Ok(a) => a.len() == VERIFY_PK_SIZE + ENC_PK_SIZE,
    }
}

pub fn public_key_from_shielded_address(addr: String) -> (BigInt, [u8; ENC_PK_SIZE]) {
    assert!(is_shielded_address(&addr));
    let ck = bs58::decode(addr.as_str()).into_vec().unwrap();
    let ck = ck.as_slice();
    let mut vk = [0u8; VERIFY_PK_SIZE];
    let mut ek = [0u8; ENC_PK_SIZE];
    vk.copy_from_slice(&ck[0..VERIFY_PK_SIZE]);
    ek.copy_from_slice(&ck[VERIFY_PK_SIZE..]);
    (BigInt::from_bytes_le(Sign::Plus, &vk[..]), ek)
}
