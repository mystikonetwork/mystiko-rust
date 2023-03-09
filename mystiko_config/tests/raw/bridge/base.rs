use mystiko_config::common::BridgeType;
use mystiko_config::raw::base::{RawConfig, Validator};
use mystiko_config::raw::bridge::base::RawBridgeConfig;

async fn default_config() -> RawBridgeConfig {
    RawConfig::create_from_object::<RawBridgeConfig>(
        RawBridgeConfig::new("TBridge config".to_string(), BridgeType::Tbridge)
    ).await.unwrap()
}

#[tokio::test]
async fn test_invalid_name() {
    let mut config = default_config().await;
    config.name = "".to_string();
    assert_eq!(config.validation().is_err(), true);
}