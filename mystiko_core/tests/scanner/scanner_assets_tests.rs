use crate::scanner::{create_scanner, insert_commitments};
use mystiko_core::{ScannerError, ScannerHandler};
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::scanner::v1::AssetsOptions;
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_utils::convert::decimal_to_number;
use num_bigint::BigUint;

#[tokio::test]
async fn test_assets_balance_with_default_option() {
    let account_count = 3_usize;
    let commitment_count = 20_usize;
    let (scanner, db, _) = create_scanner(account_count).await;
    let accounts = db.accounts.find_all().await.unwrap();
    let (mut cms, _) = insert_commitments(db.clone(), commitment_count, None).await;
    let option = AssetsOptions::builder().build();
    let result = scanner.assets(option.clone()).await.unwrap();
    assert!(result.is_empty());

    for (i, account) in accounts.iter().enumerate() {
        cms.iter_mut().skip(i * 2).take(2).for_each(|cm| {
            cm.data.nullifier = Some(BigUint::from(8_u8));
            cm.data.shielded_address = Some(account.data.shielded_address.clone());
        });
    }
    db.commitments.update_batch(&cms).await.unwrap();

    let option = AssetsOptions::builder().build();
    let result = scanner.assets(option.clone()).await.unwrap();
    assert!(result.is_empty());

    let mut amount_mtt = BigUint::default();
    let mut amount_eth = BigUint::default();
    for (i, account) in accounts.iter().enumerate() {
        cms.iter_mut().skip(i).take(1).for_each(|cm| {
            cm.data.status = CommitmentStatus::Included as i32;
            cm.data.contract_address = "0x5050F69a9786F081509234F1a7F4684b5E5b76C9".to_string();
            cm.data.asset_symbol = "MTT".to_string();
            cm.data.shielded_address = Some(account.data.shielded_address.clone());
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_mtt += cm_amount.clone();
            }
        });
        cms.iter_mut().skip(account_count + i).take(1).for_each(|cm| {
            cm.data.status = CommitmentStatus::Included as i32;
            cm.data.contract_address = "0x223903804Ee95e264F74C88B4F8583429524593c".to_string();
            cm.data.asset_symbol = "ETH".to_string();
            cm.data.shielded_address = Some(account.data.shielded_address.clone());
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_eth += cm_amount.clone();
            }
        });
        cms.iter_mut().skip(account_count * 2 + i).take(1).for_each(|cm| {
            cm.data.status = CommitmentStatus::SrcSucceeded as i32;
            cm.data.contract_address = "0x223903804Ee95e264F74C88B4F8583429524593c".to_string();
            cm.data.asset_symbol = "ETH".to_string();
            cm.data.shielded_address = Some(account.data.shielded_address.clone());
            cm.data.spent = false;
        });
    }
    db.commitments.update_batch(&cms).await.unwrap();

    let mut result = scanner.assets(option).await.unwrap();
    let balance_mtt = decimal_to_number::<f64, BigUint>(&amount_mtt, Some(18)).unwrap();
    let balance_eth = decimal_to_number::<f64, BigUint>(&amount_eth, Some(18)).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].chain_id, 5);
    assert_eq!(result[0].bridges.len(), 1);
    assert_eq!(result[0].bridges[0].bridge_type, BridgeType::Loop as i32);
    result[0].bridges[0].symbols.sort_by_key(|b| b.asset_symbol.clone());
    assert_eq!(result[0].bridges[0].symbols.len(), 2);
    assert_eq!(result[0].bridges[0].symbols[0].asset_symbol, "ETH");
    assert_eq!(result[0].bridges[0].symbols[0].versions.len(), 1);
    assert_eq!(result[0].bridges[0].symbols[0].versions[0].pool_version, 2);
    assert_eq!(
        result[0].bridges[0].symbols[0].versions[0].pool_name,
        "A Pool(since 07/20/2022)"
    );
    assert_eq!(result[0].bridges[0].symbols[0].versions[0].balance, balance_eth);
    assert_eq!(result[0].bridges[0].symbols[1].asset_symbol, "MTT");
    assert_eq!(result[0].bridges[0].symbols[1].versions.len(), 1);
    assert_eq!(result[0].bridges[0].symbols[1].versions[0].pool_version, 2);
    assert_eq!(
        result[0].bridges[0].symbols[1].versions[0].pool_name,
        "A Pool(since 07/20/2022)"
    );
    assert_eq!(result[0].bridges[0].symbols[1].versions[0].balance, balance_mtt);
}

