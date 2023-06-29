use crate::chain::explorer::ExplorerStub;
use crate::chain::indexer::IndexerStub;
use crate::common::env::{load_coin_market_api_key, load_roller_config_path};
use crate::common::error::Result;
use crate::common::trace::trace_init;
use crate::config::mystiko_parser::MystikoConfigParser;
use crate::config::roller::create_token_price_config;
use crate::config::roller::RollerConfig;
use crate::config::roller::{create_roller_config, ChainDataSource};
use crate::db::database::create_roller_database;
use crate::db::database::RollerDatabase;
use async_trait::async_trait;
use mystiko_ethers::provider::factory::{
    DefaultProviderFactory, Provider, ProviderFactory, ProvidersOptions, HTTP_REGEX, WS_REGEX,
};
use mystiko_ethers::provider::pool::ProviderPool;
use mystiko_ethers::provider::types::ProviderOptions;
use mystiko_server_utils::token_price::price::TokenPrice;
use mystiko_storage::formatter::sql::SqlStatementFormatter;
use mystiko_storage_sqlite::SqliteStorage;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use tracing::info;

#[async_trait]
pub trait ContextTrait: Send + Sync {
    async fn new() -> Result<Self>
    where
        Self: Sized;

    fn core_cfg_parser(&self) -> Arc<MystikoConfigParser>;
    fn cfg(&self) -> Arc<RollerConfig>;
    async fn db(&self) -> RwLockReadGuard<RollerDatabase<SqlStatementFormatter, SqliteStorage>>;
    fn indexer(&self) -> Option<Arc<IndexerStub>>;
    fn chain_explorer(&self) -> Option<Arc<ExplorerStub>>;
    fn provider(&self) -> Arc<Provider>;
    fn signer(&self) -> Arc<Provider>;
    async fn token_price(&self) -> RwLockWriteGuard<'_, TokenPrice>;
}

pub struct Context {
    core_cfg_parser: Arc<MystikoConfigParser>,
    cfg: Arc<RollerConfig>,
    db: RwLock<RollerDatabase<SqlStatementFormatter, SqliteStorage>>,
    indexer: Option<Arc<IndexerStub>>,
    chain_explorer: Option<Arc<ExplorerStub>>,
    provider: Arc<Provider>,
    signer: Arc<Provider>,
    token_price: Arc<RwLock<TokenPrice>>,
}

#[async_trait]
impl ContextTrait for Context {
    async fn new() -> Result<Self> {
        let roller_cfg = create_roller_config();
        trace_init(&roller_cfg.log_level);
        info!("roller config path: {:?}", load_roller_config_path());

        let token_price_cfg = create_token_price_config();
        let core_cfg_parser = MystikoConfigParser::new(&roller_cfg.core).await;
        let db = create_roller_database().await;
        let indexer = match roller_cfg.is_data_source_enable(ChainDataSource::Indexer) {
            true => core_cfg_parser.indexer_cfg().map(IndexerStub::new),
            false => None,
        };

        let chain_explorer = match roller_cfg.is_data_source_enable(ChainDataSource::Explorer) {
            true => core_cfg_parser
                .chain_explorer_cfg(roller_cfg.chain.chain_id)
                .map(ExplorerStub::new),
            false => None,
        };

        let api_key = load_coin_market_api_key()?;
        let token_price = TokenPrice::new(&token_price_cfg, &api_key)?;

        let mut providers = ProviderPool::builder()
            .chain_providers_options(Box::new(core_cfg_parser.clone()))
            .build();
        let provider = providers.get_or_create_provider(roller_cfg.chain.chain_id).await?;
        let sign_provider = create_sign_provider(roller_cfg.chain.chain_id, &core_cfg_parser).await;

        Ok(Context {
            core_cfg_parser: Arc::new(core_cfg_parser),
            cfg: Arc::new(roller_cfg),
            db: RwLock::new(db),
            indexer: indexer.map(Arc::new),
            chain_explorer: chain_explorer.map(Arc::new),
            provider,
            signer: Arc::new(sign_provider),
            token_price: Arc::new(RwLock::new(token_price)),
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
        self.signer.clone()
    }

    async fn token_price(&self) -> RwLockWriteGuard<'_, TokenPrice> {
        self.token_price.write().await
    }
}

async fn create_sign_provider(chain_id: u64, core_cfg_parser: &MystikoConfigParser) -> Provider {
    let url = core_cfg_parser.sign_endpoint(chain_id);
    let option = ProviderOptions::builder().url(url.to_string()).build();
    if HTTP_REGEX.is_match(url) {
        let options = ProvidersOptions::Http(option);
        DefaultProviderFactory::new()
            .create_provider(options)
            .await
            .expect("create sign provider failed")
    } else if WS_REGEX.is_match(url) {
        let options = ProvidersOptions::Ws(option);
        DefaultProviderFactory::new()
            .create_provider(options)
            .await
            .expect("create sign provider failed")
    } else {
        panic!("sign_endpoint is not valid");
    }
}
