use std::collections::HashMap;
use strum::IntoEnumIterator;
use mystiko_utils::check::check;
use crate::common::{BridgeType, CircuitType};
use crate::raw::mystiko::{RawBridgeConfigType, RawMystikoConfig};
use crate::wrapper::base::BaseConfig;
use crate::wrapper::bridge::axelar::AxelarBridgeConfig;
use crate::wrapper::bridge::celer::CelerBridgeConfig;
use crate::wrapper::bridge::layer_zero::LayerZeroBridgeConfig;
use crate::wrapper::bridge::poly::PolyBridgeConfig;
use crate::wrapper::bridge::tbridge::TBridgeConfig;
use crate::wrapper::chain::{AuxData, ChainConfig};
use crate::wrapper::circuit::CircuitConfig;
use crate::wrapper::contract::deposit::DepositContractConfig;
use crate::wrapper::contract::pool::PoolContractConfig;
use crate::wrapper::indexer::IndexerConfig;

#[derive(Clone)]
pub enum BridgeConfigType {
    AxelarBridgeConfig(AxelarBridgeConfig),
    CelerBridgeConfig(CelerBridgeConfig),
    PolyBridgeConfig(PolyBridgeConfig),
    LayerZeroBridgeConfig(LayerZeroBridgeConfig),
    TBridgeConfig(TBridgeConfig),
}

pub struct MystikoConfig {
    base: BaseConfig<RawMystikoConfig>,
    default_circuit_configs: HashMap<CircuitType, CircuitConfig>,
    circuit_configs_by_name: HashMap<String, CircuitConfig>,
    bridge_configs: HashMap<BridgeType, BridgeConfigType>,
    chain_configs: HashMap<u32, ChainConfig>,
    indexer_config: Option<IndexerConfig>,
}

impl MystikoConfig {
    pub fn new(data: RawMystikoConfig) -> Self {
        let base = BaseConfig::new(data, None);
        let (
            default_circuit_configs,
            circuit_configs_by_name
        ) = MystikoConfig::init_circuit_configs(&base);
        let config = Self {
            base: base.clone(),
            default_circuit_configs: default_circuit_configs.clone(),
            circuit_configs_by_name: circuit_configs_by_name.clone(),
            bridge_configs: MystikoConfig::init_bridge_configs(&base),
            chain_configs: MystikoConfig::init_chain_configs(
                &base,
                default_circuit_configs,
                circuit_configs_by_name,
            ),
            indexer_config: MystikoConfig::init_indexer_config(&base),
        };
        config.validate();
        config
    }

    pub fn version(&self) -> &str {
        &self.base.data.version
    }

    pub fn circuits(&self) -> Vec<CircuitConfig> {
        self.circuit_configs_by_name.values().cloned().collect()
    }

    pub fn bridges(&self) -> Vec<BridgeConfigType> {
        self.bridge_configs.values().cloned().collect()
    }

    pub fn chains(&self) -> Vec<ChainConfig> {
        self.chain_configs.values().cloned().collect()
    }

    pub fn indexer(&self) -> &Option<IndexerConfig> {
        &self.indexer_config
    }

    pub fn get_chain_config(&self, chain_id: u32) -> Option<&ChainConfig> {
        self.chain_configs.get(&chain_id)
    }

    pub fn get_peer_chain_configs(&self, chain_id: u32) -> Vec<ChainConfig> {
        let mut peer_chain_configs: Vec<ChainConfig> = Vec::new();
        let chain_config = self.get_chain_config(chain_id);
        if chain_config.is_some() {
            for peer_chain_id in chain_config.unwrap().peer_chain_ids() {
                let peer_chain_config = self.get_chain_config(peer_chain_id);
                if peer_chain_config.is_some() {
                    peer_chain_configs.push(peer_chain_config.unwrap().clone())
                }
            }
        }

        peer_chain_configs
    }

    pub fn get_asset_symbols(&self, chain_id: u32, peer_chain_id: u32) -> Vec<String> {
        match self.get_chain_config(chain_id) {
            None => { vec![] }
            Some(config) => {
                config.get_asset_symbols(peer_chain_id)
            }
        }
    }

