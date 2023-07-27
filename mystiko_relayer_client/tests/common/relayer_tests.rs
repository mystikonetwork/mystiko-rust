use crate::common::{create_client, create_provider_pool, mock_handshake_supported_server};
use anyhow::Result;
use async_trait::async_trait;
use mockall::mock;
use mockito::Server;
use mystiko_ethers::provider::factory::ProvidersOptions;
use mystiko_ethers::provider::pool::{ChainProvidersOptions, ProviderPool};
use mystiko_relayer_client::client::{RelayerClient, RelayerClientOptions, TRANSACTION_STATUS_URL_PATH};
use mystiko_relayer_client::error::RelayerClientError;
use mystiko_relayer_types::response::{failed, success, ResponseCode};
use mystiko_relayer_types::{RelayTransactStatusRequest, RelayTransactStatusResponse, TransactStatus};
use mystiko_types::TransactionType;
use serde_json::to_string;
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

#[tokio::test]
async fn test_handle_error_response() {
    // mock server
    let mock_server = Arc::new(RwLock::new(Server::new_async().await));
    // mock url
    let mock_url = mock_server.write().await.url();

    // create client
    let pool = create_provider_pool(97, None).await;
    let client = create_client(pool, None, None).await;

    // mock handshake
    let _mock_0 = mock_handshake_supported_server(mock_server.clone()).await;
    let mock_1 = mock_server
        .write()
        .await
        .mock("GET", format!("/{}/123456", TRANSACTION_STATUS_URL_PATH).as_str())
        .with_status(200)
        .with_body(
            to_string(&failed(
                Some(RelayTransactStatusResponse {
                    uuid: "".to_string(),
                    chain_id: 97,
                    transaction_type: TransactionType::Transfer,
                    status: TransactStatus::Queued,
                    transaction_hash: None,
                    error_msg: None,
                }),
                Some(String::from("failed")),
            ))
            .unwrap(),
        )
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let result = client
        .relay_transaction_status(RelayTransactStatusRequest {
            relayer_url: mock_url.clone(),
            uuid: "123456".to_string(),
        })
        .await;
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        RelayerClientError::ApiResponseError {
            code: ResponseCode::Failed as i32,
            message: String::from("failed")
        }
        .to_string()
    );

    mock_1.assert_async().await;

    let mock_2 = mock_server
        .write()
        .await
        .mock("GET", format!("/{}/123456", TRANSACTION_STATUS_URL_PATH).as_str())
        .with_status(200)
        .with_body(
            to_string(&success(Some(RelayTransactStatusResponse {
                uuid: "".to_string(),
                chain_id: 97,
                transaction_type: TransactionType::Transfer,
                status: TransactStatus::Succeeded,
                transaction_hash: None,
                error_msg: None,
            })))
            .unwrap(),
        )
        .create_async()
        .await;

    let result = client
        .relay_transaction_status(RelayTransactStatusRequest {
            relayer_url: mock_url,
            uuid: "123456".to_string(),
        })
        .await;

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        RelayerClientError::UnsupportedContentTypeError("".to_string()).to_string()
    );

    mock_2.assert_async().await;
}
