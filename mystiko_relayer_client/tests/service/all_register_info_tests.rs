use crate::common::{
    create_client, create_provider_pool, deploy_contract, mock_handshake_supported_server, mock_register_info_server,
    register_relayer,
};
use ethers::signers::LocalWallet;
use ethers_core::utils::Anvil;
use log::LevelFilter;
use mockito::Server;
use mystiko_relayer_client::client::{RelayerClient, RelayerClientOptions};
use mystiko_relayer_client::error::RelayerClientError;
use mystiko_relayer_types::{RegisterInfoRequest, RegisterOptions};
use mystiko_types::CircuitType;
use std::sync::Arc;
use tokio::sync::RwLock;

const CHAIN_ID: u64 = 31337;

#[tokio::test]
async fn test_all_register_info_successful() {
    // mock server
    let mock_server = Arc::new(RwLock::new(Server::new_async().await));
    // mock url
    let mock_url = mock_server.write().await.url();

    // run anvil
    let anvil = Anvil::new().chain_id(CHAIN_ID).spawn();
    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let provider_url = anvil.endpoint();

    // deploy contract and register
    let contract_address = deploy_contract(CHAIN_ID, &provider_url, wallet.clone()).await;
    register_relayer(CHAIN_ID, wallet, &provider_url, contract_address, &mock_url).await;

    // mock handshake
    let mock_0 = mock_handshake_supported_server(mock_server.clone()).await;
    // mock register info
    let mock_1 = mock_register_info_server(mock_server.clone(), CHAIN_ID).await;

    // create provider pool
    let pool = create_provider_pool(CHAIN_ID, Some(provider_url)).await;
    let client = create_client(pool, Some(CHAIN_ID), Some(contract_address)).await;

    let result = client
        .all_register_info(RegisterInfoRequest {
            chain_id: CHAIN_ID,
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
    assert_eq!(register_info.chain_id, CHAIN_ID);
    assert_eq!(register_info.url, mock_url);

    mock_0.assert_async().await;
    mock_1.assert_async().await;
}

#[tokio::test]
async fn test_all_register_info_concurrent() {
    // mock server 0
    let mock_server_0 = Arc::new(RwLock::new(Server::new_async().await));
    let mock_url_0 = mock_server_0.write().await.url();
    let mock_0 = mock_handshake_supported_server(mock_server_0.clone()).await;
    let mock_1 = mock_register_info_server(mock_server_0.clone(), CHAIN_ID).await;

    // mock server 1
    let mock_server_1 = Arc::new(RwLock::new(Server::new_async().await));
    let mock_url_1 = mock_server_1.write().await.url();
    let mock_2 = mock_handshake_supported_server(mock_server_1.clone()).await;
    let mock_3 = mock_register_info_server(mock_server_1.clone(), CHAIN_ID).await;

    // run anvil
    let anvil = Anvil::new().chain_id(CHAIN_ID).spawn();
    let wallet_0: LocalWallet = anvil.keys()[0].clone().into();
    let wallet_1: LocalWallet = anvil.keys()[1].clone().into();
    let provider_url = anvil.endpoint();

    // deploy contract and register
    let contract_address = deploy_contract(CHAIN_ID, &provider_url, wallet_0.clone()).await;
    // register relayer 0
    register_relayer(CHAIN_ID, wallet_0, &provider_url, contract_address, &mock_url_0).await;
    // register relayer 1
    register_relayer(CHAIN_ID, wallet_1, &provider_url, contract_address, &mock_url_1).await;

    // create provider pool
    let pool = create_provider_pool(CHAIN_ID, Some(provider_url)).await;
    let client = create_client(pool, Some(CHAIN_ID), Some(contract_address)).await;

    let result = client
        .all_register_info(RegisterInfoRequest {
            chain_id: CHAIN_ID,
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

    let client = RelayerClient::new(
        Arc::new(RwLock::new(pool)),
        Some(
            RelayerClientOptions::builder()
                .is_testnet(true)
                .log_level(LevelFilter::Debug)
                .build(),
        ),
    )
    .await
    .unwrap();

    let result = client
        .all_register_info(RegisterInfoRequest {
            chain_id,
            options: None,
        })
        .await;
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        RelayerClientError::RelayerConfigNotFoundError(chain_id).to_string()
    );
}
