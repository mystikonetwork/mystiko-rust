use mystiko_config::{create_raw_from_file, IndexerConfig, RawIndexerConfig};
use std::sync::Arc;

const VALID_CONFIG_FILE: &str = "tests/files/indexer/valid.json";

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

#[tokio::test]
async fn test_to_proto() {
    let raw_config = create_raw_from_file::<RawIndexerConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let config = IndexerConfig::new(Arc::new(raw_config)).to_proto();
    assert_eq!(&config.url, "https://example.com");
    assert_eq!(config.timeout_ms, 1000);
}
