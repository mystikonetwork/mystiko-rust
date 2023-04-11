use anyhow::Result;
use hex::FromHexError;

pub fn encode_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

pub fn encode_hex_with_prefix(bytes: &[u8]) -> String {
    let hex_str = hex::encode(bytes);
    if hex_str.is_empty() {
        String::new()
    } else {
        format!("0x{}", hex_str)
    }
}

pub fn encode_fixed_len_hex(bytes: &[u8], length: usize) -> String {
    if length <= bytes.len() {
        hex::encode(&bytes[0..length])
    } else {
        let mut hex_str = hex::encode(bytes);
        hex_str.extend(std::iter::repeat('0').take(length - bytes.len()));
        hex_str
    }
}

pub fn encode_fixed_len_hex_with_prefix(bytes: &[u8], length: usize) -> String {
    let hex_str = encode_fixed_len_hex(bytes, length);
    if hex_str.is_empty() {
        String::new()
    } else {
        format!("0x{}", hex_str)
    }
}

pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, FromHexError> {
    if let Some(stripped) = hex_str.to_lowercase().strip_prefix("0x") {
        Ok(hex::decode(stripped)?)
    } else {
        Ok(hex::decode(hex_str)?)
    }
}

pub fn decode_hex_with_length<const N: usize>(hex_str: &str) -> Result<[u8; N], FromHexError> {
    let bytes = decode_hex(hex_str)?;
    match bytes.try_into() {
        Ok(bytes_array) => Ok(bytes_array),
        Err(_) => Err(FromHexError::InvalidStringLength),
    }
}
