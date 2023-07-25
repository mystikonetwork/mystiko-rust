extern crate ff;
extern crate mystiko_crypto;
extern crate mystiko_protocol;
extern crate num_bigint;

use ff::hex;
use num_bigint::BigUint;

use crate::mystiko_protocol::utils::{compute_nullifier, compute_sig_pk_hash};
use mystiko_crypto::crypto::decrypt_asymmetric;
use mystiko_crypto::utils::random_bytes;
use mystiko_protocol::address::ShieldedAddress;
use mystiko_protocol::commitment::{Commitment, EncryptedData, Note};
use mystiko_protocol::error::ProtocolError;
use mystiko_protocol::key::{
    encryption_public_key, encryption_secret_key, verification_public_key, verification_secret_key,
};

#[tokio::test]
async fn test_nullifier_compatible_with_js() {
    let raw_key = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2,
    ];
    let expect_sn =
        BigUint::parse_bytes(b"16287b6ec504e86341497b36ca78a4b94ea41a2daa58e4a26b2b3b39dfb39e53", 16).unwrap();

    let sk = verification_secret_key(&raw_key);
    let random_p = b"1234567812345678";
    let sn = compute_nullifier(&sk, random_p);
    assert_eq!(sn, expect_sn);
}

#[tokio::test]
async fn test_sig_pk_hash_compatible_with_js() {
    let raw_key = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2,
    ];
    let sig_pk = hex::decode("fb8B7C14EB7251D8A62876424E13D27d47C84288").unwrap();
    let sig_pk = sig_pk.try_into().unwrap();
    let expect_sig_pk_hash = BigUint::parse_bytes(
        b"17300623865218087938631561261083046777856264605308935115400651673035276248790",
        10,
    )
    .unwrap();
    let sk = verification_secret_key(&raw_key);
    let sig_pk_hash = compute_sig_pk_hash(&sig_pk, &sk);
    assert_eq!(sig_pk_hash, expect_sig_pk_hash);
}

#[tokio::test]
async fn test_build_commitment_compatible_with_js() {
    let raw_verify_key = hex::decode("07abadd82fbee412006c12c12c462fb54f6e471744926d0946e8f165abc29c51").unwrap();
    let raw_enc_key = hex::decode("a58b8123f6322b5fccdb4cf37449f6cb99b80f431072f081f8721a0c409bc8c6").unwrap();
    let js_encrypt_note =
        hex::decode("fe180cd06620582c0eb07de543ee105104fd6a15fe793f688f02b83440d1d2930d527a7dccc05dcd6a2b709c9b72bab3474d21c9d443157efe3a23287f383d37680d8e568f7bbd9f2d372b4581ed33d3db64b6d2284cf94fd303bcf2cdc12f66f968340f796da94c76db51baf17875312e559aa02f45b256f19b474c9ae42a0e42af4469e584f4800744dc332ea68fff6e9772bda20f8db127612735cdb2bdf15d69fbaee541fff61093ec76bc0e73509fed89e8188e101aec11775a9ae1b9b3a8e5cc2642c8b5291a99285132f756d06c")
            .unwrap();

    let pk_verify = verification_public_key(raw_verify_key.as_slice().try_into().unwrap());
    let pk_enc = encryption_public_key(raw_enc_key.as_slice().try_into().unwrap());
    let sk_enc = encryption_secret_key(raw_enc_key.as_slice().try_into().unwrap());
    let note = decrypt_asymmetric(&sk_enc, &js_encrypt_note).unwrap();
    let js_decrypt_note = Note::from_vec(note).unwrap();

    let amount = js_decrypt_note.amount.clone();
    let cm = Commitment::new(
        ShieldedAddress::from_public_key(&pk_verify, &pk_enc),
        Some(Note::new(
            Some(amount),
            Some((
                js_decrypt_note.random_p,
                js_decrypt_note.random_r,
                js_decrypt_note.random_s,
            )),
        )),
        None,
    )
    .unwrap();

    assert_eq!(cm.note.amount, js_decrypt_note.amount);
    assert_eq!(cm.note.random_p.clone(), js_decrypt_note.random_p);
    assert_eq!(cm.note.random_r.clone(), js_decrypt_note.random_r);
    assert_eq!(cm.note.random_s.clone(), js_decrypt_note.random_s);

    let note = decrypt_asymmetric(&sk_enc, &cm.encrypted_note).unwrap();
    let decrypt_note = Note::from_vec(note).unwrap();
    assert_eq!(decrypt_note, js_decrypt_note);
}

#[tokio::test]
async fn test_build_commitment() {
    let raw_verify_key = random_bytes(32);
    let raw_enc_key = random_bytes(32);
    let pk_verify = verification_public_key(raw_verify_key.as_slice().try_into().unwrap());
    let pk_enc = encryption_public_key(raw_enc_key.as_slice().try_into().unwrap());
    let sk_enc = encryption_secret_key(raw_enc_key.as_slice().try_into().unwrap());
    let amount = BigUint::from(10u32);
    let note = Note::new(Some(amount.clone()), None);
    let cm1 = Commitment::new(
        ShieldedAddress::from_public_key(&pk_verify, &pk_enc),
        Some(note.clone()),
        None,
    )
    .unwrap();

    let shield_address = ShieldedAddress::from_public_key(&pk_verify, &pk_enc);
    assert_eq!(cm1.note.amount, amount);
    assert_eq!(cm1.note.random_p, note.random_p);
    assert_eq!(cm1.note.random_r, note.random_r);
    assert_eq!(cm1.note.random_s, note.random_s);
    assert_eq!(cm1.shielded_address, shield_address);

    let note_vec = decrypt_asymmetric(&sk_enc, &cm1.encrypted_note).unwrap();
    let decrypt_note = Note::from_vec(note_vec).unwrap();
    assert_eq!(decrypt_note.amount, amount);
    assert_eq!(decrypt_note.random_p, note.random_p);
    assert_eq!(decrypt_note.random_r, note.random_r);
    assert_eq!(decrypt_note.random_s, note.random_s);

    let cm2 = Commitment::new(
        shield_address,
        None,
        Some(EncryptedData {
            sk_enc,
            encrypted_note: cm1.encrypted_note.clone(),
        }),
    )
    .unwrap();
    let _ = cm2;
    assert_eq!(cm2.commitment_hash, cm1.commitment_hash);

    let raw_sk_enc_3 = random_bytes(32);
    let sk_enc_3 = encryption_secret_key(&raw_sk_enc_3.try_into().unwrap());
    let cm3 = Commitment::new(
        ShieldedAddress::from_public_key(&pk_verify, &pk_enc),
        None,
        Some(EncryptedData {
            sk_enc: sk_enc_3,
            encrypted_note: cm1.encrypted_note.clone(),
        }),
    );
    assert!(matches!(cm3.err().unwrap(), ProtocolError::ECCryptoError(_)));

    let enc_data = EncryptedData {
        sk_enc: sk_enc_3,
        encrypted_note: cm1.encrypted_note,
    };
    let _ = enc_data;
}

#[tokio::test]
async fn test_build_note() {
    let note = Note::from_vec(vec![1]);
    assert!(matches!(note.err().unwrap(), ProtocolError::InvalidNoteSize));
}