    pub fn get_bridges(&self, chain_id: u32, peer_chain_id: u32, asset_symbol: &str) -> Vec<BridgeConfigType> {
        let mut bridges: Vec<BridgeConfigType> = Vec::new();
        let chain_config = self.get_chain_config(chain_id);
        if chain_config.is_some() {
            for bridge_type in chain_config.unwrap().get_bridges(peer_chain_id, asset_symbol) {
                let bridge_config = self.get_bridge_config(bridge_type);
                if bridge_config.is_some() {
                    bridges.push(bridge_config.unwrap().clone());
                }
            }
        }

        bridges
    }

    pub fn get_deposit_contract_config(
        &self,
        chain_id: u32,
        peer_chain_id: u32,
        asset_symbol: &str,
        bridge_type: BridgeType,
    ) -> Option<DepositContractConfig> {
        match self.get_chain_config(chain_id) {
            None => { None }
            Some(config) => {
                config.get_deposit_contract(peer_chain_id, asset_symbol, bridge_type)
            }
        }
    }

    pub fn get_deposit_contract_config_by_address(
        &self,
        chain_id: u32,
        address: String,
    ) -> Option<&DepositContractConfig> {
        let chain_config = self.get_chain_config(chain_id).clone();
        match chain_config {
            Some(config) => {
                config.get_deposit_contract_by_address(address)
            }
            None => { None }
        }
    }

    pub fn get_pool_contract_config(
        &self,
        chain_id: u32,
        asset_symbol: &str,
        bridge_type: BridgeType,
        version: u32,
    ) -> Option<&PoolContractConfig> {
        match self.get_chain_config(chain_id) {
            None => { None }
            Some(config) => {
                config.get_pool_contract(asset_symbol, bridge_type, version)
            }
        }
    }

    pub fn get_pool_contract_configs(
        &self,
        chain_id: u32,
        asset_symbol: &str,
        bridge_type: BridgeType,
    ) -> Vec<PoolContractConfig> {
        match self.get_chain_config(chain_id) {
            None => { vec![] }
            Some(config) => {
                config.get_pool_contracts(asset_symbol, bridge_type)
            }
        }
    }

    pub fn get_pool_contract_config_by_address(&self, chain_id: u32, address: &str) -> Option<&PoolContractConfig> {
        let chain_config = self.get_chain_config(chain_id);
        if chain_config.is_some() {
            return chain_config.unwrap().get_pool_contract_by_address(address);
        }

        None
    }

    pub fn get_bridge_config(&self, bridge_type: BridgeType) -> Option<&BridgeConfigType> {
        self.bridge_configs.get(&bridge_type)
    }

    pub fn get_default_circuit_config(&self, circuit_type: CircuitType) -> Option<&CircuitConfig> {
        self.default_circuit_configs.get(&circuit_type)
    }

    pub fn get_circuit_config_by_name(&self, name: &str) -> Option<&CircuitConfig> {
        self.circuit_configs_by_name.get(name)
    }

    pub fn get_transaction_url(&self, chain_id: u32, transaction_hash: &str) -> Option<String> {
        let chain_config = self.get_chain_config(chain_id);
        if chain_config.is_some() {
            return Some(chain_config.unwrap().get_transaction_url(transaction_hash));
        }

        None
    }

    pub fn mutate(&self, data: Option<RawMystikoConfig>) -> Self {
        match data {
            None => {
                MystikoConfig::new(self.base.data.clone())
            }
            Some(config) => {
                MystikoConfig::new(config)
            }
        }
    }

