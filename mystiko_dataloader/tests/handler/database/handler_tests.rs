use anyhow::Result;
use mystiko_config::MystikoConfig;
use mystiko_dataloader::data::{ChainData, ContractData, FullData, LiteData, LoadedData};
use mystiko_dataloader::handler::document::{
    Commitment, Contract, DatabaseCommitment, DatabaseContract, DatabaseNullifier, Nullifier,
};
use mystiko_dataloader::handler::DatabaseHandler;
use mystiko_dataloader::handler::{CommitmentQueryOption, DataHandler, HandleOption, NullifierQueryOption};
use mystiko_protos::data::v1::{Commitment as CommitmentProto, CommitmentStatus, Nullifier as NullifierProto};
use mystiko_protos::storage::v1::{QueryFilter, SubFilter};
use mystiko_storage::{Collection, Document, SqlStatementFormatter};
use mystiko_storage_sqlite::SqliteStorage;
use mystiko_utils::convert::biguint_to_bytes;
use mystiko_utils::hex::decode_hex;
use num_bigint::BigUint;
use std::sync::Arc;

#[tokio::test]
async fn test_query_chain_loaded_block() {
    let (database_handler, collection, _config) = setup::<FullData>().await.unwrap();
    let mock_contracts = [
        Contract {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            loaded_block: 1000_u64,
        },
        Contract {
            chain_id: 1_u64,
            contract_address: "address2".to_string(),
            loaded_block: 2000_u64,
        },
        Contract {
            chain_id: 1_u64,
            contract_address: "address3".to_string(),
            loaded_block: 3000_u64,
        },
    ];
    collection.insert_batch(&mock_contracts).await.unwrap();
    let loaded_block = database_handler.query_chain_loaded_block(1_u64).await.unwrap();
    assert!(loaded_block.is_some());
    assert_eq!(loaded_block.unwrap(), 1000_u64);
}

#[tokio::test]
async fn test_query_contract_loaded_block() {
    let (database_handler, collection, _config) = setup::<FullData>().await.unwrap();
    let mock_contracts = [
        Contract {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            loaded_block: 1000_u64,
        },
        Contract {
            chain_id: 1_u64,
            contract_address: "address2".to_string(),
            loaded_block: 2000_u64,
        },
        Contract {
            chain_id: 1_u64,
            contract_address: "address3".to_string(),
            loaded_block: 3000_u64,
        },
    ];
    collection.insert_batch(&mock_contracts).await.unwrap();
    let contract_loaded_block = database_handler
        .query_contract_loaded_block(1_u64, "address1")
        .await
        .unwrap();
    assert!(contract_loaded_block.is_some());
    assert_eq!(contract_loaded_block.unwrap(), 1000_u64);
    let contract_loaded_block = database_handler
        .query_contract_loaded_block(1_u64, "address2")
        .await
        .unwrap();
    assert!(contract_loaded_block.is_some());
    assert_eq!(contract_loaded_block.unwrap(), 2000_u64);
    let contract_loaded_block = database_handler
        .query_contract_loaded_block(1_u64, "address3")
        .await
        .unwrap();
    assert!(contract_loaded_block.is_some());
    assert_eq!(contract_loaded_block.unwrap(), 3000_u64);
    let contract_loaded_block = database_handler
        .query_contract_loaded_block(1_u64, "address4")
        .await
        .unwrap();
    assert!(contract_loaded_block.is_none());
}

#[tokio::test]
async fn test_query_commitment() {
    let (database_handler, collection, _config) = setup::<FullData>().await.unwrap();
    let mock_contracts = [Contract {
        chain_id: 1_u64,
        contract_address: "address1".to_string(),
        loaded_block: 3000_u64,
    }];
    collection.insert_batch(&mock_contracts).await.unwrap();
    let mock_commitments = [
        Commitment {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            commitment_hash: BigUint::from(1111_u32),
            status: CommitmentStatus::Included.into(),
            block_number: 1000_u64,
            src_chain_block_number: None,
            included_block_number: Some(900_u64),
            leaf_index: Some(1_u64),
            rollup_fee: None,
            encrypted_notes: None,
            queued_transaction_hash: Some("1111".to_string()),
            included_transaction_hash: Some("1111".to_string()),
            src_chain_transaction_hash: Some("1111".to_string()),
        },
        Commitment {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            commitment_hash: BigUint::from(2222_u32),
            status: CommitmentStatus::Queued.into(),
            block_number: 2000_u64,
            src_chain_block_number: None,
            included_block_number: None,
            leaf_index: Some(2_u64),
            rollup_fee: None,
            encrypted_notes: None,
            queued_transaction_hash: Some("2222".to_string()),
            included_transaction_hash: Some("2222".to_string()),
            src_chain_transaction_hash: Some("2222".to_string()),
        },
    ];
    collection.insert_batch(&mock_commitments).await.unwrap();
    let option = CommitmentQueryOption::builder()
        .chain_id(1_u64)
        .contract_address("address1".to_string())
        .end_block(1000_u64)
        .status(CommitmentStatus::Included)
        .start_block(800_u64)
        .build();
    let commitment = database_handler.query_commitment(&option).await.unwrap();
    assert!(commitment.is_some());
    let commitment = commitment.unwrap();
    assert_eq!(commitment.block_number, 1000_u64);
    assert_eq!(commitment.leaf_index, Some(1_u64));
    assert_eq!(commitment.commitment_hash, biguint_to_bytes(&BigUint::from(1111_u32)));
}

