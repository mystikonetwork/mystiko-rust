use std::collections::{HashMap, HashSet};
use std::error::Error;
use flamer::flame;
use num_bigint::BigInt;
use validator::Validate;
use mystiko_utils::check::check;
use crate::common::{AssetType, BridgeType, CircuitType};
use crate::errors::ValidationError;
use crate::raw::asset::RawAssetConfig;
use crate::raw::chain::{EXPLORER_TX_PLACEHOLDER, RawChainConfig};
use crate::wrapper::asset::AssetConfig;
use crate::wrapper::base::BaseConfig;
use crate::wrapper::circuit::CircuitConfig;
use crate::wrapper::contract;
use crate::wrapper::contract::deposit::DepositContractConfig;
use crate::wrapper::contract::pool::PoolContractConfig;
use crate::wrapper::provider::ProviderConfig;

const MAIN_ASSET_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AuxData {
    default_circuit_configs: HashMap<CircuitType, CircuitConfig>,
    circuit_configs_by_name: HashMap<String, CircuitConfig>,
    chain_configs: Option<HashMap<u32, ChainConfig>>,
}

impl AuxData {
    pub fn new(
        default_circuit_configs: HashMap<CircuitType, CircuitConfig>,
        circuit_configs_by_name: HashMap<String, CircuitConfig>,
        chain_configs: Option<HashMap<u32, ChainConfig>>,
    ) -> Self {
        Self {
            default_circuit_configs,
            circuit_configs_by_name,
            chain_configs,
        }
    }

    pub fn deposit_contract_getter(&self, chain_id: u32, address: String) -> Option<&DepositContractConfig> {
        match &self.chain_configs {
            None => { None }
            Some(chain_configs) => {
                let chain_config = chain_configs.get(&chain_id);
                match chain_config {
                    None => { None }
                    Some(config) => { config.get_deposit_contract_by_address(address) }
                }
            }
        }
    }
}

