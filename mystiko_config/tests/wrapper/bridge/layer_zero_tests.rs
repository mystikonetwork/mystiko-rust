use mystiko_config::raw::bridge::layer_zero::RawLayerZeroBridgeConfig;
use mystiko_config::raw::create_raw_from_file;
use mystiko_config::types::BridgeType;
use mystiko_config::wrapper::bridge::layer_zero::LayerZeroBridgeConfig;
use std::sync::Arc;

const VALID_CONFIG_FILE: &str = "tests/files/bridge/layer_zero.valid.json";

#[tokio::test]
async fn test_create() {
    let raw_config = create_raw_from_file::<RawLayerZeroBridgeConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let config = LayerZeroBridgeConfig::new(Arc::new(raw_config));
    config.validate().unwrap();
    assert_eq!(config.name(), "LayerZero Bridge");
    assert_eq!(config.bridge_type(), &BridgeType::LayerZero);
}
