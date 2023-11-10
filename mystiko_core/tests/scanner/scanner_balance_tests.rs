use crate::scanner::{create_scanner, insert_commitments};
use mystiko_core::ScannerHandler;
use mystiko_protos::core::scanner::v1::BalanceOptions;
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_utils::convert::decimal_to_number;
use num_bigint::BigUint;

#[tokio::test]
async fn test_scan_pending_balance_default() {
    let account_count = 3_usize;
    let commitment_count = 10_usize;
    let (scanner, db, _) = create_scanner(account_count).await;
    let accounts = db.accounts.find_all().await.unwrap();
    let option = BalanceOptions::builder().build();
    let (mut cms, _) = insert_commitments(db.clone(), commitment_count, None).await;
    let mut amount = BigUint::from(0_u8);
    for i in 0..account_count {
        cms.iter_mut().skip(i * 2).take(2).for_each(|cm| {
            cm.data.nullifier = Some(BigUint::from(8_u8));
            cm.data.shielded_address = Some(accounts[0].data.shielded_address.clone());
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount += cm_amount.clone();
            }
        });
    }
    db.commitments.update_batch(&cms).await.unwrap();

    let result = scanner.balance(option.clone()).await.unwrap();
    let pending = decimal_to_number::<f64, BigUint>(&amount, Some(18)).unwrap();
    assert_eq!(result.balances.len(), 1);
    assert_eq!(result.balances[0].asset_symbol, "MTT");
    assert_eq!(result.balances[0].pending, pending);
    assert_eq!(result.balances[0].unspent, 0.0);
    assert_eq!(result.balances[0].spent, None);

    let mut amount_mtt = BigUint::default();
    let mut amount_eth = BigUint::default();
    for i in 0..account_count {
        cms.iter_mut().skip(i).take(1).for_each(|cm| {
            cm.data.status = CommitmentStatus::Queued as i32;
            cm.data.asset_symbol = "MTT".to_string();
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_mtt += cm_amount.clone();
            }
        });
        cms.iter_mut().skip(account_count + i).take(1).for_each(|cm| {
            cm.data.status = CommitmentStatus::Queued as i32;
            cm.data.asset_symbol = "ETH".to_string();
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_eth += cm_amount.clone();
            }
        });
    }
    db.commitments.update_batch(&cms).await.unwrap();

    let mut result = scanner.balance(option).await.unwrap();
    let pending_mtt = decimal_to_number::<f64, BigUint>(&amount_mtt, Some(18)).unwrap();
    let pending_eth = decimal_to_number::<f64, BigUint>(&amount_eth, Some(18)).unwrap();
    result.balances.sort_by_key(|b| b.asset_symbol.clone());
    assert_eq!(result.balances.len(), 2);
    assert_eq!(result.balances[0].asset_symbol, "ETH");
    assert_eq!(result.balances[0].pending, pending_eth);
    assert_eq!(result.balances[0].unspent, 0.0);
    assert_eq!(result.balances[0].spent, None);
    assert_eq!(result.balances[1].asset_symbol, "MTT");
    assert_eq!(result.balances[1].pending, pending_mtt);
    assert_eq!(result.balances[1].unspent, 0.0);
    assert_eq!(result.balances[1].spent, None);
}

