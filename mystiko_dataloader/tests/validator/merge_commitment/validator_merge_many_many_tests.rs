use crate::validator::common::{create_single_rule_full_data_validator, load_commitments, RuleCheckerType};
use mystiko_config::MystikoConfig;
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::validator::rule::{
    DataMergeError, SequenceCheckerError, ValidateCommitment, ValidateMergedData,
};
use mystiko_dataloader::validator::{DataValidator, ValidateOption};
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_utils::convert::bytes_to_biguint;

#[tokio::test]
async fn test_one_queued_many_included_same_commitment() {
    for concurrency in 1_usize..3_usize {
        one_queued_many_included_same_commitment(concurrency).await;
    }
}

async fn one_queued_many_included_same_commitment(concurrency: usize) {
    let (validator, handler, _mock, mock_checker_validator, mock_checker) =
        create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Sequence]));
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
    let mut fetched_cms = vec![];
    let mut merged_cms = vec![];
    for cm in cms.iter().take(10) {
        fetched_cms.push(cm.clone());
        let mut included = cm.clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        fetched_cms.push(included);

        merged_cms.push(
            ValidateCommitment::builder()
                .commitment_hash(bytes_to_biguint(&cm.commitment_hash))
                .status(CommitmentStatus::Included)
                .leaf_index(cm.leaf_index.unwrap())
                .inner_merge(true)
                .build(),
        )
    }
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
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
    handler.add_commitments(vec![]).await;
    handler.add_commitments(vec![]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    mock_checker_validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
    assert!(
        mock_checker
            .cmp_data(Some(
                &ValidateMergedData::builder()
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
}

#[tokio::test]
async fn test_many_queued_many_included_part_same_commitment() {
    let (validator, handler, _mock, mock_checker_validator, mock_checker) =
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
    let mut merged_cms = vec![];
    for i in 0..10 {
        fetched_cms.push(cms[i + 5].clone());
        let mut included = cms[i].clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        fetched_cms.push(included);
    }

    for cm in cms.iter().take(5) {
        merged_cms.push(
            ValidateCommitment::builder()
                .commitment_hash(bytes_to_biguint(&cm.commitment_hash))
                .status(CommitmentStatus::Included)
                .leaf_index(cm.leaf_index.unwrap())
                .inner_merge(false)
                .build(),
        )
    }

    for cm in cms.iter().take(10).skip(5) {
        merged_cms.push(
            ValidateCommitment::builder()
                .commitment_hash(bytes_to_biguint(&cm.commitment_hash))
                .status(CommitmentStatus::Included)
                .leaf_index(cm.leaf_index.unwrap())
                .inner_merge(true)
                .build(),
        )
    }

    for cm in cms.iter().take(15).skip(10) {
        merged_cms.push(
            ValidateCommitment::builder()
                .commitment_hash(bytes_to_biguint(&cm.commitment_hash))
                .status(CommitmentStatus::Queued)
                .leaf_index(cm.leaf_index.unwrap())
                .inner_merge(false)
                .build(),
        )
    }

    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
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
    handler.add_commitments(cms[0..5].to_vec()).await;
    handler.add_commitments(cms[0..5].to_vec()).await;
    handler.add_commitments(vec![]).await;
    handler.add_commitments(vec![]).await;
    handler.add_commitments(cms[0..5].to_vec()).await;
    let result = validator.validate(&data, &option).await.unwrap();
    mock_checker_validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
    assert!(
        mock_checker
            .cmp_data(Some(
                &ValidateMergedData::builder()
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

    let mut fetched_cms = vec![];
    for i in 0..10 {
        fetched_cms.push(cms[i].clone());
        let mut included = cms[i + 5].clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        fetched_cms.push(included);
    }

    for cm in merged_cms.iter_mut().take(5) {
        cm.status = CommitmentStatus::Queued;
    }

    for cm in merged_cms.iter_mut().take(15).skip(10) {
        cm.status = CommitmentStatus::Included;
    }

    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
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
    handler.add_commitments(cms[10..15].to_vec()).await;
    handler.add_commitments(cms[10..15].to_vec()).await;
    let result = validator.validate(&data, &option).await.unwrap();
    let result2 = mock_checker_validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        SequenceCheckerError::CommitmentStatusNotSequencedError.to_string()
    );
    assert!(result2.contract_results[0].result.is_ok());
    assert!(
        mock_checker
            .cmp_data(Some(
                &ValidateMergedData::builder()
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

    let mut fetched_cms = vec![];
    for i in 0..10 {
        fetched_cms.push(cms[i].clone());
        let mut included = cms[i + 5].clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        fetched_cms.push(included);
    }

    for cm in merged_cms.iter_mut().take(5) {
        cm.status = CommitmentStatus::Queued;
    }

    for cm in merged_cms.iter_mut().take(15).skip(10) {
        cm.status = CommitmentStatus::Included;
    }

    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
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
    handler.add_commitments(cms[10..15].to_vec()).await;
    handler.add_commitments(cms[10..15].to_vec()).await;
    let result = validator.validate(&data, &option).await.unwrap();
    let result2 = mock_checker_validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        SequenceCheckerError::CommitmentStatusNotSequencedError.to_string()
    );
    assert!(result2.contract_results[0].result.is_ok());
    assert!(
        mock_checker
            .cmp_data(Some(
                &ValidateMergedData::builder()
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

    let mut fetched_cms = vec![];
    let mut query_cms = vec![];
    let mut merged_cms = vec![];
    for cm in cms.iter().take(5) {
        let mut included = cm.clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        fetched_cms.push(included);
        merged_cms.push(
            ValidateCommitment::builder()
                .commitment_hash(bytes_to_biguint(&cm.commitment_hash))
                .status(CommitmentStatus::Included)
                .leaf_index(cm.leaf_index.unwrap())
                .inner_merge(false)
                .build(),
        );
        query_cms.push(cm.clone());
    }

    for cm in cms.iter().take(10).skip(5) {
        fetched_cms.push(cm.clone());
        merged_cms.push(
            ValidateCommitment::builder()
                .commitment_hash(bytes_to_biguint(&cm.commitment_hash))
                .status(CommitmentStatus::Queued)
                .leaf_index(cm.leaf_index.unwrap())
                .inner_merge(false)
                .build(),
        )
    }

    for cm in cms.iter().take(15).skip(10) {
        let mut included = cm.clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        fetched_cms.push(included);
        merged_cms.push(
            ValidateCommitment::builder()
                .commitment_hash(bytes_to_biguint(&cm.commitment_hash))
                .status(CommitmentStatus::Included)
                .leaf_index(cm.leaf_index.unwrap())
                .inner_merge(false)
                .build(),
        );
        query_cms.push(cm.clone());
    }

    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
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
    handler.add_commitments(query_cms.clone()).await;
    handler.add_commitments(query_cms).await;
    let result = validator.validate(&data, &option).await.unwrap();
    let result2 = mock_checker_validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        SequenceCheckerError::CommitmentStatusNotSequencedError.to_string()
    );
    assert!(result2.contract_results[0].result.is_ok());
    assert!(
        mock_checker
            .cmp_data(Some(
                &ValidateMergedData::builder()
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
}

#[tokio::test]
async fn test_many_queued_many_included_different_commitment() {
    let (validator, handler, _mock, mock_checker_validator, mock_checker) =
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
    let mut merged_cms = vec![];
    for i in 0..10 {
        fetched_cms.push(cms[i + 10].clone());
        let mut included = cms[i].clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        fetched_cms.push(included);

        merged_cms.push(
            ValidateCommitment::builder()
                .commitment_hash(bytes_to_biguint(&cms[i].commitment_hash))
                .status(CommitmentStatus::Included)
                .leaf_index(cms[i].leaf_index.unwrap())
                .inner_merge(false)
                .build(),
        );
        merged_cms.push(
            ValidateCommitment::builder()
                .commitment_hash(bytes_to_biguint(&cms[i + 10].commitment_hash))
                .status(CommitmentStatus::Queued)
                .leaf_index(cms[i + 10].leaf_index.unwrap())
                .inner_merge(false)
                .build(),
        );
    }
    merged_cms.sort_by(|a, b| a.leaf_index.cmp(&b.leaf_index));

    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
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
    handler.add_commitments(cms[0..10].to_vec()).await;
    handler.add_commitments(cms[0..10].to_vec()).await;
    handler.add_commitments(vec![]).await;
    handler.add_commitments(vec![]).await;
    handler.add_commitments(cms[0..10].to_vec()).await;
    let result = validator.validate(&data, &option).await.unwrap();
    let result2 = mock_checker_validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
    assert!(result2.contract_results[0].result.is_ok());
    assert!(
        mock_checker
            .cmp_data(Some(
                &ValidateMergedData::builder()
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

    let mut fetched_cms = vec![];
    for i in 0..10 {
        fetched_cms.push(cms[i].clone());
        let mut included = cms[i + 10].clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        fetched_cms.push(included);
    }
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
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
    handler.add_commitments(cms[0..10].to_vec()).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        DataMergeError::CommitmentHashMismatchError.to_string()
    );
}

#[tokio::test]
async fn test_many_queued_many_included_handler_disorder_commitment() {
    let (validator, handler, _mock, mock_checker_validator, mock_checker) =
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
    let mut merged_cms = vec![];
    for i in 0..10 {
        fetched_cms.push(cms[i + 10].clone());
        let mut included = cms[i].clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        fetched_cms.push(included);

        merged_cms.push(
            ValidateCommitment::builder()
                .commitment_hash(bytes_to_biguint(&cms[i].commitment_hash))
                .status(CommitmentStatus::Included)
                .leaf_index(cms[i].leaf_index.unwrap())
                .inner_merge(false)
                .build(),
        );
        merged_cms.push(
            ValidateCommitment::builder()
                .commitment_hash(bytes_to_biguint(&cms[i + 10].commitment_hash))
                .status(CommitmentStatus::Queued)
                .leaf_index(cms[i + 10].leaf_index.unwrap())
                .inner_merge(false)
                .build(),
        );
    }
    merged_cms.sort_by(|a, b| a.leaf_index.cmp(&b.leaf_index));

    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
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

    let mut handler_cms = cms[0..10].to_vec();
    handler_cms.sort_by(|a, b| a.commitment_hash.cmp(&b.commitment_hash));
    handler.add_commitments(handler_cms.clone()).await;
    handler.add_commitments(handler_cms.clone()).await;
    handler.add_commitments(vec![]).await;
    handler.add_commitments(vec![]).await;
    handler.add_commitments(handler_cms.clone()).await;
    let result = validator.validate(&data, &option).await.unwrap();
    let result2 = mock_checker_validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
    assert!(result2.contract_results[0].result.is_ok());
    assert!(
        mock_checker
            .cmp_data(Some(
                &ValidateMergedData::builder()
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

    let mut handler_cms = cms[0..10].to_vec();
    handler_cms.reverse();
    handler.add_commitments(handler_cms.clone()).await;
    handler.add_commitments(handler_cms.clone()).await;
    handler.add_commitments(vec![]).await;
    handler.add_commitments(vec![]).await;
    handler.add_commitments(handler_cms.clone()).await;
    let result = validator.validate(&data, &option).await.unwrap();
    let result2 = mock_checker_validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
    assert!(result2.contract_results[0].result.is_ok());
    assert!(
        mock_checker
            .cmp_data(Some(
                &ValidateMergedData::builder()
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
}

#[tokio::test]
async fn test_many_queued_many_included_leaf_index_error_commitment() {
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
        fetched_cms.push(cms[i + 10].clone());
        let mut included = cms[i].clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        fetched_cms.push(included);
    }
    fetched_cms.push(cms[10 + 10 + 2].clone());
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
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
    handler.add_commitments(cms[0..10].to_vec()).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        SequenceCheckerError::LeafIndexNotSequencedError(19, 22).to_string()
    );

    let mut fetched_cms = vec![];
    let mut cms1 = cms[0..10].to_vec();
    cms1[2].leaf_index = Some(100);
    for i in 0..10 {
        fetched_cms.push(cms[i + 10].clone());
        let mut included = cms[i].clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        fetched_cms.push(included);
    }
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
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
    handler.add_commitments(cms1).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        SequenceCheckerError::CommitmentStatusNotSequencedError.to_string()
    );

    let mut fetched_cms = vec![];
    let mut cms1 = cms[0..10].to_vec();
    cms1[2].leaf_index = None;
    for i in 0..10 {
        fetched_cms.push(cms[i + 10].clone());
        let mut included = cms[i].clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        fetched_cms.push(included);
    }
    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
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
    handler.add_commitments(cms1).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        DataMergeError::LeafIndexIsNoneError.to_string()
    );
}
