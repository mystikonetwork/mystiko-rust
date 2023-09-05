use crate::{ChainProvidersOptions, ProviderOptions, ProviderPool, ProvidersOptions, QuorumProviderOptions};
use anyhow::Result;
use async_trait::async_trait;
use ethers_providers::Quorum;
use mystiko_config::MystikoConfig;
use mystiko_types::ProviderType;
use std::sync::Arc;
use std::time::Duration;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ChainConfigProvidersOptions {
    pub config: Arc<MystikoConfig>,
}

#[async_trait]
impl ChainProvidersOptions for ChainConfigProvidersOptions {
    async fn providers_options(&self, chain_id: u64) -> Result<Option<ProvidersOptions>> {
        if let Some(chain_config) = self.config.find_chain(chain_id) {
            let providers_options: Vec<ProviderOptions> = chain_config
                .providers()
                .into_iter()
                .map(|provider_config| {
                    ProviderOptions::builder()
                        .url(provider_config.url().to_string())
                        .quorum_weight(provider_config.quorum_weight() as u64)
                        .timeout_retries(provider_config.max_try_count() - 1)
                        .rate_limit_retries(provider_config.max_try_count() - 1)
                        .request_timeout(Duration::from_millis(provider_config.timeout_ms() as u64))
                        .build()
                })
                .collect();
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

impl ProviderPool<ChainConfigProvidersOptions> {
    pub fn new(config: Arc<MystikoConfig>) -> Self {
        Self::builder().chain_providers_options(config).build()
    }
}

impl From<Arc<MystikoConfig>> for ChainConfigProvidersOptions {
    fn from(config: Arc<MystikoConfig>) -> Self {
        Self::builder().config(config).build()
    }
}

impl From<Arc<MystikoConfig>> for ProviderPool<ChainConfigProvidersOptions> {
    fn from(config: Arc<MystikoConfig>) -> Self {
        Self::new(config)
    }
}
