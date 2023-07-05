use anyhow::Result;
use async_trait::async_trait;
use ethereum_types::U256;
use ethers_core::types::Bytes;
use ethers_middleware::providers::MockProvider;
use ethers_providers::Provider;
use mockall::mock;
use mockito::{Server, ServerGuard};
use mystiko_abi::commitment_pool::TransactRequest;
use mystiko_ethers::provider::factory::ProvidersOptions;
use mystiko_ethers::provider::pool::{ChainProvidersOptions, ProviderPool};
use mystiko_ethers::provider::wrapper::ProviderWrapper;
use mystiko_relayer_server::channel::consumer::TransactionConsumer;
use mystiko_relayer_server::channel::{transact_channel, TransactSendersMap};
use mystiko_relayer_server::common::{init_app_state, AppState, AppStateOptions};
use mystiko_relayer_server::database::Database;
use mystiko_relayer_server::handler::account::AccountHandler;
use mystiko_relayer_server::handler::transaction::TransactionHandler;
use mystiko_relayer_types::TransactRequestData;
use mystiko_server_utils::token_price::config::TokenPriceConfig;
use mystiko_server_utils::token_price::price::TokenPrice;
use mystiko_storage::formatter::sql::SqlStatementFormatter;
use mystiko_storage_sqlite::{SqliteStorage, SqliteStorageBuilder};
use mystiko_types::{BridgeType, CircuitType, TransactionType};
use std::str::FromStr;
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

#[allow(dead_code)]
pub const SERVER_CONFIG_SYMBOL_INVALID: &str = "./tests/files/config_symbol_invalid.toml";
#[allow(dead_code)]
pub const TOKEN_PRICE_CONFIG_PATH: &str = "./tests/files/token_price.json";
#[allow(dead_code)]
pub const TEST_RELAYER_CONFIG_SINGLE_PATH: &str = "./tests/files/relayer_config_single.json";
#[allow(dead_code)]
pub const SERVER_CONFIG_ID_NOT_FOUND: &str = "./tests/files/config_id_not_found.toml";
pub const TESTNET_CONFIG_PATH: &str = "./tests/files/config_test_testnet.toml";
pub const MAINNET_CONFIG_PATH: &str = "./tests/files/config_test_mainnet.toml";
pub const TEST_RELAYER_CONFIG_PATH: &str = "./tests/files/relayer_config.json";
pub const TEST_MYSTIKO_CONFIG_PATH: &str = "./tests/files/mystiko_config.json";
pub const ARRAY_QUEUE_CAPACITY: usize = 10;

pub struct TestServer {
    pub app_state: AppState,
    pub senders: TransactSendersMap,
    pub consumers: Vec<TransactionConsumer>,
    pub account_handler: Arc<AccountHandler<SqlStatementFormatter, SqliteStorage>>,
    pub transaction_handler: Arc<TransactionHandler<SqlStatementFormatter, SqliteStorage>>,
    pub token_price: Arc<RwLock<TokenPrice>>,
    pub providers: Arc<RwLock<ProviderPool>>,
    pub mock_server: ServerGuard,
}

impl TestServer {
    pub async fn new(mock: Option<MockProvider>) -> Result<Self> {
        // init app state
        let app_state = init_app_state(
            AppStateOptions::builder()
                .server_config_path(TESTNET_CONFIG_PATH)
                .relayer_config_path(Some(TEST_RELAYER_CONFIG_PATH))
                .mystiko_config_path(Some(TEST_MYSTIKO_CONFIG_PATH))
                .log_level("debug")
                .build(),
        )
        .await
        .unwrap();

        // create database in memory
        let storage = SqliteStorageBuilder::new().in_memory().build().await.unwrap();
        let database = Database::new(SqlStatementFormatter::default(), storage);
        database.migrate().await.unwrap();
        let db = Arc::new(database);

        // handler
        let account_handler = Arc::new(
            AccountHandler::new(db.clone(), &app_state.server_config.accounts)
                .await
                .unwrap(),
        );
        let transaction_handler = Arc::new(TransactionHandler::new(db.clone()));

        // init mock token price
        let server = Server::new_async().await;
        let mut default_cfg = TokenPriceConfig::new("testnet", None)?;
        default_cfg.base_url = server.url();
        let token_price = Arc::new(RwLock::new(TokenPrice::new(&default_cfg, "")?));

        // mock provider
        let mock = mock.unwrap_or(MockProvider::new());
        let provider: Arc<mystiko_ethers::provider::factory::Provider> =
            Arc::new(Provider::new(ProviderWrapper::new(Box::new(mock))));
        let mut mock_chain_config = MockChainConfig::new();
        mock_chain_config.expect_providers_options().returning(|_| Ok(None));
        let mut pool = ProviderPool::builder()
            .chain_providers_options(Box::new(mock_chain_config))
            .build();
        pool.set_provider(5, provider.clone());
        pool.set_provider(97, provider.clone());
        let providers = Arc::new(RwLock::new(pool));

        // senders and consumers
        let (senders, consumers) = transact_channel::init(
            &app_state.server_config,
            &app_state.relayer_config,
            providers.clone(),
            transaction_handler.clone(),
            token_price.clone(),
            ARRAY_QUEUE_CAPACITY,
        )
        .await?;

        Ok(TestServer {
            app_state,
            senders,
            consumers,
            account_handler,
            transaction_handler,
            token_price,
            providers,
            mock_server: server,
        })
    }
}

