use crate::scanner::{
    build_mock_provider_with_queued_event, create_scanner, MockCommitmentPoolContracts, DEFAULT_WALLET_PASSWORD,
};
use ethers_core::types::U256;
use mystiko_core::{Commitment, ScannerHandler};
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::scanner::v1::{
    AssetChainImportOptions, AssetChainImportResult, AssetImportOptions, AssetImportResult,
};
use mystiko_protos::data::v1::CommitmentStatus;
use num_bigint::BigUint;
use std::collections::HashMap;

#[tokio::test]
async fn test_import_one_queued_commitment() {
    let chain_id = 5_u64;
    let account_count = 1_usize;
    let mut mock_commitment_pool = MockCommitmentPoolContracts::default();
    mock_commitment_pool
        .expect_get_commitment_included_count()
        .returning(|_| Ok(U256::from(0)));
    mock_commitment_pool
        .expect_is_spent_nullifier()
        .returning(|_| Ok(false));
    let provider = build_mock_provider_with_queued_event(1);
    let (scanner, db, _) = create_scanner(
        account_count,
        Some("fragile hat december author fancy include nominee spot produce priority income inmate catch aware level poet group pretty rude exit route pizza perfect anger".to_string()),
        HashMap::from([(chain_id, provider)]),
        Some(mock_commitment_pool),None,
    )
        .await;

    let options = AssetImportOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .chains([AssetChainImportOptions::builder()
            .chain_id(chain_id)
            .tx_hashes(vec![
                "0xa5832c0a90837280d29de8498144c40c295fbf4adae7efc97046c322cb81c1c2".to_string(),
            ])
            .build()])
        .build();
    let r = scanner.import(options).await.unwrap();
    assert_eq!(
        r,
        AssetImportResult::builder()
            .chains([AssetChainImportResult::builder()
                .chain_id(chain_id)
                .found_count(1_u32)
                .imported_count(1_u32)
                .build()])
            .build()
    );
    let cm = db.commitments.find_all().await.unwrap();
    assert_eq!(cm.len(), 1);
    assert_eq!(cm[0].data.status, CommitmentStatus::Queued as i32);
    assert_eq!(cm[0].data.leaf_index, Some(10));
    assert!(!cm[0].data.spent);
}

#[tokio::test]
async fn test_import_two_queued_commitment() {
    let chain_id = 5_u64;
    let account_count = 1_usize;
    let mut mock_commitment_pool = MockCommitmentPoolContracts::default();
    mock_commitment_pool
        .expect_get_commitment_included_count()
        .returning(|_| Ok(U256::from(0)));
    mock_commitment_pool
        .expect_is_spent_nullifier()
        .returning(|_| Ok(false));
    let provider = build_mock_provider_with_queued_event(2);
    let (scanner, db, _) = create_scanner(
        account_count,
        Some("fragile hat december author fancy include nominee spot produce priority income inmate catch aware level poet group pretty rude exit route pizza perfect anger".to_string()),
        HashMap::from([(chain_id, provider)]),
        Some(mock_commitment_pool),None ,
    )
        .await;

    let options = AssetImportOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .chains([AssetChainImportOptions::builder()
            .chain_id(chain_id)
            .tx_hashes(vec![
                "0xa5832c0a90837280d29de8498144c40c295fbf4adae7efc97046c322cb81c1c2".to_string(),
            ])
            .build()])
        .build();
    let r = scanner.import(options).await.unwrap();
    assert_eq!(
        r,
        AssetImportResult::builder()
            .chains([AssetChainImportResult::builder()
                .chain_id(chain_id)
                .found_count(2_u32)
                .imported_count(2_u32)
                .build()])
            .build()
    );
    let cm = db.commitments.find_all().await.unwrap();
    assert_eq!(cm.len(), 2);
    assert_eq!(cm[0].data.status, CommitmentStatus::Queued as i32);
    assert_eq!(cm[0].data.leaf_index, Some(10));
    assert!(!cm[0].data.spent);
    assert_eq!(cm[1].data.status, CommitmentStatus::Queued as i32);
    assert_eq!(cm[1].data.leaf_index, Some(11));
    assert!(!cm[1].data.spent);
}

