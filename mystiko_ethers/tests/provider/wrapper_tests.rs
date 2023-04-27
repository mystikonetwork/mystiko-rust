use crate::provider::common::TestProvider;
use ethers_providers::{Middleware, Provider, ProviderError};
use mystiko_ethers::provider::wrapper::ProviderWrapper;

#[tokio::test]
async fn test_provider_wrapper() {
    let inner = TestProvider {
        result: serde_json::to_value(String::from("0xdeadbeef")).unwrap(),
        ..TestProvider::default()
    };
    let wrapper = ProviderWrapper::new(inner);
    let provider = Provider::new(wrapper);
    assert_eq!(provider.get_block_number().await.unwrap().as_u64(), 0xdeadbeef);
}

#[tokio::test]
async fn test_provider_wrapper_with_params() {
    let inner = TestProvider {
        result: serde_json::to_value(String::from("0xdeadbeef")).unwrap(),
        ..TestProvider::default()
    };
    let wrapper = ProviderWrapper::new(inner);
    let provider = Provider::new(wrapper);
    assert_eq!(provider.get_uncle_count(1224).await.unwrap().as_u64(), 0xdeadbeef);
}

#[tokio::test]
async fn test_provider_wrapper_with_error() {
    let inner = TestProvider {
        error: Some(|| ProviderError::UnsupportedRPC),
        ..TestProvider::default()
    };
    let wrapper = ProviderWrapper::new(inner);
    let provider = Provider::new(wrapper);
    assert!(provider.get_block_number().await.is_err());
}
