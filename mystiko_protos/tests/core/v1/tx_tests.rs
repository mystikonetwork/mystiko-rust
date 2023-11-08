use ethers_core::abi::AbiDecode;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::transaction::eip2930::{AccessList, AccessListItem};
use ethers_core::types::{Bytes, Eip1559TransactionRequest, Eip2930TransactionRequest, H256, U256, U64};
use ethers_core::types::{NameOrAddress, TransactionRequest};
use mystiko_protos::core::v1::{
    transaction::Transaction as TransactionEnum, Eip1559Transaction, Eip2930Transaction, LegacyTransaction, Transaction,
};
use mystiko_utils::address::{ethers_address_from_string, ethers_address_to_string};
use mystiko_utils::convert::u256_to_hex_string;
use mystiko_utils::hex::{decode_hex, encode_hex_with_prefix};

#[test]
fn test_from_legacy_transaction() {
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
        chain_id: Some(U64::from(1_u64)),
    };
    let tx: Transaction = TypedTransaction::Legacy(ether_tx).into();
    match tx.transaction.unwrap() {
        TransactionEnum::LegacyTransaction(tx) => {
            assert_eq!(tx.from.unwrap(), "0x388C818CA8B9251b393131C08a736A67ccB19297");
            assert_eq!(tx.to.unwrap(), "0xD71b82542A8629B172c5878e32a9cD1257CafEDB");
            assert_eq!(tx.gas.unwrap(), "0x7b");
            assert_eq!(tx.gas_price.unwrap(), "0xea");
            assert_eq!(tx.value.unwrap(), "0x159");
            assert_eq!(tx.data.unwrap(), "0xdeadbeef");
            assert_eq!(tx.nonce.unwrap(), "0x1");
            assert_eq!(tx.chain_id.unwrap(), 1);
        }
        _ => panic!("Expected legacy transaction"),
    }
}

#[test]
fn test_try_into_legacy_transaction() {
    let tx = Transaction {
        transaction: Some(TransactionEnum::LegacyTransaction(LegacyTransaction {
            from: Some("0x388C818CA8B9251b393131C08a736A67ccB19297".to_string()),
            to: Some("0xD71b82542A8629B172c5878e32a9cD1257CafEDB".to_string()),
            gas: Some("0x7b".to_string()),
            gas_price: Some("0xea".to_string()),
            value: Some("0x159".to_string()),
            data: Some("0xdeadbeef".to_string()),
            nonce: Some("0x1".to_string()),
            chain_id: Some(1),
        })),
    };
    let tx: TypedTransaction = tx.try_into().unwrap();
    match tx {
        TypedTransaction::Legacy(tx) => {
            assert_eq!(
                ethers_address_to_string(&tx.from.unwrap()),
                "0x388C818CA8B9251b393131C08a736A67ccB19297"
            );
            assert_eq!(
                ethers_address_to_string(tx.to.unwrap().as_address().unwrap()),
                "0xD71b82542A8629B172c5878e32a9cD1257CafEDB"
            );
            assert_eq!(u256_to_hex_string(&tx.gas.unwrap()), "0x7b");
            assert_eq!(u256_to_hex_string(&tx.gas_price.unwrap()), "0xea");
            assert_eq!(u256_to_hex_string(&tx.value.unwrap()), "0x159");
            assert_eq!(encode_hex_with_prefix(tx.data.unwrap()), "0xdeadbeef");
            assert_eq!(u256_to_hex_string(&tx.nonce.unwrap()), "0x1");
            assert_eq!(tx.chain_id.unwrap().as_u64(), 1);
        }
        _ => panic!("Expected legacy transaction"),
    }
}

