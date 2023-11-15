use crate::scanner::{create_scanner, insert_commitments};
use mystiko_core::{ScannerError, ScannerHandler};
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::scanner::v1::AssetsOptions;
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_utils::convert::decimal_to_number;
use num_bigint::BigUint;

#[tokio::test]
async fn test_assets() {
    let account_count = 3_usize;
    let commitment_count = 10_usize;
    let (scanner, db, _) = create_scanner(account_count).await;
    let accounts = db.accounts.find_all().await.unwrap();
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

    let option = AssetsOptions::builder().build();
    let result = scanner.assets(option.clone()).await.unwrap();
    let pending = decimal_to_number::<f64, BigUint>(&amount, Some(18)).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].chain_id, 5);
    assert_eq!(result[0].bridges.len(), 1);
    assert_eq!(result[0].bridges[0].bridge_type, BridgeType::Loop as i32);
    assert_eq!(result[0].bridges[0].balances.len(), 1);
    assert_eq!(result[0].bridges[0].balances[0].asset_symbol, "MTT");
    assert_eq!(result[0].bridges[0].balances[0].pending, pending);
    assert_eq!(result[0].bridges[0].balances[0].unspent, 0.0);
    assert_eq!(result[0].bridges[0].balances[0].spent, None);

    let mut amount_mtt = BigUint::default();
    let mut amount_eth = BigUint::default();
    for i in 0..account_count {
        cms.iter_mut().skip(i).take(1).for_each(|cm| {
            cm.data.status = CommitmentStatus::Queued as i32;
            cm.data.asset_symbol = "MTT".to_string();
            cm.data.shielded_address = Some(accounts[1].data.shielded_address.clone());
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_mtt += cm_amount.clone();
            }
        });
        cms.iter_mut().skip(account_count + i).take(1).for_each(|cm| {
            cm.data.status = CommitmentStatus::Queued as i32;
            cm.data.asset_symbol = "ETH".to_string();
            cm.data.shielded_address = Some(accounts[2].data.shielded_address.clone());
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_eth += cm_amount.clone();
            }
        });
    }
    db.commitments.update_batch(&cms).await.unwrap();

    let mut result = scanner.assets(option).await.unwrap();
    let pending_mtt = decimal_to_number::<f64, BigUint>(&amount_mtt, Some(18)).unwrap();
    let pending_eth = decimal_to_number::<f64, BigUint>(&amount_eth, Some(18)).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].chain_id, 5);
    assert_eq!(result[0].bridges.len(), 1);
    assert_eq!(result[0].bridges[0].bridge_type, BridgeType::Loop as i32);
    result[0].bridges[0].balances.sort_by_key(|b| b.asset_symbol.clone());
    assert_eq!(result[0].bridges[0].balances.len(), 2);
    assert_eq!(result[0].bridges[0].balances[0].asset_symbol, "ETH");
    assert_eq!(result[0].bridges[0].balances[0].pending, pending_eth);
    assert_eq!(result[0].bridges[0].balances[0].unspent, 0.0);
    assert_eq!(result[0].bridges[0].balances[0].spent, None);
    assert_eq!(result[0].bridges[0].balances[1].asset_symbol, "MTT");
    assert_eq!(result[0].bridges[0].balances[1].pending, pending_mtt);
    assert_eq!(result[0].bridges[0].balances[1].unspent, 0.0);
    assert_eq!(result[0].bridges[0].balances[1].spent, None);

    let option = AssetsOptions::builder()
        .shielded_addresses(vec![
            accounts[0].data.shielded_address.clone(),
            accounts[1].data.shielded_address.clone(),
        ])
        .build();
    let result = scanner.assets(option).await.unwrap();
    assert_eq!(result[0].chain_id, 5);
    assert_eq!(result[0].bridges.len(), 1);
    assert_eq!(result[0].bridges[0].bridge_type, BridgeType::Loop as i32);
    assert_eq!(result[0].bridges[0].balances[0].asset_symbol, "MTT");
    assert_eq!(result[0].bridges[0].balances[0].pending, pending_mtt);
    assert_eq!(result[0].bridges[0].balances[0].unspent, 0.0);
    assert_eq!(result[0].bridges[0].balances[0].spent, None);

    let option = AssetsOptions::builder()
        .shielded_addresses(vec!["wrong_shielded_address".to_string()])
        .build();
    let result = scanner.assets(option).await;
    assert!(matches!(result.err().unwrap(), ScannerError::NoSuchAccountError));
}

