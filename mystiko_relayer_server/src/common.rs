use crate::configs::ServerConfig;
use anyhow::Result;
use async_trait::async_trait;
use ethers_middleware::providers::Quorum;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_ethers::provider::factory::ProvidersOptions;
use mystiko_ethers::provider::pool::ChainProvidersOptions;
use mystiko_ethers::provider::types::{ProviderOptions, QuorumProviderOptions};
use mystiko_relayer_config::wrapper::relayer::RelayerConfig;
use mystiko_types::NetworkType::Testnet;
use mystiko_types::ProviderType;
use std::sync::Arc;
use std::time::Duration;

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

pub async fn init_app_state(server_config: ServerConfig) -> Result<AppState> {
    let relayer_config_path = &server_config.options.relayer_config_path;
    let mystiko_config_path = &server_config.options.mystiko_config_path;

    // load default relayer config
    let relayer_config = match relayer_config_path {
        None => {
            let mut options = if let Some(base_url) = &server_config.options.relayer_remote_config_base_url {
                mystiko_relayer_config::wrapper::relayer::RemoteOptions::builder()
                    .base_url(base_url.to_string())
                    .build()
            } else {
                mystiko_relayer_config::wrapper::relayer::RemoteOptions::builder().build()
            };
            if server_config.settings.network_type == Testnet {
                options.is_testnet = true;
            }
            RelayerConfig::from_remote(&options).await?
        }
        Some(path) => RelayerConfig::from_json_file(path).await?,
    };

    // load default mystiko config
    let mystiko_config = match mystiko_config_path {
        None => {
            let mut options = if let Some(base_url) = &server_config.options.mystiko_remote_config_base_url {
                mystiko_config::wrapper::mystiko::RemoteOptions::builder()
                    .base_url(base_url.to_string())
                    .build()
            } else {
                mystiko_config::wrapper::mystiko::RemoteOptions::builder().build()
            };
            if server_config.settings.network_type == Testnet {
                options.is_testnet = true;
            }
            MystikoConfig::from_remote(&options).await?
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
