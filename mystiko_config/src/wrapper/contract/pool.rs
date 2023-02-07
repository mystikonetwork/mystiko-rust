use std::collections::HashMap;
use std::str::FromStr;
use num_bigint::BigInt;
use crate::common::{AssetType, BridgeType, CircuitType};
use crate::common::ContractType::Pool;
use crate::raw::contract::base::RawContractConfigTrait;
use crate::raw::contract::pool::RawPoolContractConfig;
use crate::wrapper::asset::AssetConfig;
use crate::wrapper::circuit::CircuitConfig;
use crate::wrapper::contract::base::ContractConfig;

#[derive(Clone)]
pub struct AuxData {
    default_circuit_configs: HashMap<CircuitType, CircuitConfig>,
    circuit_configs_by_name: HashMap<String, CircuitConfig>,
    main_asset_config: AssetConfig,
    asset_configs: HashMap<String, AssetConfig>,
}

impl AuxData {
    pub fn new(
        default_circuit_configs: HashMap<CircuitType, CircuitConfig>,
        circuit_configs_by_name: HashMap<String, CircuitConfig>,
        main_asset_config: AssetConfig,
        asset_configs: HashMap<String, AssetConfig>,
    ) -> Self {
        Self { default_circuit_configs, circuit_configs_by_name, main_asset_config, asset_configs }
    }
}

#[derive(Clone)]
pub struct PoolContractConfig {
    pub base: ContractConfig<RawPoolContractConfig, AuxData>,
    pub circuit_configs: HashMap<CircuitType, CircuitConfig>,
    pub main_asset_config: AssetConfig,
    pub asset_config: Option<AssetConfig>,
}

impl PoolContractConfig {
    pub fn new(data: RawPoolContractConfig, aux_data: Option<AuxData>) -> Self {
        let contract_config = ContractConfig::new(data, aux_data);
        let config = Self {
            base: contract_config.clone(),
            circuit_configs: PoolContractConfig::init_circuits_configs(
                &contract_config,
                contract_config.base.aux_data_not_empty().default_circuit_configs,
                contract_config.base.aux_data_not_empty().circuit_configs_by_name,
            ),
            main_asset_config: contract_config.base.aux_data_not_empty().main_asset_config,
            asset_config: PoolContractConfig::init_asset_config(
                &contract_config,
                contract_config.base.aux_data_not_empty().asset_configs,
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
        self.asset().asset_symbol()
    }

    pub fn asset_decimals(&self) -> &u32 {
        &self.asset().asset_decimals()
    }

    pub fn asset_address(&self) -> &Option<String> {
        &self.base.base.data.asset_address
    }

    pub fn recommended_amounts(&self) -> Vec<BigInt> {
        self.asset().recommended_amounts()
    }

    pub fn recommended_amounts_number(&self) -> Vec<u32> {
        self.asset().recommended_amounts_number()
    }

    pub fn min_rollup_fee(&self) -> BigInt {
        BigInt::from_str(
            &self.base.base.data.min_rollup_fee,
        ).unwrap()
    }

    // TODO Complete this method
    pub fn min_rollup_fee_number(&self) -> u32 {
        return 1;
    }

    pub fn circuits(&self) -> Vec<CircuitConfig> {
        self.circuit_configs.values().cloned().collect()
    }

    pub fn circuit_config(&self, t: CircuitType) -> Option<&CircuitConfig> {
        self.circuit_configs.get(&t)
    }

    pub fn mutate(&self, data: Option<RawPoolContractConfig>, aux_data: Option<AuxData>) -> Self {
        match data {
            None => {
                PoolContractConfig::new(self.base.base.data.clone(), aux_data)
            }
            Some(value) => {
                PoolContractConfig::new(value, aux_data)
            }
        }
    }

    fn init_circuits_configs(
        base: &ContractConfig<RawPoolContractConfig, AuxData>,
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
        for circuit_name in &base.base.data.circuits {
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

    fn init_asset_config(
        base: &ContractConfig<RawPoolContractConfig, AuxData>,
        asset_configs: HashMap<String, AssetConfig>,
    ) -> Option<AssetConfig> {
        match &base.base.data.asset_address {
            None => {
                None
            }
            Some(value) => {
                let asset_config = asset_configs.get(value);
                Some(
                    asset_config.expect(
                        format!(
                            "asset address={:?} config has not been defined for pool contract address={:?}",
                            value,
                            base.base.data.address()
                        ).as_str()
                    ).clone()
                )
            }
        }
    }

    fn validate(&self) {}
}