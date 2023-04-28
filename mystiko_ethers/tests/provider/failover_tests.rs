use crate::provider::common::TestProvider;
use ethers_providers::{HttpClientError, JsonRpcError, Middleware, Provider, ProviderError};
use mystiko_ethers::provider::failover::{FailoverPolicy, FailoverProvider};
use mystiko_ethers::provider::wrapper::JsonRpcClientWrapper;

#[derive(Debug, Default)]
struct DoNotFailoverPolicy {}

impl FailoverPolicy for DoNotFailoverPolicy {
    fn should_failover(&self, _error: &ProviderError) -> bool {
        false
    }
}

#[tokio::test]
async fn test_failover_provider() {
    let test_provider1 = TestProvider {
        error: Some(|| {
            ProviderError::JsonRpcClientError(Box::new(HttpClientError::JsonRpcError(JsonRpcError {
                code: -1,
                message: String::from("something is wrong"),
                data: None,
            })))
        }),
        ..TestProvider::default()
    };
    let test_provider2 = TestProvider {
        error: Some(|| ProviderError::UnsupportedRPC),
        ..TestProvider::default()
    };
    let test_provider3 = TestProvider {
        error: Some(|| ProviderError::UnsupportedNodeClient),
        ..TestProvider::default()
    };
    let test_provider4 = TestProvider {
        result: serde_json::to_value(String::from("0xdeadbeef")).unwrap(),
        ..TestProvider::default()
    };
    let failure_providers: Vec<Box<dyn JsonRpcClientWrapper>> = vec![
        Box::new(test_provider1),
        Box::new(test_provider2),
        Box::new(test_provider3),
    ];
    let failover = FailoverProvider::dyn_rpc()
        .add_providers(failure_providers)
        .add_provider(Box::new(test_provider4))
        .build();
    assert_eq!(failover.providers().len(), 4);
    let provider = Provider::<FailoverProvider>::new(failover);
    let block_number = provider.get_block_number().await.unwrap();
    assert_eq!(block_number.as_u64(), 0xdeadbeef);
}

#[tokio::test]
async fn test_no_provider_available() {
    let failover = FailoverProvider::dyn_rpc().build();
    let provider = Provider::<FailoverProvider>::new(failover);
    let block_number = provider.get_block_number().await;
    assert!(block_number.is_err());
}

#[tokio::test]
async fn test_failure_cannot_failover() {
    let test_provider1 = TestProvider {
        error: Some(|| ProviderError::UnsupportedRPC),
        ..TestProvider::default()
    };
    let test_provider2 = TestProvider {
        result: serde_json::to_value(String::from("0xdeadbeef")).unwrap(),
        ..TestProvider::default()
    };
    let failure_providers: Vec<Box<dyn JsonRpcClientWrapper>> =
        vec![Box::new(test_provider1), Box::new(test_provider2)];
    let failover = FailoverProvider::dyn_rpc()
        .failover_policy(Box::<DoNotFailoverPolicy>::default())
        .add_providers(failure_providers)
        .build();
    let provider = Provider::<FailoverProvider>::new(failover);
    let block_number = provider.get_block_number().await;
    assert!(block_number.is_err());
}

#[tokio::test]
async fn test_failover_request_with_params() {
    let test_provider1 = TestProvider {
        error: Some(|| ProviderError::UnsupportedRPC),
        ..TestProvider::default()
    };
    let test_provider2 = TestProvider {
        result: serde_json::to_value(String::from("0xdeadbeef")).unwrap(),
        ..TestProvider::default()
    };
    let failure_providers: Vec<Box<dyn JsonRpcClientWrapper>> =
        vec![Box::new(test_provider1), Box::new(test_provider2)];
    let failover = FailoverProvider::dyn_rpc().add_providers(failure_providers).build();
    let provider = Provider::<FailoverProvider>::new(failover);
    let uncle_count = provider.get_uncle_count(0xbaadbabeu64).await.unwrap();
    assert_eq!(uncle_count.as_u64(), 0xdeadbeef);
}
