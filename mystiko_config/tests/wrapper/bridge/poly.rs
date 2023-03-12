use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mystiko_config::raw::base::RawConfig;
use mystiko_config::raw::bridge::poly::RawPolyBridgeConfig;
use mystiko_config::wrapper::bridge::poly::PolyBridgeConfig;

async fn default_raw_config() -> RawPolyBridgeConfig {
    RawConfig::create_from_file::<RawPolyBridgeConfig>("tests/files/bridge/poly.valid.json").await.unwrap()
}

async fn default_poly_config() -> PolyBridgeConfig {
    PolyBridgeConfig::new(default_raw_config().await)
}

lazy_static! {
    static ref RAW_CONFIG_CREATER: AsyncOnce<RawPolyBridgeConfig> = AsyncOnce::new(async {
        default_raw_config().await
    });
    static ref CONFIG_CREATER: AsyncOnce<PolyBridgeConfig> = AsyncOnce::new(async {
        default_poly_config().await
    });
}

#[tokio::test]
async fn test_equality() {
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let config = CONFIG_CREATER.get().await;
    assert_eq!(&raw_config.base.name, config.name());
    assert_eq!(&raw_config.bridge_type, config.bridge_type());
    assert_eq!(&raw_config.explorer_url, config.explorer_url());
    assert_eq!(&raw_config.explorer_prefix, config.explorer_prefix());
    assert_eq!(&raw_config.api_url, config.api_url());
    assert_eq!(&raw_config.api_prefix, config.api_prefix());
}

#[tokio::test]
async fn test_copy() {
    let config = CONFIG_CREATER.get().await;
    let copy = PolyBridgeConfig::new(config.copy_data());
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
    assert_eq!(new_config.name(), &"another name".to_string());
}

#[tokio::test]
async fn test_to_json_string() {
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let config = CONFIG_CREATER.get().await;
    let json_string = config.to_json_string();
    let loaded_raw_config =
        RawConfig::create_from_json_string::<RawPolyBridgeConfig>(&json_string).await.unwrap();
    assert_eq!(&loaded_raw_config, raw_config);
}