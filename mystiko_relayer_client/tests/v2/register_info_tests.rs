use crate::common::{create_provider_pool, deploy_contract, register_relayer, relayer_config_json_string};
use crate::v2::{create_client, mock_handshake_supported_server, mock_register_info_server, CHAIN_ID};
use ethers::signers::LocalWallet;
use ethers_core::utils::Anvil;
use log::LevelFilter;
use mockito::Server;
use mystiko_protos::relayer::v1::RelayerClientOptions;
use mystiko_relayer_client::error::RelayerClientError;
use mystiko_relayer_client::v2::client::RelayerClientV2;
use mystiko_relayer_client::RelayerClient;
use mystiko_relayer_types::{RegisterInfoRequest, RegisterOptions};
use mystiko_types::CircuitType;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::test]
async fn test_register_info_successful() {
    // mock server
    let mock_server = Arc::new(RwLock::new(Server::new_async().await));
    // mock url
    let mock_url = mock_server.write().await.url();

    // start anvil
    let anvil = Anvil::new().chain_id(CHAIN_ID).spawn();
    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let provider_url = anvil.endpoint();

    // deploy contract and register
    let contract_address = deploy_contract(CHAIN_ID, &provider_url, wallet.clone()).await;
    register_relayer(CHAIN_ID, wallet, &provider_url, contract_address, &mock_url, None).await;

    // mock handshake
    let mock_0 = mock_handshake_supported_server(mock_server.clone(), 1).await;
    // mock register info
    let mock_1 = mock_register_info_server(mock_server.clone(), CHAIN_ID, 1).await;

    // create provider pool
    let pool = create_provider_pool(CHAIN_ID, Some(provider_url)).await;
    let client = create_client(pool, Some(CHAIN_ID), Some(contract_address)).await;

    let result = client
        .register_info(RegisterInfoRequest {
            chain_id: CHAIN_ID,
            name: None,
            options: Some(RegisterOptions {
                asset_symbol: "TEST".to_string(),
                circuit_type: CircuitType::Transaction1x0,
                show_unavailable: false,
            }),
        })
        .await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.len(), 1);

    let register_info = &response[0];
    assert!(register_info.support);
    assert!(register_info.available);
    assert_eq!(register_info.chain_id, CHAIN_ID);
    assert_eq!(register_info.url, mock_url);
    assert_eq!(register_info.name, format!("test_{}", mock_url));
    assert_eq!(
        register_info.relayer_address,
        "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
    );
    assert_eq!(
        register_info.relayer_contract_address,
        "0x45B22A8CefDfF00989882CAE48Ad06D57938Efcc"
    );

    mock_0.assert_async().await;
    mock_1.assert_async().await;
}

#[tokio::test]
async fn test_register_info_concurrent() {
    // mock server 0
    let mock_server_0 = Arc::new(RwLock::new(Server::new_async().await));
    let mock_url_0 = mock_server_0.write().await.url();
    let mock_0 = mock_handshake_supported_server(mock_server_0.clone(), 1).await;
    let mock_1 = mock_register_info_server(mock_server_0.clone(), CHAIN_ID, 1).await;

    // mock server 1
    let mock_server_1 = Arc::new(RwLock::new(Server::new_async().await));
    let mock_url_1 = mock_server_1.write().await.url();
    let mock_2 = mock_handshake_supported_server(mock_server_1.clone(), 1).await;
    let mock_3 = mock_register_info_server(mock_server_1.clone(), CHAIN_ID, 1).await;

    // start anvil
    let anvil = Anvil::new().chain_id(CHAIN_ID).spawn();
    let wallet_0: LocalWallet = anvil.keys()[0].clone().into();
    let wallet_1: LocalWallet = anvil.keys()[1].clone().into();
    let provider_url = anvil.endpoint();

    // deploy contract and register
    let contract_address = deploy_contract(CHAIN_ID, &provider_url, wallet_0.clone()).await;
    // register relayer 0
    register_relayer(CHAIN_ID, wallet_0, &provider_url, contract_address, &mock_url_0, None).await;
    // register relayer 1
    register_relayer(CHAIN_ID, wallet_1, &provider_url, contract_address, &mock_url_1, None).await;

    // create provider pool
    let pool = create_provider_pool(CHAIN_ID, Some(provider_url)).await;
    let client = create_client(pool, Some(CHAIN_ID), Some(contract_address)).await;

    let result = client
        .register_info(RegisterInfoRequest {
            chain_id: CHAIN_ID,
            name: None,
            options: Some(RegisterOptions {
                asset_symbol: "TEST".to_string(),
                circuit_type: CircuitType::Transaction1x0,
                show_unavailable: false,
            }),
        })
        .await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.len(), 2);

    let register_info_0 = &response[0];
    assert_eq!(register_info_0.chain_id, CHAIN_ID);

    let register_info_1 = &response[0];
    assert_eq!(register_info_1.chain_id, CHAIN_ID);

    mock_0.assert_async().await;
    mock_1.assert_async().await;
    mock_2.assert_async().await;
    mock_3.assert_async().await;
}

