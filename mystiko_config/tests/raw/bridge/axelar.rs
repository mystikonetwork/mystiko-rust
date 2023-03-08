use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use lazy_static::lazy_static;
use mystiko_config::raw::base::{RawConfig, Validator};
use async_once::AsyncOnce;
use mystiko_config::common::BridgeType;
use mystiko_config::raw::bridge::axelar::RawAxelarBridgeConfig;
use mystiko_config::raw::bridge::base::RawBridgeConfigTrait;

async fn default_config() -> RawAxelarBridgeConfig {
    RawConfig::create_from_object::<RawAxelarBridgeConfig>(
        RawAxelarBridgeConfig::new(String::from("Axelar Bridge"))
    ).await
}

lazy_static! {
    static ref CONFIG_CREATER: AsyncOnce<RawAxelarBridgeConfig> = AsyncOnce::new(async {
        default_config().await
    });
}

#[tokio::test]
async fn test_hash() {
    let config1 = CONFIG_CREATER.get().await;
    let mut hasher = DefaultHasher::new();
    config1.hash(&mut hasher);
    let hash1 = hasher.finish();

    hasher = DefaultHasher::new();
    let mut config2 = default_config().await;
    config2.bridge_type = BridgeType::Tbridge;
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
#[should_panic]
async fn test_invalid_type() {
    let mut config = default_config().await;
    config.bridge_type = BridgeType::Tbridge;
    config.validation();
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config =
        RawConfig::create_from_file::<RawAxelarBridgeConfig>("tests/files/bridge/axelar.valid.json").await;
    assert_eq!(file_config, default_config().await);
    assert_eq!(file_config.bridge_type, file_config.base.bridge_type);
}

#[tokio::test]
#[should_panic]
async fn test_import_invalid_json_file() {
    let _file_config =
        RawConfig::create_from_file::<RawAxelarBridgeConfig>("tests/files/bridge/axelar.invalid.json").await;
}

#[tokio::test]
async fn test_import_valid_json_str() {
    let json_str = r#"{
            "name": "Axelar Bridge"
        }"#;
    let str_config =
        RawConfig::create_from_json_string::<RawAxelarBridgeConfig>(json_str).await;
    assert_eq!(str_config.bridge_type, BridgeType::Axelar);
    assert_eq!(str_config.bridge_type, str_config.base.bridge_type)
}