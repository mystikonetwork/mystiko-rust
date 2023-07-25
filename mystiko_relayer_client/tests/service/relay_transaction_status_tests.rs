use crate::common::{create_client, create_provider_pool, mock_handshake_supported_server, mock_transaction_status};
use mockito::Server;
use mystiko_relayer_types::RelayTransactStatusRequest;
use std::sync::Arc;
use tokio::sync::RwLock;

const CHAIN_ID: u64 = 31337;

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
    let mock_1 = mock_transaction_status(mock_server.clone()).await;
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
