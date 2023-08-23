use crate::config::roller::{create_mystiko_config, CoreConfig};
use anyhow::Result;
use mystiko_config::wrapper::chain::ChainConfig;
use mystiko_config::wrapper::contract::pool::PoolContractConfig;
use mystiko_config::wrapper::indexer::IndexerConfig;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MystikoConfigParser {
    pub cfg: Arc<MystikoConfig>,
}

impl MystikoConfigParser {
    pub async fn new(core_cfg: &CoreConfig, cfg_path: &str) -> Result<Self> {
        let cfg = create_mystiko_config(core_cfg, cfg_path).await?;
        Ok(MystikoConfigParser { cfg: Arc::new(cfg) })
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
        Some(chain.explorer_api_url())
    }

    pub fn chain_cfg(&self, chain_id: u64) -> &ChainConfig {
        self.cfg.find_chain(chain_id).expect("can not find the chain")
    }

    pub fn signer_endpoint(&self, chain_id: u64) -> &str {
        let chain = self.cfg.find_chain(chain_id).expect("can not find the chain");
        chain.signer_endpoint()
    }
}