#[tokio::test]
async fn test_assets_balance_with_option() {
    let account_count = 3_usize;
    let commitment_count = 10_usize;
    let (scanner, db, _) = create_scanner(account_count).await;
    let accounts = db.accounts.find_all().await.unwrap();
    let (mut cms, _) = insert_commitments(db.clone(), commitment_count, None).await;
    let option = AssetsOptions::builder()
        .shielded_addresses(vec![
            accounts[0].data.shielded_address.clone(),
            accounts[1].data.shielded_address.clone(),
        ])
        .build();
    let result = scanner.assets(option.clone()).await.unwrap();
    assert!(result.is_empty());

    for i in 0..account_count {
        cms.iter_mut().skip(i * 2).take(2).for_each(|cm| {
            cm.data.nullifier = Some(BigUint::from(8_u8));
            cm.data.shielded_address = Some(accounts[0].data.shielded_address.clone());
        });
    }
    db.commitments.update_batch(&cms).await.unwrap();

    let result = scanner.assets(option.clone()).await.unwrap();
    assert!(result.is_empty());

    let mut amount_mtt = BigUint::default();
    let mut amount_eth = BigUint::default();
    for i in 0..account_count {
        cms.iter_mut().skip(i).take(1).for_each(|cm| {
            cm.data.status = CommitmentStatus::Included as i32;
            cm.data.contract_address = "0x5050F69a9786F081509234F1a7F4684b5E5b76C9".to_string();
            cm.data.asset_symbol = "MTT".to_string();
            cm.data.shielded_address = Some(accounts[2].data.shielded_address.clone());
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_mtt += cm_amount.clone();
            }
        });
        cms.iter_mut().skip(account_count + i).take(1).for_each(|cm| {
            cm.data.status = CommitmentStatus::Included as i32;
            cm.data.contract_address = "0x223903804Ee95e264F74C88B4F8583429524593c".to_string();
            cm.data.asset_symbol = "ETH".to_string();
            cm.data.shielded_address = Some(accounts[1].data.shielded_address.clone());
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_eth += cm_amount.clone();
            }
        });
        cms.iter_mut().skip(account_count * 2 + i).take(1).for_each(|cm| {
            cm.data.status = CommitmentStatus::SrcSucceeded as i32;
            cm.data.contract_address = "0x223903804Ee95e264F74C88B4F8583429524593c".to_string();
            cm.data.asset_symbol = "ETH".to_string();
            cm.data.shielded_address = Some(accounts[1].data.shielded_address.clone());
        });
    }
    db.commitments.update_batch(&cms).await.unwrap();

    let mut result = scanner.assets(option.clone()).await.unwrap();
    let balance_eth = decimal_to_number::<f64, BigUint>(&amount_eth, Some(18)).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].chain_id, 5);
    assert_eq!(result[0].bridges.len(), 1);
    assert_eq!(result[0].bridges[0].bridge_type, BridgeType::Loop as i32);
    result[0].bridges[0].symbols.sort_by_key(|b| b.asset_symbol.clone());
    assert_eq!(result[0].bridges[0].symbols.len(), 1);
    assert_eq!(result[0].bridges[0].symbols[0].asset_symbol, "ETH");
    assert_eq!(result[0].bridges[0].symbols[0].versions.len(), 1);
    assert_eq!(result[0].bridges[0].symbols[0].versions[0].pool_version, 2);
    assert_eq!(
        result[0].bridges[0].symbols[0].versions[0].pool_name,
        "A Pool(since 07/20/2022)"
    );
    assert_eq!(result[0].bridges[0].symbols[0].versions[0].balance, balance_eth);
}

