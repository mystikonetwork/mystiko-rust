use crate::types::{ENC_PK_SIZE, ENC_SK_SIZE, VERIFY_PK_SIZE, VERIFY_SK_SIZE};
use babyjubjub_rs::PrivateKey;
use bs58;
use k256::SecretKey;
// use elliptic_curve::SecretKey;
use crate::utils::big_int_to_u256;
use crate::utils::u256_to_fixed_bytes;
use ethers::core::types::U256;
use mystiko_crypto::constants::FIELD_SIZE;
use mystiko_crypto::eccrypto::public_key_to_vec;
use mystiko_crypto::utils::fr_to_big_int;

pub fn secret_key_for_verification(raw_secret_key: &[u8]) -> U256 {
    assert_eq!(raw_secret_key.len(), VERIFY_SK_SIZE);
    let pk = PrivateKey::import(raw_secret_key.to_vec()).unwrap();
    let sk = pk.scalar_key();
    assert!(sk < FIELD_SIZE.clone());
    big_int_to_u256(&sk)
}

pub fn public_key_for_verification(raw_secret_key: &[u8]) -> U256 {
    assert_eq!(raw_secret_key.len(), VERIFY_SK_SIZE);
    let pk = PrivateKey::import(raw_secret_key.to_vec()).unwrap();
    let point = pk.public();
    let pk_x = fr_to_big_int(&point.x);
    assert!(pk_x < FIELD_SIZE.clone());
    big_int_to_u256(&pk_x)
}

pub fn secret_key_for_encryption(raw_secret_key: &[u8]) -> U256 {
    assert_eq!(raw_secret_key.len(), ENC_SK_SIZE);
    U256::from_little_endian(raw_secret_key)
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
    pk_verify: &U256,
    pk_enc: &[u8; ENC_PK_SIZE],
) -> [u8; VERIFY_PK_SIZE + ENC_PK_SIZE] {
    let pk = u256_to_fixed_bytes(pk_verify);
    let mut combined = [0u8; VERIFY_PK_SIZE + ENC_PK_SIZE];
    combined[..VERIFY_PK_SIZE].copy_from_slice(&pk);
    combined[VERIFY_PK_SIZE..].copy_from_slice(pk_enc);
    combined
}

pub fn full_secret_key(sk_verify: &U256, sk_enc: &U256) -> [u8; VERIFY_SK_SIZE + ENC_SK_SIZE] {
    let sk_v = u256_to_fixed_bytes(sk_verify);
    let sk_e = u256_to_fixed_bytes(sk_enc);

    let mut combined = [0u8; VERIFY_SK_SIZE + ENC_SK_SIZE];
    combined[..VERIFY_SK_SIZE].copy_from_slice(&sk_v);
    combined[VERIFY_SK_SIZE..].copy_from_slice(&sk_e);
    combined
}

pub fn separated_public_keys(long_pk: &[u8; VERIFY_PK_SIZE + ENC_PK_SIZE]) -> (U256, &[u8]) {
    let (v_pk, e_pk) = long_pk.split_at(VERIFY_PK_SIZE);
    (U256::from_little_endian(v_pk), e_pk)
}

pub fn separated_secret_keys(long_sk: &[u8; VERIFY_SK_SIZE + ENC_SK_SIZE]) -> (U256, U256) {
    let (v_sk, e_sk) = long_sk.split_at(VERIFY_SK_SIZE);
    (
        U256::from_little_endian(v_sk),
        U256::from_little_endian(e_sk),
    )
}

pub fn shielded_address(pk_verify: &U256, pk_enc: &[u8; ENC_PK_SIZE]) -> String {
    bs58::encode(full_public_key(pk_verify, pk_enc)).into_string()
}

pub fn is_shielded_address(addr: &str) -> bool {
    match bs58::decode(addr).into_vec() {
        Err(_) => false,
        Ok(a) => a.len() == VERIFY_PK_SIZE + ENC_PK_SIZE,
    }
}

