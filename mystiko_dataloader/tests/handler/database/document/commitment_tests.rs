use mystiko_config::MystikoConfig;
use mystiko_dataloader::handler::document::{Commitment, DatabaseCommitment};
use mystiko_protos::data::v1::{Commitment as ProtoCommitment, CommitmentStatus};
use mystiko_utils::convert::biguint_to_bytes;
use mystiko_utils::hex::encode_hex_with_prefix;
use num_bigint::BigUint;
use std::sync::Arc;

#[test]
fn test_commitment() {
    assert_eq!(Commitment::column_chain_id(), "chain_id".to_string());
    assert_eq!(Commitment::column_contract_address(), "contract_address".to_string());
    assert_eq!(Commitment::column_bridge_type(), "bridge_type".to_string());
    assert_eq!(Commitment::column_commitment_hash(), "commitment_hash".to_string());
    assert_eq!(Commitment::column_status(), "status".to_string());
    assert_eq!(Commitment::column_block_number(), "block_number".to_string());
    assert_eq!(
        Commitment::column_src_block_number(),
        "src_chain_block_number".to_string()
    );
    assert_eq!(
        Commitment::column_included_block_number(),
        "included_block_number".to_string()
    );
    assert_eq!(Commitment::column_leaf_index(), "leaf_index".to_string());
    assert_eq!(Commitment::column_rollup_fee(), "rollup_fee".to_string());
    assert_eq!(Commitment::column_encrypted_note(), "encrypted_notes".to_string());
    assert_eq!(
        Commitment::column_queued_transaction_hash(),
        "queued_transaction_hash".to_string()
    );
    assert_eq!(
        Commitment::column_included_transaction_hash(),
        "included_transaction_hash".to_string()
    );
    assert_eq!(
        Commitment::column_src_transaction_hash(),
        "src_chain_transaction_hash".to_string()
    );
    let test_commitment = Commitment::builder()
        .chain_id(1_u64)
        .contract_address("address1".to_string())
        .bridge_type(1_i32)
        .commitment_hash(BigUint::from(1111_u32))
        .status(CommitmentStatus::Queued as i32)
        .block_number(1000u64)
        .queued_transaction_hash("queue_tx_hash1".to_string())
        .build();
    assert_eq!(test_commitment.get_chain_id(), test_commitment.chain_id);
    assert_eq!(
        test_commitment.get_contract_address(),
        &test_commitment.contract_address
    );
    assert_eq!(test_commitment.get_commitment_hash(), &test_commitment.commitment_hash);
    assert_eq!(test_commitment.get_status(), test_commitment.status);
    assert_eq!(test_commitment.get_block_number(), test_commitment.block_number);
    assert_eq!(
        test_commitment.get_src_block_number(),
        test_commitment.src_chain_block_number
    );
    assert_eq!(
        test_commitment.get_included_block_number(),
        test_commitment.included_block_number
    );
    assert_eq!(test_commitment.get_leaf_index(), test_commitment.leaf_index);
    assert_eq!(test_commitment.get_rollup_fee(), test_commitment.rollup_fee.as_ref());
    assert_eq!(
        test_commitment.get_encrypted_note(),
        test_commitment.encrypted_notes.as_ref()
    );
    assert_eq!(
        test_commitment.get_queued_transaction_hash(),
        test_commitment.queued_transaction_hash.as_ref()
    );
    assert_eq!(
        test_commitment.get_included_transaction_hash(),
        test_commitment.included_transaction_hash.as_ref()
    );
    assert_eq!(
        test_commitment.get_src_transaction_hash(),
        test_commitment.src_chain_transaction_hash.as_ref()
    );
}

#[tokio::test]
async fn test_convert_with_proto() {
    let config = Arc::new(
        MystikoConfig::from_json_file("./tests/files/handler/config.json")
            .await
            .unwrap(),
    );
    let test_proto1 = ProtoCommitment::builder()
        .commitment_hash(biguint_to_bytes(&BigUint::from(100000u64)))
        .block_number(100u64)
        .status(CommitmentStatus::Queued as i32)
        .leaf_index(10u64)
        .rollup_fee(biguint_to_bytes(&BigUint::from(100u64)))
        .encrypted_note(vec![1u8, 2u8, 3u8])
        .queued_transaction_hash(vec![4u8, 5u8, 6u8])
        .src_chain_transaction_hash(vec![7u8, 8u8, 9u8])
        .build();
    let commitment = Commitment::from_proto(Arc::clone(&config), 1_u64, "address1", 1, test_proto1);
    assert!(commitment.is_ok());
    let mut commitment = commitment.unwrap();
    assert_eq!(commitment.commitment_hash, BigUint::from(100000u64));
    assert_eq!(
        commitment.encrypted_notes,
        Some(encode_hex_with_prefix([1u8, 2u8, 3u8]))
    );
    let test_proto2 = ProtoCommitment::builder()
        .commitment_hash(biguint_to_bytes(&BigUint::from(100000u64)))
        .block_number(200u64)
        .status(CommitmentStatus::Included as i32)
        .leaf_index(20u64)
        .included_block_number(200u64)
        .rollup_fee(biguint_to_bytes(&BigUint::from(200u64)))
        .encrypted_note(vec![11u8, 22u8, 33u8])
        .queued_transaction_hash(vec![44u8, 55u8, 66u8])
        .src_chain_transaction_hash(vec![77u8, 88u8, 99u8])
        .build();
    let update_result = commitment.update_by_proto(Arc::clone(&config), &test_proto2);
    assert!(update_result.is_ok());
    assert_eq!(commitment.commitment_hash, BigUint::from(100000u64));
    assert_eq!(commitment.status, CommitmentStatus::Included as i32);
    assert_eq!(
        commitment.encrypted_notes,
        Some(encode_hex_with_prefix([1u8, 2u8, 3u8]))
    );
    let into_result = commitment.into_proto();
    assert!(into_result.is_ok());
    let proto: ProtoCommitment = into_result.unwrap();
    assert_eq!(proto.encrypted_note, Some(vec![1u8, 2u8, 3u8]));
    assert_eq!(proto.included_block_number, Some(200u64));
}
