use mystiko_config::raw::bridge::axelar::RawAxelarBridgeConfig;
use mystiko_config::raw::bridge::celer::RawCelerBridgeConfig;
use mystiko_config::raw::bridge::layer_zero::RawLayerZeroBridgeConfig;
use mystiko_config::raw::bridge::RawBridgeConfig;
use std::sync::Arc;

#[test]
fn test_compare_raw_bridge_config_type() {
    let config1 = RawBridgeConfig::Axelar(Arc::new(
        RawAxelarBridgeConfig::builder().name("test1".to_string()).build(),
    ));
    let config2 = RawBridgeConfig::Axelar(Arc::new(
        RawAxelarBridgeConfig::builder().name("test1".to_string()).build(),
    ));
    assert_eq!(config1, config2);
    let config1 = RawBridgeConfig::Celer(Arc::new(
        RawCelerBridgeConfig::builder().name("test1".to_string()).build(),
    ));
    let config2 = RawBridgeConfig::Celer(Arc::new(
        RawCelerBridgeConfig::builder().name("test1".to_string()).build(),
    ));
    assert_eq!(config1, config2);
    let config1 = RawBridgeConfig::LayerZero(Arc::new(
        RawLayerZeroBridgeConfig::builder().name("test1".to_string()).build(),
    ));
    let config2 = RawBridgeConfig::LayerZero(Arc::new(
        RawLayerZeroBridgeConfig::builder().name("test1".to_string()).build(),
    ));
    assert_eq!(config1, config2);
}
