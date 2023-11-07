use crate::core::document::v1::Deposit;
use num_bigint::{BigUint, ParseBigIntError};
use std::str::FromStr;

impl Deposit {
    pub fn commitment_hash_as_biguint(&self) -> Result<BigUint, ParseBigIntError> {
        BigUint::from_str(&self.commitment_hash)
    }

    pub fn hash_k_as_biguint(&self) -> Result<BigUint, ParseBigIntError> {
        BigUint::from_str(&self.hash_k)
    }

    pub fn random_s_as_biguint(&self) -> Result<BigUint, ParseBigIntError> {
        BigUint::from_str(&self.random_s)
    }
}
