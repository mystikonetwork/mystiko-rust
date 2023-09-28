use crate::validator::common::{create_full_rule_full_data_validator, load_commitments};
use ethers_core::types::Bytes;
use mystiko_config::MystikoConfig;
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::validator::{DataValidator, ValidateOption};
use mystiko_protos::data::v1::CommitmentStatus;
use std::str::FromStr;

#[tokio::test]
async fn test_empty_commitment() {
    let (validator, _handler, _mock) = create_full_rule_full_data_validator();
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    for concurrency in 1_usize..3_usize {
        let option = ValidateOption::builder()
            .config(core_cfg.clone())
            .validate_concurrency(concurrency)
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
        assert!(result.contract_results[0]
            .result
            .as_ref()
            .err()
            .unwrap()
            .to_string()
            .contains("empty responses array"));
    }
}

#[tokio::test]
async fn test_some_included_commitment() {
    let (validator, handler, mock) = create_full_rule_full_data_validator();
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    for concurrency in 1_usize..3_usize {
        let option = ValidateOption::builder()
            .config(core_cfg.clone())
            .validate_concurrency(concurrency)
            .build();
        let chain_id = 1_u64;
        let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
        let mut cms = load_commitments("./tests/files/validator/commitments_100.json").await;
        let mut fetched_cms = vec![];
        for cm in cms.iter_mut().take(10) {
            cm.rollup_fee = Some(vec![1, 2, 3]);
            cm.queued_transaction_hash = Some(vec![1, 2, 3]);
            cm.encrypted_note = Some(vec![1, 2, 3]);
            cm.included_block_number = Some(16690440);
            cm.included_transaction_hash = Some(vec![1, 2, 3]);
            cm.block_number = 16690440;
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
        handler.add_commitments(vec![]).await;
        handler.add_commitments(vec![]).await;
        handler.add_commitments(vec![]).await;
        let tree_root = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
        mock.push::<Bytes, _>(tree_root.clone()).unwrap();
        let include_count =
            Bytes::from_str("000000000000000000000000000000000000000000000000000000000000000a").unwrap();
        mock.push::<Bytes, _>(include_count.clone()).unwrap();
        let result = validator.validate(&data, &option).await.unwrap();
        assert_eq!(result.chain_id, chain_id);
        assert_eq!(result.contract_results.len(), 1);
        assert_eq!(result.contract_results[0].address, contract_address);
        assert!(result.contract_results[0].result.is_ok());
    }
}
