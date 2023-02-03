use std::collections::HashMap;
use num_bigint::BigInt;
use crate::common::{AssetType, BridgeType, CircuitType};
use crate::raw::contract::pool::RawPoolContractConfig;
use crate::wrapper::asset::AssetConfig;
use crate::wrapper::circuit::CircuitConfig;
use crate::wrapper::contract::base::ContractConfig;

struct AuxData {
    default_circuit_configs: HashMap<CircuitType, CircuitConfig>,
    circuit_configs_by_name: HashMap<String, CircuitConfig>,
    main_asset_config: AssetConfig,
    asset_configs: HashMap<String, AssetConfig>,
}

pub struct PoolContractConfig {
    base: ContractConfig<RawPoolContractConfig, AuxData>,
    circuit_configs: HashMap<CircuitType, CircuitConfig>,
    main_asset_config: AssetConfig,
    asset_config: Option<AssetConfig>,
}

impl PoolContractConfig {
    pub fn new(&self, data: RawPoolContractConfig, aux_data: Option<AuxData>) -> Self {
        let config = Self {
            base: ContractConfig::new(data, aux_data),
            circuit_configs: self.init_circuits_configs(
                self.base.base.aux_data_not_empty().default_circuit_configs,
                self.base.base.aux_data_not_empty().circuit_configs_by_name,
            ),
            main_asset_config: self.base.base.aux_data_not_empty().main_asset_config,
            asset_config: self.init_asset_config(
                self.base.base.aux_data_not_empty().asset_configs
            ),
        };
        config.validate();
        config
    }

    pub fn pool_name(&self) -> &str {
        &self.base.base.data.pool_name
    }

    pub fn bridge_type(&self) -> &BridgeType {
        &self.base.base.data.bridge_type
    }

    pub fn asset(&self) -> &AssetConfig {
        match &self.asset_config {
            None => {
                &self.main_asset_config
            }
            Some(value) => {
                value
            }
        }
    }

    pub fn asset_type(&self) -> &AssetType {
        &self.asset().asset_type()
    }

    pub fn asset_symbol(&self) -> &str {
        &self.asset().asset_symbol()
    }

    pub fn asset_decimals(&self) -> &u32 {
        &self.asset().asset_decimals()
    }

    pub fn asset_address(&self) -> &Option<String> {
        &self.base.base.data.asset_address
    }

    pub fn recommended_amounts(&self) -> &Vec<BigInt> {
        &self.asset().recommended_amounts()
    }

    pub fn recommended_amounts_number(&self) -> &Vec<u32> {
        &self.asset().recommended_amounts_number()
    }

    fn init_circuits_configs(
        &self,
        default_circuit_configs: HashMap<CircuitType, CircuitConfig>,
        circuit_configs_by_name: HashMap<String, CircuitConfig>,
    ) -> HashMap<CircuitType, CircuitConfig> {
        let mut circuit_configs: HashMap<CircuitType, CircuitConfig> = HashMap::new();
        for (_, circuit_conf) in default_circuit_configs {
            circuit_configs.insert(
                circuit_conf.circuit_type().clone(),
                circuit_conf,
            );
        }
        for circuit_name in &self.base.base.data.circuits {
            let circuit_conf = circuit_configs_by_name.get(circuit_name);
            match circuit_conf {
                None => {}
                Some(value) => {
                    circuit_configs.insert(
                        value.circuit_type().clone(),
                        value.clone(),
                    );
                }
            }
        }
        circuit_configs
    }

    fn init_asset_config(&self, asset_configs: HashMap<String, AssetConfig>) -> Option<AssetConfig> {
        match &self.base.base.data.asset_address {
            None => {
                None
            }
            Some(value) => {
                let asset_config = asset_configs.get(value);
                asset_config.expect(
                    format!(
                        "asset address={:?} config has not been defined " +
                            "for pool contract address={:?}",
                        value,
                        self.base.base.data.base.address
                    ).as_str()
                )
            }
        }
    }

    fn validate(&self) {}
}