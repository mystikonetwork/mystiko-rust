extern crate mystiko_crypto;
extern crate mystiko_protocol;
extern crate num_bigint;

use mystiko_protocol::address::ShieldedAddress;
use mystiko_protocol::error::ProtocolError;
use mystiko_protocol::types::{ENC_PK_SIZE, VERIFY_PK_SIZE};

#[tokio::test]
async fn test_shielded_address() {
    let vk: [u8; VERIFY_PK_SIZE] = [
        0, 134, 214, 195, 195, 39, 41, 91, 119, 1, 111, 223, 51, 69, 90, 161, 81, 86, 103, 62, 118, 211, 151, 234, 51,
        125, 197, 58, 86, 95, 32, 15,
    ];
    let ek: [u8; ENC_PK_SIZE] = [
        2, 177, 145, 47, 171, 168, 15, 206, 205, 42, 100, 197, 116, 254, 254, 66, 44, 97, 16, 96, 1, 236, 88, 138, 241,
        189, 157, 117, 72, 184, 16, 100, 203,
    ];
    let expect_address =
        String::from("13hMt2P6h8zp5t8Cxm5oAzTULg1boVEvzjaEPXmLtSBUmF4KKnaooWkBKBqZs9BYncvY6rA6TpCkAJ6cEXFEHWMHt");
    let sa = ShieldedAddress::from_public_key(&vk, &ek);
    assert_eq!(sa.address(), expect_address);

    let sa2 = ShieldedAddress::from_string(&expect_address).unwrap();
    assert_eq!(sa, sa2);

    assert!(ShieldedAddress::is_valid_address(&sa.address()));
    let wrong_address = "*3hMt2P6h8zp5t8Cxm5oAzTULg1boVEvzjaEPXmLtSBUmF4KKnaooWkBKBqZs9BYncvY6rA6TpCkAJ6cEXFEHWMHt";
    let sa3 = ShieldedAddress::from_string(wrong_address);
    assert!(matches!(sa3.err().unwrap(), ProtocolError::InvalidShieldedAddress));

    let (vk_f, ek_f) = sa.public_key();

    assert_eq!(vk, vk_f);
    assert_eq!(ek, ek_f);
}
