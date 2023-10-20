use ethers_core::types::{Bytes, U256};
use ethers_core::types::{NameOrAddress, TransactionRequest};
use mystiko_protos::core::v1::Transaction;
use mystiko_utils::address::ethers_address_from_string;
use mystiko_utils::hex::decode_hex;

#[test]
fn test_from_ether_transaction() {
    let ether_tx = TransactionRequest {
        from: Some(ethers_address_from_string("0x388C818CA8B9251b393131C08a736A67ccB19297").unwrap()),
        to: Some(NameOrAddress::Address(
            ethers_address_from_string("0xD71b82542A8629B172c5878e32a9cD1257CafEDB").unwrap(),
        )),
        gas: Some(U256::from(123_u16)),
        gas_price: Some(U256::from(234_u16)),
        value: Some(U256::from(345_u16)),
        data: Some(Bytes::from(decode_hex("0xdeadbeef").unwrap())),
        nonce: Some(U256::one()),
        chain_id: None,
    };
    let tx: Transaction = ether_tx.into();
    assert_eq!(tx.from.unwrap(), "0x388C818CA8B9251b393131C08a736A67ccB19297");
    assert_eq!(tx.to.unwrap(), "0xD71b82542A8629B172c5878e32a9cD1257CafEDB");
    assert_eq!(tx.gas.unwrap(), "0x7b");
    assert_eq!(tx.gas_price.unwrap(), "0xea");
    assert_eq!(tx.value.unwrap(), "0x159");
    assert_eq!(tx.data.unwrap(), "0xdeadbeef");
    assert_eq!(tx.nonce.unwrap(), "0x1");
}
