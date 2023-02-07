use std::collections::HashMap;
use std::fmt::format;
use std::hash::Hash;
use std::io::Chain;
use mystiko_utils::check::check;
use crate::common::{AssetType, BridgeType, CircuitType};
use crate::raw::asset::RawAssetConfig;
use crate::raw::base::RawConfig;
use crate::raw::chain::RawChainConfig;
use crate::wrapper::asset::AssetConfig;
use crate::wrapper::base::BaseConfig;
use crate::wrapper::circuit::CircuitConfig;
use crate::wrapper::contract;
use crate::wrapper::contract::deposit::DepositContractConfig;
use crate::wrapper::contract::pool::PoolContractConfig;
use crate::wrapper::mystiko::MystikoConfig;
use crate::wrapper::provider::ProviderConfig;

const MAIN_ASSET_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

#[derive(Clone)]
pub struct AuxData {
    default_circuit_configs: HashMap<CircuitType, CircuitConfig>,
    circuit_configs_by_name: HashMap<String, CircuitConfig>,
    // TODO Optimize getter method, save data and impl this method by AuxData
    deposit_contract_getter: fn(&MystikoConfig, u32, String) -> Option<DepositContractConfig>,
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
    pub fn new(data: RawChainConfig, aux_data: Option<AuxData>) -> Self {
        let base_config =
            BaseConfig::new(data, aux_data);
        let main_asset_config =
            ChainConfig::init_main_asset_config(&base_config);
        let asset_configs =
            ChainConfig::init_asset_configs(&base_config);
        let pool_contract_configs =
            ChainConfig::init_pool_contract_configs(
                &base_config,
                &base_config.aux_data_not_empty().default_circuit_configs,
                &base_config.aux_data_not_empty().circuit_configs_by_name,
                &main_asset_config,
                &asset_configs,
            );
        let pool_configs_by_asset_and_bridge =
            ChainConfig::init_pool_configs_by_asset_and_bridge(
                pool_contract_configs.values().cloned().collect()
            );
        let deposit_contract_configs =
            ChainConfig::init_deposit_contract_configs(
                &base_config,
                &pool_contract_configs,
                base_config.aux_data_not_empty().deposit_contract_getter,
                &main_asset_config,
                &asset_configs,
            );
        let provider_configs =
            base_config.data.providers.iter().map(
                |raw| ProviderConfig::new(raw.clone())
            ).collect();
        Self {
            base: base_config,
            pool_contract_configs,
            pool_configs_by_asset_and_bridge,
            deposit_contract_configs,
            main_asset_config,
            asset_configs,
            provider_configs,
        }
    }

    pub fn get_deposit_contract_by_address(&self, address: String) -> Option<&DepositContractConfig> {
        self.deposit_contract_configs.get(&address)
    }

    pub fn pool_contracts(&self) -> Vec<PoolContractConfig> {
        self.pool_contract_configs.values().cloned().collect()
    }

    pub fn get_pool_contract_by_address(&self, address: &str) -> Option<&PoolContractConfig> {
        self.pool_contract_configs.get(address)
    }

    pub fn mutate(&self, data: Option<RawChainConfig>, aux_data: Option<AuxData>) -> Self {
        let d: RawChainConfig = match data {
            None => {
                self.base.data.clone()
            }
            Some(value) => {
                value
            }
        };
        let a = match aux_data {
            None => {
                self.base.aux_data.clone()
            }
            Some(_) => {
                aux_data
            }
        };
        ChainConfig::new(d, a)
    }

    fn init_pool_contract_configs(
        base: &BaseConfig<RawChainConfig, AuxData>,
        default_circuit_configs: &HashMap<CircuitType, CircuitConfig>,
        circuit_configs_by_name: &HashMap<String, CircuitConfig>,
        main_asset_config: &AssetConfig,
        asset_configs: &HashMap<String, AssetConfig>,
    ) -> HashMap<String, PoolContractConfig> {
        let mut pool_contract_configs: HashMap<String, PoolContractConfig> = HashMap::new();
        for raw in &base.data.pool_contracts {
            check(
                !pool_contract_configs.contains_key(raw.base.address.as_str()),
                format!(
                    "duplicate pool contract={:?} definition in configuration", raw.base.address
                ).as_str(),
            );
            pool_contract_configs.insert(
                raw.base.address.clone(),
                PoolContractConfig::new(
                    raw.clone(),
                    Some(
                        contract::pool::AuxData::new(
                            default_circuit_configs.clone(),
                            circuit_configs_by_name.clone(),
                            main_asset_config.clone(),
                            asset_configs.clone(),
                        )
                    ),
                ),
            );
        }

        pool_contract_configs
    }

