use crate::scanner::{create_scanner, insert_commitments, DEFAULT_WALLET_PASSWORD};
use mystiko_core::{Nullifier, ScannerError, ScannerHandler};
use mystiko_protos::core::scanner::v1::ScanOptions;
use mystiko_protos::data::v1::CommitmentStatus;

#[tokio::test]
async fn test_scan_default_options() {
    let account_count = 1_usize;
    let (scanner, _, _) = create_scanner(account_count).await;
    let option = ScanOptions::builder().build();
    let result = scanner.scan(option).await;
    assert!(matches!(result.err().unwrap(), ScannerError::WalletHandlerError(_)));

    let option = ScanOptions::builder().wallet_password("wrong_password").build();
    let result = scanner.scan(option).await;
    assert!(matches!(result.err().unwrap(), ScannerError::WalletHandlerError(_)));

    let option = ScanOptions::builder()
        .wallet_password(String::from(DEFAULT_WALLET_PASSWORD))
        .build();
    let result = scanner.scan(option).await.unwrap();
    assert_eq!(result.total_count, 0);
    assert_eq!(result.owned_count, 0);
    assert_eq!(result.scanned_shielded_addresses.len(), account_count);
    assert_eq!(result.to_id, None);

    let option = ScanOptions::builder()
        .wallet_password(String::from(DEFAULT_WALLET_PASSWORD))
        .shielded_addresses(vec!["wrong_shielded_address".to_string()])
        .build();
    let result = scanner.scan(option).await;
    assert!(matches!(result.err().unwrap(), ScannerError::NoAccountFoundError));
}

#[tokio::test]
async fn test_scan_batch_without_owned() {
    for concurrency in 0..3 {
        for batch_size in 0..5 {
            for account_count in 1..3 {
                for commitment_count in 1..12 {
                    let (scanner, db, _) = create_scanner(account_count).await;
                    let option = ScanOptions::builder()
                        .batch_size(batch_size)
                        .concurrency(concurrency)
                        .wallet_password(String::from(DEFAULT_WALLET_PASSWORD))
                        .build();
                    let (cms, _) = insert_commitments(db.clone(), commitment_count, None).await;
                    let result = scanner.scan(option).await.unwrap();
                    assert_eq!(result.total_count, commitment_count as u64);
                    assert_eq!(result.owned_count, 0);
                    assert_eq!(result.scanned_shielded_addresses.len(), account_count);
                    assert_eq!(result.to_id, Some(cms[commitment_count - 1].id.clone()));
                    let updated_cms = db.commitments.find_all().await.unwrap();
                    assert_eq!(cms, updated_cms);
                }
            }
        }
    }
}

#[tokio::test]
async fn test_scan_with_start() {
    let account_count = 3_usize;
    let commitment_count = 2_usize;
    let (scanner, db, _) = create_scanner(account_count).await;
    let mut accounts = db.accounts.find_all().await.unwrap();
    let option = ScanOptions::builder()
        .shielded_addresses(vec![accounts[0].data.shielded_address.clone()])
        .wallet_password(String::from(DEFAULT_WALLET_PASSWORD))
        .build();
    let (cms, _) = insert_commitments(db.clone(), commitment_count, None).await;
    accounts.iter_mut().for_each(|account| {
        account.data.scanned_to_id = Some(cms[commitment_count - 1].id.clone());
    });
    db.accounts.update_batch(&accounts).await.unwrap();
    let result = scanner.scan(option.clone()).await.unwrap();
    assert_eq!(result.total_count, 0);
    assert_eq!(result.owned_count, 0);
    assert_eq!(result.scanned_shielded_addresses.len(), 1);
    assert_eq!(result.scanned_shielded_addresses[0], accounts[0].data.shielded_address);
    assert_eq!(result.to_id, None);

    accounts[0].data.scanned_to_id = Some(cms[commitment_count - 2].id.clone());
    db.accounts.update_batch(&accounts).await.unwrap();
    let result = scanner.scan(option).await.unwrap();
    assert_eq!(result.total_count, 1);
    assert_eq!(result.owned_count, 0);
    assert_eq!(result.scanned_shielded_addresses.len(), 1);
    assert_eq!(result.scanned_shielded_addresses[0], accounts[0].data.shielded_address);
    assert_eq!(result.to_id, Some(cms[commitment_count - 1].id.clone()));
}

