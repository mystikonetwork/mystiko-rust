use crate::error::ProtocolError;
use crate::key::combined_public_key;
use crate::types::{EncPk, FullPk, VerifyPk};
use crate::types::{ENC_PK_SIZE, FULL_PK_SIZE, VERIFY_PK_SIZE};
use bs58;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ShieldedAddress {
    addr: String,
}

impl ShieldedAddress {
    pub fn address(&self) -> String {
        self.addr.clone()
    }

    pub fn is_valid_address(addr: &str) -> bool {
        match bs58::decode(addr).into_vec() {
            Err(_) => false,
            Ok(key) => key.len() == FULL_PK_SIZE,
        }
    }

    pub fn from_string(addr: &str) -> Result<Self, ProtocolError> {
        if !ShieldedAddress::is_valid_address(addr) {
            return Err(ProtocolError::InvalidShieldedAddress);
        }

        Ok(Self { addr: addr.to_string() })
    }

    pub fn from_full_public_key(full_pk: &FullPk) -> Self {
        let addr = bs58::encode(full_pk).into_string();
        Self { addr }
    }

    pub fn from_public_key(pk_verify: &VerifyPk, pk_enc: &EncPk) -> Self {
        ShieldedAddress::from_full_public_key(&combined_public_key(pk_verify, pk_enc))
    }

    pub fn public_key(&self) -> (VerifyPk, EncPk) {
        let ck = bs58::decode(self.addr.as_str()).into_vec().unwrap();
        let ck = ck.as_slice();
        assert_eq!(ck.len(), FULL_PK_SIZE);
        let mut vk = [0u8; VERIFY_PK_SIZE];
        let mut ek = [0u8; ENC_PK_SIZE];
        vk.copy_from_slice(&ck[0..VERIFY_PK_SIZE]);
        ek.copy_from_slice(&ck[VERIFY_PK_SIZE..]);
        (vk, ek)
    }
}
