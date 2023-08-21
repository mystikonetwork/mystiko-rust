mod sha512;

pub use sha512::*;

pub trait CheckSum: Send + Sync {
    fn checksum(&self, data: &[u8]) -> String;
}

impl CheckSum for Box<dyn CheckSum> {
    fn checksum(&self, data: &[u8]) -> String {
        self.as_ref().checksum(data)
    }
}
