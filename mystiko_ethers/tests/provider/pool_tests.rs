use crate::provider::common::TestProvider;
use anyhow::Result;
use async_trait::async_trait;
use mockall::{mock, predicate};
use mystiko_ethers::{
    ChainProvidersOptions, JsonRpcClientWrapper, Provider, ProviderFactory, ProviderOptions, ProviderPool,
    ProviderWrapper, Providers, ProvidersOptions,
};
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
    let mut pool = ProviderPool::<MockChainConfig>::builder()
        .chain_providers_options(mock_chain_config)
        .build();
    assert!(!pool.has_provider(56).await);
    assert!(pool.get_provider(1).await.is_err());
    assert!(pool.get_provider(56).await.is_ok());
    assert!(pool.has_provider(56).await);
    assert!(pool.get_provider(56).await.is_ok());
    assert!(pool.delete_provider(1).await.is_none());
    assert!(pool.delete_provider(56).await.is_some());
    assert!(pool.set_provider(1, Arc::new(create_test_provider())).await.is_none());
    assert!(pool.get_provider(1).await.is_ok());
    assert!(pool.has_provider(1).await);

    let mut mock_provider_factory = MockTestProviderFactory::new();
    mock_provider_factory
        .expect_create_provider()
        .returning(|_| Ok(create_test_provider()));
    assert!(!pool.has_provider(56).await);
    pool.set_provider_factory(Box::new(mock_provider_factory));
    assert!(pool.get_provider(56).await.is_ok());

    let box_pool = Box::new(pool);
    assert!(box_pool.get_provider(56).await.is_ok());
    assert!(box_pool.has_provider(56).await);
    assert!(box_pool
        .set_provider(1, Arc::new(create_test_provider()))
        .await
        .is_some());
    assert!(box_pool.delete_provider(56).await.is_some());
}

pub fn create_test_provider() -> Provider {
    let test_raw_provider = TestProvider {
        result: serde_json::to_value(String::from("0xdeadbeef")).unwrap(),
        ..TestProvider::default()
    };
    let test_raw_provider = Box::new(test_raw_provider) as Box<dyn JsonRpcClientWrapper>;
    ethers_providers::Provider::new(ProviderWrapper::new(test_raw_provider))
}