#[tokio::test]
async fn test_scan_with_shield_address() {
    let account_count = 3_usize;
    let commitment_count = 2_usize;
    let (scanner, db, _) = create_scanner(account_count).await;
    let accounts = db.accounts.find_all().await.unwrap();
    let option = ScanOptions::builder()
        .shielded_addresses(vec![accounts[0].data.shielded_address.clone()])
        .wallet_password(String::from(DEFAULT_WALLET_PASSWORD))
        .build();
    let (cms, _) = insert_commitments(db, commitment_count, None).await;
    let result = scanner.scan(option).await.unwrap();
    assert_eq!(result.total_count, commitment_count as u64);
    assert_eq!(result.owned_count, 0);
    assert_eq!(result.scanned_shielded_addresses.len(), 1);
    assert_eq!(result.scanned_shielded_addresses[0], accounts[0].data.shielded_address);
    assert_eq!(result.to_id, Some(cms[commitment_count - 1].id.clone()));
}

#[tokio::test]
async fn test_scan_batch_with_owned_without_nullifier() {
    let (scanner, db, test_accounts) = create_scanner(1).await;
    let accounts = db.accounts.find_all().await.unwrap();
    let commitment_count = 10_usize;
    let option = ScanOptions::builder()
        .batch_size(2)
        .concurrency(2)
        .wallet_password(String::from(DEFAULT_WALLET_PASSWORD))
        .build();
    let (mut cms, nullifiers) = insert_commitments(db.clone(), commitment_count, Some(&test_accounts[0])).await;
    let result = scanner.scan(option).await.unwrap();
    assert_eq!(result.total_count, commitment_count as u64);
    assert_eq!(result.owned_count, commitment_count as u64);
    assert_eq!(result.scanned_shielded_addresses.len(), 1);
    assert_eq!(result.to_id, Some(cms[commitment_count - 1].id.clone()));
    let updated_cms = db.commitments.find_all().await.unwrap();
    for (cm, nullifier) in cms.iter_mut().zip(nullifiers.iter()) {
        cm.data.nullifier = Some(nullifier.clone());
        cm.data.shielded_address = Some(accounts[0].data.shielded_address.clone());
    }
    for (cm, updated_cm) in cms.iter().zip(updated_cms.iter()) {
        assert_eq!(cm.data, updated_cm.data);
    }
}

#[tokio::test]
async fn test_scan_batch_with_owned_with_nullifier() {
    let (scanner, db, test_accounts) = create_scanner(1).await;
    let accounts = db.accounts.find_all().await.unwrap();
    let commitment_count = 8_usize;
    let option = ScanOptions::builder()
        .batch_size(2)
        .concurrency(2)
        .wallet_password(String::from(DEFAULT_WALLET_PASSWORD))
        .build();
    let (mut cms, nullifiers) = insert_commitments(db.clone(), commitment_count, Some(&test_accounts[0])).await;
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
    assert_eq!(result.scanned_shielded_addresses.len(), 1);
    assert_eq!(result.to_id, Some(cms[commitment_count - 1].id.clone()));
    let updated_cms = db.commitments.find_all().await.unwrap();
    for (cm, nullifier) in cms.iter_mut().zip(nullifiers.iter()) {
        cm.data.nullifier = Some(nullifier.clone());
        cm.data.shielded_address = Some(accounts[0].data.shielded_address.clone());
        cm.data.status = CommitmentStatus::Included as i32;
        cm.data.spent = true;
    }
    for (cm, updated_cm) in cms.iter().zip(updated_cms.iter()) {
        assert_eq!(cm.data, updated_cm.data);
    }
}
