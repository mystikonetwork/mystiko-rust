use mystiko_config::raw::base::{RawConfig, Validator};
use mystiko_config::raw::provider::RawProviderConfig;

async fn default_config() -> RawProviderConfig {
    RawConfig::from_object::<RawProviderConfig>(
        RawProviderConfig::builder()
            .url("http://localhost:8545".to_string())
            .timeout_ms(100000)
            .max_try_count(5)
            .build(),
    )
    .unwrap()
}

#[tokio::test]
async fn test_invalid_url_0() {
    let mut config = default_config().await;
    config.url = "".to_string();
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_url_1() {
    let mut config = default_config().await;
    config.url = "not even a url".to_string();
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_url_2() {
    let mut config = default_config().await;
    config.url = "wrong_schema://localhost:8545".to_string();
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_timeout_ms() {
    let mut config = default_config().await;
    config.timeout_ms = 0;
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_invalid_max_try_count() {
    let mut config = default_config().await;
    config.max_try_count = 0;
    assert_eq!(config.validation().is_err(), true);
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config = RawConfig::from_file::<RawProviderConfig>("tests/files/provider.valid.json")
        .await
        .unwrap();
    assert_eq!(file_config, default_config().await);
}

#[tokio::test]
async fn test_import_invalid_json_file() {
    let file_config =
        RawConfig::from_file::<RawProviderConfig>("tests/files/provider.invalid.json").await;
    assert!(file_config.is_err());
}
