use crate::{Database, MystikoError, SignerProviderPool};
use async_trait::async_trait;
use mystiko_config::MystikoConfig;
use mystiko_ethers::{ChainConfigProvidersOptions, ProviderFactory, ProviderPool, Providers};
use mystiko_protos::common::v1::{ConfigOptions, ConfigOptionsOption};
use mystiko_protos::loader::v1::LoaderConfig;
use mystiko_protos::relayer::v1::RelayerClientOptions;
use mystiko_protos::screening::v1::ScreeningClientOptions;
use mystiko_static_cache::{SkipStaticCache, StaticCache};
use mystiko_storage::{StatementFormatter, Storage};
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct MystikoOptions {
    pub config_options: Option<ConfigOptions>,
    pub relayer_client_options: Option<RelayerClientOptions>,
    pub screening_client_options: Option<ScreeningClientOptions>,
    pub loader_config: Option<LoaderConfig>,
    pub provider_factory: Option<Box<dyn ProviderFactory>>,
    pub signer_provider_factory: Option<Box<dyn ProviderFactory>>,
    pub static_cache: Option<Box<dyn StaticCache>>,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct MystikoContext<F: StatementFormatter, S: Storage> {
    pub network: String,
    pub db: Arc<Database<F, S>>,
    pub static_cache: Arc<Box<dyn StaticCache>>,
    pub config: Arc<MystikoConfig>,
    pub providers: Arc<Box<dyn Providers>>,
    pub signer_providers: Arc<Box<dyn Providers>>,
    pub config_options: ConfigOptions,
    pub relayer_client_options: RelayerClientOptions,
    pub screening_client_options: ScreeningClientOptions,
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
        let static_cache = SkipStaticCache::default();
        let static_cache = mystiko_options
            .static_cache
            .unwrap_or(Box::new(static_cache) as Box<dyn StaticCache>);
        let network = mystiko_options.config_options.get_network();
        let config_options = mystiko_options.config_options.unwrap_or_default();
        let mut relayer_client_options = mystiko_options.relayer_client_options.unwrap_or_default();
        relayer_client_options.is_testnet = config_options.is_testnet;
        relayer_client_options.is_staging = config_options.is_staging;
        let screening_client_options = mystiko_options.screening_client_options.unwrap_or_default();
        let config = create_mystiko_config(config_options.clone()).await?;
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
            static_cache: Arc::new(static_cache),
            config,
            providers,
            signer_providers,
            config_options,
            relayer_client_options,
            screening_client_options,
            loader_config: mystiko_options.loader_config,
        })
    }
}

async fn create_mystiko_config(config_options: ConfigOptions) -> Result<Arc<MystikoConfig>, MystikoError> {
    let config = MystikoConfig::from_options(config_options)
        .await
        .map_err(MystikoError::ConfigError)?;
    Ok(Arc::new(config))
}