#[tokio::test]
async fn test_assets_balance_with_option_two_version() {
    let account_count = 3_usize;
    let commitment_count = 10_usize;
    let (scanner, db, _) = create_scanner(account_count).await;
    let accounts = db.accounts.find_all().await.unwrap();
    let (mut cms, _) = insert_commitments(db.clone(), commitment_count, None).await;
    let option = AssetsOptions::builder()
        .shielded_addresses(vec![
            accounts[0].data.shielded_address.clone(),
            accounts[1].data.shielded_address.clone(),
        ])
        .build();

    let mut amount_mtt_version1 = BigUint::default();
    let mut amount_mtt_version2 = BigUint::default();
    let mut amount_eth = BigUint::default();
    for i in 0..account_count {
        cms.iter_mut().skip(i).take(1).for_each(|cm| {
            cm.data.nullifier = Some(BigUint::from(8_u8));
            cm.data.status = CommitmentStatus::Included as i32;
            cm.data.contract_address = "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d".to_string();
            cm.data.asset_symbol = "MTT".to_string();
            cm.data.shielded_address = Some(accounts[0].data.shielded_address.clone());
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_mtt_version1 += cm_amount.clone();
            }
        });
        cms.iter_mut().skip(account_count + i).take(1).for_each(|cm| {
            cm.data.nullifier = Some(BigUint::from(8_u8));
            cm.data.status = CommitmentStatus::Included as i32;
            cm.data.contract_address = "0x223903804Ee95e264F74C88B4F8583429524593c".to_string();
            cm.data.asset_symbol = "MTT".to_string();
            cm.data.shielded_address = Some(accounts[0].data.shielded_address.clone());
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_mtt_version2 += cm_amount.clone();
            }
        });
        cms.iter_mut().skip(account_count * 2 + i).take(1).for_each(|cm| {
            cm.data.nullifier = Some(BigUint::from(8_u8));
            cm.data.status = CommitmentStatus::Included as i32;
            cm.data.contract_address = "0x5050F69a9786F081509234F1a7F4684b5E5b76C9".to_string();
            cm.data.asset_symbol = "ETH".to_string();
            cm.data.shielded_address = Some(accounts[1].data.shielded_address.clone());
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_eth += cm_amount.clone();
            }
        });
    }
    db.commitments.update_batch(&cms).await.unwrap();

    let mut result = scanner.assets(option).await.unwrap();
    let balance_mtt_version1 = decimal_to_number::<f64, BigUint>(&amount_mtt_version1, Some(18)).unwrap();
    let balance_mtt_version2 = decimal_to_number::<f64, BigUint>(&amount_mtt_version2, Some(18)).unwrap();
    let balance_eth = decimal_to_number::<f64, BigUint>(&amount_eth, Some(18)).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].chain_id, 5);
    assert_eq!(result[0].bridges.len(), 1);
    assert_eq!(result[0].bridges[0].bridge_type, BridgeType::Loop as i32);
    result[0].bridges[0].symbols.sort_by_key(|b| b.asset_symbol.clone());
    assert_eq!(result[0].bridges[0].symbols.len(), 2);
    assert_eq!(result[0].bridges[0].symbols[0].asset_symbol, "ETH");
    assert_eq!(result[0].bridges[0].symbols[0].versions.len(), 1);
    assert_eq!(result[0].bridges[0].symbols[0].versions[0].pool_version, 2);
    assert_eq!(
        result[0].bridges[0].symbols[0].versions[0].pool_name,
        "A Pool(since 07/20/2022)"
    );
    assert_eq!(result[0].bridges[0].symbols[0].versions[0].balance, balance_eth);
    result[0].bridges[0].symbols[1].versions.sort_by_key(|b| b.pool_version);
    assert_eq!(result[0].bridges[0].symbols[1].asset_symbol, "MTT");
    assert_eq!(result[0].bridges[0].symbols[1].versions[0].pool_version, 1);
    assert_eq!(
        result[0].bridges[0].symbols[1].versions[0].pool_name,
        "A Pool(since 07/20/2022)"
    );
    assert_eq!(
        result[0].bridges[0].symbols[1].versions[0].balance,
        balance_mtt_version1
    );
    assert_eq!(result[0].bridges[0].symbols[1].versions[1].pool_version, 2);
    assert_eq!(
        result[0].bridges[0].symbols[1].versions[1].pool_name,
        "A Pool(since 07/20/2022)"
    );
    assert_eq!(
        result[0].bridges[0].symbols[1].versions[1].balance,
        balance_mtt_version2
    );
}