#[tokio::test]
async fn test_query_commitments() {
    let (database_handler, collection, _config) = setup::<FullData>().await.unwrap();
    let mock_contracts = [
        Contract {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            loaded_block: 3000_u64,
        },
        Contract {
            chain_id: 1_u64,
            contract_address: "address2".to_string(),
            loaded_block: 2000_u64,
        },
    ];
    collection.insert_batch(&mock_contracts).await.unwrap();
    let mock_commitments = [
        Commitment {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            commitment_hash: BigUint::from(1111_u32),
            status: CommitmentStatus::Included.into(),
            block_number: 1000_u64,
            src_chain_block_number: None,
            included_block_number: Some(900_u64),
            leaf_index: Some(1_u64),
            rollup_fee: None,
            encrypted_notes: None,
            queued_transaction_hash: Some("1111".to_string()),
            included_transaction_hash: Some("1111".to_string()),
            src_chain_transaction_hash: Some("1111".to_string()),
        },
        Commitment {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            commitment_hash: BigUint::from(2222_u32),
            status: CommitmentStatus::Queued.into(),
            block_number: 2000_u64,
            src_chain_block_number: None,
            included_block_number: None,
            leaf_index: Some(2_u64),
            rollup_fee: None,
            encrypted_notes: None,
            queued_transaction_hash: Some("2222".to_string()),
            included_transaction_hash: Some("2222".to_string()),
            src_chain_transaction_hash: Some("2222".to_string()),
        },
        Commitment {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            commitment_hash: BigUint::from(3333_u32),
            status: CommitmentStatus::SrcSucceeded.into(),
            block_number: 2800_u64,
            src_chain_block_number: Some(2900_u64),
            included_block_number: None,
            leaf_index: Some(3_u64),
            rollup_fee: None,
            encrypted_notes: None,
            queued_transaction_hash: Some("3333".to_string()),
            included_transaction_hash: Some("3333".to_string()),
            src_chain_transaction_hash: Some("3333".to_string()),
        },
        Commitment {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            commitment_hash: BigUint::from(4444_u32),
            status: CommitmentStatus::Unspecified.into(),
            block_number: 3000_u64,
            src_chain_block_number: Some(3000_u64),
            included_block_number: None,
            leaf_index: Some(4_u64),
            rollup_fee: None,
            encrypted_notes: None,
            queued_transaction_hash: Some("4444".to_string()),
            included_transaction_hash: Some("4444".to_string()),
            src_chain_transaction_hash: Some("4444".to_string()),
        },
    ];
    collection.insert_batch(&mock_commitments).await.unwrap();
    // no option field
    let option = CommitmentQueryOption::builder()
        .chain_id(1_u64)
        .contract_address("address1".to_string())
        .start_block(500_u64)
        .end_block(2000_u64)
        .build();
    let commitments = database_handler.query_commitments(&option).await.unwrap();
    assert_eq!(commitments.end_block, 2000_u64);
    assert_eq!(commitments.result.len(), 2);
    assert_eq!(
        commitments.result[0].commitment_hash,
        biguint_to_bytes(&BigUint::from(1111_u32))
    );
    assert_eq!(
        commitments.result[1].commitment_hash,
        biguint_to_bytes(&BigUint::from(2222_u32))
    );
    assert_eq!(commitments.result[1].block_number, 2000_u64);
    // commitment status is Unspecified
    let option = CommitmentQueryOption::builder()
        .chain_id(1_u64)
        .contract_address("address1".to_string())
        .end_block(3000_u64)
        .status(CommitmentStatus::Unspecified)
        .start_block(1500_u64)
        .build();
    let commitments = database_handler.query_commitments(&option).await.unwrap();
    assert_eq!(commitments.end_block, 3000_u64);
    assert_eq!(commitments.result.len(), 1);
    assert_eq!(
        commitments.result[0].commitment_hash,
        biguint_to_bytes(&BigUint::from(4444_u32))
    );
    assert_eq!(commitments.result[0].block_number, 3000_u64);
    // commitment status is Included
    let option = CommitmentQueryOption::builder()
        .chain_id(1_u64)
        .contract_address("address1".to_string())
        .end_block(900_u64)
        .status(CommitmentStatus::Included)
        .start_block(100_u64)
        .build();
    let commitments = database_handler.query_commitments(&option).await.unwrap();
    assert_eq!(commitments.end_block, 900_u64);
    assert_eq!(commitments.result.len(), 1);
    assert_eq!(
        commitments.result[0].commitment_hash,
        biguint_to_bytes(&BigUint::from(1111_u32))
    );
    assert_eq!(commitments.result[0].included_block_number, Some(900_u64));
    // commitment status is Queued
    let option = CommitmentQueryOption::builder()
        .chain_id(1_u64)
        .contract_address("address1".to_string())
        .end_block(3000_u64)
        .status(CommitmentStatus::Queued)
        .start_block(1000_u64)
        .build();
    let commitments = database_handler.query_commitments(&option).await.unwrap();
    assert_eq!(commitments.end_block, 3000_u64);
    assert_eq!(commitments.result.len(), 1);
    assert_eq!(
        commitments.result[0].commitment_hash,
        biguint_to_bytes(&BigUint::from(2222_u32))
    );
    assert_eq!(commitments.result[0].block_number, 2000_u64);
    // commitment status is SrcSucceeded
    let option = CommitmentQueryOption::builder()
        .chain_id(1_u64)
        .contract_address("address1".to_string())
        .end_block(2900_u64)
        .status(CommitmentStatus::SrcSucceeded)
        .start_block(1000_u64)
        .commitment_hash(vec![BigUint::from(3333_u32), BigUint::from(3456_u32)])
        .build();
    let commitments = database_handler.query_commitments(&option).await.unwrap();
    assert_eq!(commitments.end_block, 2900_u64);
    assert_eq!(commitments.result.len(), 1);
    assert_eq!(
        commitments.result[0].commitment_hash,
        biguint_to_bytes(&BigUint::from(3333_u32))
    );
    assert_eq!(commitments.result[0].src_chain_block_number, Some(2900_u64));
}

