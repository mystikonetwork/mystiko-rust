use crate::{Database, MystikoError, SignerProviderPool};
use async_trait::async_trait;
use mystiko_config::MystikoConfig;
use mystiko_ethers::{ChainConfigProvidersOptions, ProviderFactory, ProviderPool, Providers};
use mystiko_protos::common::v1::{ConfigOptions, ConfigOptionsOption};
use mystiko_protos::loader::v1::LoaderConfig;
use mystiko_storage::{StatementFormatter, Storage};
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct MystikoOptions {
    pub config_options: Option<ConfigOptions>,
    pub loader_config: Option<LoaderConfig>,
    pub provider_factory: Option<Box<dyn ProviderFactory>>,
    pub signer_provider_factory: Option<Box<dyn ProviderFactory>>,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct MystikoContext<F: StatementFormatter, S: Storage> {
    pub network: String,
    pub db: Arc<Database<F, S>>,
    pub config: Arc<MystikoConfig>,
    pub providers: Arc<Box<dyn Providers>>,
    pub signer_providers: Arc<Box<dyn Providers>>,
    pub loader_config: Option<LoaderConfig>,
}

#[async_trait]
pub trait FromContext<F, S>: Send + Sync + Sized
where
    F: StatementFormatter,
    S: Storage,
{
    async fn from_context(context: &MystikoContext<F, S>) -> Result<Self, MystikoError>;
}

impl<F, S> MystikoContext<F, S>
where
    F: StatementFormatter,
    S: Storage,
{
    pub async fn new(db: Database<F, S>, options: Option<MystikoOptions>) -> Result<Self, MystikoError> {
        let db = Arc::new(db);
        let mystiko_options = options.unwrap_or(MystikoOptions::builder().build());
        let network = mystiko_options.config_options.get_network();
        let config = create_mystiko_config(&mystiko_options).await?;
        let providers: ProviderPool<ChainConfigProvidersOptions> =
            if let Some(provider_factory) = mystiko_options.provider_factory {
                ProviderPool::builder()
                    .chain_providers_options(ChainConfigProvidersOptions::builder().config(config.clone()).build())
                    .provider_factory(provider_factory)
                    .build()
            } else {
                config.clone().into()
            };
        let signer_providers = SignerProviderPool::builder()
            .chain_providers_options(config.clone())
            .provider_factory(mystiko_options.signer_provider_factory)
            .build();
        let providers = Arc::new(Box::new(providers) as Box<dyn Providers>);
        let signer_providers = Arc::new(Box::new(signer_providers) as Box<dyn Providers>);
        Ok(Self {
            network,
            db,
            config,
            providers,
            signer_providers,
            loader_config: mystiko_options.loader_config,
        })
    }
}

async fn create_mystiko_config(mystiko_options: &MystikoOptions) -> Result<Arc<MystikoConfig>, MystikoError> {
    let config = MystikoConfig::from_options(mystiko_options.config_options.clone())
        .await
        .map_err(MystikoError::ConfigError)?;
    Ok(Arc::new(config))
}
