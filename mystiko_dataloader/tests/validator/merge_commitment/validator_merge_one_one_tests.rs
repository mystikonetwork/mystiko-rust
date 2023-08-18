use crate::validator::common::validator_mock::{
    create_single_rule_full_data_validator, load_commitments, RuleCheckerType,
};
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::validator::rule::{SequenceCheckerError, ValidateCommitment, ValidateContractData};
use mystiko_dataloader::validator::{DataValidator, ValidateOption};
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_utils::convert::bytes_to_biguint;
use std::vec;

#[tokio::test]
async fn test_only_one_queued_no_included_commitment() {
    let (validator, handler, _mock, mock_rule_validator, mock_rule) =
        create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Sequence]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder().config(core_cfg).build();
    let chain_id = 1_u64;
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let cms = load_commitments("./tests/files/validator/commitments_100.json").await;
    let merged_cm = ValidateCommitment::builder()
        .commitment_hash(bytes_to_biguint(&cms[0].commitment_hash))
        .status(CommitmentStatus::Queued)
        .leaf_index(cms[0].leaf_index.unwrap())
        .inner_merge(false)
        .build();

    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(
            FullData::builder()
                .commitments(vec![cms[0].clone()])
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
    mock_rule_validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
    assert!(
        mock_rule
            .cmp_data(Some(
                &ValidateContractData::builder()
                    .chain_id(chain_id)
                    .contract_address(contract_address.to_string())
                    .start_block(1_u64)
                    .end_block(100_u64)
                    .commitments(vec![merged_cm])
                    .nullifiers(Some(vec![]))
                    .build()
            ))
            .await
    );
}

#[tokio::test]
async fn test_only_no_queued_one_included_commitment() {
    let (validator, handler, _mock, mock_rule_validator, mock_rule) =
        create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Sequence]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder().config(core_cfg).build();
    let chain_id = 1_u64;
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let cms = load_commitments("./tests/files/validator/commitments_100.json").await;
    let mut fetched_cms = cms[0].clone();
    fetched_cms.leaf_index = None;
    fetched_cms.status = CommitmentStatus::Included as i32;
    let merge_cm = ValidateCommitment::builder()
        .commitment_hash(bytes_to_biguint(&cms[0].commitment_hash))
        .status(CommitmentStatus::Included)
        .leaf_index(cms[0].leaf_index.unwrap())
        .inner_merge(false)
        .build();

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

    handler.add_commitments(vec![cms[0].clone()]).await;
    handler.add_commitments(vec![cms[0].clone()]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    mock_rule_validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
    assert!(
        mock_rule
            .cmp_data(Some(
                &ValidateContractData::builder()
                    .chain_id(chain_id)
                    .contract_address(contract_address.to_string())
                    .start_block(1_u64)
                    .end_block(100_u64)
                    .commitments(vec![merge_cm])
                    .nullifiers(Some(vec![]))
                    .build()
            ))
            .await
    );
}

#[tokio::test]
async fn test_only_one_queued_one_included_same_commitment() {
    let (validator, handler, _mock, mock_rule_validator, mock_rule) =
        create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Sequence]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder().config(core_cfg).build();
    let chain_id = 1_u64;
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let cms = load_commitments("./tests/files/validator/commitments_100.json").await;
    let mut fetched_cms = cms[0].clone();
    fetched_cms.leaf_index = None;
    fetched_cms.status = CommitmentStatus::Included as i32;
    let merge_cm = ValidateCommitment::builder()
        .commitment_hash(bytes_to_biguint(&cms[0].commitment_hash))
        .status(CommitmentStatus::Included)
        .leaf_index(cms[0].leaf_index.unwrap())
        .inner_merge(true)
        .build();
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(
            FullData::builder()
                .commitments(vec![fetched_cms.clone(), cms[0].clone()])
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
    mock_rule_validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
    assert!(
        mock_rule
            .cmp_data(Some(
                &ValidateContractData::builder()
                    .chain_id(chain_id)
                    .contract_address(contract_address.to_string())
                    .start_block(1_u64)
                    .end_block(100_u64)
                    .commitments(vec![merge_cm.clone()])
                    .nullifiers(Some(vec![]))
                    .build()
            ))
            .await
    );

    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(
            FullData::builder()
                .commitments(vec![cms[0].clone(), fetched_cms.clone()])
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
    mock_rule_validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
    assert!(
        mock_rule
            .cmp_data(Some(
                &ValidateContractData::builder()
                    .chain_id(chain_id)
                    .contract_address(contract_address.to_string())
                    .start_block(1_u64)
                    .end_block(100_u64)
                    .commitments(vec![merge_cm.clone()])
                    .nullifiers(Some(vec![]))
                    .build()
            ))
            .await
    );
}

#[tokio::test]
async fn test_only_one_queued_one_included_different_commitment() {
    let (validator, handler, _mock, mock_rule_validate, mock_rule) =
        create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Sequence]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder().config(core_cfg).build();
    let chain_id = 1_u64;
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let cms = load_commitments("./tests/files/validator/commitments_100.json").await;
    let mut fetched_cms = cms[0].clone();
    fetched_cms.leaf_index = None;
    fetched_cms.status = CommitmentStatus::Included as i32;
    let merged_cms = vec![
        ValidateCommitment::builder()
            .commitment_hash(bytes_to_biguint(&cms[0].commitment_hash))
            .status(CommitmentStatus::Included)
            .leaf_index(cms[0].leaf_index.unwrap())
            .inner_merge(false)
            .build(),
        ValidateCommitment::builder()
            .commitment_hash(bytes_to_biguint(&cms[1].commitment_hash))
            .status(CommitmentStatus::Queued)
            .leaf_index(cms[1].leaf_index.unwrap())
            .inner_merge(false)
            .build(),
    ];

    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(
            FullData::builder()
                .commitments(vec![fetched_cms.clone(), cms[1].clone()])
                .nullifiers(vec![])
                .build(),
        )
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();
    handler.add_commitments(vec![cms[0].clone()]).await;
    handler.add_commitments(vec![cms[0].clone()]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    let result2 = mock_rule_validate.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
    assert!(result2.contract_results[0].result.is_ok());
    assert!(
        mock_rule
            .cmp_data(Some(
                &ValidateContractData::builder()
                    .chain_id(chain_id)
                    .contract_address(contract_address.to_string())
                    .start_block(1_u64)
                    .end_block(100_u64)
                    .commitments(merged_cms.clone())
                    .nullifiers(Some(vec![]))
                    .build()
            ))
            .await
    );

    let mut fetched_cms = cms[1].clone();
    fetched_cms.leaf_index = None;
    fetched_cms.status = CommitmentStatus::Included as i32;
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(
            FullData::builder()
                .commitments(vec![fetched_cms.clone(), cms[0].clone()])
                .nullifiers(vec![])
                .build(),
        )
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();
    handler.add_commitments(vec![]).await;
    handler.add_commitments(vec![cms[1].clone()]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    mock_rule_validate.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        SequenceCheckerError::CommitmentStatusNotSequencedError.to_string()
    );
    assert!(mock_rule.cmp_data(None).await);
}
