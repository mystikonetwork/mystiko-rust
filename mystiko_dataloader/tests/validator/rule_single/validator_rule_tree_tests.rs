use crate::validator::common::{create_single_rule_full_data_validator, load_commitments, RuleCheckerType};
use ethers_core::types::Bytes;
use mystiko_config::MystikoConfig;
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::validator::rule::MerkleTreeCheckerError;
use mystiko_dataloader::validator::{DataValidator, ValidateOption};
use mystiko_protos::data::v1::CommitmentStatus;
use std::str::FromStr;

#[tokio::test]
async fn test_empty_commitment() {
    let (validator, _handler, _mock, _, _) = create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Tree]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder()
        .config(core_cfg)
        .validate_concurrency(1_usize)
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
    assert!(result.contract_results[0].result.is_ok());
}

#[tokio::test]
async fn test_some_included_commitment() {
    let (validator, handler, mock, _, _) = create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Tree]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder()
        .config(core_cfg)
        .validate_concurrency(1_usize)
        .build();
    let chain_id = 1_u64;
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let cms = load_commitments("./tests/files/validator/commitments_100.json").await;
    let mut fetched_cms = vec![];
    for cm in cms.iter().take(10) {
        fetched_cms.push(cm.clone());
        let mut included = cm.clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        fetched_cms.push(included);
    }
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(FullData::builder().commitments(fetched_cms).nullifiers(vec![]).build())
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();
    handler.add_commitments(vec![]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    println!("{:?}", result.contract_results[0].result);
    assert!(result.contract_results[0]
        .result
        .as_ref()
        .err()
        .unwrap()
        .to_string()
        .contains("empty responses array"));

    handler.add_commitments(cms[0..10].to_vec()).await;
    let root_know = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    mock.push::<Bytes, _>(root_know.clone()).unwrap();
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        MerkleTreeCheckerError::MerkleTreeRootNotKnownError.to_string()
    );

    handler.add_commitments(vec![]).await;
    let root_know = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    mock.push::<Bytes, _>(root_know.clone()).unwrap();
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());

    handler.add_commitments(cms[0..10].to_vec()).await;
    let root_know = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    mock.push::<Bytes, _>(root_know.clone()).unwrap();
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
}

#[tokio::test]
async fn test_some_included_handler_disorder_commitment() {
    let (validator, handler, mock, _, _) = create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Tree]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder()
        .config(core_cfg)
        .validate_concurrency(1_usize)
        .build();
    let chain_id = 1_u64;
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let cms = load_commitments("./tests/files/validator/commitments_100.json").await;
    let mut fetched_cms = vec![];
    for cm in cms.iter().take(10) {
        fetched_cms.push(cm.clone());
        let mut included = cm.clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        fetched_cms.push(included);
    }
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(FullData::builder().commitments(fetched_cms).nullifiers(vec![]).build())
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();

    let mut handler_cms = cms[0..10].to_vec();
    handler_cms.reverse();
    handler.add_commitments(handler_cms).await;
    let root_know = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    mock.push::<Bytes, _>(root_know.clone()).unwrap();
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());

    let mut handler_cms = cms[0..10].to_vec();
    handler_cms.sort_by(|a, b| a.commitment_hash.cmp(&b.commitment_hash));
    handler.add_commitments(handler_cms).await;
    let root_know = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    mock.push::<Bytes, _>(root_know.clone()).unwrap();
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
}
