use mystiko_config::raw::bridge::celer::RawCelerBridgeConfig;
use mystiko_config::raw::create_raw_from_file;
use mystiko_config::wrapper::bridge::celer::CelerBridgeConfig;
use mystiko_types::BridgeType;
use std::sync::Arc;

const VALID_CONFIG_FILE: &str = "tests/files/bridge/celer/valid.json";

#[tokio::test]
async fn test_create() {
    let raw_config = create_raw_from_file::<RawCelerBridgeConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let config = CelerBridgeConfig::new(Arc::new(raw_config));
    config.validate().unwrap();
    assert_eq!(config.name(), "Celer Bridge");
    assert_eq!(config.bridge_type(), &BridgeType::Celer);
}
