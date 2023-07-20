use crate::data::v1::Nullifier;
use mystiko_utils::hex::encode_hex_with_prefix;

impl Nullifier {
    pub fn transaction_hash_as_hex(&self) -> String {
        encode_hex_with_prefix(&self.transaction_hash)
    }
}