#[tokio::test]
async fn test_chain_assets() {
    let account_count = 3_usize;
    let commitment_count = 10_usize;
    let (scanner, db, _) = create_scanner(account_count).await;
    let accounts = db.accounts.find_all().await.unwrap();
    let (mut cms, _) = insert_commitments(db.clone(), commitment_count, None).await;
    let mut amount = BigUint::from(0_u8);
    for i in 0..account_count {
        cms.iter_mut().skip(i * 2).take(2).for_each(|cm| {
            cm.data.nullifier = Some(BigUint::from(8_u8));
            cm.data.shielded_address = Some(accounts[2].data.shielded_address.clone());
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount += cm_amount.clone();
            }
        });
    }
    db.commitments.update_batch(&cms).await.unwrap();

    let option = AssetsOptions::builder().build();
    let result = scanner.chain_assets(5, option.clone()).await.unwrap();
    let pending = decimal_to_number::<f64, BigUint>(&amount, Some(18)).unwrap();
    assert_eq!(result.chain_id, 5);
    assert_eq!(result.bridges.len(), 1);
    assert_eq!(result.bridges[0].bridge_type, BridgeType::Loop as i32);
    assert_eq!(result.bridges[0].balances.len(), 1);
    assert_eq!(result.bridges[0].balances[0].asset_symbol, "MTT");
    assert_eq!(result.bridges[0].balances[0].pending, pending);
    assert_eq!(result.bridges[0].balances[0].unspent, 0.0);
    assert_eq!(result.bridges[0].balances[0].spent, None);

    let mut amount_mtt = BigUint::default();
    let mut amount_eth = BigUint::default();
    for i in 0..account_count {
        cms.iter_mut().skip(i).take(1).for_each(|cm| {
            cm.data.status = CommitmentStatus::Queued as i32;
            cm.data.asset_symbol = "MTT".to_string();
            cm.data.shielded_address = Some(accounts[0].data.shielded_address.clone());
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_mtt += cm_amount.clone();
            }
        });
        cms.iter_mut().skip(account_count + i).take(1).for_each(|cm| {
            cm.data.status = CommitmentStatus::Queued as i32;
            cm.data.asset_symbol = "ETH".to_string();
            cm.data.shielded_address = Some(accounts[1].data.shielded_address.clone());
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_eth += cm_amount.clone();
            }
        });
    }
    db.commitments.update_batch(&cms).await.unwrap();

    let mut result = scanner.chain_assets(5, option).await.unwrap();
    let pending_mtt = decimal_to_number::<f64, BigUint>(&amount_mtt, Some(18)).unwrap();
    let pending_eth = decimal_to_number::<f64, BigUint>(&amount_eth, Some(18)).unwrap();
    assert_eq!(result.chain_id, 5);
    assert_eq!(result.bridges.len(), 1);
    assert_eq!(result.bridges[0].bridge_type, BridgeType::Loop as i32);
    result.bridges[0].balances.sort_by_key(|b| b.asset_symbol.clone());
    assert_eq!(result.bridges[0].balances.len(), 2);
    assert_eq!(result.bridges[0].balances[0].asset_symbol, "ETH");
    assert_eq!(result.bridges[0].balances[0].pending, pending_eth);
    assert_eq!(result.bridges[0].balances[0].unspent, 0.0);
    assert_eq!(result.bridges[0].balances[0].spent, None);
    assert_eq!(result.bridges[0].balances[1].asset_symbol, "MTT");
    assert_eq!(result.bridges[0].balances[1].pending, pending_mtt);
    assert_eq!(result.bridges[0].balances[1].unspent, 0.0);
    assert_eq!(result.bridges[0].balances[1].spent, None);

    let option = AssetsOptions::builder()
        .shielded_addresses(vec![accounts[2].data.shielded_address.clone()])
        .build();
    let result = scanner.chain_assets(5, option).await.unwrap();
    assert_eq!(result.chain_id, 5);
    assert_eq!(result.bridges.len(), 0);

    let option = AssetsOptions::builder()
        .shielded_addresses(vec!["wrong_shielded_address".to_string()])
        .build();
    let result = scanner.chain_assets(5, option).await;
    assert!(matches!(result.err().unwrap(), ScannerError::NoSuchAccountError));
}
