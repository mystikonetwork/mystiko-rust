use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mystiko_config::common::ContractType;
use mystiko_config::raw::base::{RawConfig, Validator};
use mystiko_config::raw::contract::base::RawContractConfig;

async fn default_config() -> RawContractConfig {
    RawConfig::create_from_object::<RawContractConfig>(
        RawContractConfig::new(
            2,
            "MystikoWithPolyERC20".to_string(),
            "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67".to_string(),
            ContractType::Deposit,
            1000000,
            Some(10000),
            Some(100000),
        )
    ).await.unwrap()
}

lazy_static! {
    static ref CONFIG_CREATER: AsyncOnce<RawContractConfig> = AsyncOnce::new(async {
        default_config().await
    });
}

#[tokio::test]
async fn test_validate_success() {
    let mut config = default_config().await;
    config.event_filter_size = None;
    assert_eq!(config.validation().is_err(), false);
}

#[tokio::test]
async fn test_invalid_version() {
    let mut config = default_config().await;
    config.version = 0;
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_name() {
    let mut config = default_config().await;
    config.name = String::from("");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_address_0() {
    let mut config = default_config().await;
    config.address = String::from("0xdeadbeef");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_address_1() {
    let mut config = default_config().await;
    config.address = String::from("");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_start_block() {
    let mut config = default_config().await;
    config.start_block = 0;
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_event_filter_size() {
    let mut config = default_config().await;
    config.event_filter_size = Some(0);
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_indexer_filter_size() {
    let mut config = default_config().await;
    config.indexer_filter_size = Some(0);
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let default_config = CONFIG_CREATER.get().await;
    let file_config =
        RawConfig::create_from_file::<RawContractConfig>("tests/files/contract/base.valid.json").await.unwrap();
    assert_eq!(&file_config, default_config);
}

#[tokio::test]
#[should_panic]
async fn test_import_invalid_json_file() {
    let _file_config =
        RawConfig::create_from_file::<RawContractConfig>("tests/files/contract/base.invalid.json").await;
}