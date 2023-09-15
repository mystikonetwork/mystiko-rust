use mystiko_utils::address::{string_address_from_bytes, string_address_to_bytes};

#[test]
fn test_string_address_from_bytes() {
    let address = "0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5";
    let bytes = vec![
        149, 34, 34, 144, 221, 114, 120, 170, 61, 221, 56, 156, 193, 225, 209, 101, 204, 75, 175, 229,
    ];
    assert_eq!(string_address_from_bytes(bytes), address);
}

#[test]
fn tests_string_address_to_bytes() {
    let address = "0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5";
    let bytes: Vec<u8> = vec![
        149, 34, 34, 144, 221, 114, 120, 170, 61, 221, 56, 156, 193, 225, 209, 101, 204, 75, 175, 229,
    ];
    assert_eq!(string_address_to_bytes(address).unwrap(), bytes);
}
