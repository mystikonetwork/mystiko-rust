use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mystiko_config::raw::base::RawConfig;
use mystiko_config::raw::indexer::RawIndexerConfig;
use mystiko_config::wrapper::indexer::IndexerConfig;

async fn default_raw_config() -> RawIndexerConfig {
    RawConfig::create_from_file::<RawIndexerConfig>("tests/files/indexer.valid.json")
        .await
        .unwrap()
}

async fn default_indexer_config() -> IndexerConfig {
    IndexerConfig::new(default_raw_config().await)
}

lazy_static! {
    static ref CONFIG_CREATER: AsyncOnce<IndexerConfig> =
        AsyncOnce::new(async { default_indexer_config().await });
    static ref RAW_CONFIG_CREATER: AsyncOnce<RawIndexerConfig> =
        AsyncOnce::new(async { default_raw_config().await });
}

#[tokio::test]
async fn test_equality() {
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let config = CONFIG_CREATER.get().await;
    assert_eq!(config.url(), raw_config.url);
    assert_eq!(config.time_out_ms(), &raw_config.timeout_ms);
}

#[tokio::test]
async fn test_copy() {
    let config = CONFIG_CREATER.get().await;
    assert_eq!(&IndexerConfig::new(config.copy_data()), config);
}

#[tokio::test]
async fn test_mutate() {
    let mut raw_config = default_raw_config().await;
    let config = CONFIG_CREATER.get().await;
    assert_eq!(&config.mutate(None), config);
    raw_config.url = "https://example1.com".to_string();
    let new_config = config.mutate(Some(raw_config));
    assert_eq!(new_config.url(), "https://example1.com");
}

#[tokio::test]
async fn test_to_json_string() {
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let config = CONFIG_CREATER.get().await;
    let json_string = config.to_json_string();
    let loaded_raw_config =
        RawConfig::create_from_json_string::<RawIndexerConfig>(json_string.as_str())
            .await
            .unwrap();
    assert_eq!(&loaded_raw_config, raw_config);
}
