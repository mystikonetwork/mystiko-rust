use crate::common::create_provider_pool;
use crate::v2::{create_client, mock_handshake_supported_server, CHAIN_ID};
use mockito::Server;
use mystiko_protos::core::v1::SpendType;
use mystiko_relayer_client::error::RelayerClientError;
use mystiko_relayer_client::v2::client::TRANSACTION_STATUS_URL_PATH;
use mystiko_relayer_types::response::success;
use mystiko_relayer_types::{RelayTransactStatusResponse, TransactStatus, WaitingTransactionRequest};
use serde_json::to_string;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

#[tokio::test]
async fn wait_transaction_until_queued() {
    // mock server
    let mock_server = Arc::new(RwLock::new(Server::new_async().await));
    // mock url
    let mock_url = mock_server.write().await.url();

    // create client
    let pool = create_provider_pool(CHAIN_ID, None).await;
    let client = create_client(pool, None, None).await;

    // mock transact
    let mock_0 = mock_handshake_supported_server(mock_server.clone(), 1).await;
    let mock_1 = mock_server
        .write()
        .await
        .mock("GET", format!("/{}/78d08829", TRANSACTION_STATUS_URL_PATH).as_str())
        .with_status(200)
        .with_body(
            to_string(&success(RelayTransactStatusResponse {
                uuid: "78d08829".to_string(),
                chain_id: CHAIN_ID,
                spend_type: SpendType::Withdraw,
                status: TransactStatus::Queued,
                transaction_hash: None,
                error_msg: None,
            }))
            .unwrap(),
        )
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    let result = client
        .wait_transaction(WaitingTransactionRequest {
            relayer_url: mock_url,
            uuid: "78d08829".to_string(),
            waiting_status: TransactStatus::Queued,
            timeout: Default::default(),
            interval: None,
        })
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, TransactStatus::Queued);

    mock_0.assert_async().await;
    mock_1.assert_async().await;
}

#[tokio::test]
async fn wait_transaction_until_pending() {
    // mock server
    let mock_server = Arc::new(RwLock::new(Server::new_async().await));
    // mock url
    let mock_url = mock_server.write().await.url();

    // create client
    let pool = create_provider_pool(CHAIN_ID, None).await;
    let client = create_client(pool, None, None).await;

    // mock transact
    let mock_0 = mock_handshake_supported_server(mock_server.clone(), 1).await;
    let mock_1 = mock_server
        .write()
        .await
        .mock("GET", format!("/{}/78d08829", TRANSACTION_STATUS_URL_PATH).as_str())
        .with_status(200)
        .with_body(
            to_string(&success(RelayTransactStatusResponse {
                uuid: "78d08829".to_string(),
                chain_id: CHAIN_ID,
                spend_type: SpendType::Withdraw,
                status: TransactStatus::Pending,
                transaction_hash: None,
                error_msg: None,
            }))
            .unwrap(),
        )
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    let result = client
        .wait_transaction(WaitingTransactionRequest {
            relayer_url: mock_url,
            uuid: "78d08829".to_string(),
            waiting_status: TransactStatus::Pending,
            timeout: Default::default(),
            interval: None,
        })
        .await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, TransactStatus::Pending);

    mock_0.assert_async().await;
    mock_1.assert_async().await;
}

#[tokio::test]
async fn wait_transaction_until_succeeded() {
    // mock server
    let mock_server = Arc::new(RwLock::new(Server::new_async().await));
    // mock url
    let mock_url = mock_server.write().await.url();

    // create client
    let pool = create_provider_pool(CHAIN_ID, None).await;
    let client = create_client(pool, None, None).await;

    // mock transact
    let mock_0 = mock_handshake_supported_server(mock_server.clone(), 1).await;
    let mock_1 = mock_server
        .write()
        .await
        .mock("GET", format!("/{}/78d08829", TRANSACTION_STATUS_URL_PATH).as_str())
        .with_status(200)
        .with_body(
            to_string(&success(RelayTransactStatusResponse {
                uuid: "78d08829".to_string(),
                chain_id: CHAIN_ID,
                spend_type: SpendType::Withdraw,
                status: TransactStatus::Succeeded,
                transaction_hash: None,
                error_msg: None,
            }))
            .unwrap(),
        )
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    let result = client
        .wait_transaction(WaitingTransactionRequest {
            relayer_url: mock_url,
            uuid: "78d08829".to_string(),
            waiting_status: TransactStatus::Succeeded,
            timeout: Default::default(),
            interval: None,
        })
        .await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, TransactStatus::Succeeded);

    mock_0.assert_async().await;
    mock_1.assert_async().await;
}

#[tokio::test]
async fn wait_transaction_until_confirmed_timeout() {
    // mock server
    let mock_server = Arc::new(RwLock::new(Server::new_async().await));
    // mock url
    let mock_url = mock_server.write().await.url();

    // create client
    let pool = create_provider_pool(CHAIN_ID, None).await;
    let client = create_client(pool, None, None).await;

    // mock transact
    let mock_0 = mock_handshake_supported_server(mock_server.clone(), 1).await;
    let mock_1 = mock_server
        .write()
        .await
        .mock("GET", format!("/{}/78d08829", TRANSACTION_STATUS_URL_PATH).as_str())
        .with_status(200)
        .with_body(
            to_string(&success(RelayTransactStatusResponse {
                uuid: "78d08829".to_string(),
                chain_id: CHAIN_ID,
                spend_type: SpendType::Withdraw,
                status: TransactStatus::Queued,
                transaction_hash: None,
                error_msg: None,
            }))
            .unwrap(),
        )
        .with_header("content-type", "application/json")
        .expect_at_least(1)
        .create_async()
        .await;

    let result = client
        .wait_transaction(
            WaitingTransactionRequest::builder()
                .relayer_url(mock_url)
                .uuid("78d08829".to_string())
                .timeout(Duration::from_secs(5))
                .build(),
        )
        .await;
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        RelayerClientError::WaitTransactionTimeout("78d08829".to_string()).to_string()
    );

    mock_0.assert_async().await;
    mock_1.assert_async().await;
}

#[tokio::test]
async fn wait_transaction_until_confirmed_failed() {
    // mock server
    let mock_server = Arc::new(RwLock::new(Server::new_async().await));
    // mock url
    let mock_url = mock_server.write().await.url();

    // create client
    let pool = create_provider_pool(CHAIN_ID, None).await;
    let client = create_client(pool, None, None).await;

    // mock transact
    let mock_0 = mock_handshake_supported_server(mock_server.clone(), 1).await;
    let mock_1 = mock_server
        .write()
        .await
        .mock("GET", format!("/{}/78d08829", TRANSACTION_STATUS_URL_PATH).as_str())
        .with_status(200)
        .with_body(
            to_string(&success(RelayTransactStatusResponse {
                uuid: "78d08829".to_string(),
                chain_id: CHAIN_ID,
                spend_type: SpendType::Withdraw,
                status: TransactStatus::Failed,
                transaction_hash: None,
                error_msg: Some("test error message".to_string()),
            }))
            .unwrap(),
        )
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    let result = client
        .wait_transaction(WaitingTransactionRequest {
            relayer_url: mock_url,
            uuid: "78d08829".to_string(),
            waiting_status: TransactStatus::Failed,
            timeout: Default::default(),
            interval: None,
        })
        .await;
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        RelayerClientError::TransactionFailed(mock_server.write().await.url(), String::from("78d08829")).to_string()
    );

    mock_0.assert_async().await;
    mock_1.assert_async().await;
}
