use mystiko_config::raw::create_raw_from_file;
use mystiko_config::raw::provider::RawProviderConfig;
use mystiko_config::wrapper::provider::ProviderConfig;
use std::sync::Arc;

const VALID_CONFIG_FILE: &str = "tests/files/provider.valid.json";

#[tokio::test]
async fn test_create() {
    let raw_config = create_raw_from_file::<RawProviderConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let config = ProviderConfig::new(Arc::new(raw_config));
    config.validate().unwrap();
    assert_eq!(config.url(), "http://localhost:8545");
    assert_eq!(config.timeout_ms(), 100000);
    assert_eq!(config.max_try_count(), 5);
}
