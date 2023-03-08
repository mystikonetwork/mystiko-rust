use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mystiko_config::raw::base::RawConfig;
use mystiko_config::raw::bridge::base::RawBridgeConfig;
use mystiko_config::wrapper::bridge::base::BridgeConfig;

async fn default_raw_config() -> RawBridgeConfig {
    RawConfig::create_from_file::<RawBridgeConfig>("tests/files/bridge/base.valid.json").await
}

async fn default_bridge_config() -> BridgeConfig<RawBridgeConfig> {
    BridgeConfig::new(default_raw_config().await, None)
}

lazy_static! {
    static ref RAW_CONFIG_CREATER: AsyncOnce<RawBridgeConfig> = AsyncOnce::new(async {
       default_raw_config().await
    });
    static ref CONFIG_CREATER: AsyncOnce<BridgeConfig<RawBridgeConfig>> = AsyncOnce::new(async {
        default_bridge_config().await
    });
}

#[tokio::test]
async fn test_copy() {
    let config = CONFIG_CREATER.get().await;
    let copy = BridgeConfig::new(config.base.copy_data(), None);
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
        RawConfig::create_from_json_string::<RawBridgeConfig>(&json_string).await;
    assert_eq!(&loaded_raw_config, raw_config);
}