use crate::common::{create_client, create_provider_pool, mock_handshake_supported_server, mock_relay_transact_server};
use mockito::Server;
use mystiko_relayer_client::client::{HANDSHAKE_URL_PATH, SUPPORTED_API_VERSION};
use mystiko_relayer_client::error::RelayerClientError;
use mystiko_relayer_types::response::success;
use mystiko_relayer_types::{HandshakeResponse, RelayTransactRequest, TransactRequestData};
use mystiko_types::{BridgeType, CircuitType, TransactionType};
use serde_json::to_string;
use std::sync::Arc;
use tokio::sync::RwLock;

const CHAIN_ID: u64 = 31337;

#[tokio::test]
async fn test_relay_transact() {
    // mock server
    let mock_server = Arc::new(RwLock::new(Server::new_async().await));
    // mock url
    let mock_url = mock_server.write().await.url();

    // create client
    let pool = create_provider_pool(CHAIN_ID, None).await;
    let client = create_client(pool, None, None).await;

    // mock transact
    let mock_0 = mock_handshake_supported_server(mock_server.clone()).await;
    let mock_1 = mock_relay_transact_server(mock_server.clone()).await;
    let result = client
        .relay_transact(RelayTransactRequest {
            relayer_url: mock_url,
            data: TransactRequestData {
                contract_param: Default::default(),
                transaction_type: TransactionType::Withdraw,
                bridge_type: BridgeType::Loop,
                chain_id: CHAIN_ID,
                asset_symbol: "MTT".to_string(),
                asset_decimals: 16,
                pool_address: "0x45B22A8CefDfF00989882CAE48Ad06D57938Efcc".to_string(),
                circuit_type: CircuitType::Transaction1x0,
                signature: "".to_string(),
            },
        })
        .await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.uuid, "abcd123456".to_string());

    mock_0.assert_async().await;
    mock_1.assert_async().await;
}

#[tokio::test]
async fn test_relay_transact_invalid() {
    // create client
    let pool = create_provider_pool(CHAIN_ID, None).await;
    let client = create_client(pool, None, None).await;

    let result = client
        .relay_transact(RelayTransactRequest {
            relayer_url: "http://localhost:8090".to_string(),
            data: TransactRequestData {
                contract_param: Default::default(),
                transaction_type: TransactionType::Withdraw,
                bridge_type: BridgeType::Loop,
                chain_id: CHAIN_ID,
                asset_symbol: "MTT".to_string(),
                asset_decimals: 16,
                pool_address: "0x45B22A8CefDfF00989882CAE48Ad06D57938Efcc".to_string(),
                circuit_type: CircuitType::Rollup1,
                signature: "".to_string(),
            },
        })
        .await;
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        String::from("data.circuit_type: Validation error: invalid circuit type [{\"value\": String(\"rollup1\")}]")
    );
}

#[tokio::test]
async fn test_relay_transact_unsupported_api_version() {
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
        .relay_transact(RelayTransactRequest {
            relayer_url: mock_url,
            data: TransactRequestData {
                contract_param: Default::default(),
                transaction_type: TransactionType::Withdraw,
                bridge_type: BridgeType::Loop,
                chain_id: CHAIN_ID,
                asset_symbol: "MTT".to_string(),
                asset_decimals: 16,
                pool_address: "0x45B22A8CefDfF00989882CAE48Ad06D57938Efcc".to_string(),
                circuit_type: CircuitType::Transaction1x0,
                signature: "".to_string(),
            },
        })
        .await;

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        RelayerClientError::UnsupportedApiVersion(SUPPORTED_API_VERSION.to_string()).to_string()
    );

    mock.assert_async().await;
}
