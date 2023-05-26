use anyhow::Result;
use async_trait::async_trait;
use ethers::{
    core::utils::Anvil,
    middleware::SignerMiddleware,
    providers::Http,
    signers::{LocalWallet, Signer},
};
use ethers_core::types::Address;
use log::LevelFilter;
use mockall::mock;
use mockito::{Server, ServerGuard};
use mystiko_abi::commitment_pool::TransactRequest;
use mystiko_ethers::provider::factory::{DefaultProviderFactory, ProviderFactory, ProvidersOptions};
use mystiko_ethers::provider::pool::{ChainProvidersOptions, ProviderPool};
use mystiko_ethers::provider::types::ProviderOptions;
use mystiko_relayer_abi::mystiko_gas_relayer::MystikoGasRelayer;
use mystiko_relayer_client::client::{RelayerClient, RelayerClientOptions, TRANSACTION_STATUS_URL_PATH};
use mystiko_relayer_client::error::RelayerClientError;
use mystiko_relayer_config::wrapper::relayer::RelayerConfig;
use mystiko_relayer_types::response::{failed, success, ResponseCode};
use mystiko_relayer_types::{
    RegisterInfoRequest, RegisterInfoResponse, RegisterOptions, RelayTransactRequest, RelayTransactResponse,
    RelayTransactStatusRequest, RelayTransactStatusResponse, TransactRequestData, TransactStatus,
    WaitingTransactionRequest,
};
use mystiko_types::{BridgeType, CircuitType, TransactionType};
use serde_json::{json, to_string};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

mock! {
    #[derive(Debug)]
    ChainConfig {}

    #[async_trait]
    impl ChainProvidersOptions for ChainConfig {
         async fn providers_options(&self, chain_id: u64) -> Result<Option<ProvidersOptions>>;
    }
}

struct TestRelayerClient {
    mock_server: ServerGuard,
    client: RelayerClient,
}

struct SetProvider {
    chain_id: u64,
    provider: Arc<mystiko_ethers::provider::factory::Provider>,
}

async fn setup(provider: Option<SetProvider>) -> Result<TestRelayerClient> {
    let mock_server = Server::new_async().await;
    let mock_chain_config = MockChainConfig::new();
    let mut pool = ProviderPool::builder()
        .chain_providers_options(Box::new(mock_chain_config))
        .build();
    if provider.is_some() {
        let provider = provider.unwrap();
        pool.set_provider(provider.chain_id, provider.provider);
    }
    let client = RelayerClient::new(
        Arc::new(RwLock::new(pool)),
        Some(
            RelayerClientOptions::builder()
                .is_testnet(true)
                .log_level(LevelFilter::Debug)
                .build(),
        ),
    )
    .await?;
    Ok(TestRelayerClient { mock_server, client })
}

#[tokio::test]
async fn test_create_relayer_config_error() {
    let mock_chain_config = MockChainConfig::new();
    let pool = ProviderPool::builder()
        .chain_providers_options(Box::new(mock_chain_config))
        .build();

    let client = RelayerClient::new(
        Arc::new(RwLock::new(pool)),
        Some(
            RelayerClientOptions::builder()
                .relayer_config_file_path(String::from("./path"))
                .is_testnet(true)
                .log_level(LevelFilter::Debug)
                .build(),
        ),
    )
    .await;

    assert!(client.is_err());
    assert_eq!(
        client.unwrap_err().to_string(),
        String::from("create relayer config error")
    );
}

