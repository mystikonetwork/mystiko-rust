use mystiko_config::raw::bridge::axelar::RawAxelarBridgeConfig;
use mystiko_config::raw::bridge::celer::RawCelerBridgeConfig;
use mystiko_config::raw::bridge::layer_zero::RawLayerZeroBridgeConfig;
use mystiko_config::raw::bridge::RawBridgeConfig;

#[test]
fn test_compare_raw_bridge_config_type() {
    let config1 = RawBridgeConfig::Axelar(
        RawAxelarBridgeConfig::builder()
            .name("test1".to_string())
            .build(),
    );
    let config2 = RawBridgeConfig::Axelar(
        RawAxelarBridgeConfig::builder()
            .name("test2".to_string())
            .build(),
    );
    assert_eq!(config1, config2);
    let config1 = RawBridgeConfig::Celer(
        RawCelerBridgeConfig::builder()
            .name("test1".to_string())
            .build(),
    );
    let config2 = RawBridgeConfig::Celer(
        RawCelerBridgeConfig::builder()
            .name("test2".to_string())
            .build(),
    );
    assert_eq!(config1, config2);
    let config1 = RawBridgeConfig::LayerZero(
        RawLayerZeroBridgeConfig::builder()
            .name("test1".to_string())
            .build(),
    );
    let config2 = RawBridgeConfig::LayerZero(
        RawLayerZeroBridgeConfig::builder()
            .name("test2".to_string())
            .build(),
    );
    assert_eq!(config1, config2);
}
