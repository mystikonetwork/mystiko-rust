use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mystiko_config::raw::base::RawConfig;
use mystiko_config::raw::bridge::axelar::RawAxelarBridgeConfig;
use mystiko_config::wrapper::bridge::axelar::AxelarBridgeConfig;

async fn default_raw_config() -> RawAxelarBridgeConfig {
    RawConfig::create_from_file::<RawAxelarBridgeConfig>("tests/files/bridge/axelar.valid.json").await
}

async fn default_axelar_config() -> AxelarBridgeConfig {
    AxelarBridgeConfig::new(default_raw_config().await)
}

lazy_static! {
    static ref RAW_CONFIG_CREATER: AsyncOnce<RawAxelarBridgeConfig> = AsyncOnce::new(async {
       default_raw_config().await
    });
    static ref CONFIG_CREATER: AsyncOnce<AxelarBridgeConfig> = AsyncOnce::new(async {
        default_axelar_config().await
    });
}

#[tokio::test]
async fn test_equality() {
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let config = CONFIG_CREATER.get().await;
    assert_eq!(&raw_config.base.name, config.base.name());
    assert_eq!(&raw_config.bridge_type, config.base.bridge_type());
}

#[tokio::test]
async fn test_copy() {
    let config = CONFIG_CREATER.get().await;
    let copy = AxelarBridgeConfig::new(config.base.base.copy_data());
    assert_eq!(&copy, config);
}

#[tokio::test]
async fn test_mutate() {
    let mut raw_config = default_raw_config().await;
    let config = CONFIG_CREATER.get().await;
    let mutate_config = config.mutate(None);
    assert_eq!(config, &mutate_config);

    raw_config.base.name = "another name".to_string();
    let new_config = config.mutate(Some(raw_config));
    assert_eq!(new_config.base.name(), &"another name".to_string());
}

#[tokio::test]
async fn test_to_json_string() {
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let config = CONFIG_CREATER.get().await;
    let json_string = config.base.base.to_json_string();
    let loaded_raw_config =
        RawConfig::create_from_json_string::<RawAxelarBridgeConfig>(&json_string).await;
    assert_eq!(&loaded_raw_config, raw_config);
}