#[tokio::test]
async fn test_scan_options_with_spend() {
    let account_count = 3_usize;
    let commitment_count = 12_usize;
    let (scanner, db, _) = create_scanner(account_count).await;
    let accounts = db.accounts.find_all().await.unwrap();
    let option = BalanceOptions::builder().with_spent(true).build();
    let (mut cms, _) = insert_commitments(db.clone(), commitment_count, None).await;
    let mut amount = BigUint::default();
    let mut spend_amount = BigUint::default();
    for i in 0..account_count {
        cms.iter_mut().skip(i * 2).take(2).for_each(|cm| {
            cm.data.nullifier = Some(BigUint::from(8_u8));
            cm.data.shielded_address = Some(accounts[0].data.shielded_address.clone());
            cm.data.status = CommitmentStatus::Included as i32;
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount += cm_amount.clone();
            }
        });
        cms.iter_mut().skip(account_count * 2 + i * 2).take(2).for_each(|cm| {
            cm.data.nullifier = Some(BigUint::from(8_u8));
            cm.data.shielded_address = Some(accounts[0].data.shielded_address.clone());
            cm.data.status = CommitmentStatus::Included as i32;
            cm.data.spent = true;
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                spend_amount += cm_amount.clone();
            }
        });
    }
    db.commitments.update_batch(&cms).await.unwrap();

    let result = scanner.balance(option.clone()).await.unwrap();
    let unspent = decimal_to_number::<f64, BigUint>(&amount, Some(18)).unwrap();
    let spend = decimal_to_number::<f64, BigUint>(&spend_amount, Some(18)).unwrap();
    assert_eq!(result.balances.len(), 1);
    assert_eq!(result.balances[0].asset_symbol, "MTT");
    assert_eq!(result.balances[0].pending, 0.0);
    assert_eq!(result.balances[0].unspent, unspent);
    assert_eq!(result.balances[0].spent, Some(spend));

    let mut amount_mtt = BigUint::default();
    let mut amount_eth = BigUint::default();
    for i in 0..account_count {
        cms.iter_mut().skip(i).take(1).for_each(|cm| {
            cm.data.asset_symbol = "MTT".to_string();
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_mtt += cm_amount.clone();
            }
        });
        cms.iter_mut().skip(account_count + i).take(1).for_each(|cm| {
            cm.data.asset_symbol = "ETH".to_string();
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_eth += cm_amount.clone();
            }
        });
    }
    db.commitments.update_batch(&cms).await.unwrap();

    let mut result = scanner.balance(option).await.unwrap();
    let unspent_mtt = decimal_to_number::<f64, BigUint>(&amount_mtt, Some(18)).unwrap();
    let unspent_eth = decimal_to_number::<f64, BigUint>(&amount_eth, Some(18)).unwrap();
    assert_eq!(result.balances.len(), 2);
    result.balances.sort_by_key(|b| b.asset_symbol.clone());
    assert_eq!(result.balances[0].asset_symbol, "ETH");
    assert_eq!(result.balances[0].pending, 0.0);
    assert_eq!(result.balances[0].unspent, unspent_eth);
    assert_eq!(result.balances[0].spent, Some(0.0));
    assert_eq!(result.balances[1].asset_symbol, "MTT");
    assert_eq!(result.balances[1].pending, 0.0);
    assert_eq!(result.balances[1].unspent, unspent_mtt);
    assert_eq!(result.balances[1].spent, Some(spend));
}

#[tokio::test]
async fn test_scan_options_with_asset() {
    let account_count = 3_usize;
    let commitment_count = 12_usize;
    let option = BalanceOptions::builder().asset_symbols(vec!["MTT".to_string()]).build();
    let (scanner, db, _) = create_scanner(account_count).await;
    let accounts = db.accounts.find_all().await.unwrap();
    let (mut cms, _) = insert_commitments(db.clone(), commitment_count, None).await;

    let mut amount_mtt = BigUint::default();
    let mut amount_eth = BigUint::default();
    for i in 0..account_count {
        cms.iter_mut().skip(i).take(1).for_each(|cm| {
            cm.data.nullifier = Some(BigUint::from(8_u8));
            cm.data.shielded_address = Some(accounts[0].data.shielded_address.clone());
            cm.data.status = CommitmentStatus::Included as i32;
            cm.data.asset_symbol = "MTT".to_string();
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_mtt += cm_amount.clone();
            }
        });
        cms.iter_mut().skip(account_count + i).take(1).for_each(|cm| {
            cm.data.nullifier = Some(BigUint::from(8_u8));
            cm.data.shielded_address = Some(accounts[0].data.shielded_address.clone());
            cm.data.status = CommitmentStatus::Included as i32;
            cm.data.asset_symbol = "ETH".to_string();
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_eth += cm_amount.clone();
            }
        });
    }
    db.commitments.update_batch(&cms).await.unwrap();

    let result = scanner.balance(option).await.unwrap();
    let unspent_mtt = decimal_to_number::<f64, BigUint>(&amount_mtt, Some(18)).unwrap();
    assert_eq!(result.balances.len(), 1);
    assert_eq!(result.balances[0].asset_symbol, "MTT");
    assert_eq!(result.balances[0].pending, 0.0);
    assert_eq!(result.balances[0].unspent, unspent_mtt);
    assert_eq!(result.balances[0].spent, None);
}

#[tokio::test]
async fn test_scan_options_with_chain_id() {
    let account_count = 3_usize;
    let commitment_count = 12_usize;
    let option = BalanceOptions::builder().chain_ids(vec![12345_u64]).build();
    let (scanner, db, _) = create_scanner(account_count).await;
    let accounts = db.accounts.find_all().await.unwrap();
    let (mut cms, _) = insert_commitments(db.clone(), commitment_count, None).await;

    let mut amount_mtt = BigUint::default();
    let mut amount_eth = BigUint::default();
    for i in 0..account_count {
        cms.iter_mut().skip(i).take(1).for_each(|cm| {
            cm.data.nullifier = Some(BigUint::from(8_u8));
            cm.data.shielded_address = Some(accounts[0].data.shielded_address.clone());
            cm.data.status = CommitmentStatus::Included as i32;
            cm.data.asset_symbol = "MTT".to_string();
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_mtt += cm_amount.clone();
            }
        });
        cms.iter_mut().skip(account_count + i).take(1).for_each(|cm| {
            cm.data.nullifier = Some(BigUint::from(8_u8));
            cm.data.shielded_address = Some(accounts[0].data.shielded_address.clone());
            cm.data.status = CommitmentStatus::Included as i32;
            cm.data.asset_symbol = "ETH".to_string();
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_eth += cm_amount.clone();
            }
        });
    }
    db.commitments.update_batch(&cms).await.unwrap();
    let result = scanner.balance(option).await.unwrap();
    assert!(result.balances.is_empty());
}

