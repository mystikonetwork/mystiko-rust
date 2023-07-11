use crate::config::roller::{create_mystiko_config, CoreConfig};
use anyhow::Result;
use async_trait::async_trait;
use ethers_providers::Quorum;
use mystiko_config::wrapper::chain::ChainConfig;
use mystiko_config::wrapper::contract::pool::PoolContractConfig;
use mystiko_config::wrapper::indexer::IndexerConfig;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_ethers::provider::factory::ProvidersOptions;
use mystiko_ethers::provider::pool::ChainProvidersOptions;
use mystiko_ethers::provider::types::ProviderOptions;
use mystiko_ethers::provider::types::QuorumProviderOptions;
use mystiko_types::ProviderType;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct MystikoConfigParser {
    cfg: MystikoConfig,
}

impl MystikoConfigParser {
    pub async fn new(core_cfg: &CoreConfig, cfg_path: &str) -> Result<Self> {
        let cfg = create_mystiko_config(core_cfg, cfg_path).await?;
        Ok(MystikoConfigParser { cfg })
    }

    pub fn pool_contracts_cfg(&self, chain_id: u64) -> Vec<PoolContractConfig> {
        self.cfg
            .find_chain(chain_id)
            .expect("can not find the chain")
            .pool_contracts()
            .iter()
            .map(|c| (*c).clone())
            .collect()
    }

    pub fn indexer_cfg(&self) -> Option<&IndexerConfig> {
        self.cfg.indexer()
    }

    pub fn chain_explorer_cfg(&self, chain_id: u64) -> Option<&str> {
        let chain = self.cfg.find_chain(chain_id).expect("can not find the chain");
        Some(chain.explorer_url())
    }

    pub fn chain_cfg(&self, chain_id: u64) -> &ChainConfig {
        self.cfg.find_chain(chain_id).expect("can not find the chain")
    }

    pub fn signer_endpoint(&self, chain_id: u64) -> &str {
        let chain = self.cfg.find_chain(chain_id).expect("can not find the chain");
        chain.signer_endpoint()
    }
}

#[async_trait]
impl ChainProvidersOptions for MystikoConfigParser {
    async fn providers_options(&self, chain_id: u64) -> Result<Option<ProvidersOptions>> {
        if let Some(chain_config) = self.cfg.find_chain(chain_id) {
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
