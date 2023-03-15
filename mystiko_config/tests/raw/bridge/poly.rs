use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mystiko_config::common::BridgeType;
use mystiko_config::raw::base::{RawConfig, Validator};
use mystiko_config::raw::bridge::base::RawBridgeConfigTrait;
use mystiko_config::raw::bridge::poly::RawPolyBridgeConfig;
use mystiko_config::raw::chain::EXPLORER_DEFAULT_PREFIX;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

async fn default_config() -> RawPolyBridgeConfig {
    RawConfig::create_from_object::<RawPolyBridgeConfig>(RawPolyBridgeConfig::new(
        "Poly Bridge".to_string(),
        "https://explorer.poly.network".to_string(),
        "/tx/%tx%".to_string(),
        "https://explorer.poly.network".to_string(),
        "/testnet/api/v1/getcrosstx?txhash=%tx%".to_string(),
    ))
    .await
    .unwrap()
}

lazy_static! {
    static ref CONFIG_CREATER: AsyncOnce<RawPolyBridgeConfig> =
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
async fn test_validate_success() {
    let config = CONFIG_CREATER.get().await;
    assert_eq!(config.bridge_type, BridgeType::Poly);
    assert_eq!(config.explorer_prefix, EXPLORER_DEFAULT_PREFIX);
}

#[tokio::test]
async fn test_invalid_type() {
    let mut config = default_config().await;
    config.bridge_type = BridgeType::Tbridge;
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_explorer_url_0() {
    let mut config = default_config().await;
    config.explorer_url = String::from("");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_explorer_url_1() {
    let mut config = default_config().await;
    config.explorer_url = String::from("wrong url");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_explorer_prefix_0() {
    let mut config = default_config().await;
    config.explorer_prefix = String::from("");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_explorer_prefix_1() {
    let mut config = default_config().await;
    config.explorer_prefix = String::from("wrong prefix");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_api_url_0() {
    let mut config = default_config().await;
    config.api_url = String::from("");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_api_url_1() {
    let mut config = default_config().await;
    config.api_url = String::from("wrong url");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_api_prefix_0() {
    let mut config = default_config().await;
    config.api_prefix = String::from("");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_api_prefix_1() {
    let mut config = default_config().await;
    config.api_prefix = String::from("wrong prefix");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config =
        RawConfig::create_from_file::<RawPolyBridgeConfig>("tests/files/bridge/poly.valid.json")
            .await
            .unwrap();
    assert_eq!(file_config, default_config().await)
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config =
        RawConfig::create_from_file::<RawPolyBridgeConfig>("tests/files/bridge/poly.invalid.json")
            .await;
    assert_eq!(file_config.is_err(), true);
}

#[tokio::test]
async fn test_import_valid_json_str() {
    let json_str = r#"{
            "name": "Poly Bridge",
            "explorerUrl": "https://explorer.poly.network",
            "explorerPrefix": "/tx/%tx%",
            "apiUrl": "https://explorer.poly.network",
            "apiPrefix": "/testnet/api/v1/getcrosstx?txhash=%tx%"
        }"#;
    let str_config = RawConfig::create_from_json_string::<RawPolyBridgeConfig>(json_str)
        .await
        .unwrap();
    assert_eq!(str_config.bridge_type, BridgeType::Poly);
    assert_eq!(str_config.bridge_type, str_config.base.bridge_type)
}
