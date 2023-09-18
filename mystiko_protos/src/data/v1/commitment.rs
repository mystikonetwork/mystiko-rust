use crate::data::v1::Commitment;
use mystiko_utils::convert::bytes_to_biguint;
use mystiko_utils::hex::encode_hex_with_prefix;
use num_bigint::BigUint;

impl Commitment {
    pub fn commitment_hash_as_biguint(&self) -> BigUint {
        bytes_to_biguint(&self.commitment_hash)
    }

    pub fn rollup_fee_as_biguint(&self) -> Option<BigUint> {
        self.rollup_fee.as_ref().map(bytes_to_biguint)
    }

    pub fn queued_transaction_hash_as_hex(&self) -> Option<String> {
        self.queued_transaction_hash.as_ref().map(encode_hex_with_prefix)
    }

    pub fn included_transaction_hash_as_hex(&self) -> Option<String> {
        self.included_transaction_hash.as_ref().map(encode_hex_with_prefix)
    }

    pub fn src_chain_transaction_hash_as_hex(&self) -> Option<String> {
        self.src_chain_transaction_hash.as_ref().map(encode_hex_with_prefix)
    }
}
