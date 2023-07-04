use crate::configs::{load_config, ServerConfig};
use anyhow::Result;
use async_trait::async_trait;
use ethers_middleware::providers::Quorum;
use log::{info, LevelFilter};
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_ethers::provider::factory::ProvidersOptions;
use mystiko_ethers::provider::pool::ChainProvidersOptions;
use mystiko_ethers::provider::types::{ProviderOptions, QuorumProviderOptions};
use mystiko_relayer_config::wrapper::relayer::RelayerConfig;
use mystiko_types::NetworkType::Testnet;
use mystiko_types::ProviderType;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use typed_builder::TypedBuilder;

#[derive(Clone, Debug)]
pub struct AppState {
    pub server_config: Arc<ServerConfig>,
    pub relayer_config: Arc<RelayerConfig>,
    pub mystiko_config: Arc<MystikoConfig>,
}

#[async_trait]
impl ChainProvidersOptions for AppState {
    async fn providers_options(&self, chain_id: u64) -> Result<Option<ProvidersOptions>> {
        if let Some(chain_config) = self.mystiko_config.find_chain(chain_id) {
            let mut providers_options: Vec<ProviderOptions> = vec![];
            for provider_config in chain_config.providers() {
                let provider_options = ProviderOptions::builder()
                    .url(provider_config.url().to_string())
                    .quorum_weight(provider_config.quorum_weight() as u64)
                    .timeout_retries(provider_config.max_try_count() - 1)
                    .rate_limit_retries(provider_config.max_try_count() - 1)
                    .request_timeout(Duration::from_millis(provider_config.timeout_ms() as u64))
                    .build();
                providers_options.push(provider_options);
            }

            match chain_config.provider_type() {
                ProviderType::Failover => Ok(Some(ProvidersOptions::Failover(providers_options))),
                ProviderType::Quorum => {
                    let quorum_options = QuorumProviderOptions::builder()
                        .quorum(Quorum::Percentage(chain_config.provider_quorum_percentage()))
                        .build();
                    Ok(Some(ProvidersOptions::Quorum(providers_options, quorum_options)))
                }
            }
        } else {
            Ok(None)
        }
    }
}

#[derive(TypedBuilder)]
pub struct AppStateOptions<'a> {
    log_level: &'a str,
    server_config_path: &'a str,
    #[builder(default)]
    relayer_config_path: Option<&'a str>,
    #[builder(default)]
    mystiko_config_path: Option<&'a str>,
}

#[allow(clippy::needless_lifetimes)]
pub async fn init_app_state<'a>(options: AppStateOptions<'a>) -> Result<AppState> {
    // try init logger
    let _ = env_logger::builder()
        .filter_module("", LevelFilter::from_str(options.log_level)?)
        .try_init();

    // load server config
    let server_config = load_config(options.server_config_path)?;
    info!("load server config successful");

    // load default relayer config
    let relayer_config = match options.relayer_config_path {
        None => {
            if server_config.network_type == Testnet {
                RelayerConfig::from_remote_default_testnet().await?
            } else {
                RelayerConfig::from_remote_default_mainnet().await?
            }
        }
        Some(path) => RelayerConfig::from_json_file(path).await?,
    };

    // load default mystiko config
    let mystiko_config = match options.mystiko_config_path {
        None => {
            if server_config.network_type == Testnet {
                MystikoConfig::from_remote_default_testnet().await?
            } else {
                MystikoConfig::from_remote_default_mainnet().await?
            }
        }
        Some(path) => MystikoConfig::from_json_file(path).await?,
    };

    // validation config
    server_config.validation(&relayer_config)?;

    Ok(AppState {
        server_config: Arc::new(server_config),
        relayer_config: Arc::new(relayer_config),
        mystiko_config: Arc::new(mystiko_config),
    })
}
