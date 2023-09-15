use mystiko_utils::hex::*;

#[test]
fn test_encode_hex() {
    let bytes = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    let hex_str = "0102030405060708";
    assert_eq!(encode_hex(&bytes), hex_str);
    assert_eq!(encode_hex_with_prefix(&bytes), format!("0x{}", hex_str));
    assert_eq!(encode_hex_with_prefix([]), "");
    assert_eq!(encode_fixed_len_hex(&bytes, 0), "");
    assert_eq!(encode_fixed_len_hex(&bytes, 7), "01020304050607");
    assert_eq!(encode_fixed_len_hex(&bytes, 8), hex_str);
    assert_eq!(encode_fixed_len_hex(&bytes, 16), format!("{}{}", hex_str, "00000000"));
    assert_eq!(encode_fixed_len_hex_with_prefix(&bytes, 0), "");
    assert_eq!(encode_fixed_len_hex_with_prefix(&bytes, 7), "0x01020304050607");
    assert_eq!(encode_fixed_len_hex_with_prefix(&bytes, 8), format!("0x{}", hex_str));
    assert_eq!(
        encode_fixed_len_hex_with_prefix(&bytes, 16),
        format!("0x{}{}", hex_str, "00000000")
    );
}

#[test]
fn test_decode_hex() {
    let bytes: [u8; 8] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    let hex_str = "0102030405060708";
    assert_eq!(decode_hex(format!("0x{}", hex_str).as_str()).unwrap(), bytes);
    assert_eq!(decode_hex(format!("0X{}", hex_str).as_str()).unwrap(), bytes);
    assert_eq!(decode_hex(hex_str).unwrap(), bytes);
    assert!(decode_hex("invalid_hex").is_err());
    assert_eq!(decode_hex_with_length::<8, &str>(hex_str).unwrap(), bytes);
    assert_eq!(
        decode_hex_with_length::<8, String>(format!("0x{}", hex_str)).unwrap(),
        bytes
    );
    assert_eq!(
        decode_hex_with_length::<8, String>(format!("0X{}", hex_str)).unwrap(),
        bytes
    );
    assert!(decode_hex_with_length::<8, &str>("invalid_hex").is_err());
    assert!(decode_hex_with_length::<10, &str>(hex_str).is_err());
}
