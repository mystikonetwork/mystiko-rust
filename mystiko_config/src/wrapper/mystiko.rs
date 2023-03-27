use crate::raw::chain::RawChainConfig;
use crate::raw::mystiko::RawMystikoConfig;
use crate::raw::{create_raw_from_file, create_raw_from_json};
use crate::types::{BridgeType, CircuitType};
use crate::wrapper::bridge::BridgeConfig;
use crate::wrapper::chain::ChainConfig;
use crate::wrapper::circuit::CircuitConfig;
use crate::wrapper::contract::deposit::DepositContractConfig;
use crate::wrapper::contract::pool::PoolContractConfig;
use crate::wrapper::indexer::IndexerConfig;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct MystikoConfig {
    raw: RawMystikoConfig,
    chain_configs: Vec<Arc<ChainConfig>>,
    circuit_configs: Vec<Arc<CircuitConfig>>,
    bridge_configs: Vec<Arc<BridgeConfig>>,
    indexer_config: Option<IndexerConfig>,
    default_circuit_configs: HashMap<CircuitType, Arc<CircuitConfig>>,
    circuit_configs_by_name: HashMap<String, Arc<CircuitConfig>>,
}

impl MystikoConfig {
    pub fn new(raw: RawMystikoConfig) -> Result<Self> {
        let circuit_configs: Vec<Arc<CircuitConfig>> = raw
            .circuits
            .iter()
            .map(|r| Arc::new(CircuitConfig::new(r.clone())))
            .collect();
        let bridge_configs: Vec<Arc<BridgeConfig>> = raw
            .bridges
            .iter()
            .map(|r| Arc::new(BridgeConfig::new(r.clone())))
            .collect();
        let indexer_config = raw.indexer.as_ref().map(|r| IndexerConfig::new(r.clone()));
        let default_circuit_configs = initialize_default_circuit_configs(&circuit_configs);
        let circuit_configs_by_name = initialize_circuit_configs_by_name(&circuit_configs);
        let chain_configs = initialize_chain_configs(
            &raw.chains,
            &default_circuit_configs,
            &circuit_configs_by_name,
        )?;
        Ok(MystikoConfig {
            raw,
            chain_configs,
            bridge_configs,
            circuit_configs,
            indexer_config,
            default_circuit_configs,
            circuit_configs_by_name,
        })
    }

    pub fn from_json_str(json_str: &str) -> Result<Self> {
        MystikoConfig::new(create_raw_from_json::<RawMystikoConfig>(json_str)?)
    }

    pub async fn from_json_file(json_file: &str) -> Result<Self> {
        MystikoConfig::new(create_raw_from_file::<RawMystikoConfig>(json_file).await?)
    }

    pub fn version(&self) -> &str {
        &self.raw.version
    }

    pub fn circuits(&self) -> Vec<&CircuitConfig> {
        self.circuit_configs.iter().map(|c| c.as_ref()).collect()
    }

    pub fn bridges(&self) -> Vec<&BridgeConfig> {
        self.bridge_configs.iter().map(|c| c.as_ref()).collect()
    }

    pub fn chains(&self) -> Vec<&ChainConfig> {
        self.chain_configs.iter().map(|c| c.as_ref()).collect()
    }

    pub fn indexer(&self) -> Option<&IndexerConfig> {
        self.indexer_config.as_ref()
    }

    pub fn find_default_circuit(&self, circuit_type: &CircuitType) -> Option<&CircuitConfig> {
        self.default_circuit_configs
            .get(circuit_type)
            .map(|c| c.as_ref())
    }

    pub fn find_circuit(&self, circuit_name: &str) -> Option<&CircuitConfig> {
        self.circuit_configs_by_name
            .get(circuit_name)
            .map(|c| c.as_ref())
    }

    pub fn find_chain(&self, chain_id: u32) -> Option<&ChainConfig> {
        self.chain_configs
            .iter()
            .find(|c| c.chain_id() == chain_id)
            .map(|c| c.as_ref())
    }

    pub fn find_peer_chains(&self, chain_id: u32) -> Vec<&ChainConfig> {
        let mut peer_chains: Vec<&ChainConfig> = Vec::new();
        if let Some(peer_chain_ids) = self.find_chain(chain_id).map(|c| c.find_peer_chain_ids()) {
            for peer_chain_id in peer_chain_ids {
                if let Some(peer_chain) = self.find_chain(peer_chain_id) {
                    peer_chains.push(peer_chain);
                }
            }
        }
        peer_chains
    }

