use crate::mystiko_crypto::constants::FIELD_SIZE;
use crate::mystiko_crypto::hash::{hmac_sha512, poseidon_bigint, sha256 as sha256_raw};
use crate::mystiko_crypto::utils::calc_mod;
use num_bigint::BigInt;

pub fn sha256(msg: &[u8]) -> BigInt {
    let hash = sha256_raw(msg);
    calc_mod(hash, &FIELD_SIZE)
}

pub fn poseidon_hash(arr: &[BigInt]) -> BigInt {
    assert!(arr.len() < 7);
    let hash = poseidon_bigint(arr);
    assert!(hash < FIELD_SIZE.clone());
    hash
}

pub fn checksum(data: &str, salt: Option<&str>) -> String {
    let salt_str = match salt {
        Some(a) => {
            if a.is_empty() {
                "mystiko"
            } else {
                a
            }
        }
        _ => "mystiko",
    };

    let h = hmac_sha512(salt_str.as_bytes(), data.as_bytes());
    ff::hex::encode(h)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mystiko_crypto::utils::{random_big_int, random_bytes};

    #[test]
    fn test_mod_sha256() {
        // compatible test with js
        let data1 = b"baad";
        let hash_js = BigInt::parse_bytes(
            b"21386729550194668992953605105751046526972920509576226227329140564650153218040",
            10,
        )
        .unwrap();
        let hash1 = sha256(data1);
        assert_eq!(hash1, hash_js);

        let data2 = b"effe";
        let hash2 = sha256(data2);
        assert_ne!(hash1, hash2);

        let data3 = b"baad";
        let hash3 = sha256(data3);
        assert_eq!(hash1, hash3);
    }

    #[test]
    fn test_poseidon_compatible_with_js() {
        let b1 = BigInt::from(1);
        let b2 = BigInt::from(2);
        let expect_hash = BigInt::parse_bytes(
            b"7853200120776062878684798364095072458815029376092732009249414926327459813530",
            10,
        )
        .unwrap();
        let hash = poseidon_hash(&[b1, b2]);
        assert_eq!(hash, expect_hash);
    }

    #[test]
    fn test_poseidon() {
        let size = random_bytes(1);
        let b1 = random_big_int(size[0] as usize, &FIELD_SIZE);
        let size = random_bytes(1);
        let b2 = random_big_int(size[0] as usize, &FIELD_SIZE);
        poseidon_bigint(&[b1, b2]);
    }

    #[test]
    fn test_check_sum() {
        let hash1 = checksum("hello world", None);
        let hash2 = checksum("Mystiko is awesome", Some(""));
        let hash3 = checksum("Mystiko is awesome", Some("P@ssw0rd"));
        let hash4 = checksum("hello world", None);
        assert_ne!(hash1, hash2);
        assert_ne!(hash2, hash3);
        assert_ne!(hash3, hash4);
        assert_eq!(hash4, hash1);
        assert_eq!(hash3.as_str(), "03b41505aa26437d94831f9bfd24afd4e7eaf33d6aaf368d0c77545ad2a958024222badb4d84a17f84ff15297e16199dabc88b417ce764624ed5a2443681afcd");
        assert_eq!(hash2.as_str(), "8b9fb4d5f890ea83d09f63af9dee5ba8a53a9f5dedeb9bfd0e6ed135d5dca7abbc75d455fe0ee46040828834186543e008401238aeaaab1039f3a5ab37bb1c97");
    }
}
