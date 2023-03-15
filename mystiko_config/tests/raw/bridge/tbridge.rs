use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mystiko_config::common::BridgeType;
use mystiko_config::raw::base::{RawConfig, Validator};
use mystiko_config::raw::bridge::base::RawBridgeConfigTrait;
use mystiko_config::raw::bridge::tbridge::RawTBridgeConfig;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

async fn default_config() -> RawTBridgeConfig {
    RawConfig::create_from_object::<RawTBridgeConfig>(RawTBridgeConfig::new(String::from(
        "Mystiko Testnet Bridge",
    )))
    .await
    .unwrap()
}

lazy_static! {
    static ref CONFIG_CREATER: AsyncOnce<RawTBridgeConfig> =
        AsyncOnce::new(async { default_config().await });
}

#[tokio::test]
async fn test_hash() {
    let config1 = CONFIG_CREATER.get().await;
    let mut hasher = DefaultHasher::new();
    config1.hash(&mut hasher);
    let hash1 = hasher.finish();

    hasher = DefaultHasher::new();
    let mut config2 = default_config().await;
    config2.bridge_type = BridgeType::Poly;
    config2.hash(&mut hasher);
    let hash2 = hasher.finish();

    assert_ne!(hash1, hash2);
}

#[tokio::test]
async fn test_name() {
    let config = CONFIG_CREATER.get().await;
    assert_eq!(config.name(), &config.base.name);
}

#[tokio::test]
async fn test_validate_success() {
    let config = default_config().await;
    assert_eq!(config.bridge_type, BridgeType::Tbridge);
}

#[tokio::test]
async fn test_invalid_name() {
    let mut config = default_config().await;
    config.base.name = "".to_string();
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_type() {
    let mut config = default_config().await;
    config.bridge_type = BridgeType::Poly;
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config =
        RawConfig::create_from_file::<RawTBridgeConfig>("tests/files/bridge/tbridge.valid.json")
            .await
            .unwrap();
    assert_eq!(file_config, default_config().await);
    assert_eq!(file_config.bridge_type, file_config.base.bridge_type);
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config =
        RawConfig::create_from_file::<RawTBridgeConfig>("tests/files/bridge/tbridge.invalid.json")
            .await;
    assert_eq!(file_config.is_err(), true);
}

#[tokio::test]
async fn test_import_valid_json_str() {
    let json_str = r#"{
            "name": "Mystiko Testnet Bridge"
        }"#;
    let str_config = RawConfig::create_from_json_string::<RawTBridgeConfig>(json_str)
        .await
        .unwrap();
    assert_eq!(str_config.bridge_type, BridgeType::Tbridge);
    assert_eq!(str_config.bridge_type, str_config.base.bridge_type)
}
