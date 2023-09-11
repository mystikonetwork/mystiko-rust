use crate::validator::common::validator_mock::{create_single_rule_full_data_validator, RuleCheckerType};
use ethers_core::types::Bytes;
use mystiko_config::MystikoConfig;
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::validator::{DataValidator, ValidateOption};
use std::str::FromStr;

#[tokio::test]
async fn test_wrong_concurrency() {
    let (validator, _, _, _, _) = create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Integrity]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko_testnet.json")
        .await
        .unwrap();

    let chain_id = 5_u64;
    let mut contract_data = vec![];
    core_cfg
        .find_chain(chain_id)
        .unwrap()
        .contracts()
        .iter()
        .for_each(|contract| {
            contract_data.push(
                ContractData::builder()
                    .address(contract.address())
                    .start_block(1_u64)
                    .end_block(100_u64)
                    .data(FullData::builder().commitments(vec![]).nullifiers(vec![]).build())
                    .build(),
            );
        });

    let option = ValidateOption::builder()
        .config(core_cfg.clone())
        .validate_concurrency(0_usize)
        .build();
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data.clone())
        .build();
    let result = validator.validate(&data, &option).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_all_success_empty_commitment() {
    let (validator, _, _, _, _) = create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Integrity]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko_testnet.json")
        .await
        .unwrap();

    let chain_id = 5_u64;
    let mut contract_data = vec![];
    core_cfg
        .find_chain(chain_id)
        .unwrap()
        .contracts()
        .iter()
        .for_each(|contract| {
            contract_data.push(
                ContractData::builder()
                    .address(contract.address())
                    .start_block(1_u64)
                    .end_block(100_u64)
                    .data(FullData::builder().commitments(vec![]).nullifiers(vec![]).build())
                    .build(),
            );
        });

    let total = contract_data.len();
    for concurrency in 1_usize..total + 3 {
        let option = ValidateOption::builder()
            .config(core_cfg.clone())
            .validate_concurrency(concurrency)
            .build();
        let data = ChainData::builder()
            .chain_id(chain_id)
            .contracts_data(contract_data.clone())
            .build();
        let result = validator.validate(&data, &option).await.unwrap();
        assert_eq!(result.chain_id, chain_id);
        assert_eq!(result.contract_results.len(), contract_data.len());
        result.contract_results.iter().for_each(|contract_result| {
            assert!(contract_result.result.as_ref().is_ok());
            assert!(contract_data
                .iter()
                .any(|contract| contract.address == contract_result.address));
        });
    }
}

#[tokio::test]
async fn test_some_success_empty_commitment() {
    let (validator, handler, mock, _, _) = create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Counter]));
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko_testnet.json")
        .await
        .unwrap();

    let chain_id = 5_u64;
    let mut contract_data = vec![];
    core_cfg
        .find_chain(chain_id)
        .unwrap()
        .pool_contracts()
        .iter()
        .for_each(|contract| {
            contract_data.push(
                ContractData::builder()
                    .address(contract.address())
                    .start_block(1_u64)
                    .end_block(100_u64)
                    .data(FullData::builder().commitments(vec![]).nullifiers(vec![]).build())
                    .build(),
            );
        });
    let data = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data.clone())
        .build();
    let include_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap();

    let total = contract_data.len();
    for concurrency in (1..2 * total).step_by(10) {
        let option = ValidateOption::builder()
            .config(core_cfg.clone())
            .validate_concurrency(concurrency)
            .build();

        for success in 0..total {
            for _ in 0..success {
                mock.push::<Bytes, _>(include_count.clone()).unwrap();
                handler.add_commitments(vec![]).await;
            }

            let result = validator.validate(&data, &option).await.unwrap();
            assert_eq!(result.chain_id, chain_id);
            assert_eq!(result.contract_results.len(), contract_data.len());
            assert_eq!(
                success,
                result
                    .contract_results
                    .iter()
                    .filter(|contract_result| contract_result.result.is_ok())
                    .collect::<Vec<_>>()
                    .len()
            );
            for result in result.contract_results.iter() {
                assert!(contract_data.iter().any(|contract| contract.address == result.address));
            }
        }
    }
}
