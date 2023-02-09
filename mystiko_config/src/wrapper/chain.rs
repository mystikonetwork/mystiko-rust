use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::hash::Hash;
use std::io::Chain;
use num_bigint::BigInt;
use validator::Validate;
use mystiko_utils::check::check;
use crate::common::{AssetType, BridgeType, CircuitType, validate_object};
use crate::raw::asset::RawAssetConfig;
use crate::raw::base::RawConfigTrait;
use crate::raw::chain::{EXPLORER_TX_PLACEHOLDER, RawChainConfig};
use crate::raw::contract::base::RawContractConfigTrait;
use crate::wrapper::asset::AssetConfig;
use crate::wrapper::base::BaseConfig;
use crate::wrapper::circuit::CircuitConfig;
use crate::wrapper::contract;
use crate::wrapper::contract::deposit::DepositContractConfig;
use crate::wrapper::contract::pool::PoolContractConfig;
use crate::wrapper::mystiko::MystikoConfig;
use crate::wrapper::provider::ProviderConfig;

const MAIN_ASSET_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

#[derive(Clone, Debug)]
pub struct AuxData {
    default_circuit_configs: HashMap<CircuitType, CircuitConfig>,
    circuit_configs_by_name: HashMap<String, CircuitConfig>,
    deposit_contract_configs: HashMap<u32, Vec<DepositContractConfig>>,
}

impl AuxData {
    pub fn new(
        default_circuit_configs: HashMap<CircuitType, CircuitConfig>,
        circuit_configs_by_name: HashMap<String, CircuitConfig>,
        deposit_contract_configs: HashMap<u32, Vec<DepositContractConfig>>,
    ) -> Self {
        Self { default_circuit_configs, circuit_configs_by_name, deposit_contract_configs }
    }

    pub fn deposit_contract_getter(&self, chain_id: u32, address: String) -> Option<&DepositContractConfig> {
        let deposit_configs = self.deposit_contract_configs.get(&chain_id);
        if deposit_configs.is_some() {
            for config in deposit_configs.unwrap() {
                if config.base.base.data.address() == address {
                    return Some(config);
                }
            }
        }

        None
    }
}

