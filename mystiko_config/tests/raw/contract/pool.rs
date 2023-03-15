use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mystiko_config::common::{BridgeType, ContractType};
use mystiko_config::raw::base::{RawConfig, Validator};
use mystiko_config::raw::contract::base::{RawContractConfig, RawContractConfigTrait};
use mystiko_config::raw::contract::pool::RawPoolContractConfig;

async fn default_config() -> RawPoolContractConfig {
    RawConfig::create_from_object::<RawPoolContractConfig>(RawPoolContractConfig::new(
        RawContractConfig::new(
            2,
            "CommitmentPool".to_string(),
            "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67".to_string(),
            ContractType::Pool,
            1000000,
            None,
            None,
        ),
        "A Pool(since 07/20/2022)".to_string(),
        BridgeType::Tbridge,
        Some("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a".to_string()),
        "120000000000000000".to_string(),
        vec![String::from("circuit-1.0")],
    ))
    .await
    .unwrap()
}

lazy_static! {
    static ref CONFIG_CREATER: AsyncOnce<RawPoolContractConfig> =
        AsyncOnce::new(async { default_config().await });
}

#[tokio::test]
async fn test_raw_contract_config_trait() {
    let config = CONFIG_CREATER.get().await;
    assert_eq!(config.version(), &config.base.version);
    assert_eq!(config.name(), config.base.name);
    assert_eq!(config.address(), config.base.address);
    assert_eq!(config.contract_type(), &config.contract_type);
    assert_eq!(config.start_block(), &config.base.start_block);
    assert_eq!(config.event_filter_size(), &config.base.event_filter_size);
    assert_eq!(
        config.indexer_filter_size(),
        &config.base.indexer_filter_size
    );
}

#[tokio::test]
async fn test_validate_success() {
    let mut config = default_config().await;
    config.asset_address = None;
    config.min_rollup_fee = "0".to_string();
    config.circuits = vec![];
    assert_eq!(config.validation().is_err(), false);
}

#[tokio::test]
async fn test_invalid_pool_name() {
    let mut config = default_config().await;
    config.pool_name = "".to_string();
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_contract_type() {
    let mut config = default_config().await;
    config.contract_type = ContractType::Deposit;
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_min_rollup_fee_0() {
    let mut config = default_config().await;
    config.min_rollup_fee = String::from("");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_min_rollup_fee_1() {
    let mut config = default_config().await;
    config.min_rollup_fee = String::from("0xdeadbeef");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_min_rollup_fee_2() {
    let mut config = default_config().await;
    config.min_rollup_fee = String::from("-1");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_min_rollup_fee_3() {
    let mut config = default_config().await;
    config.min_rollup_fee = String::from("1.2");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_circuits() {
    let mut config = default_config().await;
    config.circuits = vec![String::from("")];
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config = RawConfig::create_from_file::<RawPoolContractConfig>(
        "tests/files/contract/pool.valid.json",
    )
    .await
    .unwrap();
    assert_eq!(file_config, default_config().await);
    assert_eq!(file_config.contract_type, file_config.base.contract_type);
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config = RawConfig::create_from_file::<RawPoolContractConfig>(
        "tests/files/contract/pool.invalid.json",
    )
    .await;
    assert_eq!(file_config.is_err(), true);
}

#[tokio::test]
async fn test_import_valid_json_str() {
    let json_str = r#"
            {
              "version": 2,
              "name": "CommitmentPool",
              "poolName": "A Pool(since 07/20/2022)",
              "bridgeType": "tbridge",
              "address": "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67",
              "startBlock": 1000000,
              "assetAddress": "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a",
              "minRollupFee": "120000000000000000",
              "circuits": ["circuit-1.0"]
            }
        "#;
    let str_config = RawConfig::create_from_json_string::<RawPoolContractConfig>(json_str)
        .await
        .unwrap();
    assert_eq!(str_config.contract_type, ContractType::Pool);
    assert_eq!(str_config.contract_type, str_config.base.contract_type);
}
