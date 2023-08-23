use crate::data::v1::Nullifier;
use mystiko_utils::convert::bytes_to_biguint;
use mystiko_utils::hex::encode_hex_with_prefix;
use num_bigint::BigUint;

impl Nullifier {
    pub fn nullifier_as_biguint(&self) -> BigUint {
        bytes_to_biguint(&self.nullifier)
    }

    pub fn transaction_hash_as_hex(&self) -> String {
        encode_hex_with_prefix(&self.transaction_hash)
    }
}