    fn init_main_asset_config(base: &BaseConfig<RawChainConfig, AuxData>) -> AssetConfig {
        AssetConfig::new(RawAssetConfig::new(
            AssetType::Main,
            base.data.asset_symbol.clone(),
            base.data.asset_decimals.clone(),
            MAIN_ASSET_ADDRESS.to_string(),
            base.data.recommended_amounts.clone(),
        ))
    }

    fn init_asset_configs(base: &BaseConfig<RawChainConfig, AuxData>) -> HashMap<String, AssetConfig> {
        let mut asset_configs: HashMap<String, AssetConfig> = HashMap::new();
        for asset_config in &base.data.assets {
            asset_configs.insert(
                asset_config.asset_address.clone(),
                AssetConfig::new(asset_config.clone()),
            );
        }
        asset_configs
    }

    fn init_pool_configs_by_asset_and_bridge(
        pool_contracts: Vec<PoolContractConfig>,
    ) -> HashMap<String, HashMap<BridgeType, HashMap<u32, PoolContractConfig>>> {
        let mut pool_configs_by_asset_and_bridge:
            HashMap<String, HashMap<BridgeType, HashMap<u32, PoolContractConfig>>> =
            HashMap::new();

        for pool_contract_config in pool_contracts {
            let mut bridges: HashMap<BridgeType, HashMap<u32, PoolContractConfig>> =
                match pool_configs_by_asset_and_bridge.get(pool_contract_config.asset_symbol()) {
                    None => {
                        HashMap::new()
                    }
                    Some(value) => {
                        value.clone()
                    }
                };
            let mut all_versions: HashMap<u32, PoolContractConfig> =
                match bridges.get(pool_contract_config.bridge_type()) {
                    None => {
                        HashMap::new()
                    }
                    Some(value) => {
                        value.clone()
                    }
                };
            check(
                !all_versions.contains_key(pool_contract_config.base.version()),
                format!(
                    "only one pool address allowed for asset {:?} and bridge type {:?} and version {:?}",
                    pool_contract_config.asset_symbol(),
                    pool_contract_config.bridge_type(),
                    pool_contract_config.base.version()
                ).as_str(),
            );
            all_versions.insert(pool_contract_config.base.version().clone(), pool_contract_config.clone());
            bridges.insert(pool_contract_config.bridge_type().clone(), all_versions);
            pool_configs_by_asset_and_bridge.insert(
                pool_contract_config.asset_symbol().to_string(),
                bridges,
            );
        }

        pool_configs_by_asset_and_bridge
    }

    fn init_deposit_contract_configs(
        base: &BaseConfig<RawChainConfig, AuxData>,
        pool_contract_configs: &HashMap<String, PoolContractConfig>,
        deposit_contract_getter: fn(&MystikoConfig, chain_id: u32, address: String) -> Option<DepositContractConfig>,
        main_asset_config: &AssetConfig,
        asset_configs: &HashMap<String, AssetConfig>,
    ) -> HashMap<String, DepositContractConfig> {
        let mut deposit_contract_configs: HashMap<String, DepositContractConfig> = HashMap::new();
        for raw in &base.data.deposit_contracts {
            check(
                !deposit_contract_configs.contains_key(raw.base.address.as_str()),
                format!(
                    "duplicate deposit contract={:?} definition in configuration", raw.base.address
                ).as_str(),
            );
            let pool_contract_config = pool_contract_configs.get(&raw.pool_address).expect(
                format!("deposit contract={:?} poolAddress definition does not exist", raw.base.address).as_str()
            );
            check(
                raw.bridge_type == pool_contract_config.bridge_type().clone(),
                format!(
                    "deposit contract={:?} bridgeType={:?} does not equal to pool contract bridgeType={:?}",
                    raw.base.address, raw.bridge_type, pool_contract_config.bridge_type()
                ).as_str(),
            );
            if raw.bridge_type != BridgeType::Loop {
                check(
                    base.data.chain_id != raw.peer_chain_id,
                    format!(
                        "current chain id should be different with peer chain id in contract={:?}",
                        raw.base.address
                    ).as_str(),
                );
            }
            deposit_contract_configs.insert(
                raw.base.address.clone(),
                DepositContractConfig::new(raw.clone(), Some(
                    contract::deposit::AuxData::new(
                        deposit_contract_getter,
                        pool_contract_configs.clone(),
                        main_asset_config.clone(),
                        asset_configs.clone(),
                    )
                )),
            );
        }

        deposit_contract_configs
    }
}

impl RawConfig for ChainConfig {}