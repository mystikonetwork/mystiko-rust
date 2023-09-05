pub mod relayer_tests;

use anyhow::Result;
use async_trait::async_trait;
use ethers::middleware::SignerMiddleware;
use ethers::providers::Http;
use ethers::signers::{LocalWallet, Signer};
use ethers_core::types::Address;
use log::LevelFilter;
use mockall::mock;
use mockito::{Mock, Server, ServerGuard};
use mystiko_ethers::{
    ChainProvidersOptions, DefaultProviderFactory, ProviderFactory, ProviderOptions, ProviderPool, Providers,
    ProvidersOptions,
};
use mystiko_relayer_abi::mystiko_gas_relayer::MystikoGasRelayer;
use mystiko_relayer_client::client::{
    RelayerClient, RelayerClientOptions, HANDSHAKE_URL_PATH, INFO_URL_PATH, SUPPORTED_API_VERSION,
    TRANSACTION_STATUS_URL_PATH, TRANSACT_URL_PATH,
};
use mystiko_relayer_config::wrapper::relayer::RelayerConfig;
use mystiko_relayer_types::response::success;
use mystiko_relayer_types::{
    HandshakeResponse, RegisterInfoResponse, RelayTransactResponse, RelayTransactStatusResponse, TransactStatus,
};
use mystiko_types::TransactionType;
use serde_json::{json, to_string};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

mock! {
    #[derive(Debug)]
    pub ChainConfig {}

    #[async_trait]
    impl ChainProvidersOptions for ChainConfig {
         async fn providers_options(&self, chain_id: u64) -> Result<Option<ProvidersOptions>>;
    }
}

const MOCK_CONTRACT_ADDRESS: &str = "0x45B22A8CefDfF00989882CAE48Ad06D57938Efcc";

