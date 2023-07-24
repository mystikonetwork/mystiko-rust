use crate::data::v1::Commitment;
use mystiko_utils::convert::bytes_to_big_int;
use mystiko_utils::hex::encode_hex_with_prefix;
use num_bigint::BigInt;

impl Commitment {
    pub fn commitment_hash_as_bigint(&self) -> BigInt {
        bytes_to_big_int(&self.commitment_hash)
    }

    pub fn rollup_fee_as_bigint(&self) -> Option<BigInt> {
        self.rollup_fee.as_ref().map(|fee| bytes_to_big_int(fee))
    }

    pub fn creation_transaction_hash_as_hex(&self) -> Option<String> {
        self.creation_transaction_hash
            .as_ref()
            .map(|hash| encode_hex_with_prefix(hash))
    }

    pub fn rollup_transaction_hash_as_hex(&self) -> Option<String> {
        self.rollup_transaction_hash
            .as_ref()
            .map(|hash| encode_hex_with_prefix(hash))
    }

    pub fn relay_transaction_hash_as_hex(&self) -> Option<String> {
        self.relay_transaction_hash
            .as_ref()
            .map(|hash| encode_hex_with_prefix(hash))
    }
}
