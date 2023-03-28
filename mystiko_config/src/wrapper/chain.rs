use crate::raw::asset::RawAssetConfig;
use crate::raw::chain::{RawChainConfig, EXPLORER_TX_PLACEHOLDER};
use crate::raw::contract::deposit::RawDepositContractConfig;
use crate::raw::contract::pool::RawPoolContractConfig;
use crate::types::{AssetType, BridgeType, CircuitType};
use crate::wrapper::asset::AssetConfig;
use crate::wrapper::circuit::CircuitConfig;
use crate::wrapper::contract::deposit::DepositContractConfig;
use crate::wrapper::contract::pool::PoolContractConfig;
use crate::wrapper::provider::ProviderConfig;
use anyhow::{Error, Result};
use num_bigint::BigInt;
use num_traits::{NumCast, Zero};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use validator::Validate;

pub const MAIN_ASSET_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

#[derive(Clone, Debug)]
pub struct ChainConfig {
    raw: Arc<RawChainConfig>,
    main_asset_config: Arc<AssetConfig>,
    asset_configs: Vec<Arc<AssetConfig>>,
    pool_contract_configs: Vec<Arc<PoolContractConfig>>,
    deposit_contract_configs: Vec<Arc<DepositContractConfig>>,
    provider_configs: Vec<Arc<ProviderConfig>>,
}

impl ChainConfig {
    pub fn new(
        raw: Arc<RawChainConfig>,
        default_circuit_configs: &HashMap<CircuitType, Arc<CircuitConfig>>,
        circuit_configs_by_name: &HashMap<String, Arc<CircuitConfig>>,
    ) -> Result<Self> {
        let main_asset_config = Arc::new(AssetConfig::new(Arc::new(RawAssetConfig {
            asset_type: AssetType::Main,
            asset_symbol: raw.asset_symbol.to_string(),
            asset_decimals: raw.asset_decimals,
            asset_address: MAIN_ASSET_ADDRESS.to_string(),
            recommended_amounts: raw.recommended_amounts.clone(),
        })));
        let asset_configs: Vec<Arc<AssetConfig>> = raw
            .assets
            .iter()
            .map(|r| Arc::new(AssetConfig::new(r.clone())))
            .collect();
        let provider_configs = raw
            .providers
            .iter()
            .map(|r| Arc::new(ProviderConfig::new(r.clone())))
            .collect();
        let pool_contract_configs = initialize_pool_contracts(
            &raw.pool_contracts,
            &main_asset_config,
            &asset_configs,
            default_circuit_configs,
            circuit_configs_by_name,
        )?;
        let deposit_contract_configs = initialize_deposit_contracts(
            &raw.deposit_contracts,
            &pool_contract_configs,
            &main_asset_config,
            &asset_configs,
        )?;
        Ok(ChainConfig {
            raw,
            main_asset_config,
            asset_configs,
            pool_contract_configs,
            deposit_contract_configs,
            provider_configs,
        })
    }

    pub fn chain_id(&self) -> u32 {
        self.raw.chain_id
    }

    pub fn name(&self) -> &str {
        &self.raw.name
    }

    pub fn main_asset(&self) -> &AssetConfig {
        &self.main_asset_config
    }

    pub fn asset_symbol(&self) -> &str {
        &self.raw.asset_symbol
    }

    pub fn asset_decimals(&self) -> u32 {
        self.raw.asset_decimals
    }

    pub fn recommended_amounts(&self) -> Result<Vec<BigInt>> {
        self.main_asset().recommended_amounts()
    }

    pub fn recommended_amounts_number<T>(&self) -> Result<Vec<T>>
    where
        T: NumCast + Zero,
    {
        self.main_asset().recommended_amounts_number()
    }

    pub fn explorer_url(&self) -> &str {
        &self.raw.explorer_url
    }

    pub fn explorer_prefix(&self) -> &str {
        &self.raw.explorer_prefix
    }

    pub fn providers(&self) -> Vec<&ProviderConfig> {
        self.provider_configs.iter().map(|c| c.as_ref()).collect()
    }

    pub fn signer_endpoint(&self) -> &str {
        &self.raw.signer_endpoint
    }

    pub fn event_filter_size(&self) -> u64 {
        self.raw.event_filter_size
    }

    pub fn indexer_filter_size(&self) -> u64 {
        self.raw.indexer_filter_size
    }

