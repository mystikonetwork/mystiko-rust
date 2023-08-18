use crate::validator::common::validator_mock::{
    create_single_rule_full_data_validator, load_commitments, RuleCheckerType,
};
use ethers_core::rand;
use ethers_core::rand::Rng;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::validator::rule::{DataMergeError, ValidateCommitment, ValidateContractData};
use mystiko_dataloader::validator::{DataValidator, ValidateOption};
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_utils::convert::bytes_to_biguint;

#[tokio::test]
async fn test_only_queued_duplicate_commitment() {
    let (validator, handler, _mock, mock_rule_validator, mock_rule) =
        create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Sequence]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder().config(core_cfg).build();
    let chain_id = 1_u64;
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let cms1 = load_commitments("./tests/files/validator/commitments_100.json").await;
    let mut fetched_cms = load_commitments("./tests/files/validator/commitments_999.json").await;
    let merged_cms = fetched_cms
        .iter()
        .map(|cm| {
            ValidateCommitment::builder()
                .commitment_hash(bytes_to_biguint(&cm.commitment_hash))
                .status(CommitmentStatus::Queued)
                .leaf_index(cm.leaf_index.unwrap())
                .inner_merge(false)
                .build()
        })
        .collect::<Vec<_>>();
    fetched_cms.append(&mut cms1.clone());
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
                    .commitments(merged_cms.clone())
                    .nullifiers(Some(vec![]))
                    .build()
            ))
            .await
    );

    fetched_cms.extend(cms1.clone());
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
                    .commitments(merged_cms)
                    .nullifiers(Some(vec![]))
                    .build()
            ))
            .await
    );

    let mut cms2 = cms1.clone();
    cms2[10].leaf_index = Some(1000_u64);
    fetched_cms.extend(cms2);
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
    let result = validator.validate(&data, &option).await.unwrap();
    mock_rule_validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        DataMergeError::LeafIndexMismatchError(10, 1000).to_string()
    );
    assert!(mock_rule.cmp_data(None).await);
}

#[tokio::test]
async fn test_only_included_duplicate_commitment() {
    let (validator, handler, _mock, mock_rule_validator, mock_rule) =
        create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Sequence]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder().config(core_cfg).build();
    let chain_id = 1_u64;
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let cms = load_commitments("./tests/files/validator/commitments_100.json").await;
    let mut fetched_cms = vec![];
    let mut merged_cms = vec![];
    for cm in cms.iter().take(10) {
        let mut included = cm.clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        fetched_cms.push(included.clone());
        fetched_cms.push(included);

        merged_cms.push(
            ValidateCommitment::builder()
                .commitment_hash(bytes_to_biguint(&cm.commitment_hash))
                .status(CommitmentStatus::Included)
                .leaf_index(cm.leaf_index.unwrap())
                .inner_merge(false)
                .build(),
        );
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
    handler.add_commitments(cms[0..10].to_vec()).await;
    handler.add_commitments(cms[0..10].to_vec()).await;
    let result = validator.validate(&data, &option).await.unwrap();
    let mock_rule_result = mock_rule_validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());
    assert!(mock_rule_result.contract_results[0].result.is_ok());
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
}

#[tokio::test]
async fn test_queued_duplicate_leaf_index_mismatch() {
    let (validator, handler, _mock, mock_rule_validator, mock_rule) =
        create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Sequence]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder().config(core_cfg).build();
    let chain_id = 1_u64;
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let cms = load_commitments("./tests/files/validator/commitments_100.json").await;
    let mut fetched_cms = vec![];
    for cm in cms.iter().take(10) {
        let mut cm = cm.clone();
        fetched_cms.push(cm.clone());
        cm.leaf_index = Some(1000_u64);
        fetched_cms.push(cm);
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
    mock_rule_validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        DataMergeError::LeafIndexMismatchError(0, 1000).to_string()
    );
    assert!(mock_rule.cmp_data(None).await);
}

#[tokio::test]
async fn test_queued_and_included_duplicate() {
    let (validator, handler, _mock, mock_rule_validator, mock_rule) =
        create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Sequence]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder().config(core_cfg).build();
    let chain_id = 1_u64;
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let cms = load_commitments("./tests/files/validator/commitments_100.json").await;
    for j in 0..4 {
        let mut fetched_cms = vec![];
        let mut merged_cms = vec![];
        for cm in cms.iter().take(10) {
            let mut included = cm.clone();
            included.leaf_index = None;
            included.status = CommitmentStatus::Included as i32;

            let push_count = rand::thread_rng().gen_range(1..=j + 1);
            for _ in 0..push_count {
                let push_included = rand::thread_rng().gen_bool(0.5);
                if push_included {
                    fetched_cms.push(included.clone());
                } else {
                    fetched_cms.push(cm.clone());
                }
            }
            fetched_cms.push(cm.clone());
            fetched_cms.push(included.clone());

            merged_cms.push(
                ValidateCommitment::builder()
                    .commitment_hash(bytes_to_biguint(&cm.commitment_hash))
                    .status(CommitmentStatus::Included)
                    .leaf_index(cm.leaf_index.unwrap())
                    .inner_merge(true)
                    .build(),
            );
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
        let result = validator.validate(&data, &option).await.unwrap();
        let mock_rule_result = mock_rule_validator.validate(&data, &option).await.unwrap();
        assert_eq!(result.chain_id, chain_id);
        assert_eq!(result.contract_results.len(), 1);
        assert_eq!(result.contract_results[0].address, contract_address);
        assert!(result.contract_results[0].result.is_ok());
        assert!(mock_rule_result.contract_results[0].result.is_ok());
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
    }

    let mut fetched_cms = vec![];
    for cm in cms.iter().take(10) {
        let mut cm = cm.clone();
        fetched_cms.push(cm.clone());
        let mut included = cm.clone();
        included.leaf_index = None;
        included.status = CommitmentStatus::Included as i32;
        fetched_cms.push(included.clone());
        cm.leaf_index = Some(1000_u64);
        fetched_cms.push(cm);
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
    mock_rule_validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        DataMergeError::LeafIndexMismatchError(0, 1000).to_string()
    );
    assert!(mock_rule.cmp_data(None).await);
}