#[test]
fn test_from_eip1559_transaction() {
    let ether_tx = Eip1559TransactionRequest {
        from: Some(ethers_address_from_string("0x388C818CA8B9251b393131C08a736A67ccB19297").unwrap()),
        to: Some(NameOrAddress::Address(
            ethers_address_from_string("0xD71b82542A8629B172c5878e32a9cD1257CafEDB").unwrap(),
        )),
        gas: Some(U256::from(123_u16)),
        max_fee_per_gas: Some(U256::from(234_u16)),
        max_priority_fee_per_gas: Some(U256::from(345_u16)),
        value: Some(U256::from(456_u16)),
        data: Some(Bytes::from(decode_hex("0xdeadbeef").unwrap())),
        nonce: Some(U256::one()),
        chain_id: Some(U64::from(1_u64)),
        access_list: AccessList(vec![AccessListItem {
            address: ethers_address_from_string("0x6088B06c5a187058434655B71057a9ee93E13d0d").unwrap(),
            storage_keys: vec![
                H256::decode_hex("0xb33fae07e1262a169631d35aea9ecbe5068d6554504dd06c26d0dcd35b199d22").unwrap(),
            ],
        }]),
    };
    let tx: Transaction = TypedTransaction::Eip1559(ether_tx).into();
    match tx.transaction.unwrap() {
        TransactionEnum::Eip1559Transaction(mut tx) => {
            assert_eq!(tx.from.unwrap(), "0x388C818CA8B9251b393131C08a736A67ccB19297");
            assert_eq!(tx.to.unwrap(), "0xD71b82542A8629B172c5878e32a9cD1257CafEDB");
            assert_eq!(tx.gas.unwrap(), "0x7b");
            assert_eq!(tx.max_fee_per_gas.unwrap(), "0xea");
            assert_eq!(tx.max_priority_fee_per_gas.unwrap(), "0x159");
            assert_eq!(tx.value.unwrap(), "0x1c8");
            assert_eq!(tx.data.unwrap(), "0xdeadbeef");
            assert_eq!(tx.nonce.unwrap(), "0x1");
            assert_eq!(tx.chain_id.unwrap(), 1);
            assert_eq!(tx.access_list.len(), 1);
            let access_list_item = tx.access_list.pop().unwrap();
            assert_eq!(access_list_item.address, "0x6088B06c5a187058434655B71057a9ee93E13d0d");
            assert_eq!(access_list_item.storage_keys.len(), 1);
            assert_eq!(
                access_list_item.storage_keys[0],
                "0xb33fae07e1262a169631d35aea9ecbe5068d6554504dd06c26d0dcd35b199d22"
            );
        }
        _ => panic!("Expected EIP-1559 transaction"),
    }
}

#[test]
fn test_try_into_eip1559_transaction() {
    let tx = Transaction {
        transaction: Some(TransactionEnum::Eip1559Transaction(Eip1559Transaction {
            from: Some("0x388C818CA8B9251b393131C08a736A67ccB19297".to_string()),
            to: Some("0xD71b82542A8629B172c5878e32a9cD1257CafEDB".to_string()),
            gas: Some("0x7b".to_string()),
            max_fee_per_gas: Some("0xea".to_string()),
            max_priority_fee_per_gas: Some("0x159".to_string()),
            value: Some("0x1c8".to_string()),
            data: Some("0xdeadbeef".to_string()),
            nonce: Some("0x1".to_string()),
            chain_id: Some(1),
            access_list: vec![mystiko_protos::core::v1::AccessListItem {
                address: "0x6088B06c5a187058434655B71057a9ee93E13d0d".to_string(),
                storage_keys: vec!["0xb33fae07e1262a169631d35aea9ecbe5068d6554504dd06c26d0dcd35b199d22".to_string()],
            }],
        })),
    };
    let tx: TypedTransaction = tx.try_into().unwrap();
    match tx {
        TypedTransaction::Eip1559(mut tx) => {
            assert_eq!(
                ethers_address_to_string(&tx.from.unwrap()),
                "0x388C818CA8B9251b393131C08a736A67ccB19297"
            );
            assert_eq!(
                ethers_address_to_string(tx.to.unwrap().as_address().unwrap()),
                "0xD71b82542A8629B172c5878e32a9cD1257CafEDB"
            );
            assert_eq!(u256_to_hex_string(&tx.gas.unwrap()), "0x7b");
            assert_eq!(u256_to_hex_string(&tx.max_fee_per_gas.unwrap()), "0xea");
            assert_eq!(u256_to_hex_string(&tx.max_priority_fee_per_gas.unwrap()), "0x159");
            assert_eq!(u256_to_hex_string(&tx.value.unwrap()), "0x1c8");
            assert_eq!(encode_hex_with_prefix(tx.data.unwrap()), "0xdeadbeef");
            assert_eq!(u256_to_hex_string(&tx.nonce.unwrap()), "0x1");
            assert_eq!(tx.chain_id.unwrap().as_u64(), 1);
            assert_eq!(tx.access_list.0.len(), 1);
            let access_list_item = tx.access_list.0.pop().unwrap();
            assert_eq!(
                ethers_address_to_string(&access_list_item.address),
                "0x6088B06c5a187058434655B71057a9ee93E13d0d"
            );
            assert_eq!(access_list_item.storage_keys.len(), 1);
            assert_eq!(
                format!("0x{:x}", access_list_item.storage_keys[0]),
                "0xb33fae07e1262a169631d35aea9ecbe5068d6554504dd06c26d0dcd35b199d22"
            );
        }
        _ => panic!("Expected legacy transaction"),
    }
}

