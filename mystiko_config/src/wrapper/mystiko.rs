use std::collections::HashMap;
use strum::IntoEnumIterator;
use mystiko_utils::check::check;
use crate::common::{BridgeType, CircuitType};
use crate::raw::mystiko::{RawBridgeConfigType, RawMystikoConfig};
use crate::wrapper::base::BaseConfig;
use crate::wrapper::bridge::axelar::AxelarBridgeConfig;
use crate::wrapper::bridge::celer::CelerBridgeConfig;
use crate::wrapper::chain::ChainConfig;
use crate::wrapper::circuit::CircuitConfig;
use crate::wrapper::indexer::IndexerConfig;

pub enum BridgeConfigType {
    AxelarBridgeConfig(AxelarBridgeConfig),
    CelerBridgeConfig(CelerBridgeConfig),
}

struct MystikoConfig {
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
        return MystikoConfig {
            base,
            default_circuit_configs,
            circuit_configs_by_name,
            bridge_configs: Default::default(),
            chain_configs: Default::default(),
            indexer_config: None,
        };
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
                _ => {}
            }
        }

        bridge_configs
    }
}
