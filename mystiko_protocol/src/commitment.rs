use crate::crypto::{decrypt_asymmetric, encrypt_asymmetric};
use crate::error::ProtocolError;
use crate::hash::poseidon_hash;
use crate::types::{DecryptedNote, EncryptedData, EncryptedNote, RandomSecrets};
use crate::types::{EncPk, RandomSk, SigPk, TxAmount, VerifyPk, VerifySk};
use crate::utils::big_int_to_u256;
use crate::utils::{u128_to_big_int, u256_to_big_int, u256_to_fixed_bytes};
use crate::wallet::{public_key_from_shielded_address, shielded_address};
use ethers::core::types::U256;
use num_bigint::{BigInt, Sign};

pub fn serial_number(sk_verify: &VerifySk, random_p: &RandomSk) -> U256 {
    let sk = u256_to_big_int(sk_verify);
    let rp = u128_to_big_int(random_p);
    let nullifier_key = poseidon_hash(&[sk]);
    let sn = poseidon_hash(&[rp, nullifier_key]);
    big_int_to_u256(&sn)
}

pub fn sig_pk_hash(sig_pk: &SigPk, secret_key: &VerifySk) -> U256 {
    let pk = BigInt::from_bytes_be(Sign::Plus, sig_pk);
    let sk = u256_to_big_int(secret_key);
    let sig_hash = poseidon_hash(&[sk, pk]);
    big_int_to_u256(&sig_hash)
}

pub enum PublicKeys {
    String(String),
    Object { pk_verify: VerifyPk, pk_enc: EncPk },
}

pub struct CommitmentInput {
    pub public_keys: PublicKeys,
    pub amount: Option<TxAmount>,
    pub random_secrets: Option<RandomSecrets>,
    pub encrypted_note: Option<EncryptedData>,
}

pub struct CommitmentOutput {
    pub encrypted_note: EncryptedNote,
    pub shielded_address: String,
    pub commitment_hash: BigInt,
    pub amount: TxAmount,
    pub random_p: RandomSk,
    pub random_r: RandomSk,
    pub random_s: RandomSk,
    pub k: BigInt,
}

