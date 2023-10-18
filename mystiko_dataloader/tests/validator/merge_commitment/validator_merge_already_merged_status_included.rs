use crate::validator::common::{create_single_rule_full_data_validator, load_commitments, RuleCheckerType};
use mystiko_config::MystikoConfig;
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::validator::rule::DataMergeError;
use mystiko_dataloader::validator::rule::SequenceCheckerError;
use mystiko_dataloader::validator::{DataValidator, ValidateOption};
use mystiko_protos::data::v1::CommitmentStatus;

#[tokio::test]
async fn test_all_already_merged_commitment() {
    let (validator, handler, _mock, _, _) =
        create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Sequence]));
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
    for (i, cmo) in cms.iter().enumerate().take(10) {
        let mut cm = cmo.clone();
        cm.leaf_index = Some(i as u64);
        cm.included_block_number = Some(i as u64);
        cm.status = CommitmentStatus::Included as i32;
        cm.rollup_fee = Some(vec![1]);
        cm.encrypted_note = Some(vec![1]);
        cm.included_transaction_hash = Some(vec![1]);
        cm.queued_transaction_hash = Some(vec![1]);
        fetched_cms.push(cm.clone());
    }

    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(3000_u64)
        .end_block(4000_u64)
        .data(
            FullData::builder()
                .commitments(fetched_cms.clone())
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
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        DataMergeError::CommitmentDataLenError(10, 0).to_string()
    );

    handler.add_commitments(vec![]).await;
    handler.add_commitments(cms[0..10].to_vec()).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
}

#[tokio::test]
async fn test_some_already_merged_commitment() {
    let (validator, handler, _mock, _, _) =
        create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Sequence]));
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
    for i in 0..10 {
        let mut cm = cms[i].clone();
        cm.included_block_number = Some(i as u64);
        cm.status = CommitmentStatus::Included as i32;
        cm.rollup_fee = Some(vec![1]);
        cm.encrypted_note = Some(vec![1]);
        cm.included_transaction_hash = Some(vec![1]);
        cm.queued_transaction_hash = Some(vec![1]);
        fetched_cms.push(cms[10 + i].clone());
        fetched_cms.push(cm.clone());
    }

    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(3000_u64)
        .end_block(4000_u64)
        .data(
            FullData::builder()
                .commitments(fetched_cms.clone())
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
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        DataMergeError::CommitmentDataLenError(10, 0).to_string()
    );

    handler.add_commitments(vec![]).await;
    handler.add_commitments(vec![]).await;
    handler.add_commitments(cms[0..10].to_vec()).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        SequenceCheckerError::CommitmentNotSequenceWithHandlerError(2, 0, 10).to_string()
    );

    handler.add_commitments(cms[0..10].to_vec()).await;
    handler.add_commitments(vec![]).await;
    handler.add_commitments(cms[0..10].to_vec()).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
}

#[tokio::test]
async fn test_some_already_merged_commitment_some_not() {
    let (validator, handler, _mock, _, _) =
        create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Sequence]));
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
    for i in 0..5 {
        let mut cm = cms[i].clone();
        cm.block_number = 10 + i as u64;
        cm.included_block_number = Some(i as u64);
        cm.status = CommitmentStatus::Included as i32;
        cm.rollup_fee = Some(vec![1]);
        cm.encrypted_note = Some(vec![1]);
        cm.included_transaction_hash = Some(vec![1]);
        cm.queued_transaction_hash = Some(vec![1]);
        fetched_cms.push(cm.clone());
    }

    for i in 5..10 {
        let mut cm = cms[i].clone();
        cm.block_number = 3000 + i as u64;
        cm.included_block_number = Some(i as u64);
        cm.status = CommitmentStatus::Included as i32;
        cm.rollup_fee = Some(vec![1]);
        cm.encrypted_note = Some(vec![1]);
        cm.included_transaction_hash = Some(vec![1]);
        cm.queued_transaction_hash = Some(vec![1]);
        fetched_cms.push(cm.clone());
        fetched_cms.push(cms[i + 5].clone());
    }

    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(3000_u64)
        .end_block(4000_u64)
        .data(
            FullData::builder()
                .commitments(fetched_cms.clone())
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
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        DataMergeError::CommitmentDataLenError(5, 0).to_string()
    );

    handler.add_commitments(vec![]).await;
    handler.add_commitments(cms[5..10].to_vec()).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        DataMergeError::CommitmentHashMismatchError.to_string()
    );

    handler.add_commitments(vec![]).await;
    handler.add_commitments(cms[0..5].to_vec()).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        SequenceCheckerError::CommitmentNotSequenceWithHandlerError(2, 0, 5).to_string()
    );

    handler.add_commitments(cms[0..5].to_vec()).await;
    handler.add_commitments(vec![]).await;
    handler.add_commitments(cms[0..5].to_vec()).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
}

#[tokio::test]
async fn test_some_already_merged_commitment_some_not_wrong_sequence() {
    let (validator, handler, _mock, _, _) =
        create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Sequence]));
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
    for i in 0..5 {
        let mut cm = cms[i].clone();
        cm.block_number = 3000 + i as u64;
        cm.included_block_number = Some(i as u64);
        cm.status = CommitmentStatus::Included as i32;
        cm.rollup_fee = Some(vec![1]);
        cm.encrypted_note = Some(vec![1]);
        cm.included_transaction_hash = Some(vec![1]);
        cm.queued_transaction_hash = Some(vec![1]);
        fetched_cms.push(cm.clone());
    }

    for i in 5..10 {
        let mut cm = cms[i].clone();
        cm.block_number = 10 + i as u64;
        cm.included_block_number = Some(i as u64);
        cm.status = CommitmentStatus::Included as i32;
        cm.rollup_fee = Some(vec![1]);
        cm.encrypted_note = Some(vec![1]);
        cm.included_transaction_hash = Some(vec![1]);
        cm.queued_transaction_hash = Some(vec![1]);
        fetched_cms.push(cm.clone());
        fetched_cms.push(cms[i + 5].clone());
    }

    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(3000_u64)
        .end_block(4000_u64)
        .data(
            FullData::builder()
                .commitments(fetched_cms.clone())
                .nullifiers(vec![])
                .build(),
        )
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();

    handler.add_commitments(cms[5..10].to_vec()).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        SequenceCheckerError::CommitmentMergedNotSequencedError.to_string()
    );
}