#[test]
fn test_from_eip2930_transaction() {
    let ether_tx = Eip2930TransactionRequest {
        tx: TransactionRequest {
            from: Some(ethers_address_from_string("0x388C818CA8B9251b393131C08a736A67ccB19297").unwrap()),
            to: Some(NameOrAddress::Address(
                ethers_address_from_string("0xD71b82542A8629B172c5878e32a9cD1257CafEDB").unwrap(),
            )),
            gas: Some(U256::from(123_u16)),
            gas_price: Some(U256::from(234_u16)),
            value: Some(U256::from(345_u16)),
            data: Some(Bytes::from(decode_hex("0xdeadbeef").unwrap())),
            nonce: Some(U256::one()),
            chain_id: Some(U64::from(1_u64)),
        },
        access_list: AccessList(vec![AccessListItem {
            address: ethers_address_from_string("0x6088B06c5a187058434655B71057a9ee93E13d0d").unwrap(),
            storage_keys: vec![
                H256::decode_hex("0xb33fae07e1262a169631d35aea9ecbe5068d6554504dd06c26d0dcd35b199d22").unwrap(),
            ],
        }]),
    };
    let tx: Transaction = TypedTransaction::Eip2930(ether_tx).into();
    match tx.transaction.unwrap() {
        TransactionEnum::Eip2930Transaction(tx) => {
            let mut access_list = tx.access_list;
            let tx = tx.tx.unwrap();
            assert_eq!(tx.from.unwrap(), "0x388C818CA8B9251b393131C08a736A67ccB19297");
            assert_eq!(tx.to.unwrap(), "0xD71b82542A8629B172c5878e32a9cD1257CafEDB");
            assert_eq!(tx.gas.unwrap(), "0x7b");
            assert_eq!(tx.gas_price.unwrap(), "0xea");
            assert_eq!(tx.value.unwrap(), "0x159");
            assert_eq!(tx.data.unwrap(), "0xdeadbeef");
            assert_eq!(tx.nonce.unwrap(), "0x1");
            assert_eq!(tx.chain_id.unwrap(), 1);
            assert_eq!(access_list.len(), 1);
            let access_list_item = access_list.pop().unwrap();
            assert_eq!(access_list_item.address, "0x6088B06c5a187058434655B71057a9ee93E13d0d");
            assert_eq!(access_list_item.storage_keys.len(), 1);
            assert_eq!(
                access_list_item.storage_keys[0],
                "0xb33fae07e1262a169631d35aea9ecbe5068d6554504dd06c26d0dcd35b199d22"
            );
        }
        _ => panic!("Expected EIP-2930 transaction"),
    }
}

#[test]
fn test_try_into_eip2930_transaction() {
    let tx = Transaction {
        transaction: Some(TransactionEnum::Eip2930Transaction(Eip2930Transaction {
            tx: Some(LegacyTransaction {
                from: Some("0x388C818CA8B9251b393131C08a736A67ccB19297".to_string()),
                to: Some("0xD71b82542A8629B172c5878e32a9cD1257CafEDB".to_string()),
                gas: Some("0x7b".to_string()),
                gas_price: Some("0xea".to_string()),
                value: Some("0x159".to_string()),
                data: Some("0xdeadbeef".to_string()),
                nonce: Some("0x1".to_string()),
                chain_id: Some(1),
            }),
            access_list: vec![mystiko_protos::core::v1::AccessListItem {
                address: "0x6088B06c5a187058434655B71057a9ee93E13d0d".to_string(),
                storage_keys: vec!["0xb33fae07e1262a169631d35aea9ecbe5068d6554504dd06c26d0dcd35b199d22".to_string()],
            }],
        })),
    };
    let tx: TypedTransaction = tx.try_into().unwrap();
    match tx {
        TypedTransaction::Eip2930(tx) => {
            let mut access_list = tx.access_list.0;
            let tx = tx.tx;
            assert_eq!(
                ethers_address_to_string(&tx.from.unwrap()),
                "0x388C818CA8B9251b393131C08a736A67ccB19297"
            );
            assert_eq!(
                ethers_address_to_string(tx.to.unwrap().as_address().unwrap()),
                "0xD71b82542A8629B172c5878e32a9cD1257CafEDB"
            );
            assert_eq!(u256_to_hex_string(&tx.gas.unwrap()), "0x7b");
            assert_eq!(u256_to_hex_string(&tx.gas_price.unwrap()), "0xea");
            assert_eq!(u256_to_hex_string(&tx.value.unwrap()), "0x159");
            assert_eq!(encode_hex_with_prefix(tx.data.unwrap()), "0xdeadbeef");
            assert_eq!(u256_to_hex_string(&tx.nonce.unwrap()), "0x1");
            assert_eq!(tx.chain_id.unwrap().as_u64(), 1);
            assert_eq!(access_list.len(), 1);
            let access_list_item = access_list.pop().unwrap();
            assert_eq!(
                ethers_address_to_string(&access_list_item.address),
                "0x6088B06c5a187058434655B71057a9ee93E13d0d"
            );
            assert_eq!(access_list_item.storage_keys.len(), 1);
            assert_eq!(
                format!("0x{:x}", access_list_item.storage_keys[0]),
                "0xb33fae07e1262a169631d35aea9ecbe5068d6554504dd06c26d0dcd35b199d22"
            );
        }
        _ => panic!("Expected EIP-2930 transaction"),
    }
}
