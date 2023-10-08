use crate::validator::common::{create_single_rule_lite_data_validator, load_commitments, RuleCheckerType};
use mystiko_config::MystikoConfig;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::data::{ChainData, LiteData};
use mystiko_dataloader::validator::rule::SequenceCheckerError;
use mystiko_dataloader::validator::{DataValidator, ValidateOption};
use mystiko_protos::data::v1::CommitmentStatus;

#[tokio::test]
async fn test_one_queued_many_included_same_commitment() {
    for concurrency in 1_usize..3_usize {
        one_queued_many_included_same_commitment(concurrency).await;
    }
}

async fn one_queued_many_included_same_commitment(concurrency: usize) {
    let (validator, handler, _mock) = create_single_rule_lite_data_validator(Some(vec![RuleCheckerType::Sequence]));
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
    let cms1 = cms[1..10].to_vec();
    let mut fetched_cms = vec![];
    for cm in cms.iter().take(10) {
        let mut included = cm.clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        fetched_cms.push(included);
    }
    fetched_cms.push(cms[0].clone());
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(LiteData::builder().commitments(fetched_cms.clone()).build())
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();
    handler.add_commitments(cms1.clone()).await;
    handler.add_commitments(vec![]).await;
    handler.add_commitments(cms1).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        SequenceCheckerError::CommitmentMergedNotSequencedError.to_string()
    );

    let _ = fetched_cms.pop().unwrap();
    fetched_cms.push(cms[5].clone());
    let mut cms2 = cms[0..5].to_vec();
    cms2.extend(cms[6..10].to_vec());
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(LiteData::builder().commitments(fetched_cms.clone()).build())
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();
    handler.add_commitments(vec![]).await;
    handler.add_commitments(cms2).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        SequenceCheckerError::CommitmentMergedNotSequencedError.to_string()
    );

    let _ = fetched_cms.pop().unwrap();
    fetched_cms.push(cms[9].clone());
    let cms3 = cms[0..9].to_vec();
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
    handler.add_commitments(cms3.clone()).await;
    handler.add_commitments(vec![]).await;
    handler.add_commitments(cms3).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
}

#[tokio::test]
async fn test_many_queued_one_included_different_commitment() {
    let (validator, handler, _mock) = create_single_rule_lite_data_validator(Some(vec![RuleCheckerType::Sequence]));
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
    let mut fetched_cms = cms[0].clone();
    fetched_cms.leaf_index = None;
    fetched_cms.status = CommitmentStatus::Included as i32;
    let mut cms1 = cms[1..10].to_vec();
    cms1.push(fetched_cms);
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(LiteData::builder().commitments(cms1).build())
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();
    handler.add_commitments(vec![cms[0].clone()]).await;
    handler.add_commitments(vec![]).await;
    handler.add_commitments(vec![cms[0].clone()]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());

    let mut fetched_cms = cms[10].clone();
    fetched_cms.leaf_index = None;
    fetched_cms.status = CommitmentStatus::Included as i32;
    let mut cms2 = cms[0..10].to_vec();
    cms2.push(fetched_cms);
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(LiteData::builder().commitments(cms2).build())
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();
    handler.add_commitments(vec![]).await;
    handler.add_commitments(vec![cms[10].clone()]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        SequenceCheckerError::CommitmentStatusNotSequencedError.to_string()
    );
}
