use anyhow::Result;
use async_trait::async_trait;
use mockall::mock;
use mystiko_ethers::provider::factory::ProvidersOptions;
use mystiko_ethers::provider::pool::{ChainProvidersOptions, ProviderPool};
use mystiko_relayer_client::client::{RelayerClient, RelayerClientOptions};
use std::sync::Arc;
use tokio::sync::RwLock;

mock! {
    #[derive(Debug)]
    ChainConfig {}

    #[async_trait]
    impl ChainProvidersOptions for ChainConfig {
         async fn providers_options(&self, chain_id: u64) -> Result<Option<ProvidersOptions>>;
    }
}

#[tokio::test]
async fn test_create_with_config_file() {
    let relayer_options = RelayerClientOptions::builder()
        .relayer_config_file_path(String::from("tests/files/relayer/config.json"))
        .build();
    let mock_chain_config = MockChainConfig::new();
    let pool = ProviderPool::builder()
        .chain_providers_options(Box::new(mock_chain_config))
        .build();
    let relayer_client = RelayerClient::new(Arc::new(RwLock::new(pool)), Some(relayer_options)).await;
    assert!(relayer_client.is_ok());
    assert_eq!(relayer_client.unwrap().relayer_config.version(), "0.1.0");
}
