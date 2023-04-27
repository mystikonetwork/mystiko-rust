use ethers_providers::Quorum;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_ethers::provider::factory::ProvidersOptions;
use mystiko_ethers::provider::pool::ChainProvidersOptions;
use mystiko_ethers::provider::types::{ProviderOptions, QuorumProviderOptions};
use mystiko_types::ProviderType;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug)]
pub struct ProvidersConfig {
    config: Arc<MystikoConfig>,
}

impl ProvidersConfig {
    pub fn new(config: Arc<MystikoConfig>) -> Self {
        ProvidersConfig { config }
    }
}

impl ChainProvidersOptions for ProvidersConfig {
    fn providers_options(&self, chain_id: u64) -> Option<ProvidersOptions> {
        if let Some(chain_config) = self.config.find_chain(chain_id) {
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
                ProviderType::Failover => Some(ProvidersOptions::Failover(providers_options)),
                ProviderType::Quorum => {
                    let quorum_options = QuorumProviderOptions::builder()
                        .quorum(Quorum::Percentage(
                            chain_config.provider_quorum_percentage(),
                        ))
                        .build();
                    Some(ProvidersOptions::Quorum(providers_options, quorum_options))
                }
            }
        } else {
            None
        }
    }
}