#[derive(Validate, Clone, Debug)]
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
                base_config.aux_data_not_empty().deposit_contract_configs,
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

    pub fn chain_id(&self) -> &u32 {
        &self.base.data.chain_id
    }

    pub fn name(&self) -> &str {
        &self.base.data.name
    }

    pub fn asset_symbol(&self) -> &str {
        &self.main_asset_config.asset_symbol()
    }

    pub fn asset_decimals(&self) -> &u32 {
        &self.main_asset_config.asset_decimals()
    }

    pub fn recommend_amounts(&self) -> Vec<BigInt> {
        self.main_asset_config.recommended_amounts()
    }

    pub fn recommend_amounts_numbers(&self) -> Vec<u32> {
        self.main_asset_config.recommended_amounts_number()
    }

    pub fn explorer_url(&self) -> &str {
        &self.base.data.explorer_url
    }

    pub fn explorer_url_prefix(&self) -> &str {
        &self.base.data.explorer_prefix
    }

    pub fn providers(&self) -> &Vec<ProviderConfig> {
        &self.provider_configs
    }

    pub fn signer_endpoint(&self) -> &str {
        &self.base.data.signer_endpoint
    }

    pub fn event_filter_size(&self) -> &u32 {
        &self.base.data.event_filter_size
    }

    pub fn indexer_filter_size(&self) -> &u32 {
        &self.base.data.indexer_filter_size
    }

    pub fn pool_contracts(&self) -> Vec<PoolContractConfig> {
        self.pool_contract_configs.values().cloned().collect()
    }

    pub fn deposit_contracts(&self) -> Vec<DepositContractConfig> {
        let configs = self.deposit_contracts_with_disabled();
        configs
            .iter()
            .filter(|conf| !conf.disabled())
            .cloned()
            .collect::<Vec<DepositContractConfig>>()
    }

    pub fn deposit_contracts_with_disabled(&self) -> Vec<DepositContractConfig> {
        self.deposit_contract_configs.values().cloned().collect()
    }

    pub fn assets(&self) -> Vec<AssetConfig> {
        self.asset_configs.values().cloned().collect()
    }

    pub fn peer_chain_ids(&self) -> Vec<u32> {
        let mut chain_ids: HashSet<u32> = HashSet::new();
        for config in self.deposit_contracts() {
            if config.bridge_type() == BridgeType::Loop {
                chain_ids.insert(*self.chain_id());
            } else if config.peer_chain_id().is_some() {
                chain_ids.insert(config.peer_chain_id().unwrap());
            }
        }

        chain_ids.into_iter().collect()
    }

    pub fn get_asset_symbols(&self, peer_chain_id: u32) -> Vec<String> {
        let mut asset_symbols: HashSet<String> = HashSet::new();
        for config in self.deposit_contracts() {
            if peer_chain_id == *self.chain_id() {
                if config.bridge_type() == BridgeType::Loop {
                    asset_symbols.insert(config.asset_symbol());
                }
            } else if config.bridge_type() != BridgeType::Loop {
                if config.peer_chain_id().is_some() &&
                    peer_chain_id == config.peer_chain_id().unwrap() {
                    asset_symbols.insert(config.asset_symbol());
                }
            }
        }

        asset_symbols.into_iter().collect()
    }

    pub fn get_bridges(&self, peer_chain_id: u32, asset_symbol: &str) -> Vec<BridgeType> {
        let mut bridges: HashSet<BridgeType> = HashSet::new();
        for config in self.deposit_contracts() {
            if peer_chain_id != *self.chain_id() && config.asset_symbol().as_str() == asset_symbol {
                if config.peer_chain_id().is_some() &&
                    peer_chain_id == config.peer_chain_id().unwrap() {
                    bridges.insert(config.bridge_type());
                }
            }
        }

        bridges.into_iter().collect()
    }


    pub fn get_deposit_contract_by_address(&self, address: String) -> Option<&DepositContractConfig> {
        self.deposit_contract_configs.get(&address)
    }


    pub fn get_pool_contract_by_address(&self, address: &str) -> Option<&PoolContractConfig> {
        self.pool_contract_configs.get(address)
    }

    pub fn get_deposit_contract(
        &self,
        peer_chain_id: u32,
        asset_symbol: &str,
        bridge_type: BridgeType,
    ) -> Option<DepositContractConfig> {
        for deposit_contract_config in self.deposit_contracts() {
            if deposit_contract_config.asset_symbol().as_str() == asset_symbol &&
                deposit_contract_config.bridge_type() == bridge_type {
                if peer_chain_id == *self.chain_id() && bridge_type == BridgeType::Loop {
                    return Some(deposit_contract_config.clone());
                }
                if deposit_contract_config.peer_chain_id().is_some() &&
                    peer_chain_id == deposit_contract_config.peer_chain_id().unwrap() &&
                    bridge_type != BridgeType::Loop {
                    return Some(deposit_contract_config.clone());
                }
            }
        }

        None
    }

    pub fn get_pool_contract(
        &self,
        asset_symbol: &str,
        bridge_type: BridgeType,
        version: u32,
    ) -> Option<&PoolContractConfig> {
        let pool_config = &self.pool_configs_by_asset_and_bridge.get(asset_symbol);
        if pool_config.is_some() {
            let pool_contract_config = pool_config.unwrap().get(&bridge_type);
            if pool_contract_config.is_some() {
                return pool_contract_config.unwrap().get(&version);
            }
        }

        None
    }

    pub fn get_pool_contracts(
        &self,
        asset_symbol: &str,
        bridge_type: BridgeType,
    ) -> Vec<PoolContractConfig> {
        let pool_config = self.pool_configs_by_asset_and_bridge.get(asset_symbol);
        if pool_config.is_some() {
            let config = pool_config.unwrap().get(&bridge_type);
            if config.is_some() {
                return config.unwrap().values().cloned().collect();
            }
        }

        vec![]
    }

    pub fn get_transaction_url(&self, transaction_hash: &str) -> String {
        let mut result = String::from(self.explorer_url());
        result.push_str(self.explorer_url_prefix());
        result.replace(EXPLORER_TX_PLACEHOLDER, transaction_hash)
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
                pool_contract_config.asset_symbol().to_owned(),
                bridges,
            );
        }

        pool_configs_by_asset_and_bridge
    }

    fn init_deposit_contract_configs(
        base: &BaseConfig<RawChainConfig, AuxData>,
        pool_contract_configs: &HashMap<String, PoolContractConfig>,
        aux_deposit_contract_configs: HashMap<u32, Vec<DepositContractConfig>>,
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
                if raw.peer_chain_id.is_some() {
                    check(
                        base.data.chain_id != raw.peer_chain_id.unwrap(),
                        format!(
                            "current chain id should be different with peer chain id in contract={:?}",
                            raw.base.address
                        ).as_str(),
                    );
                }
            }
            deposit_contract_configs.insert(
                raw.base.address.clone(),
                DepositContractConfig::new(raw.clone(), Some(
                    contract::deposit::AuxData::new(
                        aux_deposit_contract_configs.clone(),
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

impl RawConfigTrait for ChainConfig {
    fn validate(&self) -> Result<(), Vec<String>> {
        let result = validate_object(self);
        if result.is_err() {
            return Err(result.unwrap_err());
        }
        Ok(())
    }
}