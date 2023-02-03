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
use crate::wrapper::indexer::IndexerConfig;

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
    pub fn new(&self, data: RawMystikoConfig) -> Self {
        let base = BaseConfig::new(data, None);
        let (
            default_circuit_configs,
            circuit_configs_by_name
        ) = self.init_circuit_configs();
        let config = Self {
            base,
            default_circuit_configs: default_circuit_configs.clone(),
            circuit_configs_by_name: circuit_configs_by_name.clone(),
            bridge_configs: self.init_bridge_configs(),
            chain_configs: self.init_chain_configs(
                default_circuit_configs,
                circuit_configs_by_name,
            ),
            indexer_config: self.init_indexer_config(),
        };
        config.validate();
        config
    }

    fn get_chain_config(&self, chain_id: u32) -> Option<&ChainConfig> {
        self.chain_configs.get(&chain_id)
    }

    fn get_deposit_contract_config_by_address(&self, chain_id: u32, address: String) -> Option<&DepositContractConfig> {
        let chain_config = self.get_chain_config(chain_id).clone();
        match chain_config {
            Some(config) => {
                config.get_deposit_contract_by_address(address)
            }
            None => {
                None
            }
        }
    }

    fn init_circuit_configs(&self) -> (HashMap<CircuitType, CircuitConfig>, HashMap<String, CircuitConfig>) {
        let mut default_circuit_configs: HashMap<CircuitType, CircuitConfig> = HashMap::new();
        let mut circuit_config_by_names: HashMap<String, CircuitConfig> = HashMap::new();

        let circuits = &self.base.data.circuits;
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
        for chain in &self.base.data.chains {
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

    fn init_bridge_configs(&self) -> HashMap<BridgeType, BridgeConfigType> {
        let mut bridge_configs: HashMap<BridgeType, BridgeConfigType> = HashMap::new();
        for bridge in &self.base.data.bridges {
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
        &self,
        default_circuit_configs: HashMap<CircuitType, CircuitConfig>,
        circuit_configs_by_name: HashMap<String, CircuitConfig>,
    ) -> HashMap<u32, ChainConfig> {
        let mut chain_configs: HashMap<u32, ChainConfig> = HashMap::new();
        for chain in &self.base.data.chains {
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

    fn init_indexer_config(&self) -> Option<IndexerConfig> {
        match &self.base.data.indexer {
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
        for chain_config in self.chain_configs {}
    }
}
