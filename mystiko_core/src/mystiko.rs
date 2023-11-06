use crate::{
    AccountHandler, Database, MystikoError, Synchronizer, SynchronizerHandler, SynchronizerOptions, WalletHandler,
};
use anyhow::Result;
use mystiko_config::MystikoConfig;
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::loader::ChainDataLoader;
use mystiko_ethers::{ChainConfigProvidersOptions, ProviderFactory, ProviderPool, Providers};
use mystiko_protos::common::v1::{ConfigOptions, ConfigOptionsOption};
use mystiko_protos::core::synchronizer::v1::{SyncOptions, SynchronizerStatus};
use mystiko_protos::loader::v1::LoaderConfig;
use mystiko_storage::{StatementFormatter, Storage};
use std::sync::Arc;
use typed_builder::TypedBuilder;

pub struct Mystiko<
    F: StatementFormatter,
    S: Storage,
    Y: SynchronizerHandler<SyncOptions, SynchronizerStatus> = Synchronizer<ChainDataLoader<FullData>>,
> {
    pub db: Arc<Database<F, S>>,
    pub config: Arc<MystikoConfig>,
    pub accounts: AccountHandler<F, S>,
    pub wallets: WalletHandler<F, S>,
    pub synchronizer: Y,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct MystikoOptions {
    pub config_options: Option<ConfigOptions>,
    pub loader_config: Option<LoaderConfig>,
    pub provider_factory: Option<Box<dyn ProviderFactory>>,
}

impl<F, S> Mystiko<F, S>
where
    F: StatementFormatter + 'static,
    S: Storage + 'static,
{
    pub async fn new(database: Database<F, S>, options: Option<MystikoOptions>) -> Result<Self, MystikoError> {
        let mystiko_options = options.unwrap_or(MystikoOptions::builder().build());
        database.migrate().await.map_err(MystikoError::DatabaseMigrationError)?;
        let db = Arc::new(database);
        let config = create_mystiko_config(&mystiko_options).await?;
        let accounts = AccountHandler::new(db.clone());
        let wallets = WalletHandler::new(db.clone());
        let providers: ProviderPool<ChainConfigProvidersOptions> =
            if let Some(provider_factory) = mystiko_options.provider_factory {
                ProviderPool::builder()
                    .chain_providers_options(ChainConfigProvidersOptions::builder().config(config.clone()).build())
                    .provider_factory(provider_factory)
                    .build()
            } else {
                config.clone().into()
            };
        let providers = Arc::new(Box::new(providers) as Box<dyn Providers>);
        let synchronizer = Synchronizer::new(
            SynchronizerOptions::<F, S>::builder()
                .db(db.clone())
                .mystiko_config(config.clone())
                .providers(providers.clone())
                .loader_config(mystiko_options.loader_config)
                .build(),
        )
        .await?;
        let mystiko = Self {
            db,
            config: config.clone(),
            accounts,
            wallets,
            synchronizer,
        };
        log::info!(
            "mystiko on {} has been initialized, config git revision {}",
            mystiko_options.config_options.get_network(),
            config.git_revision().unwrap_or("unknown")
        );
        Ok(mystiko)
    }
}

async fn create_mystiko_config(mystiko_options: &MystikoOptions) -> Result<Arc<MystikoConfig>, MystikoError> {
    let config = MystikoConfig::from_options(mystiko_options.config_options.clone())
        .await
        .map_err(MystikoError::ConfigError)?;
    Ok(Arc::new(config))
}
