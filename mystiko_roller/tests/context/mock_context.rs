use crate::common::env_tests::{load_env_mock_indexer_port, set_env_mock_indexer_port, ENV_MOCK_INDEXER_PORT_MUTEX};
use crate::context::context_tests::do_evn_init;
use async_trait::async_trait;
use ethers_providers::{MockError, MockProvider, Provider as EthersProvider, RetryClientBuilder, RetryPolicy};
use mystiko_config::raw::indexer::RawIndexerConfig;
use mystiko_config::wrapper::contract::pool::PoolContractConfig;
use mystiko_config::wrapper::indexer::IndexerConfig;
use mystiko_ethers::provider::factory::Provider;
use mystiko_ethers::provider::failover::FailoverProvider;
use mystiko_ethers::provider::wrapper::ProviderWrapper;
use mystiko_roller::common::env::load_coin_market_api_key;
use mystiko_roller::common::error::Result;
use mystiko_roller::common::trace::trace_init;
use mystiko_roller::config::mystiko_config_parser::MystikoConfigParser;
use mystiko_roller::config::settings::{create_roller_config, create_token_price_config, RollerConfig};
use mystiko_roller::context::ContextTrait;
use mystiko_roller::db::db::RollerDatabase;
use mystiko_roller::instance::sync::indexer::IndexerInstance;
use mystiko_roller::instance::sync::x_scan::XScanInstance;
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
    indexer: Option<RwLock<IndexerInstance>>,
    xscan: Option<RwLock<XScanInstance>>,
    token_price: Arc<RwLock<TokenPrice>>,
    provider: RwLock<Arc<Provider>>,
    mock_provider: Arc<RwLock<MockProvider>>,
}

impl MockContext {
    pub async fn mock_provider(&self) -> RwLockWriteGuard<'_, MockProvider> {
        self.mock_provider.write().await
    }

    pub fn disable_indexer(&mut self) {
        self.indexer = None;
    }
}

#[async_trait]
impl ContextTrait for MockContext {
    async fn new() -> Result<Self> {
        let roller_cfg = create_roller_config();
        trace_init(&roller_cfg.log_level);

        let token_price_cfg = create_token_price_config();
        let core_cfg_parser = MystikoConfigParser::new(&roller_cfg.core).await;
        let db = create_memory_database().await;

        let indexer_port = load_env_mock_indexer_port();
        let raw_indexer_cfg = RawIndexerConfig {
            url: format!("http://127.0.0.1:{}", indexer_port).to_string(),
            timeout_ms: 15000,
        };
        let indexer = IndexerInstance::new(&IndexerConfig::new(Arc::new(raw_indexer_cfg)));
        let xscan = core_cfg_parser
            .xscan_cfg(roller_cfg.chain.chain_id)
            .map(XScanInstance::new);
        let api_key = load_coin_market_api_key().unwrap();
        let token_price = TokenPrice::new(&token_price_cfg, &api_key).unwrap();

        let (_, mock) = EthersProvider::mocked();
        let provider = create_mock_provider(&mock);

        Ok(MockContext {
            core_cfg_parser: Arc::new(core_cfg_parser),
            cfg: Arc::new(roller_cfg),
            db: RwLock::new(db),
            indexer: Some(RwLock::new(indexer)),
            xscan: xscan.map(RwLock::new),
            token_price: Arc::new(RwLock::new(token_price)),
            provider: RwLock::new(Arc::new(provider)),
            mock_provider: Arc::new(RwLock::new(mock)),
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

    async fn indexer(&self) -> Option<RwLockReadGuard<IndexerInstance>> {
        Some(self.indexer.as_ref()?.read().await)
    }

    async fn xscan(&self) -> Option<RwLockReadGuard<XScanInstance>> {
        Some(self.xscan.as_ref()?.read().await)
    }

    async fn provider(&self) -> Result<Arc<Provider>> {
        let provider_guard = self.provider.read().await;
        Ok(Arc::clone(&*provider_guard))
    }

    async fn sign_provider(&self) -> Arc<Provider> {
        let provider_guard = self.provider.read().await;
        Arc::clone(&*provider_guard)
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

pub async fn create_mock_context(indexer_port: u64) -> MockContext {
    do_evn_init();
    let _guard = ENV_MOCK_INDEXER_PORT_MUTEX.lock();
    set_env_mock_indexer_port(&indexer_port.to_string());
    MockContext::new().await.unwrap()
}

pub fn get_pool_contracts(c: &MockContext) -> PoolContractConfig {
    let pool_contracts = c.core_cfg_parser().pool_contracts(c.cfg().chain.chain_id);
    pool_contracts[0].clone()
}
