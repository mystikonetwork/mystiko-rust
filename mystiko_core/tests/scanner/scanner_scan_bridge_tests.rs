use crate::scanner::{create_scanner, insert_commitments, DEFAULT_WALLET_PASSWORD};
use mystiko_core::{CommitmentColumn, Nullifier, ScannerHandler};
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::scanner::v1::ScanOptions;
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_protos::storage::v1::SubFilter;

#[tokio::test]
async fn test_scan_batch_without_owned() {
    let account_count = 1_usize;
    let commitment_count = 20_usize;
    let (scanner, db, _) = create_scanner(account_count).await;
    let option = ScanOptions::builder()
        .batch_size(10)
        .concurrency(1)
        .wallet_password(String::from(DEFAULT_WALLET_PASSWORD))
        .build();
    let (mut cms, _) = insert_commitments(db.clone(), commitment_count, None).await;
    cms.iter_mut().for_each(|cm| {
        cm.data.status = CommitmentStatus::SrcSucceeded as i32;
    });
    db.commitments.update_batch(&cms).await.unwrap();
    let result = scanner.scan(option).await.unwrap();
    assert_eq!(result.total_count, 0);
    assert_eq!(result.owned_count, 0);
    assert_eq!(result.scanned_shielded_addresses.len(), account_count);
    assert_eq!(result.to_id, None);
    let updated_cms = db.commitments.find_all().await.unwrap();
    for (cm, updated_cm) in cms.iter().zip(updated_cms.iter()) {
        assert_eq!(cm.data, updated_cm.data);
    }
    let accounts = db.accounts.find_all().await.unwrap();
    for account in accounts {
        assert_eq!(account.data.scanned_to_id, None);
    }
}

#[tokio::test]
async fn test_scan_batch_with_owned() {
    let account_count = 1_usize;
    let commitment_count = 20_usize;
    let (scanner, db, accounts) = create_scanner(account_count).await;
    let option = ScanOptions::builder()
        .batch_size(10)
        .concurrency(2)
        .wallet_password(String::from(DEFAULT_WALLET_PASSWORD))
        .build();
    let (mut pool_cms, _) = insert_commitments(db.clone(), commitment_count, Some(&accounts[0])).await;
    pool_cms.iter_mut().for_each(|cm| {
        cm.data.chain_id = 5;
        cm.data.contract_address = String::from("0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d");
        cm.data.bridge_type = BridgeType::Tbridge as i32;
    });
    db.commitments.update_batch(&pool_cms).await.unwrap();

    let (mut deposit_cms, nullifiers) = insert_commitments(db.clone(), commitment_count, Some(&accounts[0])).await;
    for (deposit_cm, pool_cm) in deposit_cms.iter_mut().zip(pool_cms.iter()) {
        deposit_cm.data.chain_id = 97;
        deposit_cm.data.contract_address = String::from("0xd791049D0a154bC7860804e1A18ACD148Eb0afD9");
        deposit_cm
            .data
            .commitment_hash
            .clone_from(&pool_cm.data.commitment_hash);
        deposit_cm.data.shielded_address = Some(accounts[0].shielded_address.address().clone());
        deposit_cm.data.status = CommitmentStatus::SrcSucceeded as i32;
        deposit_cm.data.bridge_type = BridgeType::Tbridge as i32;
        deposit_cm.data.spent = false;
    }
    db.commitments.update_batch(&deposit_cms).await.unwrap();
    let db_nullifiers = nullifiers
        .iter()
        .map(|nullifier| Nullifier {
            chain_id: 5,
            contract_address: String::from("contract_address 1"),
            block_number: 10000000u64,
            nullifier: nullifier.clone(),
            transaction_hash: String::from("transaction_hash 1"),
        })
        .collect::<Vec<_>>();
    db.nullifiers.insert_batch(&db_nullifiers).await.unwrap();
    let result = scanner.scan(option).await.unwrap();
    assert_eq!(result.total_count, commitment_count as u64);
    assert_eq!(result.owned_count, commitment_count as u64);
    assert_eq!(result.scanned_shielded_addresses.len(), account_count);
    assert_eq!(result.to_id, Some(pool_cms[commitment_count - 1].id.clone()));
    let updated_pool_cms = db
        .commitments
        .find(SubFilter::equal(
            CommitmentColumn::Status,
            CommitmentStatus::Included as i32,
        ))
        .await
        .unwrap();
    for (cm, nullifier) in pool_cms.iter_mut().zip(nullifiers.iter()) {
        cm.data.nullifier = Some(nullifier.clone());
        cm.data.shielded_address = Some(accounts[0].shielded_address.address().clone());
        cm.data.status = CommitmentStatus::Included as i32;
        cm.data.spent = true;
    }
    for (cm, updated_cm) in pool_cms.iter().zip(updated_pool_cms.iter()) {
        assert_eq!(cm.data, updated_cm.data);
    }

    let updated_deposit_cms = db
        .commitments
        .find(SubFilter::equal(
            CommitmentColumn::Status,
            CommitmentStatus::SrcSucceeded as i32,
        ))
        .await
        .unwrap();
    deposit_cms.iter_mut().for_each(|cm| cm.data.spent = true);
    for (cm, updated_cm) in deposit_cms.iter().zip(updated_deposit_cms.iter()) {
        assert_eq!(cm.data, updated_cm.data);
    }

    let accounts = db.accounts.find_all().await.unwrap();
    for account in accounts {
        assert_eq!(
            account.data.scanned_to_id,
            Some(pool_cms[commitment_count - 1].id.clone())
        );
    }
}

