use mystiko_utils::hex::encode_hex;
use sha2::{Digest, Sha512};

#[derive(Debug, Clone, Default)]
pub struct Sha512CheckSum;

impl crate::CheckSum for Sha512CheckSum {
    fn checksum(&self, data: &[u8]) -> String {
        let mut hasher = Sha512::new();
        hasher.update(data);
        encode_hex(hasher.finalize())
    }
}