#[tokio::test]
async fn test_assets_error() {
    let account_count = 3_usize;
    let commitment_count = 10_usize;
    let (scanner, db, _) = create_scanner(account_count).await;
    let accounts = db.accounts.find_all().await.unwrap();
    let (mut cms, _) = insert_commitments(db.clone(), commitment_count, None).await;
    for i in 0..account_count {
        cms.iter_mut().skip(i * 2).take(2).for_each(|cm| {
            cm.data.nullifier = Some(BigUint::from(8_u8));
            cm.data.shielded_address = Some(accounts[0].data.shielded_address.clone());
        });
    }
    db.commitments.update_batch(&cms).await.unwrap();
    let option = AssetsOptions::builder()
        .shielded_addresses(vec!["wrong_shielded_address".to_string()])
        .build();
    let result = scanner.assets(option).await;
    assert!(matches!(result.err().unwrap(), ScannerError::NoSuchAccountError));

    for i in 0..account_count {
        cms.iter_mut().skip(i * 2).take(2).for_each(|cm| {
            cm.data.nullifier = Some(BigUint::from(8_u8));
            cm.data.status = CommitmentStatus::Included as i32;
            cm.data.shielded_address = Some(accounts[0].data.shielded_address.clone());
        });
    }
    db.commitments.update_batch(&cms).await.unwrap();
    let option = AssetsOptions::builder().build();
    let result = scanner.assets(option).await;
    assert!(matches!(
        result.err().unwrap(),
        ScannerError::NoSuchContractConfigError(_, _)
    ));
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
    assert!(result.is_none());

    let mut amount_mtt = BigUint::default();
    let mut amount_eth = BigUint::default();
    for i in 0..account_count {
        cms.iter_mut().skip(i).take(1).for_each(|cm| {
            cm.data.status = CommitmentStatus::Queued as i32;
            cm.data.asset_symbol = "MTT".to_string();
            cm.data.status = CommitmentStatus::Included as i32;
            cm.data.contract_address = "0x223903804Ee95e264F74C88B4F8583429524593c".to_string();
            cm.data.shielded_address = Some(accounts[0].data.shielded_address.clone());
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_mtt += cm_amount.clone();
            }
        });
        cms.iter_mut().skip(account_count + i).take(1).for_each(|cm| {
            cm.data.status = CommitmentStatus::Queued as i32;
            cm.data.asset_symbol = "ETH".to_string();
            cm.data.status = CommitmentStatus::Included as i32;
            cm.data.contract_address = "0x5050F69a9786F081509234F1a7F4684b5E5b76C9".to_string();
            cm.data.shielded_address = Some(accounts[1].data.shielded_address.clone());
            if let Some(cm_amount) = cm.data.amount.as_ref() {
                amount_eth += cm_amount.clone();
            }
        });
    }
    db.commitments.update_batch(&cms).await.unwrap();

    let mut result = scanner.chain_assets(5, option).await.unwrap().unwrap();
    let balance_mtt = decimal_to_number::<f64, BigUint>(&amount_mtt, Some(18)).unwrap();
    let balance_eth = decimal_to_number::<f64, BigUint>(&amount_eth, Some(18)).unwrap();
    assert_eq!(result.chain_id, 5);
    assert_eq!(result.bridges.len(), 1);
    assert_eq!(result.bridges[0].bridge_type, BridgeType::Loop as i32);
    result.bridges[0].symbols.sort_by_key(|b| b.asset_symbol.clone());
    assert_eq!(result.bridges[0].symbols.len(), 2);
    result.bridges[0].symbols.sort_by_key(|b| b.asset_symbol.clone());
    assert_eq!(result.bridges[0].symbols[0].asset_symbol, "ETH");
    assert_eq!(result.bridges[0].symbols[0].versions.len(), 1);
    assert_eq!(result.bridges[0].symbols[0].versions[0].pool_version, 2);
    assert_eq!(
        result.bridges[0].symbols[0].versions[0].pool_name,
        "A Pool(since 07/20/2022)"
    );
    assert_eq!(result.bridges[0].symbols[0].versions[0].balance, balance_eth);
    assert_eq!(result.bridges[0].symbols[1].asset_symbol, "MTT");
    assert_eq!(result.bridges[0].symbols[0].versions.len(), 1);
    assert_eq!(result.bridges[0].symbols[1].versions[0].pool_version, 2);
    assert_eq!(
        result.bridges[0].symbols[1].versions[0].pool_name,
        "A Pool(since 07/20/2022)"
    );
    assert_eq!(result.bridges[0].symbols[1].versions[0].balance, balance_mtt);

    let option = AssetsOptions::builder()
        .shielded_addresses(vec![accounts[2].data.shielded_address.clone()])
        .build();
    let result = scanner.chain_assets(5, option).await.unwrap();
    assert!(result.is_none());

    let option = AssetsOptions::builder()
        .shielded_addresses(vec!["wrong_shielded_address".to_string()])
        .build();
    let result = scanner.chain_assets(5, option).await;
    assert!(matches!(result.err().unwrap(), ScannerError::NoSuchAccountError));
}