#[actix_rt::test]
async fn create_default_providers() {
    let server = TestServer::new(None).await.unwrap();
    let app_state = server.app_state;
    let mut providers = ProviderPool::builder()
        .chain_providers_options(Box::new(app_state))
        .build();
    let provider = providers.get_or_create_provider(5).await;
    assert!(provider.is_ok());
}

#[actix_rt::test]
async fn init_app_state_from_remote() {
    // testnet
    let app_state = init_app_state(
        AppStateOptions::builder()
            .log_level("error")
            .server_config_path(TESTNET_CONFIG_PATH)
            .relayer_config_path(None)
            .mystiko_config_path(None)
            .build(),
    )
    .await;
    assert!(app_state.is_ok());
    // mainnet
    let app_state = init_app_state(
        AppStateOptions::builder()
            .log_level("error")
            .server_config_path(MAINNET_CONFIG_PATH)
            .relayer_config_path(None)
            .mystiko_config_path(None)
            .build(),
    )
    .await;
    assert!(app_state.is_ok());
}

#[allow(dead_code)]
pub fn get_valid_transact_request_data() -> TransactRequestData {
    TransactRequestData {
        contract_param: TransactRequest {
            proof: Default::default(),
            root_hash: Default::default(),
            serial_numbers: vec![U256::from_str_radix(
                "0x19aaddbfd3840e5d9363793cc8a91c8e223db9775095316e528fe335db42956d",
                16,
            )
            .unwrap()],
            sig_hashes: vec![U256::from_str_radix(
                "0x0e5a093c5390514adad7e5277500319e7cc35d7682a4fa2ac84f4b5332909a5f",
                16,
            )
            .unwrap()],
            sig_pk: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 122, 235, 117, 149, 215, 12, 61, 182, 5, 183, 46, 6, 199, 169, 27,
                55, 117, 42, 27, 83,
            ],
            public_amount: U256::from_str_radix(
                "0x00000000000000000000000000000000000000000000000003fba0faba898000",
                16,
            )
            .unwrap(),
            relayer_fee_amount: U256::from_str_radix(
                "0x000000000000000000000000000000000000000000000000000aa87bee538000",
                16,
            )
            .unwrap(),
            out_commitments: vec![U256::from_str_radix(
                "0x1da10644733ab072dc6ea8aa6087d797b5002aa53238b753132448ba981102c5",
                16,
            )
            .unwrap()],
            out_rollup_fees: vec![U256::from_str_radix(
                "0x000000000000000000000000000000000000000000000000002386f26fc10000",
                16,
            )
            .unwrap()],
            public_recipient: Default::default(),
            relayer_address: Default::default(),
            out_encrypted_notes: vec![Bytes::from_str(
                "0x013b356d8d7b70e3896a4673b9a2c58904394a7d50bc92a6325b8\
                bedf6ec69ae938edaa562b23b50a7c62400ee344e6cedbb22233d53020d25e33650be5449b9ccd\
                94ca38c8ac66942c2d292b23149ec48b87de118acfab3895123e6eac243acf7a7055dbae309261\
                99852844ef19e2e43b065b697ae7a1faba33430240d380aa088ea5d207757780f412c401c503d7\
                3e3394703b6427a277f583a4bf368063966c32c4b3b238ebe0d60c544693d69c127529194da3bf\
                e5726064b96f7580802fa591dffea922139cfe2eccb6220d322a3",
            )
            .unwrap()],
            random_auditing_public_key: Default::default(),
            encrypted_auditor_notes: vec![],
        },
        transaction_type: TransactionType::Withdraw,
        bridge_type: BridgeType::Loop,
        chain_id: 5,
        asset_symbol: "ETH".to_string(),
        asset_decimals: 16,
        pool_address: "0x4F416Acfd1153F9Af782056e68607227Af29D931".to_string(),
        circuit_type: CircuitType::Transaction1x0,
        signature: "0x800157ae47e94a156c42584190c33362b13ff94a7e8f5ef6ffd602c8d19ae\
        0684a4da6afd3c10bae9bd252dd20a9388d86c617bacb807a236a0285603e4086d61b"
            .to_string(),
    }
}
