use crate::validator::validator_mock::{create_full_data_validator, load_commitments};
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::data::chain::ChainData;
use mystiko_dataloader::data::contract::ContractData;
use mystiko_dataloader::data::types::FullData;
use mystiko_dataloader::validator::rule::RuleCheckerType;
use mystiko_dataloader::validator::{DataValidator, ValidateOption};
use mystiko_protos::data::v1::CommitmentStatus;

#[tokio::test]
async fn test_one_queued_many_included_same_commitment() {
    let (validator, handler, _mock) = create_full_data_validator(Some(vec![RuleCheckerType::Sequence]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder().config(core_cfg).build();
    let chain_id = 1_u64;
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let cms = load_commitments("./tests/files/validator/commitments_100.json").await;
    let mut included_cms = vec![];
    for cm in cms.iter().take(10) {
        included_cms.push(cm.clone());
        let mut included = cm.clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        included_cms.push(included);
    }
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(
            FullData::builder()
                .commitments(included_cms.clone())
                .nullifiers(vec![])
                .build(),
        )
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
    assert!(result.contract_results[0].result.is_ok());
}

#[tokio::test]
async fn test_many_queued_many_included_part_same_commitment() {
    let (validator, handler, _mock) = create_full_data_validator(Some(vec![RuleCheckerType::Sequence]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder().config(core_cfg).build();
    let chain_id = 1_u64;
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let cms = load_commitments("./tests/files/validator/commitments_100.json").await;
    let mut included_cms = vec![];
    for i in 0..10 {
        included_cms.push(cms[i + 5].clone());
        let mut included = cms[i].clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        included_cms.push(included);
    }
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(
            FullData::builder()
                .commitments(included_cms.clone())
                .nullifiers(vec![])
                .build(),
        )
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();
    handler.add_commitments(vec![]).await;
    handler.add_commitments(cms[0..5].to_vec()).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());

    let mut included_cms = vec![];
    for i in 0..10 {
        included_cms.push(cms[i].clone());
        let mut included = cms[i + 5].clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        included_cms.push(included);
    }
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(
            FullData::builder()
                .commitments(included_cms.clone())
                .nullifiers(vec![])
                .build(),
        )
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();
    handler.add_commitments(vec![]).await;
    handler.add_commitments(cms[10..15].to_vec()).await;
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
        .contains("commitment status not all queued"));
}

#[tokio::test]
async fn test_many_queued_many_included_different_commitment() {
    let (validator, handler, _mock) = create_full_data_validator(Some(vec![RuleCheckerType::Sequence]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder().config(core_cfg).build();
    let chain_id = 1_u64;
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let cms = load_commitments("./tests/files/validator/commitments_100.json").await;
    let mut included_cms = vec![];
    for i in 0..10 {
        included_cms.push(cms[i + 10].clone());
        let mut included = cms[i].clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        included_cms.push(included);
    }
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(
            FullData::builder()
                .commitments(included_cms.clone())
                .nullifiers(vec![])
                .build(),
        )
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();
    handler.add_commitments(vec![]).await;
    handler.add_commitments(cms[0..10].to_vec()).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());

    let mut included_cms = vec![];
    for i in 0..10 {
        included_cms.push(cms[i].clone());
        let mut included = cms[i + 10].clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        included_cms.push(included);
    }
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(
            FullData::builder()
                .commitments(included_cms.clone())
                .nullifiers(vec![])
                .build(),
        )
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();
    handler.add_commitments(vec![]).await;
    handler.add_commitments(cms[0..10].to_vec()).await;
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
        .contains("query commitment hash mismatch"));
}

#[tokio::test]
async fn test_many_queued_many_included_leaf_index_error_commitment() {
    let (validator, handler, _mock) = create_full_data_validator(Some(vec![RuleCheckerType::Sequence]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder().config(core_cfg).build();
    let chain_id = 1_u64;
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let cms = load_commitments("./tests/files/validator/commitments_100.json").await;
    let mut included_cms = vec![];
    for i in 0..10 {
        included_cms.push(cms[i + 10].clone());
        let mut included = cms[i].clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        included_cms.push(included);
    }
    included_cms.push(cms[10 + 10 + 2].clone());
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(
            FullData::builder()
                .commitments(included_cms.clone())
                .nullifiers(vec![])
                .build(),
        )
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();
    handler.add_commitments(vec![]).await;
    handler.add_commitments(cms[0..10].to_vec()).await;
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
        .contains("leaf index values not sequence"));

    let mut included_cms = vec![];
    let mut cms1 = cms[0..10].to_vec();
    cms1[2].leaf_index = Some(100);
    for i in 0..10 {
        included_cms.push(cms[i + 10].clone());
        let mut included = cms[i].clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        included_cms.push(included);
    }
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(
            FullData::builder()
                .commitments(included_cms.clone())
                .nullifiers(vec![])
                .build(),
        )
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();
    handler.add_commitments(vec![]).await;
    handler.add_commitments(cms1).await;
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
        .contains("leaf index values not sequence"));

    let mut included_cms = vec![];
    let mut cms1 = cms[0..10].to_vec();
    cms1[2].leaf_index = None;
    for i in 0..10 {
        included_cms.push(cms[i + 10].clone());
        let mut included = cms[i].clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        included_cms.push(included);
    }
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(
            FullData::builder()
                .commitments(included_cms.clone())
                .nullifiers(vec![])
                .build(),
        )
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();
    handler.add_commitments(vec![]).await;
    handler.add_commitments(cms1).await;
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
        .contains("query commitment leaf index is none"));
}