    pub fn find_asset_symbols(&self, chain_id: u32, peer_chain_id: u32) -> Vec<&str> {
        self.find_chain(chain_id)
            .map(|c| c.find_asset_symbols(peer_chain_id))
            .unwrap_or(vec![])
    }

    pub fn find_bridges(
        &self,
        chain_id: u32,
        peer_chain_id: u32,
        asset_symbol: &str,
    ) -> Vec<&BridgeType> {
        self.find_chain(chain_id)
            .map(|c| c.find_bridges(peer_chain_id, asset_symbol))
            .unwrap_or(vec![])
    }

    pub fn find_bridge(&self, bridge_type: &BridgeType) -> Option<&BridgeConfig> {
        self.bridges()
            .into_iter()
            .find(|c| c.bridge_type() == bridge_type)
    }

    pub fn find_deposit_contract(
        &self,
        chain_id: u32,
        peer_chain_id: u32,
        asset_symbol: &str,
        bridge_type: &BridgeType,
    ) -> Option<&DepositContractConfig> {
        if let Some(chain_config) = self.find_chain(chain_id) {
            chain_config.find_deposit_contract(peer_chain_id, asset_symbol, bridge_type)
        } else {
            None
        }
    }

    pub fn find_deposit_contract_by_address(
        &self,
        chain_id: u32,
        address: &str,
    ) -> Option<&DepositContractConfig> {
        if let Some(chain_config) = self.find_chain(chain_id) {
            chain_config.find_deposit_contract_by_address(address)
        } else {
            None
        }
    }

    pub fn find_pool_contracts(
        &self,
        chain_id: u32,
        asset_symbol: &str,
        bridge_type: &BridgeType,
    ) -> Vec<&PoolContractConfig> {
        self.find_chain(chain_id)
            .map(|c| c.find_pool_contracts(asset_symbol, bridge_type))
            .unwrap_or(vec![])
    }

    pub fn find_pool_contract(
        &self,
        chain_id: u32,
        asset_symbol: &str,
        bridge_type: &BridgeType,
        version: u32,
    ) -> Option<&PoolContractConfig> {
        if let Some(chain_config) = self.find_chain(chain_id) {
            chain_config.find_pool_contract(asset_symbol, bridge_type, version)
        } else {
            None
        }
    }

    pub fn find_pool_contract_by_address(
        &self,
        chain_id: u32,
        address: &str,
    ) -> Option<&PoolContractConfig> {
        if let Some(chain_config) = self.find_chain(chain_id) {
            chain_config.find_pool_contract_by_address(address)
        } else {
            None
        }
    }

    pub fn transaction_url(&self, chain_id: u32, tx_hash: &str) -> Option<String> {
        self.find_chain(chain_id)
            .map(|c| c.transaction_url(tx_hash))
    }
}

fn initialize_chain_configs(
    raw_chain_configs: &[Arc<RawChainConfig>],
    default_circuit_configs: &HashMap<CircuitType, Arc<CircuitConfig>>,
    circuit_configs_by_name: &HashMap<String, Arc<CircuitConfig>>,
) -> Result<Vec<Arc<ChainConfig>>> {
    let mut chain_configs: Vec<Arc<ChainConfig>> = Vec::new();
    for raw_chain_config in raw_chain_configs {
        chain_configs.push(Arc::new(ChainConfig::new(
            raw_chain_config.clone(),
            default_circuit_configs,
            circuit_configs_by_name,
        )?));
    }
    Ok(chain_configs)
}

fn initialize_default_circuit_configs(
    circuit_configs: &[Arc<CircuitConfig>],
) -> HashMap<CircuitType, Arc<CircuitConfig>> {
    let mut configs: HashMap<CircuitType, Arc<CircuitConfig>> = HashMap::new();
    for circuit_config in circuit_configs.iter() {
        if circuit_config.is_default() {
            configs.insert(
                *circuit_config.circuit_type(),
                circuit_config.clone(),
            );
        }
    }
    configs
}

fn initialize_circuit_configs_by_name(
    circuit_configs: &[Arc<CircuitConfig>],
) -> HashMap<String, Arc<CircuitConfig>> {
    let mut configs: HashMap<String, Arc<CircuitConfig>> = HashMap::new();
    for circuit_config in circuit_configs.iter() {
        configs.insert(circuit_config.name().to_string(), circuit_config.clone());
    }
    configs
}
