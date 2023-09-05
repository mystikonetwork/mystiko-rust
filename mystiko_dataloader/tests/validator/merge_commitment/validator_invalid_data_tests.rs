use crate::validator::common::validator_mock::{create_single_rule_full_data_validator, RuleCheckerType};
use mystiko_config::MystikoConfig;
use mystiko_dataloader::data::{ChainData, ContractData};
use mystiko_dataloader::validator::rule::DataMergeError;
use mystiko_dataloader::validator::{DataValidator, ValidateOption};

#[tokio::test]
async fn test_empty_data() {
    let (validator, _handler, _mock, _, _) =
        create_single_rule_full_data_validator(Some(vec![RuleCheckerType::Sequence]));
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
            .address(contract_address.to_string())
            .start_block(0_u64)
            .end_block(100_u64)
            .build();
        let data = ChainData::builder()
            .chain_id(chain_id)
            .contracts_data(vec![contract_data])
            .build();
        let result = validator.validate(&data, &option).await.unwrap();
        assert_eq!(result.chain_id, chain_id);
        assert_eq!(result.contract_results.len(), 1);
        assert_eq!(result.contract_results[0].address, contract_address);
        assert_eq!(
            result.contract_results[0].result.as_ref().err().unwrap().to_string(),
            DataMergeError::StartBlockError.to_string()
        );
    }
}