#[tokio::test]
async fn test_relayer_config_not_found() {
    let chain_id = 19999;
    let pool = create_provider_pool(chain_id, Some("http://127.0.0.1:50009".to_string())).await;

    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/relayer_config/production/testnet/latest.json")
        .with_body(relayer_config_json_string())
        .create_async()
        .await;

    let _ = env_logger::builder()
        .filter_module("mystiko_relayer_client", LevelFilter::Debug)
        .try_init();

    let client = RelayerClientV2::new(
        Arc::new(pool),
        RelayerClientOptions::builder()
            .is_testnet(true)
            .relayer_config_remote_base_url(format!("{}/relayer_config", server.url()))
            .build(),
    )
    .await
    .unwrap();

    mock.assert_async().await;

    let result = client
        .register_info(RegisterInfoRequest {
            chain_id,
            name: None,
            options: None,
        })
        .await;
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        RelayerClientError::RelayerConfigNotFoundError(chain_id).to_string()
    );
}

#[tokio::test]
async fn test_register_info_with_name() {
    // mock server 0
    let mock_server_0 = Arc::new(RwLock::new(Server::new_async().await));
    let mock_url_0 = mock_server_0.write().await.url();
    let mock_0 = mock_handshake_supported_server(mock_server_0.clone(), 2).await;
    let mock_1 = mock_register_info_server(mock_server_0.clone(), CHAIN_ID, 2).await;

    // mock server 1
    let mock_server_1 = Arc::new(RwLock::new(Server::new_async().await));
    let mock_url_1 = mock_server_1.write().await.url();
    let mock_2 = mock_handshake_supported_server(mock_server_1.clone(), 2).await;
    let mock_3 = mock_register_info_server(mock_server_1.clone(), CHAIN_ID, 2).await;

    // start anvil
    let anvil = Anvil::new().chain_id(CHAIN_ID).spawn();
    let wallet_0: LocalWallet = anvil.keys()[0].clone().into();
    let wallet_1: LocalWallet = anvil.keys()[1].clone().into();
    let provider_url = anvil.endpoint();

    // deploy contract
    let contract_address = deploy_contract(CHAIN_ID, &provider_url, wallet_0.clone()).await;
    // register relayer 0
    register_relayer(
        CHAIN_ID,
        wallet_0,
        &provider_url,
        contract_address,
        &mock_url_0,
        Some("Thor"),
    )
    .await;
    // register relayer 1
    register_relayer(
        CHAIN_ID,
        wallet_1,
        &provider_url,
        contract_address,
        &mock_url_1,
        Some("Groot"),
    )
    .await;

    // create provider pool
    let pool = create_provider_pool(CHAIN_ID, Some(provider_url)).await;
    let client = create_client(pool, Some(CHAIN_ID), Some(contract_address)).await;

    // get all register
    let result = client
        .register_info(RegisterInfoRequest {
            chain_id: CHAIN_ID,
            name: None,
            options: Some(RegisterOptions {
                asset_symbol: "TEST".to_string(),
                circuit_type: CircuitType::Transaction1x0,
                show_unavailable: false,
            }),
        })
        .await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.len(), 2);

    // get register thor
    let result = client
        .register_info(RegisterInfoRequest {
            chain_id: CHAIN_ID,
            name: Some("Thor".to_string()),
            options: Some(RegisterOptions {
                asset_symbol: "TEST".to_string(),
                circuit_type: CircuitType::Transaction1x0,
                show_unavailable: false,
            }),
        })
        .await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.len(), 1);
    assert_eq!(response[0].url, mock_url_0);
    assert_eq!(response[0].name, "Thor");

    // get register groot
    let result = client
        .register_info(RegisterInfoRequest {
            chain_id: CHAIN_ID,
            name: Some("Groot".to_string()),
            options: Some(RegisterOptions {
                asset_symbol: "TEST".to_string(),
                circuit_type: CircuitType::Transaction1x0,
                show_unavailable: false,
            }),
        })
        .await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.len(), 1);
    assert_eq!(response[0].url, mock_url_1);
    assert_eq!(response[0].name, "Groot");

    mock_0.assert_async().await;
    mock_1.assert_async().await;
    mock_2.assert_async().await;
    mock_3.assert_async().await;
}