#[tokio::test]
async fn test_scan_options_with_contract_address() {
    let account_count = 3_usize;
    let commitment_count = 12_usize;
    let option = BalanceOptions::builder()
        .contract_addresses(vec!["wrong_address".to_string()])
        .build();
    let (scanner, db, _) = create_scanner(account_count).await;
    let accounts = db.accounts.find_all().await.unwrap();
    let (mut cms, _) = insert_commitments(db.clone(), commitment_count, None).await;

    let mut amount_mtt = BigUint::default();
    let mut amount_eth = BigUint::default();
    for i in 0..account_count {
        cms.iter_mut().skip(i).take(1).for_each(|cm| {
            cm.data.nullifier = Some(BigUint::from(8_u8));
            cm.data.shielded_address = Some(accounts[0].data.shielded_address.clone());
            cm.data.status = CommitmentStatus::Included as i32;
            cm.data.asset_symbol = "MTT".to_string();
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_mtt += cm_amount.clone();
            }
        });
        cms.iter_mut().skip(account_count + i).take(1).for_each(|cm| {
            cm.data.nullifier = Some(BigUint::from(8_u8));
            cm.data.shielded_address = Some(accounts[0].data.shielded_address.clone());
            cm.data.status = CommitmentStatus::Included as i32;
            cm.data.asset_symbol = "ETH".to_string();
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_eth += cm_amount.clone();
            }
        });
    }
    db.commitments.update_batch(&cms).await.unwrap();
    let result = scanner.balance(option).await.unwrap();
    assert!(result.balances.is_empty());
}

#[tokio::test]
async fn test_scan_options_with_bridge_type() {
    let account_count = 3_usize;
    let commitment_count = 12_usize;
    let option = BalanceOptions::builder().bridge_types(vec![123456]).build();
    let (scanner, db, _) = create_scanner(account_count).await;
    let accounts = db.accounts.find_all().await.unwrap();
    let (mut cms, _) = insert_commitments(db.clone(), commitment_count, None).await;

    let mut amount_mtt = BigUint::default();
    let mut amount_eth = BigUint::default();
    for i in 0..account_count {
        cms.iter_mut().skip(i).take(1).for_each(|cm| {
            cm.data.nullifier = Some(BigUint::from(8_u8));
            cm.data.shielded_address = Some(accounts[0].data.shielded_address.clone());
            cm.data.status = CommitmentStatus::Included as i32;
            cm.data.asset_symbol = "MTT".to_string();
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_mtt += cm_amount.clone();
            }
        });
        cms.iter_mut().skip(account_count + i).take(1).for_each(|cm| {
            cm.data.nullifier = Some(BigUint::from(8_u8));
            cm.data.shielded_address = Some(accounts[0].data.shielded_address.clone());
            cm.data.status = CommitmentStatus::Included as i32;
            cm.data.asset_symbol = "ETH".to_string();
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_eth += cm_amount.clone();
            }
        });
    }
    db.commitments.update_batch(&cms).await.unwrap();
    let result = scanner.balance(option).await.unwrap();
    assert!(result.balances.is_empty());
}

#[tokio::test]
async fn test_scan_options_with_shielded_address() {
    let account_count = 3_usize;
    let commitment_count = 12_usize;
    let option = BalanceOptions::builder()
        .shielded_addresses(vec!["wrong address".to_string()])
        .build();
    let (scanner, db, _) = create_scanner(account_count).await;
    let accounts = db.accounts.find_all().await.unwrap();
    let (mut cms, _) = insert_commitments(db.clone(), commitment_count, None).await;

    let mut amount_mtt = BigUint::default();
    let mut amount_eth = BigUint::default();
    for i in 0..account_count {
        cms.iter_mut().skip(i).take(1).for_each(|cm| {
            cm.data.nullifier = Some(BigUint::from(8_u8));
            cm.data.shielded_address = Some(accounts[0].data.shielded_address.clone());
            cm.data.status = CommitmentStatus::Included as i32;
            cm.data.asset_symbol = "MTT".to_string();
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_mtt += cm_amount.clone();
            }
        });
        cms.iter_mut().skip(account_count + i).take(1).for_each(|cm| {
            cm.data.nullifier = Some(BigUint::from(8_u8));
            cm.data.shielded_address = Some(accounts[0].data.shielded_address.clone());
            cm.data.status = CommitmentStatus::Included as i32;
            cm.data.asset_symbol = "ETH".to_string();
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_eth += cm_amount.clone();
            }
        });
    }
    db.commitments.update_batch(&cms).await.unwrap();
    let result = scanner.balance(option).await.unwrap();
    assert!(result.balances.is_empty());
}
