extern crate mystiko_crypto;
extern crate mystiko_protocol;
extern crate num_bigint;

use num_bigint::{BigInt, Sign};

use mystiko_crypto::utils::{big_int_to_32_bytes, big_int_to_33_bytes};
use mystiko_protocol::address::ShieldedAddress;
use mystiko_protocol::types::{ENC_PK_SIZE, VERIFY_PK_SIZE};

#[tokio::test]
async fn test_shielded_address() {
    let vk: [u8; VERIFY_PK_SIZE] = [
        0, 134, 214, 195, 195, 39, 41, 91, 119, 1, 111, 223, 51, 69, 90, 161, 81, 86, 103, 62, 118,
        211, 151, 234, 51, 125, 197, 58, 86, 95, 32, 15,
    ];
    let ek: [u8; ENC_PK_SIZE] = [
        2, 177, 145, 47, 171, 168, 15, 206, 205, 42, 100, 197, 116, 254, 254, 66, 44, 97, 16, 96,
        1, 236, 88, 138, 241, 189, 157, 117, 72, 184, 16, 100, 203,
    ];
    let expect_address = String::from(
        "13hMt2P6h8zp5t8Cxm5oAzTULg1boVEvzjaEPXmLtSBUmF4KKnaooWkBKBqZs9BYncvY6rA6TpCkAJ6cEXFEHWMHt",
    );
    let vk_big = BigInt::from_bytes_le(Sign::Plus, &vk);
    let ek_big = BigInt::from_bytes_le(Sign::Plus, &ek);
    let sa = ShieldedAddress::from_public_key(&vk_big, &ek_big);
    assert_eq!(sa.address(), expect_address);

    assert!(ShieldedAddress::check(&sa.address()));
    let wrong_address =
        "*3hMt2P6h8zp5t8Cxm5oAzTULg1boVEvzjaEPXmLtSBUmF4KKnaooWkBKBqZs9BYncvY6rA6TpCkAJ6cEXFEHWMHt";
    assert!(!ShieldedAddress::check(wrong_address));

    let (vk_f, ek_f) = sa.public_key();

    assert_eq!(vk, big_int_to_32_bytes(&vk_f));
    assert_eq!(ek, big_int_to_33_bytes(&ek_f));
}