#[tokio::test]
async fn test_count_commitments() {
    let (database_handler, collection, _config) = setup::<FullData>().await.unwrap();
    let mock_contracts = [
        Contract {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            loaded_block: 3000_u64,
        },
        Contract {
            chain_id: 1_u64,
            contract_address: "address2".to_string(),
            loaded_block: 2000_u64,
        },
    ];
    collection.insert_batch(&mock_contracts).await.unwrap();
    let mock_commitments = [
        Commitment {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            commitment_hash: BigUint::from(1111_u32),
            status: CommitmentStatus::Included.into(),
            block_number: 1000_u64,
            src_chain_block_number: None,
            included_block_number: Some(900_u64),
            leaf_index: Some(1_u64),
            rollup_fee: None,
            encrypted_notes: None,
            queued_transaction_hash: Some("1111".to_string()),
            included_transaction_hash: Some("1111".to_string()),
            src_chain_transaction_hash: Some("1111".to_string()),
        },
        Commitment {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            commitment_hash: BigUint::from(2222_u32),
            status: CommitmentStatus::Queued.into(),
            block_number: 2000_u64,
            src_chain_block_number: None,
            included_block_number: None,
            leaf_index: Some(2_u64),
            rollup_fee: None,
            encrypted_notes: None,
            queued_transaction_hash: Some("2222".to_string()),
            included_transaction_hash: Some("2222".to_string()),
            src_chain_transaction_hash: Some("2222".to_string()),
        },
        Commitment {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            commitment_hash: BigUint::from(3333_u32),
            status: CommitmentStatus::SrcSucceeded.into(),
            block_number: 2800_u64,
            src_chain_block_number: Some(2900_u64),
            included_block_number: None,
            leaf_index: Some(3_u64),
            rollup_fee: None,
            encrypted_notes: None,
            queued_transaction_hash: Some("3333".to_string()),
            included_transaction_hash: Some("3333".to_string()),
            src_chain_transaction_hash: Some("3333".to_string()),
        },
        Commitment {
            chain_id: 1_u64,
            contract_address: "address2".to_string(),
            commitment_hash: BigUint::from(4444_u32),
            status: CommitmentStatus::Unspecified.into(),
            block_number: 3000_u64,
            src_chain_block_number: Some(3000_u64),
            included_block_number: None,
            leaf_index: Some(4_u64),
            rollup_fee: None,
            encrypted_notes: None,
            queued_transaction_hash: Some("4444".to_string()),
            included_transaction_hash: Some("4444".to_string()),
            src_chain_transaction_hash: Some("4444".to_string()),
        },
    ];
    collection.insert_batch(&mock_commitments).await.unwrap();
    let option = CommitmentQueryOption::builder()
        .chain_id(1_u64)
        .contract_address("address1".to_string())
        .end_block(3000_u64)
        .start_block(100_u64)
        .build();
    let count = database_handler.count_commitments(&option).await.unwrap();
    assert_eq!(count.end_block, 3000_u64);
    assert_eq!(count.result, 3);
}

#[tokio::test]
async fn test_query_nullifier() {
    let (database_handler, collection, _config) = setup::<FullData>().await.unwrap();
    let mock_contracts = [Contract {
        chain_id: 1_u64,
        contract_address: "address1".to_string(),
        loaded_block: 3000_u64,
    }];
    collection.insert_batch(&mock_contracts).await.unwrap();
    let mock_nullifiers = [
        Nullifier {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            nullifier: BigUint::from(1111_u32),
            block_number: 1000_u64,
            transaction_hash: "1111".to_string(),
        },
        Nullifier {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            nullifier: BigUint::from(2222_u32),
            block_number: 2000_u64,
            transaction_hash: "2222".to_string(),
        },
    ];
    collection.insert_batch(&mock_nullifiers).await.unwrap();
    let option = NullifierQueryOption::builder()
        .chain_id(1_u64)
        .contract_address("address1".to_string())
        .end_block(1100_u64)
        .start_block(900_u64)
        .build();
    let nullifier = database_handler.query_nullifier(&option).await.unwrap();
    assert!(nullifier.is_some());
    let nullifier = nullifier.unwrap();
    assert_eq!(nullifier.block_number, 1000_u64);
    assert_eq!(nullifier.nullifier, biguint_to_bytes(&BigUint::from(1111_u32)));
    let (database_handler, _collection, _config) = setup::<LiteData>().await.unwrap();
    let error_result = database_handler.query_nullifier(&option).await;
    assert!(error_result.is_err());
}

#[tokio::test]
async fn test_query_nullifiers() {
    let (database_handler, collection, _config) = setup::<FullData>().await.unwrap();
    let mock_contracts = [
        Contract {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            loaded_block: 3000_u64,
        },
        Contract {
            chain_id: 1_u64,
            contract_address: "address2".to_string(),
            loaded_block: 2000_u64,
        },
    ];
    collection.insert_batch(&mock_contracts).await.unwrap();
    let mock_nullifiers = [
        Nullifier {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            nullifier: BigUint::from(1111_u32),
            block_number: 1000_u64,
            transaction_hash: "1111".to_string(),
        },
        Nullifier {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            nullifier: BigUint::from(2222_u32),
            block_number: 2000_u64,
            transaction_hash: "2222".to_string(),
        },
        Nullifier {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            nullifier: BigUint::from(3333_u32),
            block_number: 3000_u64,
            transaction_hash: "3333".to_string(),
        },
        Nullifier {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            nullifier: BigUint::from(4444_u32),
            block_number: 4000_u64,
            transaction_hash: "4444".to_string(),
        },
    ];
    collection.insert_batch(&mock_nullifiers).await.unwrap();
    // no option field
    let option = NullifierQueryOption::builder()
        .chain_id(1_u64)
        .contract_address("address1".to_string())
        .start_block(500_u64)
        .end_block(3500_u64)
        .build();
    let nullifiers = database_handler.query_nullifiers(&option).await.unwrap();
    assert_eq!(nullifiers.end_block, 3000);
    assert_eq!(nullifiers.result.len(), 3);
    // with option field
    let option = NullifierQueryOption::builder()
        .chain_id(1_u64)
        .contract_address("address1".to_string())
        .end_block(4500_u64)
        .start_block(100_u64)
        .nullifier(vec![BigUint::from(1111_u32), BigUint::from(3333_u32)])
        .build();
    let nullifiers = database_handler.query_nullifiers(&option).await.unwrap();
    assert_eq!(nullifiers.end_block, 3000);
    assert_eq!(nullifiers.result.len(), 2);
    assert_eq!(nullifiers.result[0].block_number, 1000_u64);
    assert_eq!(nullifiers.result[1].block_number, 3000_u64);
    let (database_handler, _collection, _config) = setup::<LiteData>().await.unwrap();
    let error_result = database_handler.query_nullifiers(&option).await;
    assert!(error_result.is_err());
}

