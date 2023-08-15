use crate::core::document::v1::Nullifier;
use mystiko_utils::convert::bytes_to_biguint;
use num_bigint::BigUint;

impl Nullifier {
    pub fn nullifier_as_biguint(&self) -> BigUint {
        bytes_to_biguint(&self.nullifier)
    }
}