pub fn build_commitment(input: CommitmentInput) -> Result<CommitmentOutput, ProtocolError> {
    let (pk_verify, pk_enc) = match input.public_keys {
        PublicKeys::String(s) => public_key_from_shielded_address(s),
        PublicKeys::Object { pk_verify, pk_enc } => (pk_verify, pk_enc),
    };

    let note = if let Some(n) = input.encrypted_note {
        decrypt_asymmetric(&u256_to_fixed_bytes(&n.sk_enc), &n.encrypted_note)
            .map(DecryptedNote::from_vec)
            .map_err(|e| ProtocolError::CryptoError(e.to_string()))?
    } else {
        let amount = input.amount.unwrap_or(U256::zero());
        let (random_p, random_r, random_s) = match input.random_secrets {
            Some(r) => (r.random_p, r.random_r, r.random_s),
            None => {
                let r = RandomSecrets::generate();
                (r.random_p, r.random_r, r.random_s)
            }
        };
        DecryptedNote {
            random_p,
            random_r,
            random_s,
            amount,
        }
    };

    let pk_verify_big = u256_to_big_int(&pk_verify);
    let random_p = u128_to_big_int(&note.random_p);
    let random_r = u128_to_big_int(&note.random_r);
    let random_s = u128_to_big_int(&note.random_s);
    let amount = u256_to_big_int(&note.amount);

    let k = poseidon_hash(&[pk_verify_big, random_p, random_r]);
    let commitment_hash = poseidon_hash(&[k.clone(), amount, random_s]);
    // todo check encrypt result
    let encrypted_note = encrypt_asymmetric(&pk_enc, note.to_vec().as_slice()).unwrap();
    let shielded_address = shielded_address(&pk_verify, &pk_enc);
    Ok(CommitmentOutput {
        encrypted_note,
        shielded_address,
        amount: note.amount,
        commitment_hash,
        random_p: note.random_p,
        random_r: note.random_r,
        random_s: note.random_s,
        k,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wallet::secret_key_for_encryption;
    use crate::wallet::{
        public_key_for_encryption, public_key_for_verification, secret_key_for_verification,
    };
    use ethers::core::types::U128;
    use ff::hex;
    use mystiko_crypto::utils::random_bytes;

    #[test]
    fn test_serial_number_compatible_with_js() {
        let raw_key = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            0, 1, 2,
        ];
        let expect_sn = BigInt::parse_bytes(
            b"16287b6ec504e86341497b36ca78a4b94ea41a2daa58e4a26b2b3b39dfb39e53",
            16,
        )
        .unwrap();
        let expect_sn = big_int_to_u256(&expect_sn);

        let sk = secret_key_for_verification(&raw_key);
        let random_p = b"1234567812345678";
        let random_p = U128::from_little_endian(&random_p[..]);
        let sn = serial_number(&sk, &random_p);
        assert_eq!(sn, expect_sn);
    }

    #[test]
    fn test_sig_pk_hash_compatible_with_js() {
        let raw_key = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            0, 1, 2,
        ];
        let sig_pk = hex::decode("fb8B7C14EB7251D8A62876424E13D27d47C84288").unwrap();
        let expect_sig_pk_hash = BigInt::parse_bytes(
            b"17300623865218087938631561261083046777856264605308935115400651673035276248790",
            10,
        )
        .unwrap();
        let expect_sig_pk_hash = big_int_to_u256(&expect_sig_pk_hash);
        let sk = secret_key_for_verification(&raw_key);
        let sig_pk_hash = sig_pk_hash(&sig_pk, &sk);
        assert_eq!(sig_pk_hash, expect_sig_pk_hash);
    }

    #[test]
    fn test_build_commitment_compatible_with_js() {
        let raw_verify_key =
            hex::decode("07abadd82fbee412006c12c12c462fb54f6e471744926d0946e8f165abc29c51")
                .unwrap();
        let raw_enc_key =
            hex::decode("a58b8123f6322b5fccdb4cf37449f6cb99b80f431072f081f8721a0c409bc8c6")
                .unwrap();
        let js_encrypt_note =
            hex::decode("fe180cd06620582c0eb07de543ee105104fd6a15fe793f688f02b83440d1d2930d527a7dccc05dcd6a2b709c9b72bab3474d21c9d443157efe3a23287f383d37680d8e568f7bbd9f2d372b4581ed33d3db64b6d2284cf94fd303bcf2cdc12f66f968340f796da94c76db51baf17875312e559aa02f45b256f19b474c9ae42a0e42af4469e584f4800744dc332ea68fff6e9772bda20f8db127612735cdb2bdf15d69fbaee541fff61093ec76bc0e73509fed89e8188e101aec11775a9ae1b9b3a8e5cc2642c8b5291a99285132f756d06c")
                .unwrap();

        let pk_verify = public_key_for_verification(&raw_verify_key);
        let pk_enc = public_key_for_encryption(&raw_enc_key);

        let sk_enc = secret_key_for_encryption(&raw_enc_key);
        let sk_enc = u256_to_fixed_bytes(&sk_enc);
        let note = decrypt_asymmetric(&sk_enc, &js_encrypt_note).unwrap();
        let js_decrypt_note = DecryptedNote::from_vec(note);
        let random_secret = RandomSecrets {
            random_p: js_decrypt_note.random_p,
            random_r: js_decrypt_note.random_r,
            random_s: js_decrypt_note.random_s,
        };
        let amount = js_decrypt_note.amount;
        let commitment_input = CommitmentInput {
            public_keys: PublicKeys::Object { pk_verify, pk_enc },
            amount: Some(amount),
            random_secrets: Some(random_secret),
            encrypted_note: None,
        };

        let output = build_commitment(commitment_input).unwrap();
        assert_eq!(output.amount, js_decrypt_note.amount);
        assert_eq!(output.random_p, js_decrypt_note.random_p);
        assert_eq!(output.random_r, js_decrypt_note.random_r);
        assert_eq!(output.random_s, js_decrypt_note.random_s);

        let note = decrypt_asymmetric(&sk_enc, &output.encrypted_note).unwrap();
        let decrypt_note = DecryptedNote::from_vec(note);
        assert_eq!(decrypt_note, js_decrypt_note);
    }

    #[test]
    fn test_build_commitment() {
        let raw_verify_key = random_bytes(32);
        let raw_enc_key = random_bytes(32);
        let pk_verify = public_key_for_verification(&raw_verify_key);
        let pk_enc = public_key_for_encryption(&raw_enc_key);
        let sk_enc = secret_key_for_encryption(&raw_enc_key);
        let amount = U256::from(10);

        let random_secrets = RandomSecrets::generate();
        let commitment_input = CommitmentInput {
            public_keys: PublicKeys::Object { pk_verify, pk_enc },
            amount: Some(amount),
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

        let sk_enc_byte = u256_to_fixed_bytes(&sk_enc);
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
            public_keys: PublicKeys::Object { pk_verify, pk_enc },
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
}
