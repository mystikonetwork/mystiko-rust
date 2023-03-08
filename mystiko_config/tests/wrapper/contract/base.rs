use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mystiko_config::raw::base::RawConfig;
use mystiko_config::raw::contract::base::RawContractConfig;
use mystiko_config::wrapper::contract::base::ContractConfig;

async fn default_raw_config() -> RawContractConfig {
    RawConfig::create_from_file::<RawContractConfig>("tests/files/contract/base.valid.json").await
}

async fn default_contract_config() -> ContractConfig<RawContractConfig> {
    ContractConfig::new(default_raw_config().await, None)
}

lazy_static! {
    static ref CONFIG_CREATER: AsyncOnce<ContractConfig<RawContractConfig>> = AsyncOnce::new(async {
        default_contract_config().await
    });
    static ref RAW_CONFIG_CREATER: AsyncOnce<RawContractConfig> = AsyncOnce::new(async {
        default_raw_config().await
    });
}

#[tokio::test]
async fn test_equality() {
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let config = CONFIG_CREATER.get().await;
    assert_eq!(config.version(), &raw_config.version);
    assert_eq!(config.address(), &raw_config.address);
    assert_eq!(config.name(), &raw_config.name);
    assert_eq!(config.contract_type(), &raw_config.contract_type);
    assert_eq!(config.start_block(), &raw_config.start_block);
    assert_eq!(config.event_filter_size(), &raw_config.event_filter_size);
    assert_eq!(config.indexer_filter_size(), &raw_config.indexer_filter_size);
}

#[tokio::test]
async fn test_copy() {
    let config = CONFIG_CREATER.get().await;
    let copy =
        ContractConfig::new(config.base.copy_data(), None);
    assert_eq!(&copy, config);
}

#[tokio::test]
async fn test_mutate() {
    let mut raw_config = default_raw_config().await;
    let config = CONFIG_CREATER.get().await;
    let mutate_config = config.mutate(None, None);
    assert_eq!(config, &mutate_config);

    raw_config.name = "another name".to_string();
    let new_config = config.mutate(Some(raw_config), None);
    assert_eq!(new_config.name(), &"another name".to_string());
}

#[tokio::test]
async fn test_to_json_string() {
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let config = CONFIG_CREATER.get().await;
    let json_string = config.base.to_json_string();
    let loaded_raw_config =
        RawConfig::create_from_json_string::<RawContractConfig>(&json_string).await;
    assert_eq!(&loaded_raw_config, raw_config);
}