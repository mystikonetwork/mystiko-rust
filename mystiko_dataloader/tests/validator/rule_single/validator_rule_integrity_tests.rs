use crate::validator::common::validator_mock::{
    create_single_rule_full_data_validator, create_single_rule_lite_data_validator, RuleCheckerType,
};
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::data::{ChainData, LiteData};
use mystiko_dataloader::validator::{DataValidator, ValidateOption};
use mystiko_protos::data::v1::{Commitment, CommitmentStatus};

#[tokio::test]
async fn test_empty_commitment() {
    let (validator, _handler, _mock, _, _) =
        create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Integrity]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder().config(core_cfg).build();
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
async fn test_check_commitment_full_data() {
    let (validator, handler, _mock, _, _) =
        create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Integrity]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder().config(core_cfg).build();
    let chain_id = 1_u64;
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let cm = Commitment {
        commitment_hash: vec![
            17, 214, 48, 198, 85, 198, 51, 72, 44, 121, 28, 13, 120, 224, 26, 20, 17, 156, 251, 233, 119, 14, 62, 130,
            72, 235, 26, 209, 81, 231, 116, 185,
        ],
        status: CommitmentStatus::Queued as i32,
        block_number: 1,
        included_block_number: Some(1),
        src_chain_block_number: Some(1),
        leaf_index: Some(1),
        rollup_fee: Some(vec![1, 2, 3]),
        encrypted_note: Some(vec![1, 2, 3]),
        queued_transaction_hash: Some(vec![1, 2, 3]),
        included_transaction_hash: Some(vec![1, 2, 3]),
        src_chain_transaction_hash: Some(vec![1, 2, 3]),
    };

    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(
            FullData::builder()
                .commitments(vec![cm.clone()])
                .nullifiers(vec![])
                .build(),
        )
        .build();
    let mut data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());

    data.contracts_data[0].data.as_mut().unwrap().commitments[0].status = CommitmentStatus::Queued as i32;
    data.contracts_data[0].data.as_mut().unwrap().commitments[0].leaf_index = None;
    let result = validator.validate(&data, &option).await.unwrap();
    assert!(result.contract_results[0].result.is_err());

    data.contracts_data[0].data.as_mut().unwrap().commitments[0].leaf_index = Some(1);
    data.contracts_data[0].data.as_mut().unwrap().commitments[0].rollup_fee = None;
    let result = validator.validate(&data, &option).await.unwrap();
    assert!(result.contract_results[0].result.is_err());

    data.contracts_data[0].data.as_mut().unwrap().commitments[0].rollup_fee = Some(vec![1, 2, 3]);
    data.contracts_data[0].data.as_mut().unwrap().commitments[0].encrypted_note = None;
    let result = validator.validate(&data, &option).await.unwrap();
    assert!(result.contract_results[0].result.is_err());

    data.contracts_data[0].data.as_mut().unwrap().commitments[0].encrypted_note = Some(vec![1, 2, 3]);
    data.contracts_data[0].data.as_mut().unwrap().commitments[0].queued_transaction_hash = None;
    let result = validator.validate(&data, &option).await.unwrap();
    assert!(result.contract_results[0].result.is_err());
    data.contracts_data[0].data.as_mut().unwrap().commitments[0].queued_transaction_hash = Some(vec![1, 2, 3]);

    data.contracts_data[0].data.as_mut().unwrap().commitments[0].status = CommitmentStatus::Included as i32;
    handler.add_commitments(vec![cm.clone()]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert!(result.contract_results[0].result.is_ok());

    data.contracts_data[0].data.as_mut().unwrap().commitments[0].status = CommitmentStatus::Included as i32;
    data.contracts_data[0].data.as_mut().unwrap().commitments[0].included_block_number = None;
    handler.add_commitments(vec![cm.clone()]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert!(result.contract_results[0].result.is_err());

    data.contracts_data[0].data.as_mut().unwrap().commitments[0].included_block_number = Some(1);
    data.contracts_data[0].data.as_mut().unwrap().commitments[0].included_transaction_hash = None;
    handler.add_commitments(vec![cm.clone()]).await;
    let result = validator.validate(&data, &option).await.unwrap();
    assert!(result.contract_results[0].result.is_err());
    data.contracts_data[0].data.as_mut().unwrap().commitments[0].included_transaction_hash = Some(vec![1, 2, 3]);

    data.contracts_data[0].data.as_mut().unwrap().commitments[0].status = CommitmentStatus::SrcSucceeded as i32;
    let result = validator.validate(&data, &option).await.unwrap();
    assert!(result.contract_results[0].result.is_ok());

    data.contracts_data[0].data.as_mut().unwrap().commitments[0].status = CommitmentStatus::SrcSucceeded as i32;
    data.contracts_data[0].data.as_mut().unwrap().commitments[0].src_chain_block_number = None;
    let result = validator.validate(&data, &option).await.unwrap();
    assert!(result.contract_results[0].result.is_err());

    data.contracts_data[0].data.as_mut().unwrap().commitments[0].src_chain_block_number = Some(1);
    data.contracts_data[0].data.as_mut().unwrap().commitments[0].src_chain_transaction_hash = None;
    let result = validator.validate(&data, &option).await.unwrap();
    assert!(result.contract_results[0].result.is_err());
}

#[tokio::test]
async fn test_check_commitment_lite_data() {
    let (validator, _handler, _mock) = create_single_rule_lite_data_validator(Some(vec![RuleCheckerType::Integrity]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let option = ValidateOption::builder().config(core_cfg).build();
    let chain_id = 1_u64;
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let cm = Commitment {
        commitment_hash: vec![
            17, 214, 48, 198, 85, 198, 51, 72, 44, 121, 28, 13, 120, 224, 26, 20, 17, 156, 251, 233, 119, 14, 62, 130,
            72, 235, 26, 209, 81, 231, 116, 185,
        ],
        status: CommitmentStatus::Queued as i32,
        block_number: 1,
        included_block_number: Some(1),
        src_chain_block_number: Some(1),
        leaf_index: Some(1),
        rollup_fee: Some(vec![1, 2, 3]),
        encrypted_note: Some(vec![1, 2, 3]),
        queued_transaction_hash: Some(vec![1, 2, 3]),
        included_transaction_hash: Some(vec![1, 2, 3]),
        src_chain_transaction_hash: Some(vec![1, 2, 3]),
    };

    let contract_data = ContractData::builder()
        .address(contract_address)
        .start_block(1_u64)
        .end_block(100_u64)
        .data(LiteData::builder().commitments(vec![cm]).build())
        .build();
    let mut data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![contract_data])
        .build();
    let result = validator.validate(&data, &option).await.unwrap();
    assert_eq!(result.chain_id, chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, contract_address);
    assert!(result.contract_results[0].result.is_ok());

    data.contracts_data[0].data.as_mut().unwrap().commitments[0].status = CommitmentStatus::Queued as i32;
    data.contracts_data[0].data.as_mut().unwrap().commitments[0].leaf_index = None;
    let result = validator.validate(&data, &option).await.unwrap();
    assert!(result.contract_results[0].result.is_err());

    data.contracts_data[0].data.as_mut().unwrap().commitments[0].status = 100000;
    let result = validator.validate(&data, &option).await.unwrap();
    assert!(result.contract_results[0].result.is_err());

    data.contracts_data[0].data.as_mut().unwrap().commitments[0].status = CommitmentStatus::Unspecified as i32;
    let result = validator.validate(&data, &option).await.unwrap();
    assert!(result.contract_results[0].result.is_err());

    data.contracts_data[0].data.as_mut().unwrap().commitments[0].status = CommitmentStatus::SrcSucceeded as i32;
    let result = validator.validate(&data, &option).await.unwrap();
    assert!(result.contract_results[0].result.is_ok());
}
