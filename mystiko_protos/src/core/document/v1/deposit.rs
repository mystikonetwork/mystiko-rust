use crate::core::document::v1::Deposit;
use mystiko_utils::convert::bytes_to_biguint;
use num_bigint::BigUint;

impl Deposit {
    pub fn commitment_hash_as_bigint(&self) -> BigUint {
        bytes_to_biguint(&self.commitment_hash)
    }

    pub fn hash_k_as_bigint(&self) -> BigUint {
        bytes_to_biguint(&self.hash_k)
    }

    pub fn random_s_as_bigint(&self) -> BigUint {
        bytes_to_biguint(&self.random_s)
    }

    pub fn amount_as_bigint(&self) -> BigUint {
        bytes_to_biguint(&self.amount)
    }

    pub fn rollup_fee_amount_as_bigint(&self) -> BigUint {
        bytes_to_biguint(&self.rollup_fee_amount)
    }

    pub fn bridge_fee_amount_as_bigint(&self) -> Option<BigUint> {
        self.bridge_fee_amount.as_ref().map(|amount| bytes_to_biguint(amount))
    }

    pub fn executor_fee_amount_as_bigint(&self) -> Option<BigUint> {
        self.executor_fee_amount.as_ref().map(|amount| bytes_to_biguint(amount))
    }

    pub fn service_fee_amount_as_bigint(&self) -> Option<BigUint> {
        self.service_fee_amount.as_ref().map(|amount| bytes_to_biguint(amount))
    }
}
