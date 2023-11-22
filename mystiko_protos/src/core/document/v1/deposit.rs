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

    pub fn decimal_amount_as_biguint(&self) -> Result<BigUint, ParseBigIntError> {
        BigUint::from_str(&self.decimal_amount)
    }

    pub fn rollup_fee_decimal_amount_as_biguint(&self) -> Result<BigUint, ParseBigIntError> {
        BigUint::from_str(&self.rollup_fee_decimal_amount)
    }

    pub fn bridge_fee_decimal_amount_as_biguint(&self) -> Result<Option<BigUint>, ParseBigIntError> {
        self.bridge_fee_decimal_amount
            .as_ref()
            .map(|s| BigUint::from_str(s))
            .transpose()
    }

    pub fn executor_fee_decimal_amount_as_biguint(&self) -> Result<Option<BigUint>, ParseBigIntError> {
        self.executor_fee_decimal_amount
            .as_ref()
            .map(|s| BigUint::from_str(s))
            .transpose()
    }
}
