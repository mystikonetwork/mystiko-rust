use anyhow::Result;
use rustc_hex::{FromHex, ToHex};

pub fn encode_hex<B>(bytes: B) -> String
where
    B: AsRef<[u8]>,
{
    bytes.as_ref().to_hex::<String>()
}

pub fn encode_hex_with_prefix<B>(bytes: B) -> String
where
    B: AsRef<[u8]>,
{
    let hex_str = encode_hex(bytes);
    if hex_str.is_empty() {
        String::new()
    } else {
        format!("0x{}", hex_str)
    }
}

pub fn encode_fixed_len_hex<B>(bytes: B, length: usize) -> String
where
    B: AsRef<[u8]>,
{
    let bytes = bytes.as_ref();
    if length <= bytes.len() {
        encode_hex(&bytes[0..length])
    } else {
        let mut hex_str = encode_hex(bytes);
        hex_str.extend(std::iter::repeat('0').take(length - bytes.len()));
        hex_str
    }
}

pub fn encode_fixed_len_hex_with_prefix<B>(bytes: B, length: usize) -> String
where
    B: AsRef<[u8]>,
{
    let hex_str = encode_fixed_len_hex(bytes, length);
    if hex_str.is_empty() {
        String::new()
    } else {
        format!("0x{}", hex_str)
    }
}

pub fn decode_hex<S>(hex_str: S) -> Result<Vec<u8>, rustc_hex::FromHexError>
where
    S: AsRef<str>,
{
    let hex_str = hex_str.as_ref();
    if let Some(stripped) = hex_str.to_lowercase().strip_prefix("0x") {
        Ok(stripped.from_hex::<Vec<u8>>()?)
    } else {
        Ok(hex_str.from_hex::<Vec<u8>>()?)
    }
}

pub fn decode_hex_with_length<const N: usize, S>(hex_str: S) -> Result<[u8; N], rustc_hex::FromHexError>
where
    S: AsRef<str>,
{
    let bytes = decode_hex(hex_str)?;
    match bytes.try_into() {
        Ok(bytes_array) => Ok(bytes_array),
        Err(_) => Err(rustc_hex::FromHexError::InvalidHexLength),
    }
}
