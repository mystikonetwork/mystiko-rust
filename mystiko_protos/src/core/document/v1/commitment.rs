use crate::core::document::v1::Commitment;
use mystiko_utils::convert::bytes_to_biguint;
use num_bigint::BigUint;

impl Commitment {
    pub fn commitment_hash_as_biguint(&self) -> BigUint {
        bytes_to_biguint(&self.commitment_hash)
    }

    pub fn rollup_fee_amount_as_biguint(&self) -> Option<BigUint> {
        self.rollup_fee_amount.as_ref().map(|fee| bytes_to_biguint(fee))
    }

    pub fn leaf_index_as_biguint(&self) -> Option<BigUint> {
        self.leaf_index.as_ref().map(|index| bytes_to_biguint(index))
    }

    pub fn amount_as_biguint(&self) -> Option<BigUint> {
        self.amount.as_ref().map(|amount| bytes_to_biguint(amount))
    }

    pub fn nullifier_as_biguint(&self) -> Option<BigUint> {
        self.nullifier.as_ref().map(|nullifier| bytes_to_biguint(nullifier))
    }
}