pub async fn create_client(
    provider_pool: ProviderPool<MockChainConfig>,
    chain_id: Option<u64>,
    contract_address: Option<Address>,
) -> RelayerClient<ProviderPool<MockChainConfig>> {
    let _ = env_logger::builder()
        .filter_module("mystiko_relayer_client", LevelFilter::Debug)
        .try_init();

    let mut server = Server::new_async().await;
    let mock = server
        .mock("GET", "/relayer_config/production/testnet/latest.json")
        .with_body(relayer_config_json_string())
        .create_async()
        .await;

    let mut client = RelayerClient::new(
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

    if chain_id.is_some() && contract_address.is_some() {
        client.relayer_config = Arc::new(
            RelayerConfig::from_json_str(create_relayer_config(chain_id.unwrap(), contract_address.unwrap()).as_str())
                .unwrap(),
        );
    }
    client
}

pub async fn create_provider_pool(chain_id: u64, provider_url: Option<String>) -> ProviderPool<MockChainConfig> {
    let mock_chain_config = MockChainConfig::new();
    let pool = ProviderPool::builder()
        .chain_providers_options(mock_chain_config)
        .build();

    if provider_url.is_some() {
        let provider_options = ProviderOptions::builder().url(provider_url.unwrap()).build();
        let factory = DefaultProviderFactory::new();
        let providers_options = ProvidersOptions::Http(provider_options);
        let provider = factory.create_provider(providers_options).await.unwrap();
        pool.set_provider(chain_id, Arc::new(provider)).await;
    }

    pool
}

pub async fn mock_handshake_supported_server(server: Arc<RwLock<ServerGuard>>) -> Mock {
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

pub async fn mock_register_info_server(server: Arc<RwLock<ServerGuard>>, chain_id: u64) -> Mock {
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

pub async fn mock_relay_transact_server(server: Arc<RwLock<ServerGuard>>) -> Mock {
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

pub async fn mock_transaction_status(server: Arc<RwLock<ServerGuard>>) -> Mock {
    let response = success(RelayTransactStatusResponse {
        uuid: String::from("78d08829"),
        chain_id: 31337,
        transaction_type: TransactionType::Withdraw,
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

pub async fn deploy_contract(chain_id: u64, provider_url: &str, wallet: LocalWallet) -> Address {
    let provider = ethers::providers::Provider::<Http>::try_from(provider_url)
        .unwrap()
        .interval(Duration::from_millis(10u64));
    let client = Arc::new(SignerMiddleware::new(provider, wallet.with_chain_id(chain_id)));
    let contract = MystikoGasRelayer::deploy(client, ()).unwrap().send().await.unwrap();

    contract.address()
}

pub async fn register_relayer(
    chain_id: u64,
    wallet: LocalWallet,
    provider_url: &str,
    contract_address: Address,
    relayer_url: &str,
) {
    let provider = ethers::providers::Provider::<Http>::try_from(provider_url)
        .unwrap()
        .interval(Duration::from_millis(10u64));
    let client = Arc::new(SignerMiddleware::new(provider, wallet.with_chain_id(chain_id)));
    let contract = MystikoGasRelayer::new(contract_address, client);

    contract
        .register_relayer(relayer_url.to_string(), format!("test_{}", relayer_url), vec![])
        .send()
        .await
        .unwrap()
        .await
        .expect("contract register relayer error");
}

pub fn relayer_config_json_string() -> String {
    let relayer_config = r#"
        {
            "chains":[
                {
                    "assetDecimals":18,
                    "assetSymbol":"ETH",
                    "chainId":5,
                    "contracts":[
                        {
                            "assetDecimals":18,
                            "assetSymbol":"ETH",
                            "assetType":"main",
                            "relayerFeeOfTenThousandth":25
                        },
                        {
                            "assetDecimals":18,
                            "assetSymbol":"MTT",
                            "assetType":"erc20",
                            "relayerFeeOfTenThousandth":25
                        },
                        {
                            "assetDecimals":6,
                            "assetSymbol":"mUSD",
                            "assetType":"erc20",
                            "relayerFeeOfTenThousandth":25
                        }
                    ],
                    "name":"Ethereum Goerli",
                    "relayerContractAddress":"0x45B22A8CefDfF00989882CAE48Ad06D57938Efcc",
                    "transactionInfo":{
                        "erc20GasCost":{
                            "transaction1x0":512985,
                            "transaction1x1":629802,
                            "transaction1x2":705494,
                            "transaction2x0":611040,
                            "transaction2x1":727970,
                            "transaction2x2":803645
                        },
                        "mainGasCost":{
                            "transaction1x0":500704,
                            "transaction1x1":617592,
                            "transaction1x2":705128,
                            "transaction2x0":598799,
                            "transaction2x1":708389,
                            "transaction2x2":803183
                        }
                    }
                },
                {
                    "assetDecimals":18,
                    "assetSymbol":"BNB",
                    "chainId":97,
                    "contracts":[
                        {
                            "assetDecimals":18,
                            "assetSymbol":"MTT",
                            "assetType":"erc20",
                            "relayerFeeOfTenThousandth":25
                        },
                        {
                            "assetDecimals":6,
                            "assetSymbol":"mUSD",
                            "assetType":"erc20",
                            "relayerFeeOfTenThousandth":25
                        },
                        {
                            "assetDecimals":18,
                            "assetSymbol":"BNB",
                            "assetType":"main",
                            "relayerFeeOfTenThousandth":25
                        }
                    ],
                    "name":"BSC Testnet",
                    "relayerContractAddress":"0xfC21Aa6a04f09565bC6eeDC182063Fd4E466670A",
                    "transactionInfo":{
                        "erc20GasCost":{
                            "transaction1x0":537145,
                            "transaction1x1":646754,
                            "transaction1x2":724302,
                            "transaction2x0":640808,
                            "transaction2x1":756699,
                            "transaction2x2":833563
                        },
                        "mainGasCost":{
                            "transaction1x0":520800,
                            "transaction1x1":636116,
                            "transaction1x2":724104,
                            "transaction2x0":630207,
                            "transaction2x1":743273,
                            "transaction2x2":833563
                        }
                    }
                }
            ],
            "gitRevision":"6335708",
            "version":"0.0.1"
        }
    "#;
    relayer_config.to_string()
}

fn create_relayer_config(chain_id: u64, address: Address) -> String {
    json!(
        {
            "version": "0.1.0",
            "gitRevision": "a123456",
            "chains": [
                {
                    "name": "UnitTestChain",
                    "chainId": chain_id,
                    "assetSymbol": "TEST",
                    "assetDecimals": 18,
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
