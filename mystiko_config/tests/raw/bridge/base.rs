use mystiko_config::common::BridgeType;
use mystiko_config::raw::base::{RawConfig, Validator};
use mystiko_config::raw::bridge::base::RawBridgeConfig;

async fn default_config() -> RawBridgeConfig {
    let raw_bridge_config = RawBridgeConfig::builder()
        .name("TBridge config".to_string())
        .bridge_type(BridgeType::Tbridge)
        .build();
    RawConfig::from_object::<RawBridgeConfig>(raw_bridge_config).unwrap()
}

#[tokio::test]
async fn test_invalid_name() {
    let mut config = default_config().await;
    config.name = "".to_string();
    assert_eq!(config.validation().is_err(), true);
}
