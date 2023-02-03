use std::collections::HashMap;
use crate::common::{BridgeType, CircuitType};
use crate::raw::base::RawConfig;
use crate::raw::chain::RawChainConfig;
use crate::wrapper::asset::AssetConfig;
use crate::wrapper::base::BaseConfig;
use crate::wrapper::circuit::CircuitConfig;
use crate::wrapper::contract::deposit::DepositContractConfig;
use crate::wrapper::contract::pool::PoolContractConfig;
use crate::wrapper::mystiko::MystikoConfig;
use crate::wrapper::provider::ProviderConfig;

pub struct AuxData {
    default_circuit_configs: HashMap<CircuitType, CircuitConfig>,
    circuit_configs_by_name: HashMap<String, CircuitConfig>,
    deposit_contract_getter: fn(&MystikoConfig, chain_id: u32, address: String) -> Option<DepositContractConfig>,
}

impl AuxData {
    pub fn new(
        default_circuit_configs: HashMap<CircuitType, CircuitConfig>,
        circuit_configs_by_name: HashMap<String, CircuitConfig>,
        deposit_contract_getter: fn(&MystikoConfig, u32, String) -> Option<DepositContractConfig>,
    ) -> Self {
        Self { default_circuit_configs, circuit_configs_by_name, deposit_contract_getter }
    }
}

pub struct ChainConfig {
    base: BaseConfig<RawChainConfig, AuxData>,
    pool_contract_configs: HashMap<String, PoolContractConfig>,
    pool_configs_by_asset_and_bridge: HashMap<String, HashMap<BridgeType, HashMap<u32, PoolContractConfig>>>,
    deposit_contract_configs: HashMap<String, DepositContractConfig>,
    main_asset_config: AssetConfig,
    asset_configs: HashMap<String, AssetConfig>,
    provider_configs: Vec<ProviderConfig>,
}

impl ChainConfig {
    // TODO Supplementary constructor
    pub fn new(data: RawChainConfig, aux_data: Option<AuxData>) -> Self {
        Self {
            base: BaseConfig::new(data, aux_data),
            pool_contract_configs: Default::default(),
            pool_configs_by_asset_and_bridge: Default::default(),
            deposit_contract_configs: Default::default(),
            main_asset_config: AssetConfig {},
            asset_configs: Default::default(),
            provider_configs: vec![],
        }
    }

    pub fn get_deposit_contract_by_address(&self, address: String) -> Option<&DepositContractConfig> {
        self.deposit_contract_configs.get(&address)
    }
}

impl RawConfig for ChainConfig {}