#[tokio::test]
async fn test_scan_batch_with_owned_without_nullifier() {
    let account_count = 1_usize;
    let commitment_count = 20_usize;
    let (scanner, db, accounts) = create_scanner(account_count).await;
    let option = ScanOptions::builder()
        .batch_size(10)
        .concurrency(2)
        .wallet_password(String::from(DEFAULT_WALLET_PASSWORD))
        .build();
    let (mut pool_cms, _) = insert_commitments(db.clone(), commitment_count, Some(&accounts[0])).await;
    pool_cms.iter_mut().for_each(|cm| {
        cm.data.chain_id = 5;
        cm.data.contract_address = String::from("0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d");
        cm.data.bridge_type = BridgeType::Tbridge as i32;
    });
    db.commitments.update_batch(&pool_cms).await.unwrap();

    let (mut deposit_cms, nullifiers) = insert_commitments(db.clone(), commitment_count, Some(&accounts[0])).await;
    for (deposit_cm, pool_cm) in deposit_cms.iter_mut().zip(pool_cms.iter()) {
        deposit_cm.data.chain_id = 97;
        deposit_cm.data.contract_address = String::from("0xd791049D0a154bC7860804e1A18ACD148Eb0afD9");
        deposit_cm
            .data
            .commitment_hash
            .clone_from(&pool_cm.data.commitment_hash);
        deposit_cm.data.shielded_address = Some(accounts[0].shielded_address.address().clone());
        deposit_cm.data.status = CommitmentStatus::SrcSucceeded as i32;
        deposit_cm.data.bridge_type = BridgeType::Tbridge as i32;
        deposit_cm.data.spent = false;
    }
    db.commitments.update_batch(&deposit_cms).await.unwrap();
    let result = scanner.scan(option).await.unwrap();
    assert_eq!(result.total_count, commitment_count as u64);
    assert_eq!(result.owned_count, commitment_count as u64);
    assert_eq!(result.scanned_shielded_addresses.len(), account_count);
    assert_eq!(result.to_id, Some(pool_cms[commitment_count - 1].id.clone()));
    let updated_pool_cms = db
        .commitments
        .find(SubFilter::equal(
            CommitmentColumn::Status,
            CommitmentStatus::Included as i32,
        ))
        .await
        .unwrap();
    for (cm, nullifier) in pool_cms.iter_mut().zip(nullifiers.iter()) {
        cm.data.nullifier = Some(nullifier.clone());
        cm.data.shielded_address = Some(accounts[0].shielded_address.address().clone());
        cm.data.status = CommitmentStatus::Included as i32;
        cm.data.spent = false;
    }
    for (cm, updated_cm) in pool_cms.iter().zip(updated_pool_cms.iter()) {
        assert_eq!(cm.data, updated_cm.data);
    }

    let updated_deposit_cms = db
        .commitments
        .find(SubFilter::equal(
            CommitmentColumn::Status,
            CommitmentStatus::SrcSucceeded as i32,
        ))
        .await
        .unwrap();
    deposit_cms.iter_mut().for_each(|cm| cm.data.spent = true);
    for (cm, updated_cm) in deposit_cms.iter().zip(updated_deposit_cms.iter()) {
        assert_eq!(cm.data, updated_cm.data);
    }

    let accounts = db.accounts.find_all().await.unwrap();
    for account in accounts {
        assert_eq!(
            account.data.scanned_to_id,
            Some(pool_cms[commitment_count - 1].id.clone())
        );
    }
}
