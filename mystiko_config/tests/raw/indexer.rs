use mystiko_config::raw::base::{RawConfig, Validator};
use mystiko_config::raw::indexer::RawIndexerConfig;

async fn default_config() -> RawIndexerConfig {
    RawConfig::create_from_object::<RawIndexerConfig>(
        RawIndexerConfig::new("https://example.com".to_string(), 1000)
    ).await
}

#[tokio::test]
#[should_panic]
async fn test_invalid_url() {
    let mut config = default_config().await;
    config.url = String::from("not a valid url");
    config.validation();
}

#[tokio::test]
#[should_panic]
async fn test_invalid_time_out_ms() {
    let mut config = default_config().await;
    config.timeout_ms = 0;
    config.validation();
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config =
        RawConfig::create_from_file::<RawIndexerConfig>("tests/files/indexer.valid.json").await;
    assert_eq!(file_config, default_config().await);
}

#[tokio::test]
#[should_panic]
async fn test_import_invalid_json_file() {
    let _file_config =
        RawConfig::create_from_file::<RawIndexerConfig>("tests/files/indexer.invalid.json").await;
}