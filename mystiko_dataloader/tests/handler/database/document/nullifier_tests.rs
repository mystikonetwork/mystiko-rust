use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::handler::document::{DatabaseNullifier, Nullifier};
use mystiko_protos::data::v1::Nullifier as ProtoNullifier;
use mystiko_utils::convert::biguint_to_bytes;
use mystiko_utils::hex::encode_hex_with_prefix;
use num_bigint::BigUint;
use std::sync::Arc;

#[test]
fn test_nullifier() {
    assert_eq!(Nullifier::column_chain_id(), "chain_id".to_string());
    assert_eq!(Nullifier::column_contract_address(), "contract_address".to_string());
    assert_eq!(Nullifier::column_nullifier(), "nullifier".to_string());
    assert_eq!(Nullifier::column_block_number(), "block_number".to_string());
    assert_eq!(Nullifier::column_transaction_hash(), "transaction_hash".to_string());
    let nullifier = Nullifier::builder()
        .chain_id(1u64)
        .contract_address("address1".to_string())
        .nullifier(BigUint::from(11u32))
        .block_number(100u64)
        .transaction_hash("111111".to_string())
        .build();
    assert_eq!(nullifier.get_chain_id(), 1u64);
    assert_eq!(nullifier.get_contract_address(), "address1");
    assert_eq!(nullifier.get_nullifier(), &BigUint::from(11_u32));
    assert_eq!(nullifier.get_block_number(), 100u64);
    assert_eq!(nullifier.get_transaction_hash(), "111111");
}

#[tokio::test]
async fn test_convert_with_proto() {
    let config = Arc::new(
        MystikoConfig::from_json_file("./tests/files/handler/config.json")
            .await
            .unwrap(),
    );
    let proto = ProtoNullifier {
        nullifier: biguint_to_bytes(&BigUint::from(1111u32)),
        block_number: 100u64,
        transaction_hash: vec![4u8, 5u8, 6u8],
    };
    let result = Nullifier::from_proto(Arc::clone(&config), 1u64, "address1", proto);
    assert!(result.is_ok());
    let mut nullifier = result.unwrap();
    assert_eq!(nullifier.contract_address, "address1".to_string());
    assert_eq!(nullifier.chain_id, 1_u64);
    assert_eq!(nullifier.transaction_hash, encode_hex_with_prefix(&[4u8, 5u8, 6u8]));
    assert_eq!(nullifier.block_number, 100_u64);
    assert_eq!(nullifier.nullifier, BigUint::from(1111u32));
    let proto2 = ProtoNullifier {
        nullifier: biguint_to_bytes(&BigUint::from(1111u32)),
        block_number: 200u64,
        transaction_hash: vec![44u8, 55u8, 66u8],
    };
    let result = nullifier.update_by_proto(Arc::clone(&config), &proto2);
    assert!(result.is_ok());
    assert_eq!(nullifier.contract_address, "address1".to_string());
    assert_eq!(nullifier.chain_id, 1u64);
    assert_eq!(nullifier.transaction_hash, encode_hex_with_prefix(&[44u8, 55u8, 66u8]));
    assert_eq!(nullifier.block_number, 200u64);
    assert_eq!(nullifier.nullifier, BigUint::from(1111u32));
}
