use crate::key::full_public_key;
use crate::types::{EncPk, VerifyPk};
use crate::types::{ENC_PK_SIZE, VERIFY_PK_SIZE};
use bs58;
use num_bigint::{BigInt, Sign};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ShieldedAddress {
    addr: String,
}

impl ShieldedAddress {
    pub fn address(&self) -> String {
        self.addr.clone()
    }

    pub fn check(addr: &str) -> bool {
        match bs58::decode(addr).into_vec() {
            Err(_) => false,
            Ok(a) => a.len() == VERIFY_PK_SIZE + ENC_PK_SIZE,
        }
    }

    pub fn from_string(addr: &str) -> Self {
        //todo
        assert!(ShieldedAddress::check(addr));
        Self {
            addr: addr.to_string(),
        }
    }

    pub fn from_public_key(pk_verify: &VerifyPk, pk_enc: &EncPk) -> Self {
        let addr = bs58::encode(full_public_key(pk_verify, pk_enc)).into_string();
        Self { addr }
    }

    pub fn public_key(&self) -> (VerifyPk, EncPk) {
        let ck = bs58::decode(self.addr.as_str()).into_vec().unwrap();
        let ck = ck.as_slice();
        assert_eq!(ck.len(), VERIFY_PK_SIZE + ENC_PK_SIZE);
        let mut vk = [0u8; VERIFY_PK_SIZE];
        let mut ek = [0u8; ENC_PK_SIZE];
        vk.copy_from_slice(&ck[0..VERIFY_PK_SIZE]);
        ek.copy_from_slice(&ck[VERIFY_PK_SIZE..]);
        (
            BigInt::from_bytes_le(Sign::Plus, &vk[..]),
            BigInt::from_bytes_le(Sign::Plus, &ek[..]),
        )
    }
}