#[tokio::test]
async fn test_import_one_included_commitment() {
    let chain_id = 5_u64;
    let account_count = 1_usize;
    let mut mock_commitment_pool = MockCommitmentPoolContracts::default();
    mock_commitment_pool
        .expect_get_commitment_included_count()
        .returning(|_| Ok(U256::from(200000)));
    mock_commitment_pool
        .expect_is_spent_nullifier()
        .returning(|_| Ok(false));
    let provider = build_mock_provider_with_queued_event(1);
    let (scanner, db, _) = create_scanner(
        account_count,
        Some("fragile hat december author fancy include nominee spot produce priority income inmate catch aware level poet group pretty rude exit route pizza perfect anger".to_string()),
        HashMap::from([(chain_id, provider)]),
        Some(mock_commitment_pool),None ,
    )
        .await;

    let options = AssetImportOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .chains([AssetChainImportOptions::builder()
            .chain_id(chain_id)
            .tx_hashes(vec![
                "0xa5832c0a90837280d29de8498144c40c295fbf4adae7efc97046c322cb81c1c2".to_string(),
            ])
            .build()])
        .build();
    let r = scanner.import(options).await.unwrap();
    assert_eq!(
        r,
        AssetImportResult::builder()
            .chains([AssetChainImportResult::builder()
                .chain_id(chain_id)
                .found_count(1_u32)
                .imported_count(1_u32)
                .build()])
            .build()
    );
    let cm = db.commitments.find_all().await.unwrap();
    assert_eq!(cm.len(), 1);
    assert_eq!(cm[0].data.status, CommitmentStatus::Included as i32);
    assert_eq!(cm[0].data.leaf_index, Some(10));
    assert!(!cm[0].data.spent);
}

#[tokio::test]
async fn test_import_two_included_commitment() {
    let chain_id = 5_u64;
    let account_count = 1_usize;
    let mut mock_commitment_pool = MockCommitmentPoolContracts::default();
    mock_commitment_pool
        .expect_get_commitment_included_count()
        .returning(|_| Ok(U256::from(200000)));
    mock_commitment_pool
        .expect_is_spent_nullifier()
        .returning(|_| Ok(false));
    let provider = build_mock_provider_with_queued_event(2);
    let (scanner, db, _) = create_scanner(
        account_count,
        Some("fragile hat december author fancy include nominee spot produce priority income inmate catch aware level poet group pretty rude exit route pizza perfect anger".to_string()),
        HashMap::from([(chain_id, provider)]),
        Some(mock_commitment_pool),None ,
    )
        .await;

    let options = AssetImportOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .chains([AssetChainImportOptions::builder()
            .chain_id(chain_id)
            .tx_hashes(vec![
                "0xa5832c0a90837280d29de8498144c40c295fbf4adae7efc97046c322cb81c1c2".to_string(),
            ])
            .build()])
        .build();
    let r = scanner.import(options).await.unwrap();
    assert_eq!(
        r,
        AssetImportResult::builder()
            .chains([AssetChainImportResult::builder()
                .chain_id(chain_id)
                .found_count(2_u32)
                .imported_count(2_u32)
                .build()])
            .build()
    );
    let cm = db.commitments.find_all().await.unwrap();
    assert_eq!(cm.len(), 2);
    assert_eq!(cm[0].data.status, CommitmentStatus::Included as i32);
    assert!(!cm[0].data.spent);
    assert_eq!(cm[0].data.leaf_index, Some(10));
    assert_eq!(cm[1].data.status, CommitmentStatus::Included as i32);
    assert_eq!(cm[1].data.leaf_index, Some(11));
    assert!(!cm[1].data.spent);
}

