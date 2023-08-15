use crate::provider::common::TestProvider;
use anyhow::Result;
use async_trait::async_trait;
use mockall::{mock, predicate};
use mystiko_ethers::provider::factory::{Provider, ProviderFactory, ProvidersOptions};
use mystiko_ethers::provider::pool::{ChainProvidersOptions, ProviderPool, Providers};
use mystiko_ethers::provider::types::ProviderOptions;
use mystiko_ethers::provider::wrapper::{JsonRpcClientWrapper, ProviderWrapper};
use std::sync::Arc;

mock! {
    #[derive(Debug)]
    ChainConfig {}

    #[async_trait]
    impl ChainProvidersOptions for ChainConfig {
         async fn providers_options(&self, chain_id: u64) -> Result<Option<ProvidersOptions>>;
    }
}

mock! {
    #[derive(Debug)]
    TestProviderFactory {}

    #[async_trait]
    impl ProviderFactory for TestProviderFactory {
        async fn create_provider(&self, providers_options: ProvidersOptions) -> Result<Provider>;
    }
}

#[tokio::test]
async fn test_provider_pool() {
    let mut mock_chain_config = MockChainConfig::new();
    mock_chain_config
        .expect_providers_options()
        .with(predicate::eq(56))
        .times(2)
        .returning(|_| {
            Ok(Some(ProvidersOptions::Http(
                ProviderOptions::builder()
                    .url(String::from("https://localhost:8545"))
                    .build(),
            )))
        });
    mock_chain_config
        .expect_providers_options()
        .with(predicate::ne(56))
        .returning(|_| Ok(None));
    let mut pool = ProviderPool::builder()
        .chain_providers_options(Box::new(mock_chain_config))
        .build();
    assert!(pool.get_provider(56).is_none());
    assert!(!pool.has_provider(56));
    assert!(pool.check_provider(56).is_err());
    assert!(pool.get_or_create_provider(1).await.is_err());
    assert!(pool.get_or_create_provider(56).await.is_ok());
    assert!(pool.get_provider(56).is_some());
    assert!(pool.has_provider(56));
    assert!(pool.check_provider(56).is_ok());
    assert!(pool.get_or_create_provider(56).await.is_ok());
    assert!(pool.delete_provider(1).is_none());
    assert!(pool.delete_provider(56).is_some());
    pool.set_provider(1, Arc::new(create_test_provider()));
    assert!(pool.get_provider(1).is_some());
    assert!(pool.has_provider(1));
    assert!(pool.get_or_create_provider(1).await.is_ok());

    let mut mock_provider_factory = MockTestProviderFactory::new();
    mock_provider_factory
        .expect_create_provider()
        .returning(|_| Ok(create_test_provider()));
    assert!(!pool.has_provider(56));
    pool.set_provider_factory(Box::new(mock_provider_factory));
    assert!(pool.get_or_create_provider(56).await.is_ok());
}

pub fn create_test_provider() -> Provider {
    let test_raw_provider = TestProvider {
        result: serde_json::to_value(String::from("0xdeadbeef")).unwrap(),
        ..TestProvider::default()
    };
    let test_raw_provider = Box::new(test_raw_provider) as Box<dyn JsonRpcClientWrapper>;
    ethers_providers::Provider::new(ProviderWrapper::new(test_raw_provider))
}
