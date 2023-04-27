use mystiko_config::raw::indexer::RawIndexerConfig;
use mystiko_config::raw::{create_raw, create_raw_from_file};
use validator::Validate;

fn default_config() -> RawIndexerConfig {
    create_raw::<RawIndexerConfig>(
        RawIndexerConfig::builder()
            .url("https://example.com".to_string())
            .timeout_ms(1000)
            .build(),
    )
    .unwrap()
}

#[test]
fn test_invalid_url() {
    let mut config = default_config();
    config.url = String::from("not a valid url");
    assert!(config.validate().is_err());
}

#[test]
fn test_invalid_time_out_ms() {
    let mut config = default_config();
    config.timeout_ms = 0;
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config = create_raw_from_file::<RawIndexerConfig>("tests/files/indexer/valid.json")
        .await
        .unwrap();
    assert_eq!(file_config, default_config());
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config = create_raw_from_file::<RawIndexerConfig>("tests/files/indexer/invalid.json").await;
    assert!(file_config.is_err());
}
