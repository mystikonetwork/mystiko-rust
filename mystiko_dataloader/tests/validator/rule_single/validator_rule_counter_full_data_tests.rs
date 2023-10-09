use crate::validator::common::load_nullifiers;
use crate::validator::common::{create_single_rule_full_data_validator, load_commitments, RuleCheckerType};
use ethers_core::types::Bytes;
use mystiko_config::MystikoConfig;
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::validator::rule::CounterCheckerError;
use mystiko_dataloader::validator::{DataValidator, ValidateOption};
use mystiko_protos::data::v1::CommitmentStatus;
use std::str::FromStr;

#[tokio::test]
async fn test_full_data_empty_commitment() {
    for concurrency in 1_usize..3_usize {
        full_data_empty_commitment(concurrency).await;
    }
}

async fn full_data_empty_commitment(concurrency: usize) {
    let (validator, handler, mock, _, _) = create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Counter]));
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
        .data(FullData::builder().commitments(vec![]).build())
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

    let total_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    mock.push::<Bytes, _>(total_count.clone()).unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_commitments(vec![]).await;
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

    let nullifier_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    mock.push::<Bytes, _>(nullifier_count.clone()).unwrap();
    mock.push::<Bytes, _>(total_count.clone()).unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_nullifiers(vec![]).await;
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
    let (validator, handler, mock, _, _) = create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Counter]));
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
        .data(FullData::builder().commitments(cms.clone()).nullifiers(vec![]).build())
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();
    let nullifier_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    let total_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000064").unwrap();
    let include_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    mock.push::<Bytes, _>(nullifier_count.clone()).unwrap();
    mock.push::<Bytes, _>(total_count.clone()).unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_nullifiers(vec![]).await;
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
    let (validator, handler, mock, _, _) = create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Counter]));
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

    let nullifier_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    let total_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    let include_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    mock.push::<Bytes, _>(nullifier_count.clone()).unwrap();
    mock.push::<Bytes, _>(total_count.clone()).unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_nullifiers(vec![]).await;
    handler.add_commitments(vec![cms[0].clone()]).await;
    handler.add_commitments(vec![cms[0].clone()]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
}

#[tokio::test]
async fn test_only_nullifier() {
    for concurrency in 1_usize..3_usize {
        only_nullifier(concurrency).await;
    }
}

async fn only_nullifier(concurrency: usize) {
    let (validator, handler, mock, _, _) = create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Counter]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder()
        .config(core_cfg)
        .validate_concurrency(concurrency)
        .build();
    let chain_id = 1_u64;
    let contract_address = "0x62121886c954d7e23077f52217b51c26ad26bE9e";
    let nullifiers = load_nullifiers("./tests/files/validator/nullifiers_10.json").await;
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(
            FullData::builder()
                .commitments(vec![])
                .nullifiers(nullifiers.clone())
                .build(),
        )
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();

    let total_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    let include_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    mock.push::<Bytes, _>(total_count.clone()).unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_commitments(vec![]).await;
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

    let nullifier_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    mock.push::<Bytes, _>(nullifier_count.clone()).unwrap();
    mock.push::<Bytes, _>(total_count.clone()).unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_nullifiers([nullifiers[0].clone()].to_vec()).await;
    handler.add_commitments(vec![]).await;
    handler.add_commitments(vec![]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        CounterCheckerError::NullifierCountMismatchError(11, 1).to_string()
    );

    let nullifier_count = Bytes::from_str("000000000000000000000000000000000000000000000000000000000000000b").unwrap();
    mock.push::<Bytes, _>(nullifier_count.clone()).unwrap();
    mock.push::<Bytes, _>(total_count.clone()).unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    handler.add_nullifiers([nullifiers[0].clone()].to_vec()).await;
    handler.add_commitments(vec![]).await;
    handler.add_commitments(vec![]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
}

#[tokio::test]
async fn test_skip_disabled_contract() {
    let (validator, handler, mock, _, _) = create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Counter]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder()
        .config(core_cfg)
        .validate_concurrency(1_usize)
        .build();
    let chain_id = 1_u64;
    let contract_address = "0x4333Ab5682Cd4589330BD9CFcedd5E054A0e9d8D";
    let nullifiers = load_nullifiers("./tests/files/validator/nullifiers_10.json").await;
    let cms = load_commitments("./tests/files/validator/commitments_100.json").await;

    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(16692001_u64)
        .end_block(16693000_u64)
        .data(
            FullData::builder()
                .commitments(cms)
                .nullifiers(nullifiers.clone())
                .build(),
        )
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();

    let nullifier_count = Bytes::from_str("000000000000000000000000000000000000000000000000000000000000000b").unwrap();
    mock.push::<Bytes, _>(nullifier_count.clone()).unwrap();
    handler.add_commitments(vec![]).await;
    handler.add_nullifiers([nullifiers[0].clone()].to_vec()).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        CounterCheckerError::CommitmentAfterContractDisabledError.to_string()
    );

    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(16692001_u64)
        .end_block(16693000_u64)
        .data(
            FullData::builder()
                .commitments(vec![])
                .nullifiers(nullifiers.clone())
                .build(),
        )
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();

    mock.push::<Bytes, _>(nullifier_count.clone()).unwrap();
    handler.add_nullifiers([nullifiers[0].clone()].to_vec()).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
}