#[tokio::test]
async fn test_all_register_info() {
    let chain_id: u64 = 31337;
    let anvil = Anvil::new().chain_id(chain_id).spawn();
    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let provider = ethers::providers::Provider::<Http>::try_from(anvil.endpoint())
        .unwrap()
        .interval(Duration::from_millis(10u64));
    let client = SignerMiddleware::new(provider, wallet.with_chain_id(chain_id));
    let client = Arc::new(client);
    let contract = MystikoGasRelayer::deploy(client, ()).unwrap().send().await.unwrap();

    let TestRelayerClient {
        mut mock_server,
        mut client,
    } = setup(Some(SetProvider {
        chain_id,
        provider: Arc::new(create_mystiko_provider(anvil.endpoint()).await.unwrap()),
    }))
    .await
    .unwrap();
    let config_str = create_relayer_config_json_str(chain_id, contract.address());
    let relayer_config = RelayerConfig::from_json_str(&config_str).unwrap();
    client.relayer_config = Arc::new(relayer_config);

    contract
        .register_relayer(mock_server.url(), String::from("test_relayer"), vec![])
        .send()
        .await
        .unwrap()
        .await
        .expect("contract register relayer error");

    let mock_status = RegisterInfoResponse {
        support: true,
        available: Some(true),
        chain_id,
        relayer_contract_address: Some(contract.address().to_string()),
        contracts: Some(vec![]),
    };

    let mock_response = success(mock_status.clone());
    let mock = mock_server
        .mock("POST", "/info")
        .with_status(200)
        .with_body(to_string(&mock_response).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;

    let response = client
        .all_register_info(RegisterInfoRequest {
            chain_id: anvil.chain_id(),
            options: Some(RegisterOptions {
                asset_symbol: "TEST".to_string(),
                asset_decimals: 16,
                circuit_type: CircuitType::Transaction1x0,
                show_unavailable: false,
            }),
        })
        .await
        .unwrap();
    let register_info = &response[0];
    assert_eq!(response.len(), 1);
    assert_eq!(register_info.name, "test_relayer");
    assert_eq!(register_info.chain_id, chain_id);
    assert_eq!(register_info.url, mock_server.url());
    assert_eq!(
        register_info.relayer_contract_address,
        mock_status.relayer_contract_address.unwrap()
    );
    assert_eq!(register_info.support, mock_status.support);
    assert_eq!(register_info.available, mock_status.available.unwrap());
    assert_eq!(register_info.contracts, mock_status.contracts.unwrap());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_relayer_config_not_found_error() {
    let chain_id: u64 = 31337;
    let anvil = Anvil::new().chain_id(chain_id).spawn();
    let TestRelayerClient {
        mock_server: _mock_server,
        client,
    } = setup(Some(SetProvider {
        chain_id,
        provider: Arc::new(create_mystiko_provider(anvil.endpoint()).await.unwrap()),
    }))
    .await
    .unwrap();

    let response = client
        .all_register_info(RegisterInfoRequest {
            chain_id,
            options: None,
        })
        .await;
    assert!(response.is_err());
    assert_eq!(
        response.unwrap_err().to_string(),
        RelayerClientError::RelayerConfigNotFoundError(chain_id).to_string()
    );
}

#[tokio::test]
async fn relay_transact() {
    let TestRelayerClient {
        mut mock_server,
        client,
    } = setup(None).await.unwrap();
    let mock_result = RelayTransactResponse {
        uuid: String::from("78d08829"),
    };

    let mock_response = success(mock_result.clone());
    let mock = mock_server
        .mock("POST", "/transact")
        .with_status(200)
        .with_body(to_string(&mock_response).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let request = RelayTransactRequest::builder()
        .relayer_url(mock_server.url())
        .data(
            TransactRequestData::builder()
                .contract_param(TransactRequest::default())
                .transaction_type(TransactionType::Withdraw)
                .bridge_type(BridgeType::Loop)
                .chain_id(56)
                .asset_symbol(String::from("MTT"))
                .asset_decimals(16)
                .pool_address(String::from("0x45B22A8CefDfF00989882CAE48Ad06D57938Efcc"))
                .circuit_type(CircuitType::Transaction1x0)
                .signature(String::from(""))
                .build(),
        )
        .build();
    let response = client.relay_transact(request).await.unwrap();
    assert_eq!(response, mock_result);
    mock.assert_async().await;
}

#[tokio::test]
async fn relay_transact_invalid_circuit_type() {
    let TestRelayerClient { mock_server, client } = setup(None).await.unwrap();

    let request = RelayTransactRequest::builder()
        .relayer_url(mock_server.url())
        .data(
            TransactRequestData::builder()
                .contract_param(TransactRequest::default())
                .transaction_type(TransactionType::Withdraw)
                .bridge_type(BridgeType::Loop)
                .chain_id(56)
                .asset_symbol(String::from("MTT"))
                .asset_decimals(16)
                .pool_address(String::from("0x45B22A8CefDfF00989882CAE48Ad06D57938Efcc"))
                .circuit_type(CircuitType::Rollup1)
                .signature(String::from(""))
                .build(),
        )
        .build();
    let response = client.relay_transact(request).await;
    assert!(response.is_err());
    assert_eq!(
        response.unwrap_err().to_string(),
        String::from("data.circuit_type: Validation error: invalid circuit type [{\"value\": String(\"rollup1\")}]")
    );
}

#[tokio::test]
async fn relay_transaction_status() {
    let TestRelayerClient {
        mut mock_server,
        client,
    } = setup(None).await.unwrap();
    let mock_result = RelayTransactStatusResponse {
        uuid: String::from("78d08829"),
        chain_id: 97,
        transaction_type: TransactionType::Withdraw,
        status: TransactStatus::Queued,
        transaction_hash: None,
        error_msg: None,
    };
    let mock_response = success(mock_result.clone());

    let mock = mock_server
        .mock("GET", format!("/{}/78d08829", TRANSACTION_STATUS_URL_PATH).as_str())
        .with_status(200)
        .with_body(to_string(&mock_response).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let response = client
        .relay_transaction_status(RelayTransactStatusRequest {
            relayer_url: mock_server.url(),
            uuid: "78d08829".to_string(),
        })
        .await
        .unwrap();
    assert_eq!(response, mock_result);
    mock.assert_async().await;
}

#[tokio::test]
async fn wait_transaction_until_queued() {
    let TestRelayerClient {
        mut mock_server,
        client,
    } = setup(None).await.unwrap();
    let mock_result = RelayTransactStatusResponse {
        uuid: String::from("78d08829"),
        chain_id: 97,
        transaction_type: TransactionType::Withdraw,
        status: TransactStatus::Queued,
        transaction_hash: None,
        error_msg: None,
    };
    let mock_response = success(mock_result.clone());

    let mock = mock_server
        .mock("GET", format!("/{}/78d08829", TRANSACTION_STATUS_URL_PATH).as_str())
        .with_status(200)
        .with_body(to_string(&mock_response).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let response = client
        .wait_transaction(WaitingTransactionRequest {
            relayer_url: mock_server.url(),
            uuid: "78d08829".to_string(),
            waiting_status: TransactStatus::Queued,
            timeout: Duration::from_secs(5),
            interval: None,
        })
        .await
        .unwrap();
    assert_eq!(response, mock_result);
    mock.assert_async().await;
}

#[tokio::test]
async fn wait_transaction_until_pending() {
    let TestRelayerClient {
        mut mock_server,
        client,
    } = setup(None).await.unwrap();
    let mock_result = RelayTransactStatusResponse {
        uuid: String::from("78d08829"),
        chain_id: 97,
        transaction_type: TransactionType::Withdraw,
        status: TransactStatus::Pending,
        transaction_hash: None,
        error_msg: None,
    };
    let mock_response = success(mock_result.clone());

    let mock = mock_server
        .mock("GET", format!("/{}/78d08829", TRANSACTION_STATUS_URL_PATH).as_str())
        .with_status(200)
        .with_body(to_string(&mock_response).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let response = client
        .wait_transaction(WaitingTransactionRequest {
            relayer_url: mock_server.url(),
            uuid: "78d08829".to_string(),
            waiting_status: TransactStatus::Pending,
            timeout: Duration::from_secs(5),
            interval: None,
        })
        .await
        .unwrap();
    assert_eq!(response, mock_result);
    mock.assert_async().await;
}

#[tokio::test]
async fn wait_transaction_until_succeeded() {
    let TestRelayerClient {
        mut mock_server,
        client,
    } = setup(None).await.unwrap();
    let mock_result = RelayTransactStatusResponse {
        uuid: String::from("78d08829"),
        chain_id: 97,
        transaction_type: TransactionType::Withdraw,
        status: TransactStatus::Succeeded,
        transaction_hash: None,
        error_msg: None,
    };
    let mock_response = success(mock_result.clone());

    let mock = mock_server
        .mock("GET", format!("/{}/78d08829", TRANSACTION_STATUS_URL_PATH).as_str())
        .with_status(200)
        .with_body(to_string(&mock_response).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let response = client
        .wait_transaction(WaitingTransactionRequest {
            relayer_url: mock_server.url(),
            uuid: "78d08829".to_string(),
            waiting_status: TransactStatus::Succeeded,
            timeout: Duration::from_secs(5),
            interval: None,
        })
        .await
        .unwrap();
    assert_eq!(response, mock_result);
    mock.assert_async().await;
}

#[tokio::test]
async fn wait_transaction_until_confirmed_timeout() {
    let TestRelayerClient {
        mut mock_server,
        client,
    } = setup(None).await.unwrap();
    let mock_result = RelayTransactStatusResponse {
        uuid: String::from("78d08829"),
        chain_id: 97,
        transaction_type: TransactionType::Withdraw,
        status: TransactStatus::Queued,
        error_msg: None,
        transaction_hash: None,
    };
    let mock_response = success(mock_result.clone());

    let mock = mock_server
        .mock("GET", format!("/{}/78d08829", TRANSACTION_STATUS_URL_PATH).as_str())
        .with_status(200)
        .with_body(to_string(&mock_response).unwrap())
        .with_header("content-type", "application/json")
        .expect(4)
        .create_async()
        .await;
    let response = client
        .wait_transaction(WaitingTransactionRequest {
            relayer_url: mock_server.url(),
            uuid: "78d08829".to_string(),
            waiting_status: TransactStatus::Succeeded,
            timeout: Duration::from_secs(3),
            interval: Some(Duration::from_secs(1)),
        })
        .await;
    assert!(response.is_err());
    assert_eq!(
        response.unwrap_err().to_string(),
        RelayerClientError::WaitTransactionTimeout("78d08829".to_string()).to_string()
    );
    mock.assert_async().await;
}

#[tokio::test]
async fn wait_transaction_until_confirmed_failed() {
    let TestRelayerClient {
        mut mock_server,
        client,
    } = setup(None).await.unwrap();
    let mock_result = RelayTransactStatusResponse {
        uuid: String::from("78d08829"),
        chain_id: 97,
        transaction_type: TransactionType::Withdraw,
        status: TransactStatus::Failed,
        transaction_hash: None,
        error_msg: None,
    };
    let mock_response = success(mock_result.clone());

    let mock = mock_server
        .mock("GET", format!("/{}/78d08829", TRANSACTION_STATUS_URL_PATH).as_str())
        .with_status(200)
        .with_body(to_string(&mock_response).unwrap())
        .with_header("content-type", "application/json")
        .expect(1)
        .create_async()
        .await;
    let response = client
        .wait_transaction(WaitingTransactionRequest {
            relayer_url: mock_server.url(),
            uuid: "78d08829".to_string(),
            waiting_status: TransactStatus::Succeeded,
            timeout: Duration::from_secs(3),
            interval: Some(Duration::from_secs(1)),
        })
        .await;
    assert!(response.is_err());
    assert_eq!(
        response.unwrap_err().to_string(),
        RelayerClientError::TransactionFailed(mock_server.url(), String::from("78d08829")).to_string()
    );
    mock.assert_async().await;
}

#[tokio::test]
async fn test_handle_error_response() {
    let TestRelayerClient {
        mut mock_server,
        client,
    } = setup(None).await.unwrap();
    let mock_result = RelayTransactStatusResponse {
        uuid: "".to_string(),
        chain_id: 97,
        transaction_type: TransactionType::Transfer,
        status: TransactStatus::Queued,
        transaction_hash: None,
        error_msg: None,
    };
    let mut mock_response = failed(Some(mock_result.clone()), Some(String::from("failed")));

    let mock = mock_server
        .mock("GET", format!("/{}/123456", TRANSACTION_STATUS_URL_PATH).as_str())
        .with_status(200)
        .with_body(to_string(&mock_response).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let response = client
        .relay_transaction_status(RelayTransactStatusRequest {
            relayer_url: mock_server.url(),
            uuid: "123456".to_string(),
        })
        .await;
    assert!(response.is_err());
    assert_eq!(
        response.unwrap_err().to_string(),
        RelayerClientError::ApiResponseError {
            code: ResponseCode::Failed as i32,
            message: String::from("failed")
        }
        .to_string()
    );
    mock.assert_async().await;

    mock_response.code = ResponseCode::Successful as i32;
    let mock = mock_server
        .mock("GET", format!("/{}/123456", TRANSACTION_STATUS_URL_PATH).as_str())
        .with_status(200)
        .with_body(to_string(&mock_response).unwrap())
        .create_async()
        .await;
    let response = client
        .relay_transaction_status(RelayTransactStatusRequest {
            relayer_url: mock_server.url(),
            uuid: "123456".to_string(),
        })
        .await;
    assert!(response.is_err());
    assert_eq!(
        response.unwrap_err().to_string(),
        RelayerClientError::UnsupportedContentTypeError("".to_string()).to_string()
    );
    mock.assert_async().await;
}

async fn create_mystiko_provider(url: String) -> Result<mystiko_ethers::provider::factory::Provider> {
    let provider_options = ProviderOptions::builder().url(url).build();
    let factory = DefaultProviderFactory::new();
    let providers_options = ProvidersOptions::Http(provider_options);
    factory.create_provider(providers_options).await
}

fn create_relayer_config_json_str(chain_id: u64, address: Address) -> String {
    json!(
        {
            "version": "0.1.0",
            "chains": [
                {
                    "name": "Local",
                    "chainId": chain_id,
                    "assetSymbol": "TEST",
                    "relayerContractAddress": address,
                    "contracts": [],
                    "transactionInfo": {
                        "mainGasCost": {
                          "transaction1x0": 500704,
                          "transaction1x1": 617592,
                          "transaction1x2": 705128,
                          "transaction2x0": 598799,
                          "transaction2x1": 708389,
                          "transaction2x2": 803183
                        },
                        "erc20GasCost": {
                          "transaction1x0": 512985,
                          "transaction1x1": 629802,
                          "transaction1x2": 705494,
                          "transaction2x0": 611040,
                          "transaction2x1": 727970,
                          "transaction2x2": 803645
                        }
                    }
                }
            ]
        }
    )
    .to_string()
}