#[tokio::test]
async fn test_count_nullifiers() {
    let (database_handler, collection, _config) = setup::<FullData>().await.unwrap();
    let mock_contracts = [
        Contract {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            loaded_block: 3000_u64,
        },
        Contract {
            chain_id: 1_u64,
            contract_address: "address2".to_string(),
            loaded_block: 2000_u64,
        },
    ];
    collection.insert_batch(&mock_contracts).await.unwrap();
    let mock_nullifiers = [
        Nullifier {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            nullifier: BigUint::from(1111_u32),
            block_number: 1000_u64,
            transaction_hash: "1111".to_string(),
        },
        Nullifier {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            nullifier: BigUint::from(2222_u32),
            block_number: 2000_u64,
            transaction_hash: "2222".to_string(),
        },
        Nullifier {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            nullifier: BigUint::from(3333_u32),
            block_number: 3000_u64,
            transaction_hash: "3333".to_string(),
        },
        Nullifier {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            nullifier: BigUint::from(4444_u32),
            block_number: 4000_u64,
            transaction_hash: "4444".to_string(),
        },
    ];
    collection.insert_batch(&mock_nullifiers).await.unwrap();
    // no option field
    let option = NullifierQueryOption::builder()
        .chain_id(1_u64)
        .contract_address("address1".to_string())
        .start_block(500_u64)
        .end_block(3500_u64)
        .build();
    let count = database_handler.count_nullifiers(&option).await.unwrap();
    assert_eq!(count.end_block, 3000);
    assert_eq!(count.result, 3_u64);
    // with option field
    let option = NullifierQueryOption::builder()
        .chain_id(1_u64)
        .contract_address("address1".to_string())
        .end_block(4500_u64)
        .start_block(200_u64)
        .nullifier(vec![BigUint::from(1111_u32), BigUint::from(3333_u32)])
        .build();
    let count = database_handler.count_nullifiers(&option).await.unwrap();
    assert_eq!(count.end_block, 3000);
    assert_eq!(count.result, 2);
    let (database_handler, _collection, _config) = setup::<LiteData>().await.unwrap();
    let error_result = database_handler.count_nullifiers(&option).await;
    assert!(error_result.is_err());
}

