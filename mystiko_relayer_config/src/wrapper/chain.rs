use crate::raw::asset::RawAssetConfig;
use crate::raw::chain::RawChainConfig;
use crate::wrapper::asset::AssetConfig;
use crate::wrapper::transaction_info::TransactionInfoConfig;
use anyhow::{bail, Result};
use mystiko_types::{AssetType, CircuitType};
use std::sync::Arc;
use validator::Validate;

#[derive(Clone, Debug, PartialEq)]
pub struct ChainConfig {
    raw: Arc<RawChainConfig>,
    asset_configs: Vec<Arc<AssetConfig>>,
    transaction_info: TransactionInfoConfig,
}

impl ChainConfig {
    pub fn new(raw: Arc<RawChainConfig>) -> Self {
        let contract_configs = initialize_contract_configs(&raw.assets);
        let transaction_info = TransactionInfoConfig::new(raw.transaction_info.clone());
        Self {
            raw,
            asset_configs: contract_configs,
            transaction_info,
        }
    }

    pub fn name(&self) -> &str {
        &self.raw.name
    }

    pub fn chain_id(&self) -> u64 {
        self.raw.chain_id
    }

    pub fn asset_symbol(&self) -> &str {
        &self.raw.asset_symbol
    }

    pub fn relayer_contract_address(&self) -> &str {
        &self.raw.relayer_contract_address
    }

    pub fn assets(&self) -> Vec<&AssetConfig> {
        self.asset_configs.iter().map(|c| c.as_ref()).collect()
    }

    pub fn transaction_info(&self) -> &TransactionInfoConfig {
        &self.transaction_info
    }

    pub fn find_asset(&self, asset_symbol: &str) -> Option<&AssetConfig> {
        for contract_config in self.asset_configs.iter() {
            if contract_config.asset_symbol().eq_ignore_ascii_case(asset_symbol) {
                return Some(contract_config.as_ref());
            }
        }
        None
    }

    pub fn find_gas_cost(&self, asset_type: &AssetType, circuit_type: &CircuitType) -> Result<u32> {
        let gas_cost_config = match asset_type {
            AssetType::Erc20 => self.transaction_info.erc20_gas_cost().gas_cost_config(),
            AssetType::Main => self.transaction_info.main_gas_cost().gas_cost_config(),
        };
        match circuit_type {
            CircuitType::Transaction1x0 => Ok(gas_cost_config.transaction1x0),
            CircuitType::Transaction1x1 => Ok(gas_cost_config.transaction1x1),
            CircuitType::Transaction1x2 => Ok(gas_cost_config.transaction1x2),
            CircuitType::Transaction2x0 => Ok(gas_cost_config.transaction2x0),
            CircuitType::Transaction2x1 => Ok(gas_cost_config.transaction2x1),
            CircuitType::Transaction2x2 => Ok(gas_cost_config.transaction2x2),
            _ => {
                bail!("unsupported circuit type {:?}", circuit_type)
            }
        }
    }

    pub fn validate(&self) -> Result<()> {
        self.raw.validate()?;
        for asset_config in self.assets() {
            asset_config.validate()?;
        }
        self.transaction_info.validate()?;
        Ok(())
    }
}

fn initialize_contract_configs(raw_contract_configs: &[Arc<RawAssetConfig>]) -> Vec<Arc<AssetConfig>> {
    let mut asset_configs: Vec<Arc<AssetConfig>> = Vec::new();
    for raw_contract_config in raw_contract_configs {
        asset_configs.push(Arc::new(AssetConfig::new(raw_contract_config.clone())));
    }
    asset_configs
}
