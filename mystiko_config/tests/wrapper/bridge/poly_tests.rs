use mystiko_config::raw::bridge::poly::RawPolyBridgeConfig;
use mystiko_config::raw::create_raw_from_file;
use mystiko_config::wrapper::bridge::poly::PolyBridgeConfig;
use mystiko_types::BridgeType;
use std::sync::Arc;

const VALID_CONFIG_FILE: &str = "tests/files/bridge/poly/valid.json";

#[tokio::test]
async fn test_create() {
    let raw_config = create_raw_from_file::<RawPolyBridgeConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let config = PolyBridgeConfig::new(Arc::new(raw_config));
    config.validate().unwrap();
    assert_eq!(config.name(), "Poly Bridge");
    assert_eq!(config.bridge_type(), &BridgeType::Poly);
    assert_eq!(config.explorer_url(), "https://explorer.poly.network");
    assert_eq!(config.explorer_prefix(), "/tx/%tx%");
    assert_eq!(config.api_url(), "https://explorer.poly.network");
    assert_eq!(
        config.api_prefix(),
        "/testnet/api/v1/getcrosstx?txhash=%tx%"
    );
}