#[tokio::test]
async fn test_full_data_handle() {
    let (database_handler, collection, _config) = setup::<FullData>().await.unwrap();
    let mock_contracts = [
        Contract {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            loaded_block: 2000_u64,
        },
        Contract {
            chain_id: 1_u64,
            contract_address: "address2".to_string(),
            loaded_block: 20000_u64,
        },
        Contract {
            chain_id: 1_u64,
            contract_address: "address3".to_string(),
            loaded_block: 5000_u64,
        },
    ];
    collection.insert_batch(&mock_contracts).await.unwrap();
    let mock_commitments = [
        Commitment {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            commitment_hash: BigUint::from(1111_u32),
            status: CommitmentStatus::Included.into(),
            block_number: 1000_u64,
            src_chain_block_number: None,
            included_block_number: Some(900_u64),
            leaf_index: Some(1_u64),
            rollup_fee: None,
            encrypted_notes: None,
            queued_transaction_hash: Some("0x1111".to_string()),
            included_transaction_hash: Some("0x1111".to_string()),
            src_chain_transaction_hash: Some("0x1111".to_string()),
        },
        Commitment {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            commitment_hash: BigUint::from(1222_u32),
            status: CommitmentStatus::Queued.into(),
            block_number: 1700_u64,
            src_chain_block_number: Some(1500_u64),
            included_block_number: None,
            leaf_index: Some(2_u64),
            rollup_fee: Some(BigUint::from(200_u32)),
            encrypted_notes: Some("0x2222".to_string()),
            queued_transaction_hash: Some("0x2222".to_string()),
            included_transaction_hash: None,
            src_chain_transaction_hash: Some("0x2222".to_string()),
        },
        Commitment {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            commitment_hash: BigUint::from(1333_u32),
            status: CommitmentStatus::SrcSucceeded.into(),
            block_number: 1800_u64,
            src_chain_block_number: Some(1800_u64),
            included_block_number: None,
            leaf_index: None,
            rollup_fee: None,
            encrypted_notes: None,
            queued_transaction_hash: None,
            included_transaction_hash: None,
            src_chain_transaction_hash: Some("0x3333".to_string()),
        },
        Commitment {
            chain_id: 1_u64,
            contract_address: "address2".to_string(),
            commitment_hash: BigUint::from(4444_u32),
            status: CommitmentStatus::Unspecified.into(),
            block_number: 1500_u64,
            src_chain_block_number: Some(1500_u64),
            included_block_number: None,
            leaf_index: None,
            rollup_fee: None,
            encrypted_notes: None,
            queued_transaction_hash: Some("0x4444".to_string()),
            included_transaction_hash: Some("0x4444".to_string()),
            src_chain_transaction_hash: Some("0x4444".to_string()),
        },
        Commitment {
            chain_id: 1_u64,
            contract_address: "address2".to_string(),
            commitment_hash: BigUint::from(21111_u32),
            status: CommitmentStatus::Unspecified.into(),
            block_number: 1600_u64,
            src_chain_block_number: Some(1600_u64),
            included_block_number: None,
            leaf_index: None,
            rollup_fee: None,
            encrypted_notes: None,
            queued_transaction_hash: Some("0x4444".to_string()),
            included_transaction_hash: Some("0x4444".to_string()),
            src_chain_transaction_hash: Some("0x4444".to_string()),
        },
    ];
    collection.insert_batch(&mock_commitments).await.unwrap();
    let mock_nullifiers = [
        Nullifier {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            nullifier: BigUint::from(1111_u32),
            block_number: 1000_u64,
            transaction_hash: "0x1111".to_string(),
        },
        Nullifier {
            chain_id: 1_u64,
            contract_address: "address2".to_string(),
            nullifier: BigUint::from(11111_u32),
            block_number: 1800_u64,
            transaction_hash: "0x11111".to_string(),
        },
    ];
    collection.insert_batch(&mock_nullifiers).await.unwrap();
    let contracts_data = vec![
        ContractData::builder()
            .address("address1")
            .start_block(2000_u64)
            .end_block(4000_u64)
            .data(
                FullData::builder()
                    .commitments(vec![
                        CommitmentProto {
                            commitment_hash: biguint_to_bytes(&BigUint::from(1222_u32)),
                            status: CommitmentStatus::Included.into(),
                            block_number: 2011_u64,
                            included_block_number: Some(2011_u64),
                            src_chain_block_number: None,
                            leaf_index: None,
                            rollup_fee: None,
                            encrypted_note: None,
                            queued_transaction_hash: None,
                            included_transaction_hash: Some(decode_hex("0x2222").unwrap()),
                            src_chain_transaction_hash: None,
                        },
                        CommitmentProto {
                            commitment_hash: biguint_to_bytes(&BigUint::from(1333_u32)),
                            status: CommitmentStatus::Queued.into(),
                            block_number: 2032_u64,
                            included_block_number: None,
                            src_chain_block_number: None,
                            leaf_index: Some(3_u64),
                            rollup_fee: Some(biguint_to_bytes(&BigUint::from(300_u32))),
                            encrypted_note: Some(decode_hex("0x3333").unwrap()),
                            queued_transaction_hash: Some(decode_hex("0x3333").unwrap()),
                            included_transaction_hash: None,
                            src_chain_transaction_hash: None,
                        },
                        CommitmentProto {
                            commitment_hash: biguint_to_bytes(&BigUint::from(1333_u32)),
                            status: CommitmentStatus::Included.into(),
                            block_number: 2033_u64,
                            included_block_number: Some(2033_u64),
                            src_chain_block_number: None,
                            leaf_index: None,
                            rollup_fee: None,
                            encrypted_note: None,
                            queued_transaction_hash: None,
                            included_transaction_hash: Some(decode_hex("0x3333").unwrap()),
                            src_chain_transaction_hash: None,
                        },
                    ])
                    .nullifiers(vec![
                        NullifierProto {
                            nullifier: biguint_to_bytes(&BigUint::from(1011_u32)),
                            block_number: 2500_u64,
                            transaction_hash: decode_hex("0x1011").unwrap(),
                        },
                        NullifierProto {
                            nullifier: biguint_to_bytes(&BigUint::from(1022_u32)),
                            block_number: 2700_u64,
                            transaction_hash: decode_hex("0x1022").unwrap(),
                        },
                    ])
                    .build(),
            )
            .build(),
        ContractData::builder()
            .address("address2")
            .start_block(20000_u64)
            .end_block(40000_u64)
            .data(
                FullData::builder()
                    .commitments(vec![
                        CommitmentProto {
                            commitment_hash: biguint_to_bytes(&BigUint::from(21111_u32)),
                            status: CommitmentStatus::SrcSucceeded.into(),
                            block_number: 21111_u64,
                            included_block_number: None,
                            src_chain_block_number: Some(21111_u64),
                            leaf_index: None,
                            rollup_fee: None,
                            encrypted_note: None,
                            queued_transaction_hash: None,
                            included_transaction_hash: None,
                            src_chain_transaction_hash: Some(decode_hex("0x2111").unwrap()),
                        },
                        CommitmentProto {
                            commitment_hash: biguint_to_bytes(&BigUint::from(21111_u32)),
                            status: CommitmentStatus::Queued.into(),
                            block_number: 22222_u64,
                            included_block_number: None,
                            src_chain_block_number: None,
                            leaf_index: Some(2_u64),
                            rollup_fee: Some(biguint_to_bytes(&BigUint::from(10000_u32))),
                            encrypted_note: Some(decode_hex("0x2111").unwrap()),
                            queued_transaction_hash: Some(decode_hex("0x2111").unwrap()),
                            included_transaction_hash: None,
                            src_chain_transaction_hash: None,
                        },
                        CommitmentProto {
                            commitment_hash: biguint_to_bytes(&BigUint::from(21111_u32)),
                            status: CommitmentStatus::Included.into(),
                            block_number: 23333_u64,
                            included_block_number: Some(23333_u64),
                            src_chain_block_number: None,
                            leaf_index: None,
                            rollup_fee: None,
                            encrypted_note: None,
                            queued_transaction_hash: None,
                            included_transaction_hash: Some(decode_hex("0x2111").unwrap()),
                            src_chain_transaction_hash: None,
                        },
                    ])
                    .nullifiers(vec![
                        NullifierProto {
                            nullifier: biguint_to_bytes(&BigUint::from(11111_u32)),
                            block_number: 25000_u64,
                            transaction_hash: decode_hex("0x1122").unwrap(),
                        },
                        NullifierProto {
                            nullifier: biguint_to_bytes(&BigUint::from(22222_u32)),
                            block_number: 25000_u64,
                            transaction_hash: decode_hex("0x2011").unwrap(),
                        },
                    ])
                    .build(),
            )
            .build(),
        ContractData::builder()
            .address("address3")
            .start_block(5000_u64)
            .end_block(10000_u64)
            .data(
                FullData::builder()
                    .commitments(vec![
                        CommitmentProto {
                            commitment_hash: biguint_to_bytes(&BigUint::from(5111_u32)),
                            status: CommitmentStatus::Unspecified.into(),
                            block_number: 5111_u64,
                            included_block_number: None,
                            src_chain_block_number: None,
                            leaf_index: None,
                            rollup_fee: None,
                            encrypted_note: None,
                            queued_transaction_hash: None,
                            included_transaction_hash: None,
                            src_chain_transaction_hash: None,
                        },
                        CommitmentProto {
                            commitment_hash: biguint_to_bytes(&BigUint::from(5111_u32)),
                            status: CommitmentStatus::SrcSucceeded.into(),
                            block_number: 5222_u64,
                            included_block_number: None,
                            src_chain_block_number: Some(5222_u64),
                            leaf_index: None,
                            rollup_fee: None,
                            encrypted_note: None,
                            queued_transaction_hash: None,
                            included_transaction_hash: None,
                            src_chain_transaction_hash: Some(decode_hex("0x2111").unwrap()),
                        },
                    ])
                    .nullifiers(Vec::new())
                    .build(),
            )
            .build(),
        ContractData::builder()
            .address("address4")
            .start_block(5000_u64)
            .end_block(10000_u64)
            .data(
                FullData::builder()
                    .commitments(vec![CommitmentProto {
                        commitment_hash: biguint_to_bytes(&BigUint::from(5111_u32)),
                        status: CommitmentStatus::Unspecified.into(),
                        block_number: 5111_u64,
                        included_block_number: None,
                        src_chain_block_number: None,
                        leaf_index: None,
                        rollup_fee: None,
                        encrypted_note: None,
                        queued_transaction_hash: None,
                        included_transaction_hash: None,
                        src_chain_transaction_hash: None,
                    }])
                    .nullifiers(Vec::new())
                    .build(),
            )
            .build(),
    ];
    let test_chain_data = ChainData::builder()
        .chain_id(1_u64)
        .contracts_data(contracts_data)
        .build();
    let config = Arc::new(
        MystikoConfig::from_json_file("./tests/files/handler/config.json")
            .await
            .unwrap(),
    );
    let handle_result = database_handler
        .handle(&test_chain_data, &HandleOption::builder().config(config).build())
        .await;
    assert!(handle_result.is_ok());
    let result = handle_result.unwrap();
    assert_eq!(result.chain_id, 1_u64);
    assert_eq!(result.contract_results.len(), 4);
    assert_eq!(result.contract_results[0].address, "address1");
    assert!(result.contract_results[0].result.is_ok());
    assert_eq!(result.contract_results[1].address, "address2");
    assert!(result.contract_results[1].result.is_ok());
    assert_eq!(result.contract_results[2].address, "address3");
    assert!(result.contract_results[2].result.is_ok());
    assert_eq!(result.contract_results[3].address, "address4");
    assert!(result.contract_results[3].result.is_err());
    let address1_commitments: Vec<Document<Commitment>> = collection
        .find(Some(QueryFilter::from(vec![
            SubFilter::equal(Commitment::column_chain_id(), 1_u64),
            SubFilter::equal(Commitment::column_contract_address(), "address1"),
        ])))
        .await
        .unwrap();
    assert_eq!(address1_commitments.len(), 3);
    let commitment_1222 = address1_commitments
        .into_iter()
        .filter(|c| c.data.commitment_hash == BigUint::from(1222_u32))
        .collect::<Vec<Document<Commitment>>>();
    assert_eq!(commitment_1222.len(), 1);
    assert_eq!(commitment_1222[0].data.commitment_hash, BigUint::from(1222_u32));
    assert_eq!(commitment_1222[0].data.src_chain_block_number, Some(1500_u64));
    assert_eq!(commitment_1222[0].data.included_block_number, Some(2011_u64));
    assert_eq!(commitment_1222[0].data.block_number, 1700_u64);
    assert_eq!(
        commitment_1222[0].data.included_transaction_hash,
        Some("0x2222".to_string())
    );
    assert_eq!(
        commitment_1222[0].data.src_chain_transaction_hash,
        Some("0x2222".to_string())
    );
    assert_eq!(
        commitment_1222[0].data.queued_transaction_hash,
        Some("0x2222".to_string())
    );
    let address1_nullifiers: Vec<Document<Nullifier>> = collection
        .find::<Nullifier, QueryFilter>(Some(QueryFilter::from(vec![
            SubFilter::equal(Nullifier::column_chain_id(), 1_u64),
            SubFilter::equal(Nullifier::column_contract_address(), "address1"),
        ])))
        .await
        .unwrap();
    assert_eq!(address1_nullifiers.len(), 3);
    let address2_nullifiers: Vec<Document<Nullifier>> = collection
        .find(Some(QueryFilter::from(vec![
            SubFilter::equal(Nullifier::column_chain_id(), 1_u64),
            SubFilter::equal(Nullifier::column_contract_address(), "address2"),
        ])))
        .await
        .unwrap();
    assert_eq!(address2_nullifiers.len(), 2);
    let address3_commitments: Vec<Document<Commitment>> = collection
        .find(Some(QueryFilter::from(vec![
            SubFilter::equal(Commitment::column_chain_id(), 1_u64),
            SubFilter::equal(Commitment::column_contract_address(), "address3"),
        ])))
        .await
        .unwrap();
    assert_eq!(address3_commitments.len(), 1);
    assert_eq!(address3_commitments[0].data.src_chain_block_number, Some(5222_u64));
    assert_eq!(
        address3_commitments[0].data.src_chain_transaction_hash,
        Some("0x2111".to_string())
    );
}

