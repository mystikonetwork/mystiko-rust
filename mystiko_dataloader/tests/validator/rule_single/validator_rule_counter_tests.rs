use crate::validator::common::{create_single_rule_full_data_validator, load_commitments, RuleCheckerType};
use ethers_core::types::Bytes;
use mystiko_config::MystikoConfig;
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::validator::rule::CounterCheckerError;
use mystiko_dataloader::validator::{DataValidator, ValidateOption};
use mystiko_protos::data::v1::CommitmentStatus;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

#[tokio::test]
async fn test_empty_commitment() {
    for concurrency in 1_usize..3_usize {
        empty_commitment(concurrency).await;
    }
}

async fn empty_commitment(concurrency: usize) {
    let (validator, handler, mock, _, _) = create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Counter]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder()
        .config(core_cfg)
        .validate_concurrency(concurrency)
        .build();
    let chain_id = 1_u64;
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(FullData::builder().commitments(vec![]).nullifiers(vec![]).build())
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0]
        .result
        .as_ref()
        .err()
        .unwrap()
        .to_string()
        .contains("empty responses array"));

    let include_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_commitments(vec![]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        CounterCheckerError::CommitmentIncludedCountMismatchError(0, 1).to_string()
    );

    let include_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_commitments(vec![]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
}

#[tokio::test]
async fn test_only_queued_commitment() {
    for concurrency in 1_usize..3_usize {
        only_queued_commitment(concurrency).await;
    }
}

async fn only_queued_commitment(concurrency: usize) {
    let (validator, handler, mock, _, _) = create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Counter]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder()
        .config(core_cfg)
        .validate_concurrency(concurrency)
        .build();
    let chain_id = 1_u64;
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let cms = load_commitments("./tests/files/validator/commitments_100.json").await;
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(FullData::builder().commitments(cms.clone()).nullifiers(vec![]).build())
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();
    let include_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_commitments(vec![]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
}

#[tokio::test]
async fn test_only_included_commitment() {
    for concurrency in 1_usize..3_usize {
        only_included_commitment(concurrency).await;
    }
}

async fn only_included_commitment(concurrency: usize) {
    let (validator, handler, mock, _, _) = create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Counter]));
    let core_cfg = Arc::new(
        MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
            .await
            .unwrap(),
    );
    let option = ValidateOption::builder()
        .config(core_cfg.clone())
        .validate_concurrency(concurrency)
        .build();
    let chain_id = 1_u64;
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let cms = load_commitments("./tests/files/validator/commitments_100.json").await;
    let mut fetched_cms = cms[0].clone();
    fetched_cms.leaf_index = None;
    fetched_cms.status = CommitmentStatus::Included as i32;
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(
            FullData::builder()
                .commitments(vec![fetched_cms])
                .nullifiers(vec![])
                .build(),
        )
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();
    let include_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_commitments(vec![cms[0].clone()]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        CounterCheckerError::CommitmentIncludedCountMismatchError(1, 0).to_string()
    );

    let mut skip_checkers = HashMap::new();
    skip_checkers.insert("counter".to_string(), false);
    let option = ValidateOption::builder()
        .config(core_cfg.clone())
        .validate_concurrency(concurrency)
        .skip_checkers(skip_checkers)
        .build();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_commitments(vec![cms[0].clone()]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        CounterCheckerError::CommitmentIncludedCountMismatchError(1, 0).to_string()
    );

    let mut skip_checkers = HashMap::new();
    skip_checkers.insert("counter".to_string(), true);
    let option = ValidateOption::builder()
        .config(core_cfg.clone())
        .validate_concurrency(concurrency)
        .skip_checkers(skip_checkers)
        .build();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_commitments(vec![cms[0].clone()]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());

    let include_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_commitments(vec![cms[0].clone()]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
}
