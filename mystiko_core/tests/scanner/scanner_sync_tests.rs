use crate::scanner::{
    build_mock_provider_with_queued_event, create_scanner, insert_commitments, MockCommitmentPoolContracts,
    MockSequencerClient, DEFAULT_WALLET_PASSWORD,
};
use ethers_core::types::U256;
use mystiko_core::ScannerHandler;
use mystiko_protos::core::scanner::v1::{BalanceOptions, ScannerSyncOptions};
use mystiko_protos::data::v1::Commitment as ProtosCommitment;
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_utils::convert::{biguint_to_bytes, decimal_to_number};
use mystiko_utils::hex::decode_hex;
use num_bigint::BigUint;
use num_traits::Zero;
use std::collections::HashMap;
use std::ops::{Add, Sub};

#[tokio::test]
async fn test_sync_commitment_from_provider() {
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
    let (scanner, db, accounts) = create_scanner(
        account_count,
        Some("fragile hat december author fancy include nominee spot produce priority income inmate catch aware level poet group pretty rude exit route pizza perfect anger".to_string()),
        HashMap::from([(chain_id, provider)]),
        Some(mock_commitment_pool),None,
    )
        .await;
    let (mut cms, _) = insert_commitments(db.clone(), 1, None).await;

    let amount = BigUint::from(1234567890000000000_u64);
    let account = accounts.first().unwrap();
    cms.iter_mut().for_each(|cm| {
        cm.data.commitment_hash = BigUint::parse_bytes(
            "13c386238b4d87f6c8c5a356d09ba44bb5633f80828c6cbd10c6e5f4db1d04b4".as_bytes(),
            16,
        )
        .unwrap();
        cm.data.queued_transaction_hash =
            Some("0xa5832c0a90837280d29de8498144c40c295fbf4adae7efc97046c322cb81c1c2".to_string());
        cm.data.shielded_address = Some(account.shielded_address.address());
        cm.data.contract_address = "0x00b73dbC8C370CA7e5F00b778280596383b62929".to_string();
        cm.data.block_number = 10000000_u64 - 1;
        cm.data.status = CommitmentStatus::SrcSucceeded as i32;
        cm.data.leaf_index = None;
        cm.data.amount = None;
        cm.data.encrypted_note = None;
    });
    let pending = decimal_to_number::<f64, BigUint>(&amount, Some(18)).unwrap();
    db.commitments.update_batch(&cms).await.unwrap();
    let result = scanner.balance(BalanceOptions::builder().build()).await.unwrap();
    assert_eq!(result.balances.len(), 0);

    let options = ScannerSyncOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let result = scanner.sync(options).await.unwrap();
    assert_eq!(result.results.len(), 1);
    assert_eq!(result.results[0].balances.len(), 1);
    assert_eq!(result.results[0].balances[0].asset_symbol, "MTT");
    assert_eq!(result.results[0].balances[0].pending, pending);
    assert_eq!(result.results[0].balances[0].unspent, 0.0);
    assert_eq!(result.results[0].balances[0].spent, None);

    let cms = db.commitments.find_all().await.unwrap();
    assert_eq!(cms.len(), 1);
    assert_eq!(cms[0].data.status, CommitmentStatus::Queued as i32);
    assert_eq!(cms[0].data.leaf_index, Some(10_u64));
    assert_eq!(cms[0].data.asset_decimals, 18);
    assert!(cms[0].data.encrypted_note.is_some());
    assert!(cms[0].data.nullifier.is_some());
}