    pub fn pool_contracts(&self) -> Vec<&PoolContractConfig> {
        self.pool_contract_configs
            .iter()
            .map(|c| c.as_ref())
            .collect()
    }

    pub fn deposit_contracts(&self) -> Vec<&DepositContractConfig> {
        self.deposit_contracts_with_disabled()
            .into_iter()
            .filter(|c| !c.disabled())
            .collect()
    }

    pub fn deposit_contracts_with_disabled(&self) -> Vec<&DepositContractConfig> {
        self.deposit_contract_configs
            .iter()
            .map(|c| c.as_ref())
            .collect()
    }

    pub fn assets(&self) -> Vec<&AssetConfig> {
        self.asset_configs.iter().map(|c| c.as_ref()).collect()
    }

    pub fn find_peer_chain_ids(&self) -> Vec<u32> {
        let mut ids: HashSet<u32> = HashSet::new();
        for deposit_contract_config in &self.deposit_contract_configs {
            if deposit_contract_config.bridge_type() == &BridgeType::Loop {
                ids.insert(self.chain_id());
            } else if let Some(peer_chain_id) = deposit_contract_config.peer_chain_id() {
                ids.insert(*peer_chain_id);
            }
        }
        ids.into_iter().collect()
    }

    pub fn find_asset_symbols(&self, peer_chain_id: u32) -> Vec<&str> {
        let mut asset_symbols: HashSet<&str> = HashSet::new();
        for deposit_contract_config in &self.deposit_contract_configs {
            if peer_chain_id == self.chain_id() {
                if deposit_contract_config.bridge_type() == &BridgeType::Loop {
                    asset_symbols.insert(deposit_contract_config.asset_symbol());
                }
            } else if deposit_contract_config.bridge_type() != &BridgeType::Loop {
                if let Some(id) = deposit_contract_config.peer_chain_id() {
                    if peer_chain_id == *id {
                        asset_symbols.insert(deposit_contract_config.asset_symbol());
                    }
                }
            }
        }
        asset_symbols.into_iter().collect()
    }

    pub fn find_bridges(&self, peer_chain_id: u32, asset_symbol: &str) -> Vec<&BridgeType> {
        let mut bridges: HashSet<&BridgeType> = HashSet::new();
        for deposit_contract_config in &self.deposit_contract_configs {
            if let Some(id) = deposit_contract_config.peer_chain_id() {
                if *id == peer_chain_id
                    && peer_chain_id != self.chain_id()
                    && asset_symbol == deposit_contract_config.asset_symbol()
                {
                    bridges.insert(deposit_contract_config.bridge_type());
                }
            }
        }
        bridges.into_iter().collect()
    }

    pub fn find_deposit_contract(
        &self,
        peer_chain_id: u32,
        asset_symbol: &str,
        bridge_type: &BridgeType,
    ) -> Option<&DepositContractConfig> {
        for deposit_contract_config in &self.deposit_contract_configs {
            if deposit_contract_config.asset_symbol() == asset_symbol
                && deposit_contract_config.bridge_type() == bridge_type
            {
                if let Some(id) = deposit_contract_config.peer_chain_id() {
                    if peer_chain_id == *id && bridge_type != &BridgeType::Loop {
                        return Some(deposit_contract_config);
                    }
                }
                if peer_chain_id == self.chain_id() && bridge_type == &BridgeType::Loop {
                    return Some(deposit_contract_config);
                }
            }
        }
        None
    }

    pub fn find_pool_contract(
        &self,
        asset_symbol: &str,
        bridge_type: &BridgeType,
        version: u32,
    ) -> Option<&PoolContractConfig> {
        self.pool_contracts().into_iter().find(|r| {
            r.asset_symbol() == asset_symbol
                && r.bridge_type() == bridge_type
                && r.version() == version
        })
    }

    pub fn find_pool_contracts(
        &self,
        asset_symbol: &str,
        bridge_type: &BridgeType,
    ) -> Vec<&PoolContractConfig> {
        self.pool_contracts()
            .into_iter()
            .filter(|r| r.asset_symbol() == asset_symbol && r.bridge_type() == bridge_type)
            .collect()
    }

    pub fn find_pool_contract_by_address(&self, address: &str) -> Option<&PoolContractConfig> {
        self.pool_contract_configs
            .iter()
            .find(|c| c.address() == address)
            .map(|c| c.as_ref())
    }

