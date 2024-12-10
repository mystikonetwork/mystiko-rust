use crate::scanner::create_scanner;
use mystiko_core::{ScannerError, ScannerHandler};
use mystiko_protos::core::scanner::v1::ScannerResetOptions;
use std::collections::HashMap;

#[tokio::test]
async fn test_scan_reset_default_option() {
    let account_count = 3_usize;
    let (scanner, db, test_accounts) = create_scanner(account_count, None, HashMap::new(), None, None).await;

    let mut accounts = db.accounts.find_all().await.unwrap();
    accounts.iter_mut().for_each(|a| {
        a.data.scanned_to_id = Some("test".to_string());
    });
    db.accounts.update_batch(&accounts).await.unwrap();
    let option = ScannerResetOptions::builder().build();
    let result = scanner.reset(option).await;
    assert!(result.is_ok());
    let accounts = db.accounts.find_all().await.unwrap();
    accounts.iter().for_each(|a| {
        assert_eq!(a.data.scanned_to_id, None);
    });

    let option = ScannerResetOptions::builder()
        .shielded_addresses(vec!["wrong_shielded_address".to_string()])
        .build();
    let result = scanner.reset(option).await;
    assert!(matches!(result.err().unwrap(), ScannerError::NoSuchAccountError));

    let option = ScannerResetOptions::builder()
        .shielded_addresses(vec![
            "wrong_shielded_address".to_string(),
            test_accounts[0].shielded_address.address(),
        ])
        .build();
    let result = scanner.reset(option).await;
    assert!(result.is_ok());

    let account_count = 0_usize;
    let (scanner, _, _) = create_scanner(account_count, None, HashMap::new(), None, None).await;
    let option = ScannerResetOptions::builder().shielded_addresses(vec![]).build();
    let result = scanner.reset(option).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_scan_reset_to_id() {
    let account_count = 3_usize;
    let (scanner, db, _) = create_scanner(account_count, None, HashMap::new(), None, None).await;

    let mut accounts = db.accounts.find_all().await.unwrap();
    accounts.iter_mut().for_each(|a| {
        a.data.scanned_to_id = Some("test".to_string());
    });
    db.accounts.update_batch(&accounts).await.unwrap();

    let option = ScannerResetOptions::builder().reset_to_id("reset".to_string()).build();
    let result = scanner.reset(option).await;
    assert!(result.is_ok());
    let accounts = db.accounts.find_all().await.unwrap();
    accounts.iter().for_each(|a| {
        assert_eq!(a.data.scanned_to_id, Some("reset".to_string()));
    });
}

#[tokio::test]
async fn test_scan_reset_some_account() {
    let account_count = 3_usize;
    let (scanner, db, _) = create_scanner(account_count, None, HashMap::new(), None, None).await;

    let mut accounts = db.accounts.find_all().await.unwrap();
    accounts.iter_mut().for_each(|a| {
        a.data.scanned_to_id = Some("test".to_string());
    });
    db.accounts.update_batch(&accounts).await.unwrap();

    let option = ScannerResetOptions::builder()
        .shielded_addresses(vec![
            "wrong_shielded_address".to_string(),
            accounts[0].data.shielded_address.clone(),
        ])
        .reset_to_id("reset".to_string())
        .build();
    let result = scanner.reset(option).await;
    assert!(result.is_ok());
    let accounts = db.accounts.find_all().await.unwrap();
    assert_eq!(accounts[0].data.scanned_to_id, Some("reset".to_string()));
    assert_eq!(accounts[1].data.scanned_to_id, Some("test".to_string()));
    assert_eq!(accounts[2].data.scanned_to_id, Some("test".to_string()));
}
