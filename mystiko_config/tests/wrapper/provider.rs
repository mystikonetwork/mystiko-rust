use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mystiko_config::raw::base::RawConfig;
use mystiko_config::raw::provider::RawProviderConfig;
use mystiko_config::wrapper::provider::ProviderConfig;

async fn default_raw_config() -> RawProviderConfig {
    RawConfig::create_from_file::<RawProviderConfig>("tests/files/provider.valid.json").await.unwrap()
}

async fn default_provider_config() -> ProviderConfig {
    ProviderConfig::new(default_raw_config().await)
}

lazy_static! {
    static ref CONFIG_CREATER: AsyncOnce<ProviderConfig> = AsyncOnce::new(async {
       default_provider_config().await
    });
    static ref RAW_CONFIG_CREATER: AsyncOnce<RawProviderConfig> = AsyncOnce::new(async {
       default_raw_config().await
    });
}

#[tokio::test]
async fn test_equality() {
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let config = CONFIG_CREATER.get().await;
    assert_eq!(config.url(), &raw_config.url);
    assert_eq!(config.timeout_ms(), &raw_config.timeout_ms);
    assert_eq!(config.max_try_count(), &raw_config.max_try_count);
}

#[tokio::test]
async fn test_copy() {
    let config = CONFIG_CREATER.get().await;
    assert_eq!(
        &ProviderConfig::new(config.base.copy_data()),
        config
    );
}

#[tokio::test]
async fn test_mutate() {
    let mut raw_config = default_raw_config().await;
    let config = CONFIG_CREATER.get().await;
    assert_eq!(&config.mutate(None), config);
    raw_config.max_try_count = 10;
    let new_config = config.mutate(Some(raw_config));
    assert_eq!(*new_config.max_try_count(), 10);
}

#[tokio::test]
async fn test_to_json_string() {
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let config = CONFIG_CREATER.get().await;
    let json_string = config.base.to_json_string();
    let loaded_raw_config =
        RawConfig::create_from_json_string::<RawProviderConfig>(json_string.as_str()).await.unwrap();
    assert_eq!(&loaded_raw_config, raw_config);
}