pub fn public_key_from_shielded_address(addr: String) -> (U256, [u8; ENC_PK_SIZE]) {
    assert!(is_shielded_address(&addr));
    let ck = bs58::decode(addr.as_str()).into_vec().unwrap();
    let ck = ck.as_slice();
    let mut vk = [0u8; VERIFY_PK_SIZE];
    let mut ek = [0u8; ENC_PK_SIZE];
    vk.copy_from_slice(&ck[0..VERIFY_PK_SIZE]);
    ek.copy_from_slice(&ck[VERIFY_PK_SIZE..]);
    (U256::from_little_endian(&vk[..]), ek)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_key_for_verification() {
        // todo do random key test
        let raw_key = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            0, 1, 2,
        ];
        let expect_vk = [
            0, 134, 214, 195, 195, 39, 41, 91, 119, 1, 111, 223, 51, 69, 90, 161, 81, 86, 103, 62,
            118, 211, 151, 234, 51, 125, 197, 58, 86, 95, 32, 15,
        ];

        let vk = secret_key_for_verification(&raw_key);
        let vk = u256_to_fixed_bytes(&vk);
        assert_eq!(vk, expect_vk);
    }

    #[test]
    fn test_public_key_for_verification() {
        let raw_key = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            0, 1, 2,
        ];
        let expect_sk = [
            35, 67, 78, 191, 181, 140, 63, 160, 165, 110, 204, 134, 242, 240, 250, 194, 198, 217,
            57, 5, 136, 71, 203, 25, 247, 30, 120, 219, 84, 207, 18, 27,
        ];

        let vk = public_key_for_verification(&raw_key);
        let vk = u256_to_fixed_bytes(&vk);
        assert_eq!(vk, expect_sk);
    }

    #[test]
    fn test_secret_key_for_encryption() {
        let raw_key = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            0, 1, 2,
        ];
        let sk = secret_key_for_encryption(&raw_key);
        let sk = u256_to_fixed_bytes(&sk);
        assert_eq!(sk, raw_key);
    }

    #[test]
    fn test_public_key_for_encryption() {
        let raw_key = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            0, 1, 2,
        ];
        let expect_pk = [
            2, 177, 145, 47, 171, 168, 15, 206, 205, 42, 100, 197, 116, 254, 254, 66, 44, 97, 16,
            96, 1, 236, 88, 138, 241, 189, 157, 117, 72, 184, 16, 100, 203,
        ];
        let pk = public_key_for_encryption(&raw_key);
        assert_eq!(pk, expect_pk);
    }

    #[test]
    fn test_full_and_separate_public_key() {
        let vk: [u8; VERIFY_PK_SIZE] = [
            0, 134, 214, 195, 195, 39, 41, 91, 119, 1, 111, 223, 51, 69, 90, 161, 81, 86, 103, 62,
            118, 211, 151, 234, 51, 125, 197, 58, 86, 95, 32, 15,
        ];
        let ek: [u8; ENC_PK_SIZE] = [
            2, 177, 145, 47, 171, 168, 15, 206, 205, 42, 100, 197, 116, 254, 254, 66, 44, 97, 16,
            96, 1, 236, 88, 138, 241, 189, 157, 117, 72, 184, 16, 100, 203,
        ];

        let expect_combine: [u8; VERIFY_PK_SIZE + ENC_PK_SIZE] = [
            0, 134, 214, 195, 195, 39, 41, 91, 119, 1, 111, 223, 51, 69, 90, 161, 81, 86, 103, 62,
            118, 211, 151, 234, 51, 125, 197, 58, 86, 95, 32, 15, 2, 177, 145, 47, 171, 168, 15,
            206, 205, 42, 100, 197, 116, 254, 254, 66, 44, 97, 16, 96, 1, 236, 88, 138, 241, 189,
            157, 117, 72, 184, 16, 100, 203,
        ];

        let vk_u256 = U256::from_little_endian(&vk);
        let combine = full_public_key(&vk_u256, &ek);
        assert_eq!(combine, expect_combine);

        let (vk_s, ek_s) = separated_public_keys(&combine);
        let vk_s = u256_to_fixed_bytes(&vk_s);
        assert_eq!(vk, vk_s);
        assert_eq!(ek, ek_s);
    }

    #[test]
    fn test_full_and_separate_secret_key() {
        let vk: [u8; VERIFY_SK_SIZE] = [
            0, 134, 214, 195, 195, 39, 41, 91, 119, 1, 111, 223, 51, 69, 90, 161, 81, 86, 103, 62,
            118, 211, 151, 234, 51, 125, 197, 58, 86, 95, 32, 15,
        ];
        let ek: [u8; ENC_SK_SIZE] = [
            2, 177, 145, 47, 171, 168, 15, 206, 205, 42, 100, 197, 116, 254, 254, 66, 44, 97, 16,
            96, 1, 236, 88, 138, 241, 189, 157, 117, 72, 184, 16, 0,
        ];

        let expect_combine: [u8; VERIFY_SK_SIZE + ENC_SK_SIZE] = [
            0, 134, 214, 195, 195, 39, 41, 91, 119, 1, 111, 223, 51, 69, 90, 161, 81, 86, 103, 62,
            118, 211, 151, 234, 51, 125, 197, 58, 86, 95, 32, 15, 2, 177, 145, 47, 171, 168, 15,
            206, 205, 42, 100, 197, 116, 254, 254, 66, 44, 97, 16, 96, 1, 236, 88, 138, 241, 189,
            157, 117, 72, 184, 16, 0,
        ];
        let vk_u256 = U256::from_little_endian(&vk);
        let ek_u256 = U256::from_little_endian(&ek);
        let combine = full_secret_key(&vk_u256, &ek_u256);
        assert_eq!(combine, expect_combine);

        let (vk_s, ek_s) = separated_secret_keys(&combine);
        let vk_s = u256_to_fixed_bytes(&vk_s);
        let ek_s = u256_to_fixed_bytes(&ek_s);
        assert_eq!(vk, vk_s);
        assert_eq!(ek, ek_s);
    }

    #[test]
    fn test_shielded_address() {
        let vk: [u8; VERIFY_PK_SIZE] = [
            0, 134, 214, 195, 195, 39, 41, 91, 119, 1, 111, 223, 51, 69, 90, 161, 81, 86, 103, 62,
            118, 211, 151, 234, 51, 125, 197, 58, 86, 95, 32, 15,
        ];
        let ek: [u8; ENC_PK_SIZE] = [
            2, 177, 145, 47, 171, 168, 15, 206, 205, 42, 100, 197, 116, 254, 254, 66, 44, 97, 16,
            96, 1, 236, 88, 138, 241, 189, 157, 117, 72, 184, 16, 100, 203,
        ];
        let expect_address = String::from("13hMt2P6h8zp5t8Cxm5oAzTULg1boVEvzjaEPXmLtSBUmF4KKnaooWkBKBqZs9BYncvY6rA6TpCkAJ6cEXFEHWMHt");
        let vk_u256 = U256::from_little_endian(&vk);
        let addr = shielded_address(&vk_u256, &ek);
        assert_eq!(addr, expect_address);

        assert!(is_shielded_address(&addr));
        let wrong_address = "*3hMt2P6h8zp5t8Cxm5oAzTULg1boVEvzjaEPXmLtSBUmF4KKnaooWkBKBqZs9BYncvY6rA6TpCkAJ6cEXFEHWMHt";
        assert!(!is_shielded_address(wrong_address));

        let (vk_f, ek_f) = public_key_from_shielded_address(addr);
        let vk_f = u256_to_fixed_bytes(&vk_f);
        assert_eq!(vk, vk_f);
        assert_eq!(ek, ek_f);
    }
}
