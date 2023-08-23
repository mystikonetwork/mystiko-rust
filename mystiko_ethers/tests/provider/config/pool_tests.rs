use crate::provider::common::TestProvider;
use anyhow::Result;
use async_trait::async_trait;
use mockall::mock;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_ethers::provider::config::ChainConfigProvidersOptions;
use mystiko_ethers::provider::factory::{Provider, ProviderFactory, ProvidersOptions};
use mystiko_ethers::provider::pool::{ProviderPool, Providers};
use mystiko_ethers::provider::wrapper::{JsonRpcClientWrapper, ProviderWrapper};
use std::sync::Arc;

mock! {
    #[derive(Debug)]
    TestProviderFactory {}

    #[async_trait]
    impl ProviderFactory for TestProviderFactory {
        async fn create_provider(&self, providers_options: ProvidersOptions) -> Result<Provider>;
    }
}

#[tokio::test]
async fn test_create_pool_from_config() {
    let config = Arc::new(
        MystikoConfig::from_json_file("tests/files/provider/config/config.json")
            .await
            .unwrap(),
    );
    let mut mock_provider_factory = MockTestProviderFactory::new();
    mock_provider_factory
        .expect_create_provider()
        .returning(|_| Ok(create_test_provider()));
    let mut pool: ProviderPool<ChainConfigProvidersOptions> = config.into();
    pool.set_provider_factory(Box::new(mock_provider_factory));
    assert!(pool.get_provider(124343).await.is_err());
    assert!(pool.get_provider(5).await.is_ok());
}

pub fn create_test_provider() -> Provider {
    let test_raw_provider = TestProvider {
        result: serde_json::to_value(String::from("0xdeadbeef")).unwrap(),
        ..TestProvider::default()
    };
    let test_raw_provider = Box::new(test_raw_provider) as Box<dyn JsonRpcClientWrapper>;
    ethers_providers::Provider::new(ProviderWrapper::new(test_raw_provider))
}