    pub fn find_deposit_contract_by_address(
        &self,
        address: &str,
    ) -> Option<&DepositContractConfig> {
        self.deposit_contract_configs
            .iter()
            .find(|c| c.address() == address)
            .map(|c| c.as_ref())
    }

    pub fn find_asset(&self, address: &str) -> Option<&AssetConfig> {
        self.asset_configs
            .iter()
            .find(|c| c.asset_address() == address)
            .map(|c| c.as_ref())
    }

    pub fn contract_event_filter_size(&self, address: &str) -> u64 {
        if let Some(deposit_contract) = self.find_deposit_contract_by_address(address) {
            deposit_contract
                .event_filter_size()
                .unwrap_or(self.event_filter_size())
        } else if let Some(pool_contract) = self.find_pool_contract_by_address(address) {
            pool_contract
                .event_filter_size()
                .unwrap_or(self.event_filter_size())
        } else {
            self.event_filter_size()
        }
    }

    pub fn contract_indexer_filter_size(&self, address: &str) -> u64 {
        if let Some(deposit_contract) = self.find_deposit_contract_by_address(address) {
            deposit_contract
                .indexer_filter_size()
                .unwrap_or(self.indexer_filter_size())
        } else if let Some(pool_contract) = self.find_pool_contract_by_address(address) {
            pool_contract
                .indexer_filter_size()
                .unwrap_or(self.indexer_filter_size())
        } else {
            self.indexer_filter_size()
        }
    }

    pub fn transaction_url(&self, tx_hash: &str) -> String {
        format!("{}{}", self.explorer_url(), self.explorer_prefix())
            .replace(EXPLORER_TX_PLACEHOLDER, tx_hash)
    }

    pub fn validate(&self) -> Result<()> {
        self.raw.validate()?;
        self.main_asset_config.validate()?;
        let mut pool_contracts: HashMap<&str, &PoolContractConfig> = HashMap::new();
        let mut pool_contracts_versions =
            HashMap::<&str, HashMap<&BridgeType, HashSet<u32>>>::new();
        for pool_contract in self.pool_contracts() {
            pool_contract.validate()?;
            if pool_contracts.contains_key(pool_contract.address()) {
                return Err(Error::msg(format!(
                    "duplicate pool contract config at {}",
                    pool_contract.address()
                )));
            }
            pool_contracts.insert(pool_contract.address(), pool_contract);
            if let Some(pool_contracts_bridges) =
                pool_contracts_versions.get_mut(pool_contract.asset_symbol())
            {
                if let Some(all_versions) =
                    pool_contracts_bridges.get_mut(pool_contract.bridge_type())
                {
                    if all_versions.contains(&pool_contract.version()) {
                        return Err(Error::msg(format!(
                            "only one version is allowed for pool contract config \
                            at {} for asset_symbol {} and bridge_type {:?}",
                            pool_contract.address(),
                            pool_contract.asset_symbol(),
                            pool_contract.bridge_type(),
                        )));
                    }
                    all_versions.insert(pool_contract.version());
                } else {
                    let mut all_versions: HashSet<u32> = HashSet::new();
                    all_versions.insert(pool_contract.version());
                    pool_contracts_bridges.insert(pool_contract.bridge_type(), all_versions);
                }
            } else {
                let mut all_versions: HashSet<u32> = HashSet::new();
                all_versions.insert(pool_contract.version());
                let mut pool_contract_bridges = HashMap::<&BridgeType, HashSet<u32>>::new();
                pool_contract_bridges.insert(pool_contract.bridge_type(), all_versions);
                pool_contracts_versions.insert(pool_contract.asset_symbol(), pool_contract_bridges);
            }
        }
        let mut deposit_contract_addresses: HashSet<&str> = HashSet::new();
        for deposit_contract in self.deposit_contracts_with_disabled() {
            deposit_contract.validate()?;
            if deposit_contract_addresses.contains(deposit_contract.address()) {
                return Err(Error::msg(format!(
                    "duplicate deposit contract config at {}",
                    deposit_contract.address()
                )));
            }
            deposit_contract_addresses.insert(deposit_contract.address());
            if let Some(peer_chain_id) = deposit_contract.peer_chain_id() {
                if self.chain_id() == *peer_chain_id {
                    return Err(Error::msg(format!(
                        "deposit contract config at {} chain_id {} \
                        cannot be same as peer_chain_id",
                        deposit_contract.address(),
                        self.chain_id()
                    )));
                }
            }
        }
        for asset_config in self.assets() {
            asset_config.validate()?;
        }
        for provider_config in self.providers() {
            provider_config.validate()?;
        }
        Ok(())
    }
}

