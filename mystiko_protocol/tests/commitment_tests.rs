extern crate ff;
extern crate mystiko_crypto;
extern crate mystiko_protocol;
extern crate num_bigint;

use ff::hex;
use num_bigint::{BigInt, Sign};

use mystiko_crypto::utils::{big_int_to_32_bytes, random_bytes};
use mystiko_protocol::commitment::{
    build_commitment, serial_number, sig_pk_hash, CommitmentInput, PublicKeys,
};
use mystiko_protocol::crypto::decrypt_asymmetric;
use mystiko_protocol::types::{DecryptedNote, EncryptedData, RandomSecrets};
use mystiko_protocol::wallet::{
    public_key_for_encryption, public_key_for_verification, secret_key_for_encryption,
    secret_key_for_verification, shielded_address,
};

#[tokio::test]
async fn test_serial_number_compatible_with_js() {
    let raw_key = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0,
        1, 2,
    ];
    let expect_sn = BigInt::parse_bytes(
        b"16287b6ec504e86341497b36ca78a4b94ea41a2daa58e4a26b2b3b39dfb39e53",
        16,
    )
    .unwrap();

    let sk = secret_key_for_verification(&raw_key);
    let random_p = b"1234567812345678";
    let random_p = BigInt::from_bytes_le(Sign::Plus, &random_p[..]);
    let sn = serial_number(sk, random_p);
    assert_eq!(sn, expect_sn);
}

#[tokio::test]
async fn test_sig_pk_hash_compatible_with_js() {
    let raw_key = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0,
        1, 2,
    ];
    let sig_pk = hex::decode("fb8B7C14EB7251D8A62876424E13D27d47C84288").unwrap();
    let expect_sig_pk_hash = BigInt::parse_bytes(
        b"17300623865218087938631561261083046777856264605308935115400651673035276248790",
        10,
    )
    .unwrap();
    let sk = secret_key_for_verification(&raw_key);
    let sig_pk_hash = sig_pk_hash(&sig_pk, sk);
    assert_eq!(sig_pk_hash, expect_sig_pk_hash);
}

#[tokio::test]
async fn test_build_commitment_compatible_with_js() {
    let raw_verify_key =
        hex::decode("07abadd82fbee412006c12c12c462fb54f6e471744926d0946e8f165abc29c51").unwrap();
    let raw_enc_key =
        hex::decode("a58b8123f6322b5fccdb4cf37449f6cb99b80f431072f081f8721a0c409bc8c6").unwrap();
    let js_encrypt_note =
        hex::decode("fe180cd06620582c0eb07de543ee105104fd6a15fe793f688f02b83440d1d2930d527a7dccc05dcd6a2b709c9b72bab3474d21c9d443157efe3a23287f383d37680d8e568f7bbd9f2d372b4581ed33d3db64b6d2284cf94fd303bcf2cdc12f66f968340f796da94c76db51baf17875312e559aa02f45b256f19b474c9ae42a0e42af4469e584f4800744dc332ea68fff6e9772bda20f8db127612735cdb2bdf15d69fbaee541fff61093ec76bc0e73509fed89e8188e101aec11775a9ae1b9b3a8e5cc2642c8b5291a99285132f756d06c")
            .unwrap();

    let pk_verify = public_key_for_verification(&raw_verify_key);
    let pk_enc = public_key_for_encryption(&raw_enc_key);

    let sk_enc = secret_key_for_encryption(&raw_enc_key);
    let sk_enc = big_int_to_32_bytes(&sk_enc);
    let note = decrypt_asymmetric(&sk_enc, &js_encrypt_note).unwrap();
    let js_decrypt_note = DecryptedNote::from_vec(note);
    let random_secret = RandomSecrets {
        random_p: js_decrypt_note.random_p.clone(),
        random_r: js_decrypt_note.random_r.clone(),
        random_s: js_decrypt_note.random_s.clone(),
    };
    let amount = js_decrypt_note.amount.clone();
    let commitment_input = CommitmentInput {
        public_keys: PublicKeys::Object { pk_verify, pk_enc },
        amount: Some(amount),
        random_secrets: Some(random_secret),
        encrypted_note: None,
    };

    let output = build_commitment(commitment_input).unwrap();
    assert_eq!(output.amount.clone(), js_decrypt_note.amount);
    assert_eq!(output.random_p.clone(), js_decrypt_note.random_p);
    assert_eq!(output.random_r.clone(), js_decrypt_note.random_r);
    assert_eq!(output.random_s.clone(), js_decrypt_note.random_s);

    let note = decrypt_asymmetric(&sk_enc, &output.encrypted_note).unwrap();
    let decrypt_note = DecryptedNote::from_vec(note);
    assert_eq!(decrypt_note, js_decrypt_note);
}

#[tokio::test]
async fn test_build_commitment() {
    let raw_verify_key = random_bytes(32);
    let raw_enc_key = random_bytes(32);
    let pk_verify = public_key_for_verification(&raw_verify_key);
    let pk_enc = public_key_for_encryption(&raw_enc_key);
    let sk_enc = secret_key_for_encryption(&raw_enc_key);
    let amount = BigInt::from(10u32);

    let random_secrets = RandomSecrets::generate();
    let commitment_input = CommitmentInput {
        public_keys: PublicKeys::Object {
            pk_verify: pk_verify.clone(),
            pk_enc,
        },
        amount: Some(amount.clone()),
        random_secrets: Some(random_secrets.clone()),
        encrypted_note: None,
    };
    let o1 = build_commitment(commitment_input).unwrap();
    let shield_address = shielded_address(&pk_verify, &pk_enc);
    assert_eq!(o1.amount, amount);
    assert_eq!(o1.random_p, random_secrets.random_p);
    assert_eq!(o1.random_r, random_secrets.random_r);
    assert_eq!(o1.random_s, random_secrets.random_s);
    assert_eq!(o1.shielded_address, shield_address.clone());

    let sk_enc_byte = big_int_to_32_bytes(&sk_enc);
    let note = decrypt_asymmetric(&sk_enc_byte, &o1.encrypted_note).unwrap();
    let decrypt_note = DecryptedNote::from_vec(note);
    assert_eq!(decrypt_note.amount, amount);
    assert_eq!(decrypt_note.random_p, random_secrets.random_p);
    assert_eq!(decrypt_note.random_r, random_secrets.random_r);
    assert_eq!(decrypt_note.random_s, random_secrets.random_s);

    let c2 = CommitmentInput {
        public_keys: PublicKeys::String { 0: shield_address },
        amount: None,
        random_secrets: None,
        encrypted_note: Some(EncryptedData {
            sk_enc,
            encrypted_note: o1.encrypted_note.clone(),
        }),
    };
    let o2 = build_commitment(c2).unwrap();
    assert_eq!(o2.commitment_hash, o1.commitment_hash);

    let raw_sk_enc_3 = random_bytes(32);
    let sk_enc_3 = secret_key_for_encryption(&raw_sk_enc_3);
    let c3 = CommitmentInput {
        public_keys: PublicKeys::Object {
            pk_verify: pk_verify.clone(),
            pk_enc,
        },
        amount: None,
        random_secrets: None,
        encrypted_note: Some(EncryptedData {
            sk_enc: sk_enc_3,
            encrypted_note: o1.encrypted_note,
        }),
    };

    let o3 = build_commitment(c3);
    assert!(o3.is_err())
}
