use crate::handler::database::handler_tests::setup;
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::handler::document::{Commitment, Contract, ContractColumn, Nullifier};
use mystiko_dataloader::handler::DataHandler;
use mystiko_dataloader::loader::ResetOptions;
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_protos::storage::v1::{QueryFilter, SubFilter};
use mystiko_storage::{Collection, SqlStatementFormatter};
use mystiko_storage_sqlite::SqliteStorage;
use num_bigint::BigUint;
use std::sync::Arc;

#[tokio::test]
async fn test_reset_default_option() {
    let (database_handler, collection, _config) = setup::<FullData>().await.unwrap();
    insert_test_data(collection.clone(), 1_u64).await;
    let options = ResetOptions::builder().chain_id(1_u64).build();
    let result = database_handler.reset(&options).await;
    assert!(result.is_ok());
    check_db(1_u64, collection.clone(), 3, &[2000_u64, 3000_u64, 1000_u64], 5, 2).await;
}

#[tokio::test]
async fn test_reset_chain_id_not_exist() {
    let (database_handler, collection, _config) = setup::<FullData>().await.unwrap();
    insert_test_data(collection.clone(), 12345678_u64).await;
    let options = ResetOptions::builder().chain_id(1_u64).build();
    let result = database_handler.reset(&options).await;
    assert!(result.is_ok());
    check_db(1_u64, collection.clone(), 3, &[2000_u64, 3000_u64, 1000_u64], 5, 2).await;
}

#[tokio::test]
async fn test_reset_with_loaded_block_1500() {
    let (database_handler, collection, _config) = setup::<FullData>().await.unwrap();
    insert_test_data(collection.clone(), 1_u64).await;
    let options = ResetOptions::builder().chain_id(1_u64).block_number(1500).build();
    let result = database_handler.reset(&options).await;
    assert!(result.is_ok());
    check_db(1_u64, collection.clone(), 3, &[1500_u64, 1500_u64, 1000_u64], 4, 1).await;
}

#[tokio::test]
async fn test_reset_with_loaded_block_900() {
    let (database_handler, collection, _config) = setup::<FullData>().await.unwrap();
    insert_test_data(collection.clone(), 1_u64).await;
    let options = ResetOptions::builder().chain_id(1_u64).block_number(900).build();
    let result = database_handler.reset(&options).await;
    assert!(result.is_ok());
    check_db(1_u64, collection.clone(), 3, &[900_u64, 900_u64, 900_u64], 0, 0).await;
}

#[tokio::test]
async fn test_reset_with_loaded_block_4000() {
    let (database_handler, collection, _config) = setup::<FullData>().await.unwrap();
    insert_test_data(collection.clone(), 1_u64).await;
    let options = ResetOptions::builder().chain_id(1_u64).block_number(4000).build();
    let result = database_handler.reset(&options).await;
    assert!(result.is_ok());
    check_db(1_u64, collection.clone(), 3, &[2000_u64, 3000_u64, 1000_u64], 5, 2).await;
}

#[tokio::test]
async fn test_reset_with_contracts() {
    let (database_handler, collection, _config) = setup::<FullData>().await.unwrap();
    insert_test_data(collection.clone(), 1_u64).await;
    let options = ResetOptions::builder()
        .chain_id(1_u64)
        .contract_addresses(vec!["address1".to_string()])
        .block_number(900)
        .build();
    let result = database_handler.reset(&options).await;
    assert!(result.is_ok());
    check_db(1_u64, collection.clone(), 3, &[900_u64, 3000_u64, 1000_u64], 2, 1).await;
}

#[tokio::test]
async fn test_reset_with_two_chain_data() {
    let (database_handler, collection, _config) = setup::<FullData>().await.unwrap();
    insert_test_data(collection.clone(), 1_u64).await;
    insert_test_data(collection.clone(), 2_u64).await;
    let options = ResetOptions::builder()
        .chain_id(1_u64)
        .contract_addresses(vec!["address1".to_string()])
        .block_number(900)
        .build();
    let result = database_handler.reset(&options).await;
    assert!(result.is_ok());
    check_db(1_u64, collection.clone(), 3, &[900_u64, 3000_u64, 1000_u64], 2, 1).await;
    check_db(2_u64, collection.clone(), 3, &[2000_u64, 3000_u64, 1000_u64], 5, 2).await;

    let options = ResetOptions::builder()
        .chain_id(2_u64)
        .contract_addresses(vec!["address2".to_string()])
        .block_number(1599)
        .build();
    let result = database_handler.reset(&options).await;
    assert!(result.is_ok());
    check_db(1_u64, collection.clone(), 3, &[900_u64, 3000_u64, 1000_u64], 2, 1).await;
    check_db(2_u64, collection.clone(), 3, &[2000_u64, 1599_u64, 1000_u64], 4, 1).await;
}

async fn check_db(
    chain_id: u64,
    collection: Arc<Collection<SqlStatementFormatter, SqliteStorage>>,
    contracts_count: usize,
    loaded_blocks: &[u64],
    commitments_count: usize,
    nullifiers_count: usize,
) {
    let filter = SubFilter::equal(ContractColumn::ChainId, chain_id);
    let contracts = collection
        .find::<Contract, QueryFilter>(Some(filter.clone().into()))
        .await
        .unwrap();
    assert_eq!(contracts.len(), contracts_count);
    contracts.iter().enumerate().for_each(|(i, contract)| {
        assert_eq!(contract.data.loaded_block, loaded_blocks[i]);
    });

    let commitments = collection
        .find::<Commitment, QueryFilter>(Some(filter.clone().into()))
        .await
        .unwrap();
    assert_eq!(commitments.len(), commitments_count);
    let nullifiers = collection
        .find::<Nullifier, QueryFilter>(Some(filter.clone().into()))
        .await
        .unwrap();
    assert_eq!(nullifiers.len(), nullifiers_count);
}

async fn insert_test_data(collection: Arc<Collection<SqlStatementFormatter, SqliteStorage>>, chain_id: u64) {
    let mock_contracts = [
        Contract {
            chain_id,
            contract_address: "address1".to_string(),
            loaded_block: 2000_u64,
        },
        Contract {
            chain_id,
            contract_address: "address2".to_string(),
            loaded_block: 3000_u64,
        },
        Contract {
            chain_id,
            contract_address: "address3".to_string(),
            loaded_block: 1000_u64,
        },
    ];
    collection.insert_batch(&mock_contracts).await.unwrap();
    let mock_commitments = [
        Commitment {
            chain_id,
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
            chain_id,
            contract_address: "address1".to_string(),
            commitment_hash: BigUint::from(1222_u32),
            status: CommitmentStatus::Queued.into(),
            block_number: 1200_u64,
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
            chain_id,
            contract_address: "address1".to_string(),
            commitment_hash: BigUint::from(1333_u32),
            status: CommitmentStatus::SrcSucceeded.into(),
            block_number: 1300_u64,
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
            chain_id,
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
            chain_id,
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
            chain_id,
            contract_address: "address1".to_string(),
            nullifier: BigUint::from(1111_u32),
            block_number: 1000_u64,
            transaction_hash: "0x1111".to_string(),
        },
        Nullifier {
            chain_id,
            contract_address: "address2".to_string(),
            nullifier: BigUint::from(11111_u32),
            block_number: 1800_u64,
            transaction_hash: "0x11111".to_string(),
        },
    ];
    collection.insert_batch(&mock_nullifiers).await.unwrap();
}
