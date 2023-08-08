use crate::data::v1::Commitment;
use mystiko_utils::convert::bytes_to_biguint;
use mystiko_utils::hex::encode_hex_with_prefix;
use num_bigint::BigUint;

impl Commitment {
    pub fn commitment_hash_as_bigint(&self) -> BigUint {
        bytes_to_biguint(&self.commitment_hash)
    }

    pub fn rollup_fee_as_bigint(&self) -> Option<BigUint> {
        self.rollup_fee.as_ref().map(|fee| bytes_to_biguint(fee))
    }

    pub fn queued_transaction_hash_as_hex(&self) -> Option<String> {
        self.queued_transaction_hash
            .as_ref()
            .map(|hash| encode_hex_with_prefix(hash))
    }

    pub fn included_transaction_hash_as_hex(&self) -> Option<String> {
        self.included_transaction_hash
            .as_ref()
            .map(|hash| encode_hex_with_prefix(hash))
    }

    pub fn src_chain_transaction_hash_as_hex(&self) -> Option<String> {
        self.src_chain_transaction_hash
            .as_ref()
            .map(|hash| encode_hex_with_prefix(hash))
    }
}
