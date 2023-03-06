use std::collections::{HashMap, HashSet};
use num_bigint::BigInt;
use validator::Validate;
use mystiko_utils::check::check;
use crate::common::{AssetType, BridgeType, CircuitType};
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
    pub(crate) base: BaseConfig<RawChainConfig, AuxData>,
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
                &main_asset_config,
                &asset_configs,
                base_config.aux_data_not_empty().chain_configs,
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
            let event_filter_size = deposit_contract_config.unwrap().base.event_filter_size();
            return if event_filter_size.is_some() {
                event_filter_size.unwrap()
            } else {
                *self.event_filter_size()
            };
        }
        let pool_contract =
            self.get_pool_contract_by_address(address);
        return if pool_contract.is_some() &&
            pool_contract.unwrap().base.event_filter_size().is_some() {
            pool_contract.unwrap().base.event_filter_size().unwrap()
        } else {
            *self.event_filter_size()
        };
    }

    pub fn get_indexer_filter_size_by_address(&self, address: &str) -> u64 {
        let deposit_contract_config =
            self.deposit_contract_configs.get(address);
        if deposit_contract_config.is_some() {
            let indexer_filter_size = deposit_contract_config.unwrap().base.indexer_filter_size();
            return if indexer_filter_size.is_some() {
                indexer_filter_size.unwrap()
            } else {
                *self.indexer_filter_size()
            };
        }
        let pool_contract = self.get_pool_contract_by_address(address);
        return if pool_contract.is_some() &&
            pool_contract.unwrap().base.indexer_filter_size().is_some() {
            pool_contract.unwrap().base.indexer_filter_size().unwrap()
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
            check(
                !all_versions.contains_key(pool_contract_config.base.version()),
                format!(
                    "only one pool address allowed for asset {} and bridge type {:?} and version {:?}",
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
        main_asset_config: &AssetConfig,
        asset_configs: &HashMap<String, AssetConfig>,
        chain_configs: Option<HashMap<u32, ChainConfig>>,
    ) -> HashMap<String, DepositContractConfig> {
        let mut deposit_contract_configs: HashMap<String, DepositContractConfig> = HashMap::new();
        for raw in &base.data.deposit_contracts {
            check(
                !deposit_contract_configs.contains_key(raw.base.address.as_str()),
                format!(
                    "duplicate deposit contract={} definition in configuration", raw.base.address
                ).as_str(),
            );
            let pool_contract_config = pool_contract_configs.get(&raw.pool_address).expect(
                format!("deposit contract={} poolAddress definition does not exist", raw.base.address).as_str()
            );
            check(
                raw.bridge_type == pool_contract_config.bridge_type().clone(),
                format!(
                    "deposit contract={} bridgeType={:?} does not equal to pool contract bridgeType={:?}",
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
                        pool_contract_configs.clone(),
                        main_asset_config.clone(),
                        asset_configs.clone(),
                        chain_configs.clone(),
                    )
                )),
            );
        }

        deposit_contract_configs
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::str::FromStr;
    use num_bigint::BigInt;
    use crate::common::{BridgeType, CircuitType, ContractType};
    use crate::raw::base::RawConfig;
    use crate::raw::chain::RawChainConfig;
    use crate::raw::contract::base::{RawContractConfig, RawContractConfigTrait};
    use crate::raw::contract::deposit::RawDepositContractConfig;
    use crate::raw::contract::pool::RawPoolContractConfig;
    use crate::raw::mystiko::RawMystikoConfig;
    use crate::wrapper::chain::{AuxData, ChainConfig};
    use crate::wrapper::circuit::CircuitConfig;
    use crate::wrapper::provider::ProviderConfig;

    async fn raw_mystiko_config() -> RawMystikoConfig {
        RawConfig::create_from_file::<RawMystikoConfig>("src/tests/files/mystiko.valid.json").await
    }

    async fn default_circuit_configs() -> HashMap<CircuitType, CircuitConfig> {
        let raw = raw_mystiko_config().await;
        let mut configs = HashMap::new();
        for circuit in raw.circuits {
            let circuit_config = CircuitConfig::new(circuit.clone());
            if circuit.is_default {
                configs.insert(circuit.circuit_type, circuit_config);
            }
        }
        configs
    }

    async fn circuit_configs_by_name() -> HashMap<String, CircuitConfig> {
        let raw = raw_mystiko_config().await;
        let mut configs = HashMap::new();
        for circuit in raw.circuits {
            let circuit_config = CircuitConfig::new(circuit.clone());
            configs.insert(circuit.name, circuit_config);
        }
        configs
    }

    async fn raw_config() -> RawChainConfig {
        RawConfig::create_from_file::<RawChainConfig>("src/tests/files/chain.valid.json").await
    }

    async fn config() -> ChainConfig {
        ChainConfig::new(
            raw_config().await,
            Some(
                AuxData::new(
                    default_circuit_configs().await,
                    circuit_configs_by_name().await,
                    None,
                )
            ),
        )
    }

    #[tokio::test]
    async fn test_equality() {
        let config = config().await;
        let raw_config = raw_config().await;
        assert_eq!(config.chain_id(), &raw_config.chain_id);
        assert_eq!(config.name(), raw_config.name);
        assert_eq!(config.asset_symbol(), raw_config.asset_symbol);
        assert_eq!(config.asset_decimals(), raw_config.asset_decimals);
        assert_eq!(config.recommend_amounts(), vec![
            BigInt::from_str("1000000000000000000").unwrap(),
            BigInt::from_str("10000000000000000000").unwrap(),
        ]);
        assert_eq!(config.recommend_amounts_numbers(), vec![1f64, 10f64]);
        assert_eq!(config.explorer_url(), raw_config.explorer_url);
        assert_eq!(config.explorer_url_prefix(), raw_config.explorer_prefix);
        assert_eq!(config.signer_endpoint(), raw_config.signer_endpoint);
        assert_eq!(config.event_filter_size(), &raw_config.event_filter_size);
        assert_eq!(config.indexer_filter_size(), &raw_config.indexer_filter_size);
        assert_eq!(
            config.providers(),
            &raw_config.providers.iter().map(|raw| ProviderConfig::new(raw.clone()))
                .collect::<Vec<ProviderConfig>>()
        );
        assert_eq!(config.pool_contracts().len(), raw_config.pool_contracts.len());
        let mut a = config.pool_contracts().iter().map(|conf| conf.base.base.data.address().to_string())
            .collect::<Vec<String>>();
        let mut b = raw_config.pool_contracts.iter().map(|conf| conf.base.address.clone())
            .collect::<Vec<String>>();
        a.sort();
        b.sort();
        assert_eq!(a, b);
        assert_eq!(config.deposit_contracts().len(), 0);
        assert_eq!(config.deposit_contracts_with_disabled().len(), raw_config.deposit_contracts.len());
        let mut a = config.deposit_contracts_with_disabled().iter().map(|conf| conf.base.address().to_string())
            .collect::<Vec<String>>();
        let mut b = raw_config.deposit_contracts.iter().map(|conf| conf.base.address.clone())
            .collect::<Vec<String>>();
        a.sort();
        b.sort();
        assert_eq!(a, b);
        assert_eq!(config.assets().len(), 1);
    }

    #[tokio::test]
    async fn test_peer_chain_ids() {
        let mut config = config().await;
        assert_eq!(config.peer_chain_ids().len(), 0);
        let mut raw_config = raw_config().await;
        let mut deposit_contracts = raw_config.deposit_contracts;
        deposit_contracts[0].disabled = false;
        raw_config.deposit_contracts = deposit_contracts;

        let default_circuit_configs = default_circuit_configs().await;
        let circuit_configs_by_name = circuit_configs_by_name().await;
        config = ChainConfig::new(raw_config.clone(), Some(
            AuxData::new(
                default_circuit_configs.clone(),
                circuit_configs_by_name.clone(),
                None,
            )
        ));
        assert_eq!(config.peer_chain_ids(), vec![97]);

        let loop_deposit_contract_config =
            RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
                RawContractConfig::new(
                    2,
                    "MystikoWithPolyERC20".to_string(),
                    "0x2f0Fe3154C281Cb25D6a615bf524230e57A462e1".to_string(),
                    ContractType::Deposit,
                    1000000,
                    None,
                    None,
                ),
                BridgeType::Loop,
                "0x20Eb345870059E688c59e89523442ade33C7c813".to_string(),
                false,
                None,
                None,
                "10000000000000000".to_string(),
                "100000000000000000".to_string(),
                "20000000000000000".to_string(),
                "30000000000000000".to_string(),
                None,
                None,
                None,
                None,
            )).await;
        let pool_contract_config =
            RawConfig::create_from_object::<RawPoolContractConfig>(RawPoolContractConfig::new(
                RawContractConfig::new(
                    2,
                    "CommitmentPool".to_string(),
                    "0x20Eb345870059E688c59e89523442ade33C7c813".to_string(),
                    ContractType::Pool,
                    1000000,
                    None,
                    None,
                ),
                "A Pool".to_string(),
                BridgeType::Loop,
                Some(String::from("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")),
                "40000000000000000".to_string(),
                vec![],
            )).await;
        raw_config.deposit_contracts.push(loop_deposit_contract_config);
        raw_config.pool_contracts.push(pool_contract_config);
        config = ChainConfig::new(raw_config, Some(
            AuxData::new(
                default_circuit_configs,
                circuit_configs_by_name,
                None,
            )
        ));
        let mut a = config.peer_chain_ids();
        a.sort();
        assert_eq!(a, vec![3, 97]);
    }

    #[tokio::test]
    async fn test_get_asset_symbols() {
        let mut config = config().await;
        assert_eq!(config.get_asset_symbols(97).len(), 0);
        let mut raw_config = raw_config().await;
        let mut deposit_contracts = raw_config.deposit_contracts;
        deposit_contracts[0].disabled = false;
        raw_config.deposit_contracts = deposit_contracts;

        let default_circuit_configs = default_circuit_configs().await;
        let circuit_configs_by_name = circuit_configs_by_name().await;
        config = ChainConfig::new(raw_config.clone(), Some(
            AuxData::new(
                default_circuit_configs.clone(),
                circuit_configs_by_name.clone(),
                None,
            )
        ));
        assert_eq!(config.get_asset_symbols(97), vec!["MTT".to_string()]);

        let pool_contract_config1 =
            RawConfig::create_from_object::<RawPoolContractConfig>(RawPoolContractConfig::new(
                RawContractConfig::new(
                    2,
                    "CommitmentPool".to_string(),
                    "0x954c6c78A2F93E6E19Ff1DE538F720311414530c".to_string(),
                    ContractType::Pool,
                    1000000,
                    None,
                    None,
                ),
                "A Pool".to_string(),
                BridgeType::Loop,
                None,
                "40000000000000000".to_string(),
                vec![],
            )).await;
        let pool_contract_config2 =
            RawConfig::create_from_object::<RawPoolContractConfig>(RawPoolContractConfig::new(
                RawContractConfig::new(
                    2,
                    "CommitmentPool".to_string(),
                    "0x20Eb345870059E688c59e89523442ade33C7c813".to_string(),
                    ContractType::Pool,
                    1000000,
                    None,
                    None,
                ),
                "A Pool".to_string(),
                BridgeType::Tbridge,
                None,
                "40000000000000000".to_string(),
                vec![],
            )).await;
        let loop_deposit_contract_config1 =
            RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
                RawContractConfig::new(
                    2,
                    "MystikoWithPolyERC20".to_string(),
                    "0x2f0Fe3154C281Cb25D6a615bf524230e57A462e1".to_string(),
                    ContractType::Deposit,
                    1000000,
                    None,
                    None,
                ),
                BridgeType::Loop,
                "0x954c6c78A2F93E6E19Ff1DE538F720311414530c".to_string(),
                false,
                None,
                None,
                "10000000000000000".to_string(),
                "100000000000000000".to_string(),
                "20000000000000000".to_string(),
                "30000000000000000".to_string(),
                None,
                None,
                None,
                None,
            )).await;
        let loop_deposit_contract_config2 =
            RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
                RawContractConfig::new(
                    2,
                    "MystikoWithPolyERC20".to_string(),
                    "0x0b1d6565d88f9bf6473e21c2ab58d28a495d7bb5".to_string(),
                    ContractType::Deposit,
                    1000000,
                    None,
                    None,
                ),
                BridgeType::Tbridge,
                "0x20Eb345870059E688c59e89523442ade33C7c813".to_string(),
                false,
                Some(97),
                Some(String::from("0x390de26d772d2e2005c6d1d24afc902bae37a4bb")),
                "10000000000000000".to_string(),
                "100000000000000000".to_string(),
                "20000000000000000".to_string(),
                "30000000000000000".to_string(),
                None,
                None,
                None,
                None,
            )).await;
        raw_config.deposit_contracts.push(loop_deposit_contract_config1);
        raw_config.deposit_contracts.push(loop_deposit_contract_config2);
        raw_config.pool_contracts.push(pool_contract_config1);
        raw_config.pool_contracts.push(pool_contract_config2);
        config = ChainConfig::new(raw_config, Some(
            AuxData::new(
                default_circuit_configs,
                circuit_configs_by_name,
                None,
            )
        ));
        let mut a = config.get_asset_symbols(97);
        a.sort();
        assert_eq!(a, vec![String::from("ETH"), String::from("MTT")]);
        assert_eq!(config.get_asset_symbols(3), vec![String::from("ETH")]);
    }

    #[tokio::test]
    async fn test_get_bridges() {
        let mut config = config().await;
        let mut raw_config = raw_config().await;
        assert_eq!(config.get_bridges(97, "MTT").len(), 0);
        let mut deposit_contracts = raw_config.deposit_contracts;
        deposit_contracts[0].disabled = false;
        raw_config.deposit_contracts = deposit_contracts;

        let default_circuit_configs = default_circuit_configs().await;
        let circuit_configs_by_name = circuit_configs_by_name().await;
        config = ChainConfig::new(raw_config.clone(), Some(
            AuxData::new(
                default_circuit_configs.clone(),
                circuit_configs_by_name.clone(),
                None,
            )
        ));
        assert_eq!(config.get_bridges(97, "MTT"), vec![BridgeType::Tbridge]);
        let loop_deposit_contract_config =
            RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
                RawContractConfig::new(
                    2,
                    "MystikoWithPolyERC20".to_string(),
                    "0x2f0Fe3154C281Cb25D6a615bf524230e57A462e1".to_string(),
                    ContractType::Deposit,
                    1000000,
                    None,
                    None,
                ),
                BridgeType::Loop,
                "0x6b8a4ea37c72f1992626eb9bd48d4aa6aa077c47".to_string(),
                false,
                None,
                None,
                "10000000000000000".to_string(),
                "100000000000000000".to_string(),
                "20000000000000000".to_string(),
                "30000000000000000".to_string(),
                None,
                None,
                None,
                None,
            )).await;
        let pool_contract_config =
            RawConfig::create_from_object::<RawPoolContractConfig>(RawPoolContractConfig::new(
                RawContractConfig::new(
                    2,
                    "CommitmentPool".to_string(),
                    "0x6b8a4ea37c72f1992626eb9bd48d4aa6aa077c47".to_string(),
                    ContractType::Pool,
                    1000000,
                    None,
                    None,
                ),
                "A Pool".to_string(),
                BridgeType::Loop,
                Some(String::from("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")),
                "40000000000000000".to_string(),
                vec![],
            )).await;
        raw_config.deposit_contracts.push(loop_deposit_contract_config);
        raw_config.pool_contracts.push(pool_contract_config);
        config = ChainConfig::new(raw_config.clone(), Some(
            AuxData::new(
                default_circuit_configs.clone(),
                circuit_configs_by_name.clone(),
                None,
            )
        ));
        assert_eq!(config.get_bridges(3, "MTT").len(), 0);
        let celer_deposit_contract_config =
            RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
                RawContractConfig::new(
                    2,
                    "MystikoWithPolyERC20".to_string(),
                    "0x4c55C41Bd839B3552fb2AbecaCFdF4a5D2879Cb9".to_string(),
                    ContractType::Deposit,
                    1000000,
                    None,
                    None,
                ),
                BridgeType::Celer,
                "0x20Eb345870059E688c59e89523442ade33C7c813".to_string(),
                false,
                Some(97),
                Some(String::from("0x390de26d772d2e2005c6d1d24afc902bae37a4bb")),
                "10000000000000000".to_string(),
                "100000000000000000".to_string(),
                "20000000000000000".to_string(),
                "30000000000000000".to_string(),
                None,
                None,
                None,
                None)
            ).await;
        let pool_contract_config2 =
            RawConfig::create_from_object::<RawPoolContractConfig>(RawPoolContractConfig::new(
                RawContractConfig::new(
                    2,
                    "CommitmentPool".to_string(),
                    "0x20Eb345870059E688c59e89523442ade33C7c813".to_string(),
                    ContractType::Deposit,
                    1000000,
                    None,
                    None,
                ),
                "A Pool".to_string(),
                BridgeType::Celer,
                Some(String::from("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")),
                "40000000000000000".to_string(),
                vec![],
            )).await;
        raw_config.deposit_contracts.push(celer_deposit_contract_config);
        raw_config.pool_contracts.push(pool_contract_config2);
        config = ChainConfig::new(raw_config.clone(), Some(
            AuxData::new(
                default_circuit_configs.clone(),
                circuit_configs_by_name.clone(),
                None,
            )
        ));
        let a = config.get_bridges(97, "MTT");
        assert_eq!(a.contains(&BridgeType::Celer), true);
        assert_eq!(a.contains(&BridgeType::Tbridge), true);
    }

    #[tokio::test]
    async fn test_get_deposit_contract() {
        let mut config = config().await;
        assert_eq!(
            config.get_deposit_contract(97, "MTT", BridgeType::Tbridge).is_none(),
            true
        );
        let tbridge_deposit_contract_config =
            RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
                RawContractConfig::new(
                    2,
                    "MystikoWithPolyERC20".to_string(),
                    "0x4c55C41Bd839B3552fb2AbecaCFdF4a5D2879Cb9".to_string(),
                    ContractType::Deposit,
                    1000000,
                    None,
                    None,
                ),
                BridgeType::Tbridge,
                "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d".to_string(),
                false,
                Some(97),
                Some(String::from("0x390de26d772d2e2005c6d1d24afc902bae37a4bb")),
                "10000000000000000".to_string(),
                "100000000000000000".to_string(),
                "20000000000000000".to_string(),
                "30000000000000000".to_string(),
                None,
                None,
                None,
                None,
            )).await;
        let loop_deposit_contract_config =
            RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
                RawContractConfig::new(
                    2,
                    "MystikoWithPolyERC20".to_string(),
                    "0x2f0Fe3154C281Cb25D6a615bf524230e57A462e1".to_string(),
                    ContractType::Deposit,
                    1000000,
                    None,
                    None,
                ),
                BridgeType::Loop,
                "0x20Eb345870059E688c59e89523442ade33C7c813".to_string(),
                false,
                None,
                None,
                "10000000000000000".to_string(),
                "100000000000000000".to_string(),
                "20000000000000000".to_string(),
                "30000000000000000".to_string(),
                None,
                None,
                None,
                None,
            )).await;
        let pool_contract_config =
            RawConfig::create_from_object::<RawPoolContractConfig>(RawPoolContractConfig::new(
                RawContractConfig::new(
                    2,
                    "CommitmentPool".to_string(),
                    "0x20Eb345870059E688c59e89523442ade33C7c813".to_string(),
                    ContractType::Pool,
                    1000000,
                    None,
                    None,
                ),
                "A Pool".to_string(),
                BridgeType::Loop,
                Some(String::from("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")),
                "40000000000000000".to_string(),
                vec![],
            )).await;
        let mut raw_config = raw_config().await;
        raw_config.deposit_contracts.push(tbridge_deposit_contract_config);
        raw_config.deposit_contracts.push(loop_deposit_contract_config);
        raw_config.pool_contracts.push(pool_contract_config);
        let default_circuit_configs = default_circuit_configs().await;
        let circuit_configs_by_name = circuit_configs_by_name().await;
        config = ChainConfig::new(raw_config.clone(), Some(
            AuxData::new(
                default_circuit_configs,
                circuit_configs_by_name,
                None,
            )
        ));
        assert_eq!(
            config.get_deposit_contract(
                97,
                "MTT",
                BridgeType::Tbridge,
            ).unwrap().base.base.data.address(),
            "0x4c55C41Bd839B3552fb2AbecaCFdF4a5D2879Cb9"
        );
        assert_eq!(
            config.get_deposit_contract(
                3,
                "MTT",
                BridgeType::Loop,
            ).unwrap().base.base.data.address(),
            "0x2f0Fe3154C281Cb25D6a615bf524230e57A462e1"
        );
        assert_eq!(
            config.get_deposit_contract_by_address(
                "0x2f0Fe3154C281Cb25D6a615bf524230e57A462e1".to_string()
            ).unwrap().pool_address(),
            "0x20Eb345870059E688c59e89523442ade33C7c813"
        );
        assert_eq!(
            config.get_deposit_contract_by_address(
                "0x5380442d3c4ec4f5777f551f5edd2fa0f691a27c".to_string()
            ).is_none(),
            true
        );
    }

    #[tokio::test]
    async fn test_get_pool_contract() {
        let tbridge_deposit_contract_config =
            RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
                RawContractConfig::new(
                    2,
                    "MystikoWithPolyERC20".to_string(),
                    "0x4c55C41Bd839B3552fb2AbecaCFdF4a5D2879Cb9".to_string(),
                    ContractType::Deposit,
                    1000000,
                    None,
                    None,
                ),
                BridgeType::Tbridge,
                "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d".to_string(),
                false,
                Some(100),
                Some(String::from("0x390de26d772d2e2005c6d1d24afc902bae37a4bb")),
                "10000000000000000".to_string(),
                "100000000000000000".to_string(),
                "20000000000000000".to_string(),
                "30000000000000000".to_string(),
                None,
                None,
                None,
                None,
            )).await;
        let loop_deposit_contract_config =
            RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
                RawContractConfig::new(
                    2,
                    "MystikoWithPolyERC20".to_string(),
                    "0x2f0Fe3154C281Cb25D6a615bf524230e57A462e1".to_string(),
                    ContractType::Deposit,
                    1000000,
                    None,
                    None,
                ),
                BridgeType::Loop,
                "0x954c6c78A2F93E6E19Ff1DE538F720311414530c".to_string(),
                false,
                None,
                None,
                "10000000000000000".to_string(),
                "100000000000000000".to_string(),
                "20000000000000000".to_string(),
                "30000000000000000".to_string(),
                None,
                None,
                None,
                None,
            )).await;
        let pool_contract_config1 =
            RawConfig::create_from_object::<RawPoolContractConfig>(RawPoolContractConfig::new(
                RawContractConfig::new(
                    1,
                    "CommitmentPool".to_string(),
                    "0x81b7e08f65bdf5648606c89998a9cc8164397647".to_string(),
                    ContractType::Pool,
                    1000000,
                    None,
                    None,
                ),
                "A Pool".to_string(),
                BridgeType::Loop,
                None,
                "40000000000000000".to_string(),
                vec![],
            )).await;
        let pool_contract_config2 =
            RawConfig::create_from_object::<RawPoolContractConfig>(RawPoolContractConfig::new(
                RawContractConfig::new(
                    2,
                    "CommitmentPool".to_string(),
                    "0x954c6c78A2F93E6E19Ff1DE538F720311414530c".to_string(),
                    ContractType::Pool,
                    1000000,
                    None,
                    None,
                ),
                "A Pool".to_string(),
                BridgeType::Loop,
                None,
                "40000000000000000".to_string(),
                vec![],
            )).await;
        let mut raw_config = raw_config().await;
        raw_config.pool_contracts.push(pool_contract_config1);
        raw_config.pool_contracts.push(pool_contract_config2);
        raw_config.deposit_contracts.push(tbridge_deposit_contract_config);
        raw_config.deposit_contracts.push(loop_deposit_contract_config);
        raw_config.deposit_contracts[0].disabled = false;
        let default_circuit_configs = default_circuit_configs().await;
        let circuit_configs_by_name = circuit_configs_by_name().await;
        let config = ChainConfig::new(raw_config.clone(), Some(
            AuxData::new(
                default_circuit_configs.clone(),
                circuit_configs_by_name.clone(),
                None,
            )
        ));
        assert_eq!(
            config.get_pool_contract(
                "MTT",
                BridgeType::Tbridge,
                2,
            ).unwrap().base.address(),
            "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"
        );
        assert_eq!(
            config.get_pool_contract(
                "MTT",
                BridgeType::Tbridge,
                3,
            ).is_none(),
            true
        );
        assert_eq!(
            config.get_pool_contract(
                "mUSD",
                BridgeType::Tbridge,
                2,
            ).is_none(),
            true
        );
        assert_eq!(
            config.get_pool_contract(
                "MTT",
                BridgeType::Loop,
                2,
            ).is_none(),
            true
        );
        assert_eq!(
            config.get_pool_contract(
                "ETH",
                BridgeType::Loop,
                1,
            ).unwrap().base.address(),
            "0x81b7e08f65bdf5648606c89998a9cc8164397647"
        );
        assert_eq!(
            config.get_pool_contract(
                "ETH",
                BridgeType::Loop,
                2,
            ).unwrap().base.address(),
            "0x954c6c78A2F93E6E19Ff1DE538F720311414530c"
        );
        assert_eq!(
            config.get_pool_contracts(
                "mUSD",
                BridgeType::Tbridge,
            ).len(),
            0
        );
        assert_eq!(
            config.get_pool_contracts(
                "MTT",
                BridgeType::Loop,
            ).len(),
            0
        );
        assert_eq!(
            config.get_pool_contracts(
                "ETH",
                BridgeType::Loop,
            ).iter().map(|c| c.base.address()).collect::<Vec<&str>>().contains(
                &"0x954c6c78A2F93E6E19Ff1DE538F720311414530c"
            ),
            true
        );
        assert_eq!(
            config.get_pool_contracts(
                "ETH",
                BridgeType::Loop,
            ).iter().map(|c| c.base.address()).collect::<Vec<&str>>().contains(
                &"0x81b7e08f65bdf5648606c89998a9cc8164397647"
            ),
            true
        );
        assert_eq!(
            config.get_pool_contracts(
                "ETH",
                BridgeType::Loop,
            ).iter().map(|c| c.base.address()).collect::<Vec<&str>>().len(),
            2
        );
        assert_eq!(
            config.get_pool_contract_by_address(
                "0x954c6c78A2F93E6E19Ff1DE538F720311414530c"
            ).unwrap().asset_symbol(),
            "ETH".to_string()
        );
        assert_eq!(
            config.get_pool_contract_by_address(
                "0x5380442d3c4ec4f5777f551f5edd2fa0f691a27c"
            ).is_none(),
            true
        );
    }

    #[tokio::test]
    async fn test_get_pool_contract_bridge_type() {
        let config = config().await;
        assert_eq!(
            config.get_pool_contract_bridge_type(
                "0x721d424047d3a8dd20f7a88f2eadad16fd2fab51"
            ).is_none(),
            true
        );
        assert_eq!(
            config.get_pool_contract_bridge_type(
                "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"
            ).unwrap(),
            BridgeType::Tbridge
        );
    }

    #[tokio::test]
    async fn test_get_event_filter_size_by_address() {
        let mut raw_config = raw_config().await;
        raw_config.event_filter_size = 12345;
        let default_circuit_configs = default_circuit_configs().await;
        let circuit_configs_by_name = circuit_configs_by_name().await;
        let mut config = ChainConfig::new(raw_config.clone(), Some(
            AuxData::new(
                default_circuit_configs.clone(),
                circuit_configs_by_name.clone(),
                None,
            )
        ));
        assert_eq!(
            config.get_event_filter_size_by_address(
                "0x5380442d3c4ec4f5777f551f5edd2fa0f691a27c"
            ),
            12345
        );
        assert_eq!(
            config.get_event_filter_size_by_address(
                "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
            ),
            12345
        );
        assert_eq!(
            config.get_event_filter_size_by_address(
                "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"
            ),
            12345
        );
        raw_config.deposit_contracts[0].base.event_filter_size = Some(87654321);
        raw_config.pool_contracts[0].base.event_filter_size = Some(987654321);
        config = ChainConfig::new(raw_config.clone(), Some(
            AuxData::new(
                default_circuit_configs.clone(),
                circuit_configs_by_name.clone(),
                None,
            )
        ));
        assert_eq!(
            config.get_event_filter_size_by_address(
                "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
            ),
            87654321
        );
        assert_eq!(
            config.get_event_filter_size_by_address(
                "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"
            ),
            987654321
        );
    }

    #[tokio::test]
    async fn test_get_indexer_filter_size_by_address() {
        let mut raw_config = raw_config().await;
        raw_config.indexer_filter_size = 123450;
        let default_circuit_configs = default_circuit_configs().await;
        let circuit_configs_by_name = circuit_configs_by_name().await;
        let config = ChainConfig::new(raw_config.clone(), Some(
            AuxData::new(
                default_circuit_configs.clone(),
                circuit_configs_by_name.clone(),
                None,
            )
        ));
        assert_eq!(
            config.get_indexer_filter_size_by_address(
                "0x5380442d3c4ec4f5777f551f5edd2fa0f691a27c"
            ),
            123450
        );
        assert_eq!(
            config.get_indexer_filter_size_by_address(
                "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
            ),
            123450
        );
        assert_eq!(
            config.get_indexer_filter_size_by_address(
                "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"
            ),
            123450
        );
        raw_config.deposit_contracts[0].base.indexer_filter_size = Some(876543210);
        raw_config.pool_contracts[0].base.indexer_filter_size = Some(9876543210);
        let config = ChainConfig::new(raw_config.clone(), Some(
            AuxData::new(
                default_circuit_configs.clone(),
                circuit_configs_by_name.clone(),
                None,
            )
        ));
        assert_eq!(
            config.get_indexer_filter_size_by_address(
                "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
            ),
            876543210
        );
        assert_eq!(
            config.get_indexer_filter_size_by_address(
                "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"
            ),
            9876543210
        );
    }

    #[tokio::test]
    #[should_panic(expected = "deposit contract=\
    0x961f315a836542e603a3df2e0dd9d4ecd06ebc67 poolAddress definition does not exist")]
    async fn test_invalid_pool_address() {
        let mut raw_config = raw_config().await;
        raw_config.deposit_contracts[0].pool_address =
            "0x5380442d3c4ec4f5777f551f5edd2fa0f691a27c".to_string();
        let default_circuit_configs = default_circuit_configs().await;
        let circuit_configs_by_name = circuit_configs_by_name().await;
        ChainConfig::new(raw_config, Some(
            AuxData::new(
                default_circuit_configs,
                circuit_configs_by_name,
                None,
            )
        ));
    }

    #[tokio::test]
    #[should_panic(expected = "only one pool address allowed for asset MTT and bridge type Tbridge and version 2")]
    async fn test_duplicate_bridge_and_asset() {
        let pool_contract_config =
            RawConfig::create_from_object::<RawPoolContractConfig>(RawPoolContractConfig::new(
                RawContractConfig::new(
                    2,
                    "CommitmentPool".to_string(),
                    "0x954c6c78A2F93E6E19Ff1DE538F720311414530c".to_string(),
                    ContractType::Pool,
                    1000000,
                    None,
                    None,
                ),
                "A Pool".to_string(),
                BridgeType::Tbridge,
                Some(String::from("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a")),
                "40000000000000000".to_string(),
                vec![],
            )).await;
        let tbridge_deposit_contract_config =
            RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
                RawContractConfig::new(
                    2,
                    "MystikoWithPolyERC20".to_string(),
                    "0x4c55C41Bd839B3552fb2AbecaCFdF4a5D2879Cb9".to_string(),
                    ContractType::Deposit,
                    1000000,
                    None,
                    None,
                ),
                BridgeType::Tbridge,
                "0x954c6c78A2F93E6E19Ff1DE538F720311414530c".to_string(),
                false,
                Some(100),
                Some(String::from("0x390de26d772d2e2005c6d1d24afc902bae37a4bb")),
                "10000000000000000".to_string(),
                "100000000000000000".to_string(),
                "20000000000000000".to_string(),
                "30000000000000000".to_string(),
                None,
                None,
                None,
                None,
            )).await;
        let mut raw_config = raw_config().await;
        raw_config.pool_contracts.push(pool_contract_config);
        raw_config.deposit_contracts.push(tbridge_deposit_contract_config);
        raw_config.deposit_contracts[0].disabled = false;
        let default_circuit_configs = default_circuit_configs().await;
        let circuit_configs_by_name = circuit_configs_by_name().await;
        ChainConfig::new(raw_config.clone(), Some(
            AuxData::new(
                default_circuit_configs.clone(),
                circuit_configs_by_name.clone(),
                None,
            )
        ));
    }

    #[tokio::test]
    #[should_panic(expected = "deposit contract=0x2f0Fe3154C281Cb25D6a615bf524230e57A462e1 \
    bridgeType=Loop does not equal to pool contract bridgeType=Tbridge")]
    async fn test_different_bridge_with_same_pool_address() {
        let mut config = config().await;
        assert_eq!(config.peer_chain_ids().len(), 0);
        let mut raw_config = raw_config().await;
        raw_config.deposit_contracts[0].disabled = false;
        let default_circuit_configs = default_circuit_configs().await;
        let circuit_configs_by_name = circuit_configs_by_name().await;
        config = ChainConfig::new(raw_config.clone(), Some(
            AuxData::new(
                default_circuit_configs.clone(),
                circuit_configs_by_name.clone(),
                None,
            )
        ));
        assert_eq!(config.peer_chain_ids(), vec![97]);
        let loop_deposit_contract_config =
            RawConfig::create_from_object::<RawDepositContractConfig>(RawDepositContractConfig::new(
                RawContractConfig::new(
                    2,
                    "MystikoWithPolyERC20".to_string(),
                    "0x2f0Fe3154C281Cb25D6a615bf524230e57A462e1".to_string(),
                    ContractType::Deposit,
                    1000000,
                    None,
                    None,
                ),
                BridgeType::Loop,
                "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d".to_string(),
                false,
                None,
                None,
                "10000000000000000".to_string(),
                "100000000000000000".to_string(),
                "20000000000000000".to_string(),
                "30000000000000000".to_string(),
                None,
                None,
                None,
                None,
            )).await;
        raw_config.deposit_contracts.push(loop_deposit_contract_config);
        ChainConfig::new(raw_config.clone(), Some(
            AuxData::new(
                default_circuit_configs.clone(),
                circuit_configs_by_name.clone(),
                None,
            )
        ));
    }

    #[tokio::test]
    async fn test_get_asset_config_by_address() {
        let config = config().await;
        assert_eq!(
            config.get_asset_config_by_address(
                "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a"
            ).unwrap().asset_decimals(),
            16
        );
        assert_eq!(
            config.get_asset_config_by_address(
                "0xBc28029D248FC60bce0bAC01cF41A53aEEaE06F9"
            ).is_none(),
            true
        );
    }

    #[tokio::test]
    async fn test_get_transaction_url() {
        let config = config().await;
        assert_eq!(
            config.get_transaction_url(
                "0xbce8d733536ee3b769456cf91bebae1e9e5be6cb89bb7490c6225384e1bc5e3e"
            ),
            String::from(
                "https://ropsten.etherscan.io/tx/0xbce8d733536ee3b769456cf91bebae1e9e5be6cb89bb7490c6225384e1bc5e3e"
            )
        );
    }

    #[tokio::test]
    async fn test_copy() {
        let config = config().await;
        let raw_config = raw_config().await;
        assert_eq!(config.base.copy_data(), raw_config);
    }

    #[tokio::test]
    async fn test_mutate() {
        let config = config().await;
        let mut raw_config = raw_config().await;
        assert_eq!(
            config.mutate(None, None).base.copy_data(),
            raw_config
        );
        raw_config.name = "another name".to_string();
        let mut new_config = config.mutate(Some(raw_config.clone()), None);
        assert_eq!(new_config.name(), "another name");
        let default_circuit_configs = default_circuit_configs().await;
        let circuit_configs_by_name = circuit_configs_by_name().await;
        new_config = config.mutate(Some(raw_config.clone()), Some(
            AuxData::new(
                default_circuit_configs.clone(),
                circuit_configs_by_name.clone(),
                None,
            )
        ));
        assert_eq!(new_config.base.copy_data(), raw_config);
    }

    #[tokio::test]
    async fn test_to_json_string() {
        let config = config().await;
        let json_string = config.base.to_json_string();
        let loaded_raw_config =
            RawConfig::create_from_json_string::<RawChainConfig>(json_string.as_str()).await;
        assert_eq!(loaded_raw_config, raw_config().await);
    }
}
