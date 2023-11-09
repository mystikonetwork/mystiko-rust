use crate::common::{create_relayer_config, relayer_config_json_string, MockChainConfig};
use ethers_core::types::Address;
use log::LevelFilter;
use mockito::{Mock, Server, ServerGuard};
use mystiko_ethers::ProviderPool;
use mystiko_protos::core::v1::SpendType;
use mystiko_relayer_client::error::RelayerClientError;
use mystiko_relayer_client::v2::client::{
    RelayerClientOptions, RelayerClientV2, HANDSHAKE_URL_PATH, INFO_URL_PATH, TRANSACTION_STATUS_URL_PATH,
    TRANSACT_URL_PATH,
};
use mystiko_relayer_client::RelayerClient;
use mystiko_relayer_config::wrapper::relayer::RelayerConfig;
use mystiko_relayer_types::response::success;
use mystiko_relayer_types::{
    HandshakeResponse, RegisterInfoResponse, RelayTransactResponse, RelayTransactStatusResponse, TransactStatus,
};
use serde_json::to_string;
use std::sync::Arc;
use tokio::sync::RwLock;

mod all_register_info_tests;
mod relay_transact_tests;
mod relay_transaction_status_tests;
mod wait_transaction_tests;

const CHAIN_ID: u64 = 31337;
const SUPPORTED_API_VERSION: &str = "v2";
const MOCK_CONTRACT_ADDRESS: &str = "0x45B22A8CefDfF00989882CAE48Ad06D57938Efcc";
const RELAYER_CONFIG_FILE_PATH: &str = "tests/files/relayer/config.json";

async fn mock_handshake_supported_server(server: Arc<RwLock<ServerGuard>>) -> Mock {
    // mock handshake response
    let response = success(HandshakeResponse {
        package_version: "0.0.1".to_string(),
        api_version: vec![SUPPORTED_API_VERSION.to_string()],
    });

    server
        .write()
        .await
        .mock("GET", format!("/{}", HANDSHAKE_URL_PATH).as_str())
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(to_string(&response).unwrap())
        .create_async()
        .await
}

async fn mock_register_info_server(server: Arc<RwLock<ServerGuard>>, chain_id: u64) -> Mock {
    // mock info response
    let response = success(RegisterInfoResponse {
        chain_id,
        support: true,
        available: Some(true),
        relayer_contract_address: Some(MOCK_CONTRACT_ADDRESS.to_string()),
        contracts: Some(vec![]),
    });
    server
        .write()
        .await
        .mock("POST", format!("/{}", INFO_URL_PATH).as_str())
        .expect(1)
        .with_status(200)
        .with_body(to_string(&response).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await
}

async fn mock_relay_transact_server(server: Arc<RwLock<ServerGuard>>) -> Mock {
    let response = success(RelayTransactResponse {
        uuid: "abcd123456".to_string(),
    });
    server
        .write()
        .await
        .mock("POST", format!("/{}", TRANSACT_URL_PATH).as_str())
        .with_status(200)
        .with_body(to_string(&response).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await
}

async fn mock_transaction_status_server(server: Arc<RwLock<ServerGuard>>) -> Mock {
    let response = success(RelayTransactStatusResponse {
        uuid: String::from("78d08829"),
        chain_id: 31337,
        spend_type: SpendType::Withdraw,
        status: TransactStatus::Queued,
        transaction_hash: None,
        error_msg: None,
    });
    server
        .write()
        .await
        .mock("GET", format!("/{}/78d08829", TRANSACTION_STATUS_URL_PATH).as_str())
        .with_status(200)
        .with_body(to_string(&response).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await
}

async fn create_client(
    provider_pool: ProviderPool<MockChainConfig>,
    chain_id: Option<u64>,
    contract_address: Option<Address>,
) -> Box<dyn RelayerClient<Error = RelayerClientError>> {
    let _ = env_logger::builder()
        .filter_module("mystiko_relayer_client", LevelFilter::Debug)
        .try_init();

    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/relayer_config/production/testnet/latest.json")
        .with_body(relayer_config_json_string())
        .create_async()
        .await;

    let mut client = RelayerClientV2::new(
        Arc::new(provider_pool),
        Some(
            RelayerClientOptions::builder()
                .is_testnet(true)
                .relayer_config_remote_base_url(format!("{}/relayer_config", server.url()))
                .build(),
        ),
    )
    .await
    .unwrap();
    mock.assert_async().await;

    if let (Some(chain), Some(address)) = (chain_id, contract_address) {
        client.relayer_config =
            Arc::new(RelayerConfig::from_json_str(create_relayer_config(chain, address).as_str()).unwrap());
    }

    Box::new(client) as Box<dyn RelayerClient<Error = RelayerClientError>>
}

#[tokio::test]
async fn test_create_with_config_file() {
    let relayer_options = RelayerClientOptions::builder()
        .relayer_config_file_path(String::from(RELAYER_CONFIG_FILE_PATH))
        .build();
    let mock_chain_config = MockChainConfig::new();
    let pool = ProviderPool::<MockChainConfig>::builder()
        .chain_providers_options(mock_chain_config)
        .build();
    let client = RelayerClientV2::new(Arc::new(pool), Some(relayer_options)).await;
    assert!(client.is_ok());
    assert_eq!(client.unwrap().relayer_config.version(), "0.1.0");
}