#[tokio::test]
async fn test_lite_data_handle() {
    let (database_handler, collection, _config) = setup::<LiteData>().await.unwrap();
    let mock_contracts = [
        Contract {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            loaded_block: 2000_u64,
        },
        Contract {
            chain_id: 1_u64,
            contract_address: "address2".to_string(),
            loaded_block: 20000_u64,
        },
    ];
    collection.insert_batch(&mock_contracts).await.unwrap();
    let mock_commitments = [
        Commitment {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            commitment_hash: BigUint::from(1111_u32),
            status: CommitmentStatus::Included.into(),
            block_number: 1000_u64,
            src_chain_block_number: None,
            included_block_number: Some(900_u64),
            leaf_index: Some(1_u64),
            rollup_fee: None,
            encrypted_notes: None,
            queued_transaction_hash: Some("0x1111".to_string()),
            included_transaction_hash: Some("0x1111".to_string()),
            src_chain_transaction_hash: Some("0x1111".to_string()),
        },
        Commitment {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            commitment_hash: BigUint::from(1222_u32),
            status: CommitmentStatus::Queued.into(),
            block_number: 1700_u64,
            src_chain_block_number: Some(1500_u64),
            included_block_number: None,
            leaf_index: Some(2_u64),
            rollup_fee: Some(BigUint::from(200_u32)),
            encrypted_notes: Some("0x2222".to_string()),
            queued_transaction_hash: Some("0x2222".to_string()),
            included_transaction_hash: None,
            src_chain_transaction_hash: Some("0x2222".to_string()),
        },
        Commitment {
            chain_id: 1_u64,
            contract_address: "address1".to_string(),
            commitment_hash: BigUint::from(1333_u32),
            status: CommitmentStatus::SrcSucceeded.into(),
            block_number: 1800_u64,
            src_chain_block_number: Some(1800_u64),
            included_block_number: None,
            leaf_index: None,
            rollup_fee: None,
            encrypted_notes: None,
            queued_transaction_hash: None,
            included_transaction_hash: None,
            src_chain_transaction_hash: Some("0x3333".to_string()),
        },
        Commitment {
            chain_id: 1_u64,
            contract_address: "address2".to_string(),
            commitment_hash: BigUint::from(4444_u32),
            status: CommitmentStatus::Unspecified.into(),
            block_number: 1500_u64,
            src_chain_block_number: Some(1500_u64),
            included_block_number: None,
            leaf_index: None,
            rollup_fee: None,
            encrypted_notes: None,
            queued_transaction_hash: Some("0x4444".to_string()),
            included_transaction_hash: Some("0x4444".to_string()),
            src_chain_transaction_hash: Some("0x4444".to_string()),
        },
        Commitment {
            chain_id: 1_u64,
            contract_address: "address2".to_string(),
            commitment_hash: BigUint::from(21111_u32),
            status: CommitmentStatus::Unspecified.into(),
            block_number: 1600_u64,
            src_chain_block_number: Some(1600_u64),
            included_block_number: None,
            leaf_index: None,
            rollup_fee: None,
            encrypted_notes: None,
            queued_transaction_hash: Some("0x4444".to_string()),
            included_transaction_hash: Some("0x4444".to_string()),
            src_chain_transaction_hash: Some("0x4444".to_string()),
        },
    ];
    collection.insert_batch(&mock_commitments).await.unwrap();
    let contracts_data = vec![
        ContractData::builder()
            .address("address1")
            .start_block(2000_u64)
            .end_block(4000_u64)
            .data(
                LiteData::builder()
                    .commitments(vec![
                        CommitmentProto {
                            commitment_hash: biguint_to_bytes(&BigUint::from(1222_u32)),
                            status: CommitmentStatus::Included.into(),
                            block_number: 2011_u64,
                            included_block_number: Some(2011_u64),
                            src_chain_block_number: None,
                            leaf_index: None,
                            rollup_fee: None,
                            encrypted_note: None,
                            queued_transaction_hash: None,
                            included_transaction_hash: Some(decode_hex("0x2222").unwrap()),
                            src_chain_transaction_hash: None,
                        },
                        CommitmentProto {
                            commitment_hash: biguint_to_bytes(&BigUint::from(1333_u32)),
                            status: CommitmentStatus::Queued.into(),
                            block_number: 2032_u64,
                            included_block_number: None,
                            src_chain_block_number: None,
                            leaf_index: Some(3_u64),
                            rollup_fee: Some(biguint_to_bytes(&BigUint::from(300_u32))),
                            encrypted_note: Some(decode_hex("0x3333").unwrap()),
                            queued_transaction_hash: Some(decode_hex("0x3333").unwrap()),
                            included_transaction_hash: None,
                            src_chain_transaction_hash: None,
                        },
                        CommitmentProto {
                            commitment_hash: biguint_to_bytes(&BigUint::from(1333_u32)),
                            status: CommitmentStatus::Included.into(),
                            block_number: 2033_u64,
                            included_block_number: Some(2033_u64),
                            src_chain_block_number: None,
                            leaf_index: None,
                            rollup_fee: None,
                            encrypted_note: None,
                            queued_transaction_hash: None,
                            included_transaction_hash: Some(decode_hex("0x3333").unwrap()),
                            src_chain_transaction_hash: None,
                        },
                    ])
                    .build(),
            )
            .build(),
        ContractData::builder()
            .address("address2")
            .start_block(20000_u64)
            .end_block(40000_u64)
            .data(
                LiteData::builder()
                    .commitments(vec![
                        CommitmentProto {
                            commitment_hash: biguint_to_bytes(&BigUint::from(21111_u32)),
                            status: CommitmentStatus::SrcSucceeded.into(),
                            block_number: 21111_u64,
                            included_block_number: None,
                            src_chain_block_number: Some(21111_u64),
                            leaf_index: None,
                            rollup_fee: None,
                            encrypted_note: None,
                            queued_transaction_hash: None,
                            included_transaction_hash: None,
                            src_chain_transaction_hash: Some(decode_hex("0x2111").unwrap()),
                        },
                        CommitmentProto {
                            commitment_hash: biguint_to_bytes(&BigUint::from(21111_u32)),
                            status: CommitmentStatus::Queued.into(),
                            block_number: 22222_u64,
                            included_block_number: None,
                            src_chain_block_number: None,
                            leaf_index: Some(2_u64),
                            rollup_fee: Some(biguint_to_bytes(&BigUint::from(10000_u32))),
                            encrypted_note: Some(decode_hex("0x2111").unwrap()),
                            queued_transaction_hash: Some(decode_hex("0x2111").unwrap()),
                            included_transaction_hash: None,
                            src_chain_transaction_hash: None,
                        },
                        CommitmentProto {
                            commitment_hash: biguint_to_bytes(&BigUint::from(21111_u32)),
                            status: CommitmentStatus::Included.into(),
                            block_number: 23333_u64,
                            included_block_number: Some(23333_u64),
                            src_chain_block_number: None,
                            leaf_index: None,
                            rollup_fee: None,
                            encrypted_note: None,
                            queued_transaction_hash: None,
                            included_transaction_hash: Some(decode_hex("0x2111").unwrap()),
                            src_chain_transaction_hash: None,
                        },
                    ])
                    .build(),
            )
            .build(),
    ];
    let test_chain_data = ChainData::builder()
        .chain_id(1_u64)
        .contracts_data(contracts_data)
        .build();
    let config = Arc::new(
        MystikoConfig::from_json_file("./tests/files/handler/config.json")
            .await
            .unwrap(),
    );
    let handle_result = database_handler
        .handle(&test_chain_data, &HandleOption::builder().config(config).build())
        .await;
    assert!(handle_result.is_ok());
    let result = handle_result.unwrap();
    assert_eq!(result.chain_id, 1_u64);
    assert_eq!(result.contract_results.len(), 2);
    assert_eq!(result.contract_results[0].address, "address1");
    assert!(result.contract_results[0].result.is_ok());
    assert_eq!(result.contract_results[1].address, "address2");
    assert!(result.contract_results[1].result.is_ok());
    let address1_commitments: Vec<Document<Commitment>> = collection
        .find(Some(QueryFilter::from(vec![
            SubFilter::equal(Commitment::column_chain_id(), 1_u64),
            SubFilter::equal(Commitment::column_contract_address(), "address1"),
        ])))
        .await
        .unwrap();
    assert_eq!(address1_commitments.len(), 3);
    let commitment_1222 = address1_commitments
        .into_iter()
        .filter(|c| c.data.commitment_hash == BigUint::from(1222_u32))
        .collect::<Vec<Document<Commitment>>>();
    assert_eq!(commitment_1222.len(), 1);
    assert_eq!(commitment_1222[0].data.commitment_hash, BigUint::from(1222_u32));
    assert_eq!(commitment_1222[0].data.src_chain_block_number, Some(1500_u64));
    assert_eq!(commitment_1222[0].data.included_block_number, Some(2011_u64));
    assert_eq!(commitment_1222[0].data.block_number, 1700_u64);
    assert_eq!(
        commitment_1222[0].data.included_transaction_hash,
        Some("0x2222".to_string())
    );
    assert_eq!(
        commitment_1222[0].data.src_chain_transaction_hash,
        Some("0x2222".to_string())
    );
    assert_eq!(
        commitment_1222[0].data.queued_transaction_hash,
        Some("0x2222".to_string())
    );
}