#[tokio::test]
async fn test_import_spent_commitment() {
    let chain_id = 5_u64;
    let account_count = 1_usize;
    let mut mock_commitment_pool = MockCommitmentPoolContracts::default();
    mock_commitment_pool
        .expect_get_commitment_included_count()
        .returning(|_| Ok(U256::from(0)));
    mock_commitment_pool.expect_is_spent_nullifier().returning(|_| Ok(true));
    let provider = build_mock_provider_with_queued_event(1);
    let (scanner, db, _) = create_scanner(
        account_count,
        Some("fragile hat december author fancy include nominee spot produce priority income inmate catch aware level poet group pretty rude exit route pizza perfect anger".to_string()),
        HashMap::from([(chain_id, provider)]),
        Some(mock_commitment_pool),None
    )
        .await;

    let options = AssetImportOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .chains([AssetChainImportOptions::builder()
            .chain_id(chain_id)
            .tx_hashes(vec![
                "0xa5832c0a90837280d29de8498144c40c295fbf4adae7efc97046c322cb81c1c2".to_string(),
            ])
            .build()])
        .build();
    let r = scanner.import(options).await.unwrap();
    assert_eq!(
        r,
        AssetImportResult::builder()
            .chains([AssetChainImportResult::builder()
                .chain_id(chain_id)
                .found_count(1_u32)
                .imported_count(1_u32)
                .build()])
            .build()
    );
    let cm = db.commitments.find_all().await.unwrap();
    assert_eq!(cm.len(), 1);
    assert_eq!(cm[0].data.status, CommitmentStatus::Included as i32);
    assert!(cm[0].data.spent);
}

#[tokio::test]
async fn test_import_others_commitment() {
    let chain_id = 5_u64;
    let account_count = 1_usize;
    let mut mock_commitment_pool = MockCommitmentPoolContracts::default();
    mock_commitment_pool
        .expect_get_commitment_included_count()
        .returning(|_| Ok(U256::from(0)));
    mock_commitment_pool.expect_is_spent_nullifier().returning(|_| Ok(true));
    let provider = build_mock_provider_with_queued_event(1);
    let (scanner, db, _) = create_scanner(
        account_count,
        None,
        HashMap::from([(chain_id, provider)]),
        Some(mock_commitment_pool),
        None,
    )
    .await;

    let options = AssetImportOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .chains([AssetChainImportOptions::builder()
            .chain_id(chain_id)
            .tx_hashes(vec![
                "0xa5832c0a90837280d29de8498144c40c295fbf4adae7efc97046c322cb81c1c2".to_string(),
            ])
            .build()])
        .build();
    let r = scanner.import(options).await.unwrap();
    assert_eq!(
        r,
        AssetImportResult::builder()
            .chains([AssetChainImportResult::builder()
                .chain_id(chain_id)
                .found_count(1_u32)
                .imported_count(0_u32)
                .build()])
            .build()
    );
    let cm = db.commitments.find_all().await.unwrap();
    assert_eq!(cm.len(), 0);
}

