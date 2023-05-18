use crate::common::env::{load_coin_market_api_key, load_roller_config_path};
use crate::common::trace::trace_init;
use crate::config::config::create_token_price_config;
use crate::config::config::RollerConfig;
use crate::config::config::{create_mystiko_config, create_roller_config};
use crate::config::mystiko_config_parser::MystikoConfigParser;
use crate::db::db::create_roller_database;
use crate::db::db::RollerDatabase;
use crate::instance::sync::indexer::IndexerInstance;
use crate::instance::sync::x_scan::XScanInstance;
use mystiko_ethers::provider::factory::{
    DefaultProviderFactory, Provider, ProviderFactory, ProvidersOptions, HTTP_REGEX, WS_REGEX,
};
use mystiko_ethers::provider::pool::ProviderPool;
use mystiko_ethers::provider::types::ProviderOptions;
use mystiko_server_utils::token_price::price::TokenPrice;
use mystiko_storage::formatter::SqlFormatter;
use mystiko_storage_sqlite::{SqliteRawData, SqliteStorage};
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard};

pub struct Context {
    core_cfg_parser: Arc<MystikoConfigParser>,
    cfg: Arc<RollerConfig>,
    db: RwLock<RollerDatabase<SqlFormatter, SqliteRawData, SqliteStorage>>,
    indexer: Option<RwLock<IndexerInstance>>,
    xscan: Option<RwLock<XScanInstance>>,
    providers: Arc<RwLock<ProviderPool>>,
    sign_provider: RwLock<Arc<Provider>>,
    token_price: Arc<RwLock<TokenPrice>>,
}

impl Context {
    pub async fn new() -> Self {
        let cfg_path = load_roller_config_path();
        let roller_cfg = create_roller_config(&cfg_path).await.unwrap();
        trace_init(&roller_cfg.log_level);

        let core_cfg = create_mystiko_config(&cfg_path, &roller_cfg.core).await.unwrap();
        let token_price_cfg = create_token_price_config(&cfg_path).await.unwrap();

        let core_cfg_parser = MystikoConfigParser::new(core_cfg);

        // todo use signer_endpoint for tx send
        let mut providers = ProviderPool::builder()
            .chain_providers_options(Box::new(core_cfg_parser.clone()))
            .build();
        let _ = providers.get_or_create_provider(roller_cfg.chain.chain_id).await;

        let sign_provider = create_sign_provider(roller_cfg.chain.chain_id, &core_cfg_parser).await;

        let db = create_roller_database().await;

        let indexer = core_cfg_parser.indexer_cfg().map(IndexerInstance::new);
        let xscan = core_cfg_parser
            .xscan_cfg(roller_cfg.chain.chain_id)
            .map(XScanInstance::new);

        let api_key = load_coin_market_api_key().unwrap();
        let token_price = TokenPrice::new(&token_price_cfg, &api_key).unwrap();

        Context {
            core_cfg_parser: Arc::new(core_cfg_parser),
            cfg: Arc::new(roller_cfg),
            db: RwLock::new(db),
            indexer: indexer.map(RwLock::new),
            xscan: xscan.map(RwLock::new),
            providers: Arc::new(RwLock::new(providers)),
            sign_provider: RwLock::new(Arc::new(sign_provider)),
            token_price: Arc::new(RwLock::new(token_price)),
        }
    }

    pub fn core_cfg_parser(&self) -> Arc<MystikoConfigParser> {
        Arc::clone(&self.core_cfg_parser)
    }

    pub fn cfg(&self) -> Arc<RollerConfig> {
        Arc::clone(&self.cfg)
    }

    pub async fn db(&self) -> RwLockReadGuard<RollerDatabase<SqlFormatter, SqliteRawData, SqliteStorage>> {
        self.db.read().await
    }

    pub async fn indexer(&self) -> Option<RwLockReadGuard<IndexerInstance>> {
        Some(self.indexer.as_ref()?.read().await)
    }

    pub async fn xscan(&self) -> Option<RwLockReadGuard<XScanInstance>> {
        Some(self.xscan.as_ref()?.read().await)
    }

    pub async fn providers(&self) -> RwLockReadGuard<ProviderPool> {
        self.providers.read().await
    }

    pub async fn sign_provider(&self) -> Arc<Provider> {
        let provider_guard = self.sign_provider.read().await;
        Arc::clone(&*provider_guard)
    }

    pub async fn token_price(&self) -> tokio::sync::RwLockWriteGuard<'_, TokenPrice> {
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
