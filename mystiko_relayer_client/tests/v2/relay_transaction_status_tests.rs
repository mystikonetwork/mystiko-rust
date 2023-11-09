use crate::common::create_provider_pool;
use crate::v2::{
    create_client, mock_handshake_supported_server, mock_transaction_status_server, CHAIN_ID, SUPPORTED_API_VERSION,
};
use mockito::Server;
use mystiko_relayer_client::error::RelayerClientError;
use mystiko_relayer_client::v2::client::HANDSHAKE_URL_PATH;
use mystiko_relayer_types::response::success;
use mystiko_relayer_types::{HandshakeResponse, RelayTransactStatusRequest};
use serde_json::to_string;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::test]
async fn test_relay_transaction_status() {
    // mock server
    let mock_server = Arc::new(RwLock::new(Server::new_async().await));
    // mock url
    let mock_url = mock_server.write().await.url();

    // create client
    let pool = create_provider_pool(CHAIN_ID, None).await;
    let client = create_client(pool, None, None).await;

    // mock transaction status
    let mock_0 = mock_handshake_supported_server(mock_server.clone()).await;
    let mock_1 = mock_transaction_status_server(mock_server.clone()).await;
    let result = client
        .relay_transaction_status(RelayTransactStatusRequest {
            relayer_url: mock_url,
            uuid: "78d08829".to_string(),
        })
        .await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.uuid, "78d08829".to_string());
    assert_eq!(response.chain_id, CHAIN_ID);

    mock_0.assert_async().await;
    mock_1.assert_async().await;
}

#[tokio::test]
async fn test_relay_transaction_status_unsupported_api_version() {
    // mock server
    let mock_server = Arc::new(RwLock::new(Server::new_async().await));
    // mock url
    let mock_url = mock_server.write().await.url();

    // create client
    let pool = create_provider_pool(CHAIN_ID, None).await;
    let client = create_client(pool, None, None).await;

    let response = success(HandshakeResponse {
        package_version: "0.0.1".to_string(),
        api_version: vec!["v1".to_string()],
    });

    let mock = mock_server
        .write()
        .await
        .mock("GET", format!("/{}", HANDSHAKE_URL_PATH).as_str())
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(to_string(&response).unwrap())
        .create_async()
        .await;

    let result = client
        .relay_transaction_status(RelayTransactStatusRequest {
            relayer_url: mock_url,
            uuid: "78d08829".to_string(),
        })
        .await;

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        RelayerClientError::UnsupportedApiVersion(SUPPORTED_API_VERSION.to_string()).to_string()
    );

    mock.assert_async().await;
}
