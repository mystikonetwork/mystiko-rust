mod sha512;

pub use sha512::*;

pub trait CheckSum {
    fn checksum(&self, data: &[u8]) -> String;
}
