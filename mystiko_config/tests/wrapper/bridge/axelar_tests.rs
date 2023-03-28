use mystiko_config::raw::bridge::axelar::RawAxelarBridgeConfig;
use mystiko_config::raw::create_raw_from_file;
use mystiko_config::types::BridgeType;
use mystiko_config::wrapper::bridge::axelar::AxelarBridgeConfig;
use std::sync::Arc;

const VALID_CONFIG_FILE: &str = "tests/files/bridge/axelar.valid.json";

#[tokio::test]
async fn test_create() {
    let raw_config = create_raw_from_file::<RawAxelarBridgeConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let config = AxelarBridgeConfig::new(Arc::new(raw_config));
    config.validate().unwrap();
    assert_eq!(config.name(), "Axelar Bridge");
    assert_eq!(config.bridge_type(), &BridgeType::Axelar);
}