#[tokio::test]
async fn test_initialize() {
    let (database_handler, collection, config) = setup::<FullData>().await.unwrap();
    let mock_contracts = [
        Contract {
            chain_id: 1_u64,
            contract_address: "0x7F88F2A3Cf18E96844E14CaE59EC97B908734C01".to_string(),
            loaded_block: 2000_u64,
        },
        Contract {
            chain_id: 1_u64,
            contract_address: "0x01BC19b166986431F3f2A348497C4634defb4C29".to_string(),
            loaded_block: 2000_u64,
        },
        Contract {
            chain_id: 56_u64,
            contract_address: "0x53DFAB04453EF45bc637c2687020990c7C8C16bc".to_string(),
            loaded_block: 2000_u64,
        },
        Contract {
            chain_id: 137_u64,
            contract_address: "test_address1".to_string(),
            loaded_block: 2000_u64,
        },
    ];
    collection.insert_batch(&mock_contracts).await.unwrap();
    let result = database_handler.initialize().await;
    assert!(result.is_ok());
    let chain_config_1 = config.find_chain(1_u64).unwrap();
    let contract_1_1 = config
        .find_deposit_contract_by_address(1_u64, "0x53DFAB04453EF45bc637c2687020990c7C8C16bc")
        .unwrap();
    let db_contract_1_1: Option<Document<Contract>> = collection
        .find_one(Some(QueryFilter::from(vec![
            SubFilter::equal(Contract::column_chain_id(), 1_u64),
            SubFilter::equal(
                Contract::column_contract_address(),
                "0x53DFAB04453EF45bc637c2687020990c7C8C16bc",
            ),
        ])))
        .await
        .unwrap();
    assert!(db_contract_1_1.is_some());
    let db_contract_1_1 = db_contract_1_1.unwrap();
    assert_eq!(contract_1_1.address(), db_contract_1_1.data.contract_address);
    assert_eq!(chain_config_1.start_block(), db_contract_1_1.data.loaded_block);
    let contract_1: Vec<Document<Contract>> = collection
        .find::<Contract, QueryFilter>(Some(QueryFilter::from(SubFilter::equal(
            Contract::column_chain_id(),
            1_u64,
        ))))
        .await
        .unwrap();
    assert_eq!(contract_1.len(), config.find_chain(1_u64).unwrap().contracts().len());
    let contract_56 = collection
        .find::<Contract, QueryFilter>(Some(QueryFilter::from(SubFilter::equal(
            Contract::column_chain_id(),
            56_u64,
        ))))
        .await
        .unwrap();
    assert_eq!(contract_56.len(), config.find_chain(56_u64).unwrap().contracts().len());
    let contract_137 = collection
        .find::<Contract, QueryFilter>(Some(QueryFilter::from(SubFilter::equal(
            Contract::column_chain_id(),
            137_u64,
        ))))
        .await
        .unwrap();
    assert_eq!(
        contract_137.len(),
        config.find_chain(137_u64).unwrap().contracts().len() + 1
    );
}

async fn setup<R: LoadedData>() -> Result<(
    DatabaseHandler<R, SqlStatementFormatter, SqliteStorage>,
    Arc<Collection<SqlStatementFormatter, SqliteStorage>>,
    Arc<MystikoConfig>,
)> {
    let _ = env_logger::builder()
        .filter_module("mystiko_dataloader", log::LevelFilter::Debug)
        .try_init();
    let storage: SqliteStorage = SqliteStorage::from_memory().await.unwrap();
    let config: Arc<MystikoConfig> = Arc::new(
        MystikoConfig::from_json_file("./tests/files/handler/config.json")
            .await
            .unwrap(),
    );
    let collection: Arc<Collection<SqlStatementFormatter, SqliteStorage>> =
        Arc::new(Collection::new(SqlStatementFormatter::sqlite(), storage));
    let database_handler: DatabaseHandler<R, SqlStatementFormatter, SqliteStorage> = DatabaseHandler::builder()
        .config(Arc::clone(&config))
        .collection(Arc::clone(&collection))
        .handle_batch_size(5usize)
        .build();
    let _ = database_handler.migrate().await?;
    Ok((database_handler, collection, config))
}
