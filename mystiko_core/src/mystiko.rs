use crate::error::MystikoError;
use crate::handler::account::AccountHandler;
use crate::handler::wallet::WalletHandler;
use crate::helper::provider::ProvidersConfig;
use anyhow::Result;
use mystiko_config::wrapper::mystiko::{MystikoConfig, RemoteOptions};
use mystiko_database::database::Database;
use mystiko_ethers::provider::factory::ProviderFactory;
use mystiko_ethers::provider::pool::{ChainProvidersOptions, ProviderPool};
use mystiko_storage::document::DocumentRawData;
use mystiko_storage::formatter::StatementFormatter;
use mystiko_storage::storage::Storage;
use std::sync::Arc;
use tokio::sync::RwLock;
use typed_builder::TypedBuilder;

pub struct Mystiko<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    pub db: Arc<Database<F, R, S>>,
    pub config: Arc<MystikoConfig>,
    pub accounts: AccountHandler<F, R, S>,
    pub wallets: WalletHandler<F, R, S>,
    pub providers: Arc<RwLock<ProviderPool>>,
}

#[derive(Debug, TypedBuilder)]
pub struct MystikoOptions {
    #[builder(default, setter(strip_option))]
    pub config_file_path: Option<String>,
    #[builder(default, setter(strip_option))]
    pub config_remote_base_url: Option<String>,
    #[builder(default, setter(strip_option))]
    pub config_git_revision: Option<String>,
    #[builder(default = false)]
    pub is_testnet: bool,
    #[builder(default = false)]
    pub is_staging: bool,
    #[builder(default, setter(strip_option))]
    pub provider_factory: Option<Box<dyn ProviderFactory>>,
}

impl<F, R, S> Mystiko<F, R, S>
where
    F: StatementFormatter,
    R: DocumentRawData,
    S: Storage<R>,
{
    pub async fn new(
        database: Database<F, R, S>,
        options: Option<MystikoOptions>,
    ) -> Result<Self, MystikoError> {
        let mystiko_options = options.unwrap_or(MystikoOptions::builder().build());
        database
            .migrate()
            .await
            .map_err(MystikoError::DatabaseMigrationError)?;
        let db = Arc::new(database);
        let config = create_mystiko_config(&mystiko_options).await?;
        let accounts = AccountHandler::new(db.clone());
        let wallets = WalletHandler::new(db.clone());
        let providers = create_provider_pool(config.clone(), mystiko_options.provider_factory);
        let mystiko = Self {
            db,
            config: config.clone(),
            accounts,
            wallets,
            providers,
        };
        log::info!(
            "mystiko on {} has been initialized, config git revision {}",
            if mystiko_options.is_testnet {
                "testnet"
            } else {
                "mainnet"
            },
            config.git_revision().unwrap_or("unknown")
        );
        Ok(mystiko)
    }
}

async fn create_mystiko_config(
    mystiko_options: &MystikoOptions,
) -> Result<Arc<MystikoConfig>, MystikoError> {
    let config = if let Some(config_file_path) = &mystiko_options.config_file_path {
        MystikoConfig::from_json_file(config_file_path)
            .await
            .map_err(MystikoError::ConfigError)?
    } else {
        let mut remote_options = RemoteOptions::builder()
            .is_testnet(mystiko_options.is_testnet)
            .is_staging(mystiko_options.is_staging)
            .build();
        remote_options.base_url = mystiko_options.config_remote_base_url.clone();
        remote_options.git_revision = mystiko_options.config_git_revision.clone();
        MystikoConfig::from_remote(&remote_options)
            .await
            .map_err(MystikoError::ConfigError)?
    };
    Ok(Arc::new(config))
}

fn create_provider_pool(
    config: Arc<MystikoConfig>,
    provider_factory: Option<Box<dyn ProviderFactory>>,
) -> Arc<RwLock<ProviderPool>> {
    let chain_config_options: Box<dyn ChainProvidersOptions> =
        Box::new(ProvidersConfig::new(config));
    let provider_pool_builder =
        ProviderPool::builder().chain_providers_options(chain_config_options);
    let provider_pool = if let Some(provider_factory) = provider_factory {
        provider_pool_builder
            .provider_factory(provider_factory)
            .build()
    } else {
        provider_pool_builder.build()
    };
    Arc::new(RwLock::new(provider_pool))
}