#[derive(Validate, Clone, Debug, PartialEq)]
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
    #[flame]
    pub fn new(data: RawChainConfig, aux_data: Option<AuxData>) -> Result<Self, ValidationError> {
        let base_config =
            BaseConfig::new(data, aux_data);
        let main_asset_config =
            ChainConfig::init_main_asset_config(&base_config)?;
        let asset_configs =
            ChainConfig::init_asset_configs(&base_config)?;
        let aux_data = &base_config.aux_data_not_empty().unwrap();
        let pool_contract_configs =
            ChainConfig::init_pool_contract_configs(
                &base_config,
                &aux_data.default_circuit_configs,
                &aux_data.circuit_configs_by_name,
                &main_asset_config,
                &asset_configs,
            )?;
        let pool_configs_by_asset_and_bridge =
            ChainConfig::init_pool_configs_by_asset_and_bridge(
                pool_contract_configs.values().cloned().collect()
            )?;
        let deposit_contract_configs =
            ChainConfig::init_deposit_contract_configs(
                &base_config,
                &pool_contract_configs,
                &main_asset_config,
                &asset_configs,
                &aux_data.chain_configs,
            )?;
        let provider_configs =
            base_config.data.providers.iter().map(
                |raw| ProviderConfig::new(raw.clone())
            ).collect();
        Ok(Self {
            base: base_config,
            pool_contract_configs,
            pool_configs_by_asset_and_bridge,
            deposit_contract_configs,
            main_asset_config,
            asset_configs,
            provider_configs,
        })
    }

    pub fn data(&self) -> &RawChainConfig {
        &self.base.data
    }

    pub fn aux_data(&self) -> &Option<AuxData> {
        &self.base.aux_data
    }

    pub fn copy_data(&self) -> RawChainConfig {
        self.base.copy_data()
    }

    pub fn to_json_string(&self) -> String {
        self.base.to_json_string()
    }

    pub fn chain_id(&self) -> &u32 {
        &self.base.data.chain_id
    }

    pub fn name(&self) -> &str {
        &self.base.data.name
    }

    pub fn asset_symbol(&self) -> String {
        self.main_asset_config.asset_symbol()
    }

    pub fn asset_decimals(&self) -> u32 {
        self.main_asset_config.asset_decimals()
    }

    pub fn recommend_amounts(&self) -> Vec<BigInt> {
        self.main_asset_config.recommended_amounts()
    }

    pub fn recommend_amounts_numbers(&self) -> Vec<f64> {
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

    pub fn event_filter_size(&self) -> &u64 {
        &self.base.data.event_filter_size
    }

    pub fn indexer_filter_size(&self) -> &u64 {
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

    pub fn get_asset_symbols(&self, peer_chain_id: u32) -> Result<Vec<String>, Box<dyn Error>> {
        let mut asset_symbols: HashSet<String> = HashSet::new();
        for config in self.deposit_contracts() {
            if peer_chain_id == *self.chain_id() {
                if config.bridge_type() == BridgeType::Loop {
                    let asset_symbol = config.asset_symbol()?;
                    asset_symbols.insert(asset_symbol);
                }
            } else if config.bridge_type() != BridgeType::Loop {
                if config.peer_chain_id().is_some() &&
                    peer_chain_id == config.peer_chain_id().unwrap() {
                    let asset_symbol = config.asset_symbol()?;
                    asset_symbols.insert(asset_symbol);
                }
            }
        }

        Ok(asset_symbols.into_iter().collect())
    }

    pub fn get_bridges(
        &self,
        peer_chain_id: u32,
        asset_symbol: &str,
    ) -> Result<Vec<BridgeType>, Box<dyn Error>> {
        let mut bridges: HashSet<BridgeType> = HashSet::new();
        for config in self.deposit_contracts() {
            if peer_chain_id != *self.chain_id() {
                let config_asset_symbol = config.asset_symbol()?;
                if config_asset_symbol.as_str() == asset_symbol {
                    if config.peer_chain_id().is_some() &&
                        peer_chain_id == config.peer_chain_id().unwrap() {
                        bridges.insert(config.bridge_type());
                    }
                }
            }
        }

        Ok(bridges.into_iter().collect())
    }

    pub fn get_deposit_contract_by_address(&self, address: String) -> Option<&DepositContractConfig> {
        self.deposit_contract_configs.get(&address)
    }

    pub fn get_pool_contract_by_address(&self, address: &str) -> Option<&PoolContractConfig> {
        self.pool_contract_configs.get(address)
    }

    pub fn get_pool_contract_bridge_type(&self, address: &str) -> Option<BridgeType> {
        match self.get_pool_contract_by_address(address) {
            None => { None }
            Some(data) => { Some(data.bridge_type().clone()) }
        }
    }

    pub fn get_event_filter_size_by_address(&self, address: &str) -> u64 {
        let deposit_contract_config =
            self.deposit_contract_configs.get(address);
        if deposit_contract_config.is_some() {
            let event_filter_size = deposit_contract_config.unwrap().event_filter_size();
            return if event_filter_size.is_some() {
                event_filter_size.unwrap()
            } else {
                *self.event_filter_size()
            };
        }
        let pool_contract =
            self.get_pool_contract_by_address(address);
        return if pool_contract.is_some() &&
            pool_contract.unwrap().event_filter_size().is_some() {
            pool_contract.unwrap().event_filter_size().unwrap()
        } else {
            *self.event_filter_size()
        };
    }

    pub fn get_indexer_filter_size_by_address(&self, address: &str) -> u64 {
        let deposit_contract_config =
            self.deposit_contract_configs.get(address);
        if deposit_contract_config.is_some() {
            let indexer_filter_size = deposit_contract_config.unwrap().indexer_filter_size();
            return if indexer_filter_size.is_some() {
                indexer_filter_size.unwrap()
            } else {
                *self.indexer_filter_size()
            };
        }
        let pool_contract = self.get_pool_contract_by_address(address);
        return if pool_contract.is_some() &&
            pool_contract.unwrap().indexer_filter_size().is_some() {
            pool_contract.unwrap().indexer_filter_size().unwrap()
        } else {
            *self.indexer_filter_size()
        };
    }

    pub fn get_asset_config_by_address(&self, asset_address: &str) -> Option<AssetConfig> {
        self.asset_configs.get(asset_address).cloned()
    }

    pub fn get_deposit_contract(
        &self,
        peer_chain_id: u32,
        asset_symbol: &str,
        bridge_type: BridgeType,
    ) -> Result<Option<DepositContractConfig>, Box<dyn Error>> {
        for deposit_contract_config in self.deposit_contracts() {
            let config_asset_symbol = deposit_contract_config.asset_symbol()?;
            if config_asset_symbol.as_str() == asset_symbol &&
                deposit_contract_config.bridge_type() == bridge_type {
                if peer_chain_id == *self.chain_id() && bridge_type == BridgeType::Loop {
                    return Ok(Some(deposit_contract_config.clone()));
                }
                if deposit_contract_config.peer_chain_id().is_some() &&
                    peer_chain_id == deposit_contract_config.peer_chain_id().unwrap() &&
                    bridge_type != BridgeType::Loop {
                    return Ok(Some(deposit_contract_config.clone()));
                }
            }
        }
        Ok(None)
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

    pub fn mutate(
        &self,
        data: Option<RawChainConfig>,
        aux_data: Option<AuxData>,
    ) -> Result<Self, ValidationError> {
        let d: RawChainConfig = match data {
            None => {
                self.data().clone()
            }
            Some(value) => {
                value
            }
        };
        let a = match aux_data {
            None => {
                self.aux_data().clone()
            }
            Some(_) => {
                aux_data
            }
        };
        ChainConfig::new(d, a)
    }

    #[flame]
    fn init_pool_contract_configs(
        base: &BaseConfig<RawChainConfig, AuxData>,
        default_circuit_configs: &HashMap<CircuitType, CircuitConfig>,
        circuit_configs_by_name: &HashMap<String, CircuitConfig>,
        main_asset_config: &AssetConfig,
        asset_configs: &HashMap<String, AssetConfig>,
    ) -> Result<HashMap<String, PoolContractConfig>, ValidationError> {
        let mut pool_contract_configs: HashMap<String, PoolContractConfig> = HashMap::new();
        for raw in &base.data.pool_contracts {
            let check_result = check(
                !pool_contract_configs.contains_key(raw.base.address.as_str()),
                format!(
                    "duplicate pool contract={:?} definition in configuration", raw.base.address
                ).as_str(),
            );
            if check_result.is_err() {
                return Err(ValidationError::new(
                    vec![
                        check_result.unwrap_err().to_string()
                    ]
                ));
            }
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
                ).unwrap(),
            );
        }

        Ok(pool_contract_configs)
    }

    #[flame]
    fn init_main_asset_config(
        base: &BaseConfig<RawChainConfig, AuxData>
    ) -> Result<AssetConfig, ValidationError> {
        AssetConfig::new(RawAssetConfig::new(
            AssetType::Main,
            base.data.asset_symbol.clone(),
            base.data.asset_decimals.clone(),
            MAIN_ASSET_ADDRESS.to_string(),
            base.data.recommended_amounts.clone(),
        ))
    }

    #[flame]
    fn init_asset_configs(
        base: &BaseConfig<RawChainConfig, AuxData>
    ) -> Result<HashMap<String, AssetConfig>, ValidationError> {
        let mut asset_configs: HashMap<String, AssetConfig> = HashMap::new();
        for raw in &base.data.assets {
            let asset_config = AssetConfig::new(raw.clone());
            if asset_config.is_err() {
                return Err(asset_config.unwrap_err());
            }
            let asset_config = asset_config.unwrap();
            asset_configs.insert(
                raw.asset_address.clone(),
                asset_config,
            );
        }
        Ok(asset_configs)
    }

    #[flame]
    fn init_pool_configs_by_asset_and_bridge(
        pool_contracts: Vec<PoolContractConfig>,
    ) -> Result<HashMap<String, HashMap<BridgeType, HashMap<u32, PoolContractConfig>>>, ValidationError> {
        let mut pool_configs_by_asset_and_bridge:
            HashMap<String, HashMap<BridgeType, HashMap<u32, PoolContractConfig>>> =
            HashMap::new();

        for pool_contract_config in pool_contracts {
            let mut bridges: HashMap<BridgeType, HashMap<u32, PoolContractConfig>> =
                match pool_configs_by_asset_and_bridge.get(&pool_contract_config.asset_symbol()) {
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
            let check_result = check(
                !all_versions.contains_key(pool_contract_config.version()),
                format!(
                    "only one pool address allowed for asset {} and bridge type {:?} and version {:?}",
                    pool_contract_config.asset_symbol(),
                    pool_contract_config.bridge_type(),
                    pool_contract_config.version()
                ).as_str(),
            );
            if check_result.is_err() {
                return Err(ValidationError::new(
                    vec![
                        check_result.unwrap_err().to_string()
                    ]
                ));
            }
            all_versions.insert(pool_contract_config.version().clone(), pool_contract_config.clone());
            bridges.insert(pool_contract_config.bridge_type().clone(), all_versions);
            pool_configs_by_asset_and_bridge.insert(
                pool_contract_config.asset_symbol().to_owned(),
                bridges,
            );
        }

        Ok(pool_configs_by_asset_and_bridge)
    }

    #[flame]
    fn init_deposit_contract_configs(
        base: &BaseConfig<RawChainConfig, AuxData>,
        pool_contract_configs: &HashMap<String, PoolContractConfig>,
        main_asset_config: &AssetConfig,
        asset_configs: &HashMap<String, AssetConfig>,
        chain_configs: &Option<HashMap<u32, ChainConfig>>,
    ) -> Result<HashMap<String, DepositContractConfig>, ValidationError> {
        let mut deposit_contract_configs: HashMap<String, DepositContractConfig> = HashMap::new();
        for raw in &base.data.deposit_contracts {
            let check_result = check(
                !deposit_contract_configs.contains_key(raw.base.address.as_str()),
                format!(
                    "duplicate deposit contract={} definition in configuration", raw.base.address
                ).as_str(),
            );
            if check_result.is_err() {
                return Err(ValidationError::new(
                    vec![
                        check_result.unwrap_err().to_string()
                    ]
                ));
            }
            let pool_contract_config = pool_contract_configs.get(&raw.pool_address);
            if pool_contract_config.is_none() {
                return Err(ValidationError::new(
                    vec![
                        format!(
                            "deposit contract={} poolAddress definition does not exist",
                            raw.base.address
                        )
                    ]
                ));
            }
            let pool_contract_config = pool_contract_configs.get(&raw.pool_address).unwrap();
            let check_result = check(
                raw.bridge_type == pool_contract_config.bridge_type().clone(),
                format!(
                    "deposit contract={} bridgeType={:?} does not equal to pool contract bridgeType={:?}",
                    raw.base.address, raw.bridge_type, pool_contract_config.bridge_type()
                ).as_str(),
            );
            if check_result.is_err() {
                return Err(ValidationError::new(
                    vec![
                        check_result.unwrap_err().to_string()
                    ]
                ));
            }
            if raw.bridge_type != BridgeType::Loop {
                if raw.peer_chain_id.is_some() {
                    let check_result = check(
                        base.data.chain_id != raw.peer_chain_id.unwrap(),
                        format!(
                            "current chain id should be different with peer chain id in contract={:?}",
                            raw.base.address
                        ).as_str(),
                    );
                    if check_result.is_err() {
                        return Err(ValidationError::new(
                            vec![
                                check_result.unwrap_err().to_string()
                            ]
                        ));
                    }
                }
            }
            let deposit_contract_config =
                DepositContractConfig::new(raw.clone(), Some(
                    contract::deposit::AuxData::new(
                        pool_contract_configs.clone(),
                        main_asset_config.clone(),
                        asset_configs.clone(),
                        chain_configs.clone(),
                    )
                ))?;
            deposit_contract_configs.insert(
                raw.base.address.clone(),
                deposit_contract_config,
            );
        }

        Ok(deposit_contract_configs)
    }
}
