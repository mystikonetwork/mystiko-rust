use crate::raw::chain::RawChainConfig;
use crate::raw::relayer::RawRelayerConfig;
use crate::raw::{create_raw_from_file, create_raw_from_json};
use crate::wrapper::chain::ChainConfig;
use anyhow::{Error, Result};
use std::collections::HashMap;
use std::sync::Arc;
use validator::Validate;

#[derive(Clone, Debug)]
pub struct RelayerConfig {
    raw: RawRelayerConfig,
    chain_configs: Vec<Arc<ChainConfig>>,
    default_chain_configs: HashMap<u32, Arc<ChainConfig>>,
}

impl RelayerConfig {
    fn new(raw: RawRelayerConfig) -> Result<Self> {
        let chain_configs = initialize_chain_configs(&raw.chains)?;
        let default_chain_configs = initialize_default_chain_configs(&chain_configs)?;
        Ok(RelayerConfig {
            raw,
            chain_configs,
            default_chain_configs,
        })
    }

    pub fn from_raw(raw: RawRelayerConfig) -> Result<Self> {
        let config = RelayerConfig::new(raw)?;
        config.validate()?;
        Ok(config)
    }

    pub fn from_json_str(json: &str) -> Result<Self> {
        RelayerConfig::from_raw(create_raw_from_json::<RawRelayerConfig>(json)?)
    }

    pub async fn from_json_file(json_file: &str) -> Result<Self> {
        RelayerConfig::from_raw(create_raw_from_file::<RawRelayerConfig>(json_file).await?)
    }

    pub fn version(&self) -> &str {
        &self.raw.version
    }

    pub fn find_chain_config(&self, chain_id: u32) -> Option<&ChainConfig> {
        self.default_chain_configs
            .get(&chain_id)
            .map(|c| c.as_ref())
    }

    pub fn chains(&self) -> Vec<&ChainConfig> {
        self.chain_configs.iter().map(|c| c.as_ref()).collect()
    }

    pub fn validate(&self) -> Result<()> {
        self.raw.validate()?;
        for chain_config in self.chains() {
            chain_config.validate()?;
        }
        Ok(())
    }
}

fn initialize_chain_configs(
    raw_chain_configs: &[Arc<RawChainConfig>],
) -> Result<Vec<Arc<ChainConfig>>> {
    let mut chain_configs: Vec<Arc<ChainConfig>> = Vec::new();
    for raw_chain_config in raw_chain_configs {
        chain_configs.push(Arc::new(ChainConfig::new(raw_chain_config.clone())?));
    }
    Ok(chain_configs)
}

fn initialize_default_chain_configs(
    chain_configs: &[Arc<ChainConfig>],
) -> Result<HashMap<u32, Arc<ChainConfig>>> {
    let mut configs: HashMap<u32, Arc<ChainConfig>> = HashMap::new();
    for chain_config in chain_configs.iter() {
        if configs.contains_key(&chain_config.chain_id()) {
            return Err(Error::msg(format!(
                "duplicate default chain config for chain_id {:?}",
                chain_config.chain_id()
            )));
        }
        configs.insert(chain_config.chain_id(), chain_config.clone());
    }
    Ok(configs)
}