#[tokio::test]
async fn test_import_merge_spend_commitment() {
    let chain_id = 5_u64;
    let account_count = 1_usize;
    let mut mock_commitment_pool = MockCommitmentPoolContracts::default();
    mock_commitment_pool
        .expect_get_commitment_included_count()
        .returning(|_| Ok(U256::from(0)));
    mock_commitment_pool.expect_is_spent_nullifier().returning(|_| Ok(true));
    let provider = build_mock_provider_with_queued_event(1);
    let (scanner, db, accounts) = create_scanner(
        account_count,
        Some("fragile hat december author fancy include nominee spot produce priority income inmate catch aware level poet group pretty rude exit route pizza perfect anger".to_string()),
        HashMap::from([(chain_id, provider)]),
        Some(mock_commitment_pool),None
    )
        .await;

    let new_commitment = Commitment {
        chain_id,
        contract_address: "0x00b73dbC8C370CA7e5F00b778280596383b62929".to_string(),
        bridge_type: BridgeType::Loop as i32,
        commitment_hash: BigUint::parse_bytes(
            "1db84c1b0bd7877f4cddd3f5b0a8ae202b017234f84dc75face85b7556951fc4".as_bytes(),
            16,
        )
        .unwrap(),
        asset_symbol: "ETH".to_string(),
        asset_decimals: 18,
        asset_address: None,
        status: CommitmentStatus::Queued as i32,
        spent: false,
        block_number: 10000_u64,
        src_chain_block_number: None,
        included_block_number: None,
        rollup_fee_amount: None,
        encrypted_note: None,
        leaf_index: None,
        amount: None,
        nullifier: None,
        shielded_address: None,
        queued_transaction_hash: None,
        included_transaction_hash: None,
        src_chain_transaction_hash: None,
    };
    db.commitments.insert(&new_commitment).await.unwrap();

    let options = AssetImportOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .chains([AssetChainImportOptions::builder()
            .chain_id(chain_id)
            .tx_hashes(vec![
                "0xa5832c0a90837280d29de8498144c40c295fbf4adae7efc97046c322cb81c1c2".to_string(),
            ])
            .build()])
        .build();
    let r = scanner.import(options).await.unwrap();
    assert_eq!(
        r,
        AssetImportResult::builder()
            .chains([AssetChainImportResult::builder()
                .chain_id(chain_id)
                .found_count(1_u32)
                .imported_count(1_u32)
                .build()])
            .build()
    );
    let cm = db.commitments.find_all().await.unwrap();
    assert_eq!(cm.len(), 1);
    assert_eq!(cm[0].data.status, CommitmentStatus::Included as i32);
    assert!(cm[0].data.spent);
    assert!(cm[0].data.rollup_fee_amount.is_some());
    assert!(cm[0].data.encrypted_note.is_some());
    assert!(cm[0].data.leaf_index.is_some());
    assert!(cm[0].data.amount.is_some());
    assert!(cm[0].data.nullifier.is_some());
    assert_eq!(
        cm[0].data.shielded_address,
        Some(accounts[0].shielded_address.address())
    );
}

#[tokio::test]
async fn test_import_two_chain_commitment() {
    let chain_id_1 = 5_u64;
    let chain_id_2 = 97_u64;
    let account_count = 1_usize;
    let mut mock_commitment_pool = MockCommitmentPoolContracts::default();
    mock_commitment_pool
        .expect_get_commitment_included_count()
        .returning(|_| Ok(U256::from(200000)));
    mock_commitment_pool
        .expect_is_spent_nullifier()
        .returning(|_| Ok(false));
    let provider1 = build_mock_provider_with_queued_event(2);
    let provider2 = build_mock_provider_with_queued_event(2);
    let (scanner, db, _) = create_scanner(
        account_count,
        Some("fragile hat december author fancy include nominee spot produce priority income inmate catch aware level poet group pretty rude exit route pizza perfect anger".to_string()),
        HashMap::from([(chain_id_1, provider1), (chain_id_2, provider2)]),
        Some(mock_commitment_pool),
        None,
    )
        .await;

    let options = AssetImportOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .chains([
            AssetChainImportOptions::builder()
                .chain_id(chain_id_1)
                .tx_hashes(vec![
                    "0xa5832c0a90837280d29de8498144c40c295fbf4adae7efc97046c322cb81c1c2".to_string(),
                ])
                .build(),
            AssetChainImportOptions::builder()
                .chain_id(chain_id_2)
                .tx_hashes(vec![
                    "0xa5832c0a90837280d29de8498144c40c295fbf4adae7efc97046c322cb81c1c2".to_string(),
                ])
                .build(),
        ])
        .build();
    let r = scanner.import(options).await.unwrap();
    assert_eq!(
        r,
        AssetImportResult::builder()
            .chains([AssetChainImportResult::builder()
                .chain_id(chain_id_1)
                .found_count(2_u32)
                .imported_count(2_u32)
                .build()])
            .build()
    );
    let cm = db.commitments.find_all().await.unwrap();
    assert_eq!(cm.len(), 2);
    assert_eq!(cm[0].data.status, CommitmentStatus::Included as i32);
    assert!(!cm[0].data.spent);
    assert_eq!(cm[0].data.leaf_index, Some(10));
    assert_eq!(cm[1].data.status, CommitmentStatus::Included as i32);
    assert_eq!(cm[1].data.leaf_index, Some(11));
    assert!(!cm[1].data.spent);
}
