use crate::core::document::v1::Transaction;
use mystiko_utils::convert::bytes_to_biguint;
use num_bigint::BigUint;

impl Transaction {
    pub fn root_hash_as_bigint(&self) -> BigUint {
        bytes_to_biguint(&self.root_hash)
    }

    pub fn input_commitments_as_bigint(&self) -> Vec<BigUint> {
        self.input_commitments
            .iter()
            .map(|commitment| bytes_to_biguint(commitment))
            .collect()
    }

    pub fn output_commitments_as_bigint(&self) -> Option<Vec<BigUint>> {
        if self.output_commitments.is_empty() {
            None
        } else {
            Some(
                self.output_commitments
                    .iter()
                    .map(|commitment| bytes_to_biguint(commitment))
                    .collect(),
            )
        }
    }

    pub fn nullifiers_as_bigint(&self) -> Option<Vec<BigUint>> {
        if self.nullifiers.is_empty() {
            None
        } else {
            Some(
                self.nullifiers
                    .iter()
                    .map(|nullifier| bytes_to_biguint(nullifier))
                    .collect(),
            )
        }
    }

    pub fn amount_as_bigint(&self) -> BigUint {
        bytes_to_biguint(&self.amount)
    }

    pub fn public_amount_as_bigint(&self) -> BigUint {
        bytes_to_biguint(&self.public_amount)
    }

    pub fn rollup_fee_amount_as_bigint(&self) -> Option<BigUint> {
        self.rollup_fee_amount.as_ref().map(|amount| bytes_to_biguint(amount))
    }

    pub fn gas_relayer_fee_amount_as_bigint(&self) -> Option<BigUint> {
        self.gas_relayer_fee_amount
            .as_ref()
            .map(|amount| bytes_to_biguint(amount))
    }

    pub fn random_auditing_public_key_as_bigint(&self) -> Option<BigUint> {
        self.random_auditing_public_key
            .as_ref()
            .map(|key| bytes_to_biguint(key))
    }
}