    fn init_circuit_configs(base: &BaseConfig<RawMystikoConfig>) -> (HashMap<CircuitType, CircuitConfig>, HashMap<String, CircuitConfig>) {
        let mut default_circuit_configs: HashMap<CircuitType, CircuitConfig> = HashMap::new();
        let mut circuit_config_by_names: HashMap<String, CircuitConfig> = HashMap::new();

        let circuits = &base.data.circuits;
        for raw in circuits {
            let circuit_config = CircuitConfig::new(
                BaseConfig::new(raw.clone(), None),
            );
            if raw.is_default {
                check(
                    !default_circuit_configs.contains_key(circuit_config.circuit_type()),
                    format!(
                        "duplicate default circuit type={:?} definition", circuit_config.circuit_type()
                    ).as_str(),
                );
                default_circuit_configs.insert(*circuit_config.circuit_type(), circuit_config.clone());
            }
            circuit_config_by_names.insert(circuit_config.name().clone(), circuit_config.clone());
        }
        let mut has_pool_contracts = false;
        for chain in &base.data.chains {
            if !chain.pool_contracts.is_empty() {
                has_pool_contracts = true;
                break;
            }
        }

        if has_pool_contracts {
            for circuit_type in CircuitType::iter() {
                check(
                    default_circuit_configs.contains_key(&circuit_type),
                    format!(
                        "missing definition of default circuit type={:?}", circuit_type
                    ).as_str(),
                );
            }
        }

        (default_circuit_configs, circuit_config_by_names)
    }

    fn init_bridge_configs(base: &BaseConfig<RawMystikoConfig>) -> HashMap<BridgeType, BridgeConfigType> {
        let mut bridge_configs: HashMap<BridgeType, BridgeConfigType> = HashMap::new();
        for bridge in &base.data.bridges {
            match bridge {
                RawBridgeConfigType::RawAxelarBridgeConfig(config) => {
                    bridge_configs.insert(
                        config.bridge_type.clone(),
                        BridgeConfigType::AxelarBridgeConfig(AxelarBridgeConfig::new(config.clone())),
                    );
                }
                RawBridgeConfigType::RawCelerBridgeConfig(config) => {
                    bridge_configs.insert(
                        config.bridge_type.clone(),
                        BridgeConfigType::CelerBridgeConfig(CelerBridgeConfig::new(config.clone())),
                    );
                }
                RawBridgeConfigType::RawPolyBridgeConfig(config) => {
                    bridge_configs.insert(
                        config.bridge_type.clone(),
                        BridgeConfigType::PolyBridgeConfig(PolyBridgeConfig::new(config.clone())),
                    );
                }
                RawBridgeConfigType::RawLayerZeroBridgeConfig(config) => {
                    bridge_configs.insert(
                        config.bridge_type.clone(),
                        BridgeConfigType::LayerZeroBridgeConfig(LayerZeroBridgeConfig::new(config.clone())),
                    );
                }
                RawBridgeConfigType::RawTBridgeConfig(config) => {
                    bridge_configs.insert(
                        config.bridge_type.clone(),
                        BridgeConfigType::TBridgeConfig(TBridgeConfig::new(config.clone())),
                    );
                }
            }
        }

        bridge_configs
    }

    fn init_chain_configs(
        base: &BaseConfig<RawMystikoConfig>,
        default_circuit_configs: HashMap<CircuitType, CircuitConfig>,
        circuit_configs_by_name: HashMap<String, CircuitConfig>,
    ) -> HashMap<u32, ChainConfig> {
        let mut chain_configs: HashMap<u32, ChainConfig> = HashMap::new();
        for chain in &base.data.chains {
            let deposit_contract_getter =
                |mystiko_config: &MystikoConfig, chain_id: u32, address: String| -> Option<DepositContractConfig> {
                    mystiko_config.get_deposit_contract_config_by_address(chain_id, address).cloned()
                };
            chain_configs.insert(
                chain.chain_id,
                ChainConfig::new(chain.clone(), Some(
                    AuxData::new(
                        default_circuit_configs.clone(),
                        circuit_configs_by_name.clone(),
                        deposit_contract_getter,
                    )
                )),
            );
        }
        chain_configs
    }

    fn init_indexer_config(base: &BaseConfig<RawMystikoConfig>) -> Option<IndexerConfig> {
        match &base.data.indexer {
            Some(config) => {
                Some(IndexerConfig::new(config.clone()))
            }
            None => {
                None
            }
        }
    }

    // TODO supplement
    fn validate(&self) {
        for chain_config in &self.chain_configs {}
    }
}