#[tokio::test]
async fn test_sync_commitment_by_commitment_hash() {
    let account_count = 1_usize;
    let mut mock_commitment_pool = MockCommitmentPoolContracts::default();
    mock_commitment_pool
        .expect_get_commitment_included_count()
        .returning(|_| Ok(U256::from(0)));
    mock_commitment_pool
        .expect_is_spent_nullifier()
        .returning(|_| Ok(false));
    let mut sequencer = MockSequencerClient::default();
    sequencer.expect_get_commitments().returning(|_, _, cm_hash| {
        let cm = ProtosCommitment::builder()
            .commitment_hash(biguint_to_bytes(&cm_hash[0]))
            .status(CommitmentStatus::Queued)
            .encrypted_note(Some(decode_hex("0x45f68f0fa27ca1f76a147f3361bf11910467f90fad56888a31544354b6b2ae4d367da8215769b872e64f7d1aa1a39a025bc62d485baa8124a7ee62dfa6072307d00f823a880ca31d8ecfe61c3a60beccadf95b44cbb22c9696ec8c918514d57b99bf910edaf4a33c0b65500e6d80bee9b564d3f3ed6b267675c588b2e366b180dd815ba1531c8ac5ae4bab228e6b999312641faae4f3bf306d9c6acf88f440f63f02da2f2a62b16679528ba25e6ca4c1d067aa66afed5b6fd2c2eb5c35bcb7db4e3ff7fff54bd72d2b75db73bac3c505f7").unwrap()))
            .leaf_index(123)
            .block_number(10000000_u64)
            .build();
        Ok(vec![cm])
    });
    let (scanner, db, accounts) = create_scanner(
        account_count,
        Some("fragile hat december author fancy include nominee spot produce priority income inmate catch aware level poet group pretty rude exit route pizza perfect anger".to_string()),
        HashMap::new(),
        Some(mock_commitment_pool),Some(sequencer),
    )
        .await;
    let (mut cms, _) = insert_commitments(db.clone(), 1, None).await;
    let amount = BigUint::from(1234567890000000000_u64);
    let asset_decimals = 16;
    let account = accounts.first().unwrap();
    cms.iter_mut().for_each(|cm| {
        cm.data.commitment_hash = BigUint::parse_bytes(
            "13c386238b4d87f6c8c5a356d09ba44bb5633f80828c6cbd10c6e5f4db1d04b4".as_bytes(),
            16,
        )
        .unwrap();
        cm.data.queued_transaction_hash = None;
        cm.data.shielded_address = Some(account.shielded_address.address());
        cm.data.contract_address = "0x00b73dbC8C370CA7e5F00b778280596383b62929".to_string();
        cm.data.block_number = 10000000_u64 - 1;
        cm.data.asset_decimals = asset_decimals;
        cm.data.status = CommitmentStatus::SrcSucceeded as i32;
        cm.data.leaf_index = None;
        cm.data.amount = None;
        cm.data.encrypted_note = None;
    });
    let pending = decimal_to_number::<f64, BigUint>(&amount, Some(asset_decimals)).unwrap();
    db.commitments.update_batch(&cms).await.unwrap();
    let result = scanner.balance(BalanceOptions::builder().build()).await.unwrap();
    assert_eq!(result.balances.len(), 0);

    let options = ScannerSyncOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let result = scanner.sync(options).await.unwrap();
    assert_eq!(result.results.len(), 1);
    assert_eq!(result.results[0].balances.len(), 1);
    assert_eq!(result.results[0].balances[0].asset_symbol, "MTT");
    assert_eq!(result.results[0].balances[0].pending, pending);
    assert_eq!(result.results[0].balances[0].unspent, 0.0);
    assert_eq!(result.results[0].balances[0].spent, None);

    let cms = db.commitments.find_all().await.unwrap();
    assert_eq!(cms.len(), 1);
    assert_eq!(cms[0].data.status, CommitmentStatus::Queued as i32);
    assert_eq!(cms[0].data.asset_decimals, asset_decimals);
    assert_eq!(cms[0].data.leaf_index, Some(123_u64));
    assert!(cms[0].data.encrypted_note.is_some());
    assert!(cms[0].data.nullifier.is_some());
}

#[tokio::test]
async fn test_sync_commitment_status() {
    let account_count = 1_usize;
    let mut leaf_index = 100;
    let mut mock_commitment_pool = MockCommitmentPoolContracts::default();
    mock_commitment_pool
        .expect_get_commitment_included_count()
        .returning(|_| Ok(U256::from(200)));
    mock_commitment_pool.expect_is_spent_nullifier().returning(|_| Ok(true));

    let (scanner, db, accounts) = create_scanner(
        account_count,
        Some("fragile hat december author fancy include nominee spot produce priority income inmate catch aware level poet group pretty rude exit route pizza perfect anger".to_string()),
        HashMap::new(),
        Some(mock_commitment_pool),None,
    )
        .await;
    let (mut cms, _) = insert_commitments(db.clone(), 2, None).await;
    let account = accounts.first().unwrap();
    let mut amount1 = BigUint::zero();
    cms.iter_mut().for_each(|cm| {
        cm.data.contract_address = "0x00b73dbC8C370CA7e5F00b778280596383b62929".to_string();
        cm.data.shielded_address = Some(account.shielded_address.address());
        cm.data.leaf_index = Some(leaf_index);
        cm.data.status = CommitmentStatus::Queued as i32;
        cm.data.nullifier = None;
        leaf_index += 1;
        amount1 = amount1.clone().add(cm.data.amount.as_ref().unwrap());
    });
    cms[0].data.nullifier = Some(BigUint::from(123_u32));
    let amount2 = amount1.clone().sub(cms[0].data.amount.as_ref().unwrap());

    db.commitments.update_batch(&cms).await.unwrap();
    let result = scanner.balance(BalanceOptions::builder().build()).await.unwrap();
    let pending = decimal_to_number::<f64, BigUint>(&amount1, Some(18)).unwrap();
    assert_eq!(result.balances.len(), 1);
    assert!((pending - result.balances[0].pending).abs() < 1e-14);
    assert_eq!(result.balances[0].unspent, 0.0);
    assert_eq!(result.balances[0].spent, None);

    let options = ScannerSyncOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let result = scanner.sync(options).await.unwrap();
    let unspent = decimal_to_number::<f64, BigUint>(&amount2, Some(18)).unwrap();
    assert_eq!(result.results.len(), 1);
    assert_eq!(result.results[0].balances.len(), 1);
    assert_eq!(result.results[0].balances[0].asset_symbol, "MTT");
    assert_eq!(result.results[0].balances[0].pending, 0.0);
    assert!((unspent - result.results[0].balances[0].unspent).abs() < 1e-14);
    assert_eq!(result.results[0].balances[0].spent, None);

    let cms = db.commitments.find_all().await.unwrap();
    assert_eq!(cms.len(), 2);
    assert_eq!(cms[0].data.status, CommitmentStatus::Included as i32);
    assert_eq!(cms[1].data.status, CommitmentStatus::Included as i32);
    assert!(cms[0].data.spent);
    assert!(!cms[1].data.spent);
}
