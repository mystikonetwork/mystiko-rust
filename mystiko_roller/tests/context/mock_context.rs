use crate::common::{env_init, load_env_mock_indexer_port, set_env_mock_indexer_port, ENV_MUTEX};
use async_trait::async_trait;
use ethers_providers::{MockError, MockProvider, Provider as EthersProvider, RetryClientBuilder, RetryPolicy};
use mystiko_config::raw::indexer::RawIndexerConfig;
use mystiko_config::wrapper::contract::pool::PoolContractConfig;
use mystiko_config::wrapper::indexer::IndexerConfig;
use mystiko_ethers::provider::factory::Provider;
use mystiko_ethers::provider::failover::FailoverProvider;
use mystiko_ethers::provider::wrapper::ProviderWrapper;
use mystiko_roller::chain::explorer::ExplorerStub;
use mystiko_roller::chain::indexer::IndexerStub;
use mystiko_roller::common::env::load_coin_market_api_key;
use mystiko_roller::common::error::Result;
use mystiko_roller::config::mystiko_parser::MystikoConfigParser;
use mystiko_roller::config::roller::{create_roller_config, create_token_price_config, RollerConfig};
use mystiko_roller::context::ContextTrait;
use mystiko_roller::db::database::RollerDatabase;
use mystiko_server_utils::token_price::price::TokenPrice;
use mystiko_storage::formatter::sql::SqlStatementFormatter;
use mystiko_storage_sqlite::{SqliteStorage, SqliteStorageBuilder};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct MockContext {
    core_cfg_parser: Arc<MystikoConfigParser>,
    cfg: Arc<RollerConfig>,
    db: RwLock<RollerDatabase<SqlStatementFormatter, SqliteStorage>>,
    indexer: Option<Arc<IndexerStub>>,
    chain_explorer: Option<Arc<ExplorerStub>>,
    token_price: Arc<RwLock<TokenPrice>>,
    provider: Arc<Provider>,
    mock_provider: Arc<MockProvider>,
}

impl MockContext {
    pub async fn mock_provider(&self) -> Arc<MockProvider> {
        self.mock_provider.clone()
    }

    pub fn disable_indexer(&mut self) {
        self.indexer = None;
    }
}

#[async_trait]
impl ContextTrait for MockContext {
    async fn new(run_mod: &str, cfg_path: &str) -> Result<Self> {
        let roller_cfg = create_roller_config(run_mod, cfg_path).unwrap();

        let core_cfg_parser = MystikoConfigParser::new(&roller_cfg.core, cfg_path).await.unwrap();
        let db = create_memory_database().await;

        let indexer_port = load_env_mock_indexer_port();
        let raw_indexer_cfg = RawIndexerConfig {
            url: format!("http://127.0.0.1:{}", indexer_port),
            timeout_ms: 15000,
        };
        let indexer_cfg = IndexerConfig::new(Arc::new(raw_indexer_cfg));
        let indexer = IndexerStub::new(&indexer_cfg).unwrap();
        let chain_explorer = core_cfg_parser
            .chain_explorer_cfg(roller_cfg.chain.chain_id)
            .map(|url| ExplorerStub::new(url, roller_cfg.pull.explorer_request_timeout_secs).unwrap());
        let api_key = load_coin_market_api_key().unwrap();

        let mut token_price_cfg = create_token_price_config(run_mod, cfg_path).unwrap();
        token_price_cfg.base_url = format!(
            "http://127.0.0.1:{}",
            token_price_server_port(indexer_port.parse::<u16>().unwrap())
        );
        let token_price = TokenPrice::new(&token_price_cfg, &api_key).unwrap();

        let (_, mock) = EthersProvider::mocked();
        let provider = create_mock_provider(&mock);

        Ok(MockContext {
            core_cfg_parser: Arc::new(core_cfg_parser),
            cfg: Arc::new(roller_cfg),
            db: RwLock::new(db),
            indexer: Some(Arc::new(indexer)),
            chain_explorer: chain_explorer.map(Arc::new),
            token_price: Arc::new(RwLock::new(token_price)),
            provider: Arc::new(provider),
            mock_provider: Arc::new(mock),
        })
    }

    fn core_cfg_parser(&self) -> Arc<MystikoConfigParser> {
        Arc::clone(&self.core_cfg_parser)
    }

    fn cfg(&self) -> Arc<RollerConfig> {
        Arc::clone(&self.cfg)
    }

    async fn db(&self) -> RwLockReadGuard<RollerDatabase<SqlStatementFormatter, SqliteStorage>> {
        self.db.read().await
    }

    fn indexer(&self) -> Option<Arc<IndexerStub>> {
        self.indexer.clone()
    }

    fn chain_explorer(&self) -> Option<Arc<ExplorerStub>> {
        self.chain_explorer.clone()
    }

    fn provider(&self) -> Arc<Provider> {
        self.provider.clone()
    }

    fn signer(&self) -> Arc<Provider> {
        self.provider.clone()
    }

    async fn token_price(&self) -> RwLockWriteGuard<'_, TokenPrice> {
        self.token_price.write().await
    }
}

async fn create_memory_database() -> RollerDatabase<SqlStatementFormatter, SqliteStorage> {
    let formatter = SqlStatementFormatter::default();
    let storage = SqliteStorageBuilder::new().in_memory().build().await.unwrap();
    RollerDatabase::new(formatter, storage).await
}

#[derive(Debug, Default)]
struct MockProviderRetryPolicy;

impl RetryPolicy<MockError> for MockProviderRetryPolicy {
    fn should_retry(&self, _error: &MockError) -> bool {
        false
    }

    fn backoff_hint(&self, _error: &MockError) -> Option<Duration> {
        Duration::from_secs(10).into()
    }
}

fn create_mock_provider(provider: &MockProvider) -> Provider {
    let retry_provider_builder = RetryClientBuilder::default();
    let inner_provider = retry_provider_builder.build(provider.clone(), Box::<MockProviderRetryPolicy>::default());

    let mut failover_provider_builder = FailoverProvider::dyn_rpc();
    failover_provider_builder = failover_provider_builder.add_provider(Box::new(inner_provider));
    Provider::new(ProviderWrapper::new(Box::new(failover_provider_builder.build())))
}

pub fn provider_server_port() -> u16 {
    20000 + 1
}

pub fn indexer_server_port(chain_id: u64) -> u16 {
    chain_id as u16 + 20000
}

pub fn token_price_server_port(indexer_server_port: u16) -> u16 {
    indexer_server_port + 5000
}

pub fn explorer_server_port(chain_id: u64) -> u16 {
    chain_id as u16 + 20000 + 8000
}

pub async fn create_mock_context(indexer_port: u16) -> MockContext {
    let _guard = ENV_MUTEX.write().await;
    env_init();
    set_env_mock_indexer_port(&indexer_port.to_string());
    MockContext::new("testnet", "tests/test_files/config/base")
        .await
        .unwrap()
}

pub fn get_pool_contracts(c: &MockContext) -> PoolContractConfig {
    let pool_contracts = c.core_cfg_parser().pool_contracts_cfg(c.cfg().chain.chain_id);
    pool_contracts[0].clone()
}
