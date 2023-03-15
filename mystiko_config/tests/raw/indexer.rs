use mystiko_config::raw::base::{RawConfig, Validator};
use mystiko_config::raw::indexer::RawIndexerConfig;

async fn default_config() -> RawIndexerConfig {
    RawConfig::create_from_object::<RawIndexerConfig>(RawIndexerConfig::new(
        "https://example.com".to_string(),
        1000,
    ))
    .await
    .unwrap()
}

#[tokio::test]
async fn test_invalid_url() {
    let mut config = default_config().await;
    config.url = String::from("not a valid url");
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_time_out_ms() {
    let mut config = default_config().await;
    config.timeout_ms = 0;
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config =
        RawConfig::create_from_file::<RawIndexerConfig>("tests/files/indexer.valid.json")
            .await
            .unwrap();
    assert_eq!(file_config, default_config().await);
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config =
        RawConfig::create_from_file::<RawIndexerConfig>("tests/files/indexer.invalid.json").await;
    assert_eq!(file_config.is_err(), true);
}