fn initialize_pool_contracts(
    raw_pool_contracts: &[Arc<RawPoolContractConfig>],
    main_asset_config: &Arc<AssetConfig>,
    asset_configs: &[Arc<AssetConfig>],
    default_circuit_configs: &HashMap<CircuitType, Arc<CircuitConfig>>,
    circuit_configs_by_name: &HashMap<String, Arc<CircuitConfig>>,
) -> Result<Vec<Arc<PoolContractConfig>>> {
    let mut pool_contracts: Vec<Arc<PoolContractConfig>> = Vec::new();
    for raw_pool_contract in raw_pool_contracts {
        let asset_config = if let Some(asset_address) = &raw_pool_contract.asset_address {
            asset_config_by_address(asset_configs, asset_address)?
        } else {
            main_asset_config.clone()
        };
        let mut circuit_configs = HashMap::<CircuitType, Arc<CircuitConfig>>::new();
        for circuit_type in &CircuitType::all() {
            if let Some(circuit_config) = default_circuit_configs.get(circuit_type) {
                circuit_configs.insert(*circuit_type, circuit_config.clone());
            }
        }
        for name in &raw_pool_contract.circuits {
            if let Some(circuit_config) = circuit_configs_by_name.get(name) {
                circuit_configs.insert(*circuit_config.circuit_type(), circuit_config.clone());
            }
        }
        pool_contracts.push(Arc::new(PoolContractConfig::new(
            raw_pool_contract.clone(),
            main_asset_config.clone(),
            asset_config,
            circuit_configs.into_values().collect(),
        )));
    }
    Ok(pool_contracts)
}

fn initialize_deposit_contracts(
    raw_deposit_contracts: &[Arc<RawDepositContractConfig>],
    pool_contracts: &[Arc<PoolContractConfig>],
    main_asset_config: &Arc<AssetConfig>,
    asset_configs: &[Arc<AssetConfig>],
) -> Result<Vec<Arc<DepositContractConfig>>> {
    let mut deposit_contracts: Vec<Arc<DepositContractConfig>> = Vec::new();
    for raw_deposit_contract in raw_deposit_contracts {
        let pool_contract =
            pool_contract_by_address(pool_contracts, &raw_deposit_contract.pool_address)?;
        let bridge_fee_asset = if let Some(bridge_fee_asset_address) =
            &raw_deposit_contract.bridge_fee_asset_address
        {
            asset_config_by_address(asset_configs, bridge_fee_asset_address)?
        } else {
            main_asset_config.clone()
        };
        let executor_fee_asset = if let Some(executor_fee_asset_address) =
            &raw_deposit_contract.executor_fee_asset_address
        {
            asset_config_by_address(asset_configs, executor_fee_asset_address)?
        } else if let Some(asset_address) = pool_contract.asset_address() {
            asset_config_by_address(asset_configs, asset_address)?
        } else {
            main_asset_config.clone()
        };
        deposit_contracts.push(Arc::new(DepositContractConfig::new(
            raw_deposit_contract.clone(),
            bridge_fee_asset,
            executor_fee_asset,
            pool_contract,
        )));
    }
    Ok(deposit_contracts)
}

fn asset_config_by_address(
    asset_configs: &[Arc<AssetConfig>],
    address: &str,
) -> Result<Arc<AssetConfig>> {
    if let Some(asset_config) = asset_configs.iter().find(|c| c.asset_address() == address) {
        Ok(asset_config.clone())
    } else {
        Err(Error::msg(format!(
            "failed to find asset config {}",
            address
        )))
    }
}

fn pool_contract_by_address(
    pool_contracts: &[Arc<PoolContractConfig>],
    address: &str,
) -> Result<Arc<PoolContractConfig>> {
    if let Some(pool_contract_config) = pool_contracts.iter().find(|c| c.address() == address) {
        Ok(pool_contract_config.clone())
    } else {
        Err(Error::msg(format!(
            "failed to find pool contract {}",
            address
        )))
    }
}
