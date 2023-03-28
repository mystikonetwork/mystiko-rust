use mystiko_config::raw::create_raw_from_file;
use mystiko_config::raw::indexer::RawIndexerConfig;
use mystiko_config::wrapper::indexer::IndexerConfig;
use std::sync::Arc;

const VALID_CONFIG_FILE: &str = "tests/files/indexer.valid.json";

#[tokio::test]
async fn test_create() {
    let raw_config = create_raw_from_file::<RawIndexerConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let config = IndexerConfig::new(Arc::new(raw_config));
    config.validate().unwrap();
    assert_eq!(config.url(), "https://example.com");
    assert_eq!(config.timeout_ms(), 1000);
}
