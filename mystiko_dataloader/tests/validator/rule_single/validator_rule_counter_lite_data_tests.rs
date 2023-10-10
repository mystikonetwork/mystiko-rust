use crate::validator::common::create_single_rule_lite_data_validator;
use crate::validator::common::{load_commitments, RuleCheckerType};
use ethers_core::types::Bytes;
use mystiko_config::MystikoConfig;
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::data::LiteData;
use mystiko_dataloader::validator::rule::CounterCheckerError;
use mystiko_dataloader::validator::{DataValidator, ValidateOption};
use mystiko_protos::data::v1::CommitmentStatus;
use std::str::FromStr;

#[tokio::test]
async fn test_empty_commitment() {
    for concurrency in 1_usize..3_usize {
        empty_commitment(concurrency).await;
    }
}

async fn empty_commitment(concurrency: usize) {
    let (validator, handler, mock) = create_single_rule_lite_data_validator(Some(vec![RuleCheckerType::Counter]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder()
        .config(core_cfg)
        .validate_concurrency(concurrency)
        .build();
    let chain_id = 1_u64;
    let contract_address = "0x62121886c954d7e23077f52217b51c26ad26bE9e";
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(LiteData::builder().commitments(vec![]).build())
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
    assert!(result.contract_results[0]
        .result
        .as_ref()
        .err()
        .unwrap()
        .to_string()
        .contains("empty responses array"));

    let total_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    mock.push::<Bytes, _>(total_count.clone()).unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_commitments(vec![]).await;
    handler.add_commitments(vec![]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        CounterCheckerError::CommitmentCountMismatchError(0, 1).to_string()
    );

    let total_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    mock.push::<Bytes, _>(total_count.clone()).unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_commitments(vec![]).await;
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
    let (validator, handler, mock) = create_single_rule_lite_data_validator(Some(vec![RuleCheckerType::Counter]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder()
        .config(core_cfg)
        .validate_concurrency(concurrency)
        .build();
    let chain_id = 1_u64;
    let contract_address = "0x62121886c954d7e23077f52217b51c26ad26bE9e";
    let cms = load_commitments("./tests/files/validator/commitments_100.json").await;
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(LiteData::builder().commitments(cms.clone()).build())
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();

    let include_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    let total_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    mock.push::<Bytes, _>(total_count.clone()).unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_commitments(vec![]).await;
    handler.add_commitments(vec![]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        CounterCheckerError::CommitmentCountMismatchError(100, 0).to_string()
    );

    let total_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000064").unwrap();
    mock.push::<Bytes, _>(total_count.clone()).unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_commitments(vec![]).await;
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
    let (validator, handler, mock) = create_single_rule_lite_data_validator(Some(vec![RuleCheckerType::Counter]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder()
        .config(core_cfg)
        .validate_concurrency(concurrency)
        .build();
    let chain_id = 1_u64;
    let contract_address = "0x62121886c954d7e23077f52217b51c26ad26bE9e";
    let cms = load_commitments("./tests/files/validator/commitments_100.json").await;
    let mut fetched_cms = cms[0].clone();
    fetched_cms.leaf_index = None;
    fetched_cms.status = CommitmentStatus::Included as i32;
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(LiteData::builder().commitments(vec![fetched_cms]).build())
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

    let include_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_commitments(vec![cms[0].clone()]).await;
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

    let total = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    let include_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    mock.push::<Bytes, _>(total.clone()).unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_commitments(vec![cms[0].clone()]).await;
    handler.add_commitments(vec![cms[0].clone()]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());

    let total = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000002").unwrap();
    let include_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    mock.push::<Bytes, _>(total.clone()).unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler
        .add_commitments(vec![cms[0].clone(), cms[1].clone(), cms[2].clone()])
        .await;
    handler.add_commitments(vec![cms[0].clone()]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        CounterCheckerError::CommitmentCountMismatchError(3, 2).to_string()
    );

    let total = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000002").unwrap();
    let include_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    mock.push::<Bytes, _>(total.clone()).unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_commitments(vec![cms[0].clone(), cms[1].clone()]).await;
    handler.add_commitments(vec![cms[0].clone()]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
}

#[tokio::test]
async fn test_queued_and_included_all_inner_merge_commitment() {
    let (validator, handler, mock) = create_single_rule_lite_data_validator(Some(vec![RuleCheckerType::Counter]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder()
        .config(core_cfg)
        .validate_concurrency(1_usize)
        .build();
    let chain_id = 1_u64;
    let contract_address = "0x62121886c954d7e23077f52217b51c26ad26bE9e";
    let cms: Vec<mystiko_protos::data::v1::Commitment> =
        load_commitments("./tests/files/validator/commitments_100.json").await;
    let mut fetched_cms = vec![];
    for cm in cms.iter().take(3) {
        fetched_cms.push(cm.clone());
        let mut cm = cm.clone();
        cm.leaf_index = None;
        cm.status = CommitmentStatus::Included as i32;
        fetched_cms.push(cm);
    }

    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(LiteData::builder().commitments(fetched_cms).build())
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();
    let total = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000003").unwrap();
    let include_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000003").unwrap();
    mock.push::<Bytes, _>(total.clone()).unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_commitments(vec![]).await;
    handler.add_commitments(vec![]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
}

#[tokio::test]
async fn test_queued_and_included_some_inner_merge_commitment() {
    let (validator, handler, mock) = create_single_rule_lite_data_validator(Some(vec![RuleCheckerType::Counter]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder()
        .config(core_cfg)
        .validate_concurrency(1_usize)
        .build();
    let chain_id = 1_u64;
    let contract_address = "0x62121886c954d7e23077f52217b51c26ad26bE9e";
    let cms: Vec<mystiko_protos::data::v1::Commitment> =
        load_commitments("./tests/files/validator/commitments_100.json").await;
    let mut fetched_cms = vec![];
    for i in 0..3 {
        let mut cm = cms[i].clone();
        cm.leaf_index = None;
        cm.status = CommitmentStatus::Included as i32;
        fetched_cms.push(cm);
        fetched_cms.push(cms[i].clone());
        fetched_cms.push(cms[i + 3].clone());
    }

    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(LiteData::builder().commitments(fetched_cms).build())
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();
    let total = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000004").unwrap();
    let include_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000003").unwrap();
    mock.push::<Bytes, _>(total.clone()).unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_commitments(vec![]).await;
    handler.add_commitments(vec![]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        CounterCheckerError::CommitmentCountMismatchError(6, 4).to_string()
    );

    let total = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000006").unwrap();
    let include_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000003").unwrap();
    mock.push::<Bytes, _>(total.clone()).unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_commitments(vec![]).await;
    handler.add_commitments(vec![]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
}
