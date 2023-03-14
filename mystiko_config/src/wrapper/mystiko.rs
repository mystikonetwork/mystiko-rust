use std::collections::HashMap;
use std::error::Error;
use flamer::flame;
use strum::IntoEnumIterator;
use mystiko_utils::check::check;
use crate::common::{BridgeType, CircuitType};
use crate::errors::ValidationError;
use crate::raw::base::{RawConfig, Validator};
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
use crate::wrapper::contract::pool::PoolContractConfig;
use crate::wrapper::indexer::IndexerConfig;

#[derive(Clone, Debug, PartialEq)]
pub enum BridgeConfigType {
    AxelarBridgeConfig(AxelarBridgeConfig),
    CelerBridgeConfig(CelerBridgeConfig),
    PolyBridgeConfig(PolyBridgeConfig),
    LayerZeroBridgeConfig(LayerZeroBridgeConfig),
    TBridgeConfig(TBridgeConfig),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MystikoConfig {
    base: BaseConfig<RawMystikoConfig>,
    default_circuit_configs: HashMap<CircuitType, CircuitConfig>,
    circuit_configs_by_name: HashMap<String, CircuitConfig>,
    bridge_configs: HashMap<BridgeType, BridgeConfigType>,
    chain_configs: HashMap<u32, ChainConfig>,
    indexer_config: Option<IndexerConfig>,
}

impl MystikoConfig {
    #[flame]
    pub fn new(data: RawMystikoConfig) -> Result<Self, ValidationError> {
        let base = BaseConfig::new(data, None);
        let mut config = Self {
            base: base.clone(),
            default_circuit_configs: HashMap::default(),
            circuit_configs_by_name: HashMap::default(),
            bridge_configs: Default::default(),
            chain_configs: Default::default(),
            indexer_config: Default::default(),
        };
        config.init_circuit_configs()?;
        config.init_chain_configs()?;
        config.init_bridge_configs();
        config.init_indexer_config();
        config.validate()?;
        Ok(config)
    }

    pub fn data(&self) -> &RawMystikoConfig {
        &self.base.data
    }

    pub fn copy_data(&self) -> RawMystikoConfig {
        self.base.copy_data()
    }

    pub fn to_json_string(&self) -> String {
        self.base.to_json_string()
    }

    pub fn version(&self) -> &str {
        &self.base.data.version
    }

    pub fn circuits(&self) -> Vec<CircuitConfig> {
        self.circuit_configs_by_name.values().cloned().collect()
    }

    pub fn bridges(&self) -> Vec<BridgeConfigType> {
        self.bridge_configs.values().cloned().collect()
    }

    pub fn chains(&self) -> Vec<ChainConfig> {
        self.chain_configs.values().cloned().collect()
    }

    pub fn indexer(&self) -> &Option<IndexerConfig> {
        &self.indexer_config
    }

    pub fn get_chain_config(&self, chain_id: u32) -> Option<&ChainConfig> {
        self.chain_configs.get(&chain_id)
    }

    pub fn get_chain_configs(&self) -> HashMap<u32, ChainConfig> {
        self.chain_configs.clone()
    }

    pub fn get_peer_chain_configs(&self, chain_id: u32) -> Vec<ChainConfig> {
        let mut peer_chain_configs: Vec<ChainConfig> = Vec::new();
        let chain_config = self.get_chain_config(chain_id);
        if chain_config.is_some() {
            for peer_chain_id in chain_config.unwrap().peer_chain_ids() {
                let peer_chain_config = self.get_chain_config(peer_chain_id);
                if peer_chain_config.is_some() {
                    peer_chain_configs.push(peer_chain_config.unwrap().clone())
                }
            }
        }

        peer_chain_configs
    }

    pub fn get_asset_symbols(
        &self,
        chain_id: u32,
        peer_chain_id: u32,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        match self.get_chain_config(chain_id) {
            None => { Ok(vec![]) }
            Some(config) => {
                config.get_asset_symbols(peer_chain_id)
            }
        }
    }

    pub fn get_bridges(
        &self,
        chain_id: u32,
        peer_chain_id: u32,
        asset_symbol: &str,
    ) -> Result<Vec<BridgeConfigType>, Box<dyn Error>> {
        let mut bridges: Vec<BridgeConfigType> = Vec::new();
        let chain_config = self.get_chain_config(chain_id);
        if chain_config.is_some() {
            let bridge_types = chain_config.unwrap().get_bridges(peer_chain_id, asset_symbol)?;
            for bridge_type in bridge_types {
                let bridge_config = self.get_bridge_config(bridge_type);
                if bridge_config.is_some() {
                    bridges.push(bridge_config.unwrap().clone());
                }
            }
        }

        Ok(bridges)
    }

    pub fn get_deposit_contract_config(
        &self,
        chain_id: u32,
        peer_chain_id: u32,
        asset_symbol: &str,
        bridge_type: BridgeType,
    ) -> Result<Option<DepositContractConfig>, Box<dyn Error>> {
        match self.get_chain_config(chain_id) {
            None => { Ok(None) }
            Some(config) => {
                config.get_deposit_contract(peer_chain_id, asset_symbol, bridge_type)
            }
        }
    }

    pub fn get_deposit_contract_config_by_address(
        &self,
        chain_id: u32,
        address: String,
    ) -> Option<&DepositContractConfig> {
        let chain_config = self.get_chain_config(chain_id).clone();
        match chain_config {
            Some(config) => {
                config.get_deposit_contract_by_address(address)
            }
            None => { None }
        }
    }

    pub fn get_pool_contract_config(
        &self,
        chain_id: u32,
        asset_symbol: &str,
        bridge_type: BridgeType,
        version: u32,
    ) -> Option<&PoolContractConfig> {
        match self.get_chain_config(chain_id) {
            None => { None }
            Some(config) => {
                config.get_pool_contract(asset_symbol, bridge_type, version)
            }
        }
    }

    pub fn get_pool_contract_configs(
        &self,
        chain_id: u32,
        asset_symbol: &str,
        bridge_type: BridgeType,
    ) -> Vec<PoolContractConfig> {
        match self.get_chain_config(chain_id) {
            None => { vec![] }
            Some(config) => {
                config.get_pool_contracts(asset_symbol, bridge_type)
            }
        }
    }

    pub fn get_pool_contract_config_by_address(&self, chain_id: u32, address: &str) -> Option<&PoolContractConfig> {
        let chain_config = self.get_chain_config(chain_id);
        if chain_config.is_some() {
            return chain_config.unwrap().get_pool_contract_by_address(address);
        }

        None
    }

    pub fn get_bridge_config(&self, bridge_type: BridgeType) -> Option<&BridgeConfigType> {
        self.bridge_configs.get(&bridge_type)
    }

    pub fn get_default_circuit_config(&self, circuit_type: CircuitType) -> Option<&CircuitConfig> {
        self.default_circuit_configs.get(&circuit_type)
    }

    pub fn get_circuit_config_by_name(&self, name: &str) -> Option<&CircuitConfig> {
        self.circuit_configs_by_name.get(name)
    }

    pub fn get_transaction_url(&self, chain_id: u32, transaction_hash: &str) -> Option<String> {
        let chain_config = self.get_chain_config(chain_id);
        if chain_config.is_some() {
            return Some(chain_config.unwrap().get_transaction_url(transaction_hash));
        }

        None
    }

    pub fn mutate(&self, data: Option<RawMystikoConfig>) -> Result<Self, ValidationError> {
        match data {
            None => {
                MystikoConfig::new(self.data().clone())
            }
            Some(config) => {
                MystikoConfig::new(config)
            }
        }
    }

    #[flame]
    fn init_circuit_configs(&mut self) -> Result<(), ValidationError> {
        let mut default_circuit_configs: HashMap<CircuitType, CircuitConfig> = HashMap::new();
        let mut circuit_config_by_names: HashMap<String, CircuitConfig> = HashMap::new();

        let circuits = &self.base.data.circuits;
        for raw in circuits {
            let circuit_config = CircuitConfig::new(raw.clone());
            if raw.is_default {
                let check_result = check(
                    !default_circuit_configs.contains_key(circuit_config.circuit_type()),
                    format!(
                        "duplicate default circuit type={:?} definition", circuit_config.circuit_type()
                    ).as_str(),
                );
                if check_result.is_err() {
                    return Err(ValidationError::new(
                        vec![
                            check_result.unwrap_err().to_string()
                        ]
                    ));
                }
                default_circuit_configs.insert(*circuit_config.circuit_type(), circuit_config.clone());
            }
            circuit_config_by_names.insert(circuit_config.name().clone(), circuit_config.clone());
        }
        let mut has_pool_contracts = false;
        for chain in &self.data().chains {
            if !chain.pool_contracts.is_empty() {
                has_pool_contracts = true;
                break;
            }
        }

        if has_pool_contracts {
            for circuit_type in CircuitType::iter() {
                let check_result = check(
                    default_circuit_configs.contains_key(&circuit_type),
                    format!(
                        "missing definition of default circuit type={:?}", circuit_type
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

        self.default_circuit_configs = default_circuit_configs;
        self.circuit_configs_by_name = circuit_config_by_names;

        Ok(())
    }

    #[flame]
    fn init_bridge_configs(&mut self) {
        let mut bridge_configs: HashMap<BridgeType, BridgeConfigType> = HashMap::new();
        for bridge in &self.base.data.bridges {
            match bridge {
                RawBridgeConfigType::Axelar(config) => {
                    bridge_configs.insert(
                        config.bridge_type.clone(),
                        BridgeConfigType::AxelarBridgeConfig(AxelarBridgeConfig::new(config.clone())),
                    );
                }
                RawBridgeConfigType::Celer(config) => {
                    bridge_configs.insert(
                        config.bridge_type.clone(),
                        BridgeConfigType::CelerBridgeConfig(CelerBridgeConfig::new(config.clone())),
                    );
                }
                RawBridgeConfigType::Poly(config) => {
                    bridge_configs.insert(
                        config.bridge_type.clone(),
                        BridgeConfigType::PolyBridgeConfig(PolyBridgeConfig::new(config.clone())),
                    );
                }
                RawBridgeConfigType::LayerZero(config) => {
                    bridge_configs.insert(
                        config.bridge_type.clone(),
                        BridgeConfigType::LayerZeroBridgeConfig(LayerZeroBridgeConfig::new(config.clone())),
                    );
                }
                RawBridgeConfigType::Tbridge(config) => {
                    bridge_configs.insert(
                        config.bridge_type.clone(),
                        BridgeConfigType::TBridgeConfig(TBridgeConfig::new(config.clone())),
                    );
                }
            }
        }

        self.bridge_configs = bridge_configs;
    }

    #[flame]
    fn init_chain_configs(
        &mut self,
    ) -> Result<(), ValidationError> {
        let mut chain_configs: HashMap<u32, ChainConfig> = HashMap::new();

        let mut aux_data_chain_configs: HashMap<u32, ChainConfig> = HashMap::new();
        for raw in &self.data().chains {
            let chain_config = ChainConfig::new(
                raw.clone(),
                // only use for get deposit contract, circuit configs is none
                Some(AuxData::default()),
            )?;
            aux_data_chain_configs.insert(
                raw.chain_id,
                chain_config,
            );
        }

        for raw in &self.base.data.chains {
            chain_configs.insert(
                raw.chain_id,
                ChainConfig::new(
                    raw.clone(),
                    Some(
                        AuxData::new(
                            self.default_circuit_configs.clone(),
                            self.circuit_configs_by_name.clone(),
                            Some(aux_data_chain_configs.clone()),
                        )
                    ),
                )?,
            );
        }
        self.chain_configs = chain_configs;
        Ok(())
    }

    #[flame]
    fn init_indexer_config(&mut self) {
        match &self.data().indexer {
            Some(config) => {
                self.indexer_config = Some(IndexerConfig::new(config.clone()));
            }
            None => {
                self.indexer_config = None;
            }
        }
    }

    #[flame]
    fn validate(&self) -> Result<(), ValidationError> {
        for (_, chain_config) in &self.chain_configs {
            for deposit_contract_config in chain_config.deposit_contracts_with_disabled() {
                if deposit_contract_config.bridge_type() != BridgeType::Loop {
                    let check_result = check(
                        self.bridge_configs.contains_key(&deposit_contract_config.bridge_type()),
                        format!(
                            "bridge type = {:?} definition does not exist", deposit_contract_config.bridge_type()
                        ).as_str(),
                    );
                    if check_result.is_err() {
                        return Err(ValidationError::new(
                            vec![
                                check_result.unwrap_err().to_string()
                            ]
                        ));
                    }
                    if deposit_contract_config.peer_chain_id().is_some() &&
                        deposit_contract_config.peer_contract_address().is_some() {
                        let peer_chain_id = deposit_contract_config.peer_chain_id().unwrap();
                        let peer_contract_address = deposit_contract_config.peer_contract_address().clone().unwrap();
                        let peer_chain_config = self.chain_configs.get(&peer_chain_id);
                        if peer_chain_config.is_none() {
                            return Err(ValidationError::new(
                                vec![
                                    format!(
                                        "no corresponding peer chain id={} definition for deposit contract {} peer chain configuration",
                                        peer_chain_id, deposit_contract_config.address()
                                    )
                                ]
                            ));
                        }
                        let peer_chain_config = peer_chain_config.unwrap();
                        let peer_deposit_contract_config =
                            peer_chain_config.get_deposit_contract_by_address(peer_contract_address.clone());
                        if peer_deposit_contract_config.is_none() {
                            return Err(ValidationError::new(
                                vec![
                                    format!(
                                        "no corresponding peer deposit contract chain id={} and address={}  \
                                        definition for deposit contract address={} peer chain configuration",
                                        peer_chain_id, peer_contract_address, deposit_contract_config.address()
                                    )
                                ]
                            ));
                        }
                        let peer_deposit_contract_config = peer_deposit_contract_config.unwrap();
                        let check_result = check(
                            peer_deposit_contract_config.bridge_type() == deposit_contract_config.bridge_type(),
                            format!(
                                "bridge type mismatch for chain id={} address={} vs chain id={} and address={}",
                                peer_chain_id,
                                peer_contract_address,
                                chain_config.chain_id(),
                                deposit_contract_config.address()
                            ).as_str(),
                        );
                        if check_result.is_err() {
                            return Err(ValidationError::new(
                                vec![
                                    check_result.unwrap_err().to_string()
                                ]
                            ));
                        }
                        let check_result = check(
                            &peer_deposit_contract_config.peer_chain_id().unwrap() == chain_config.chain_id() &&
                                peer_deposit_contract_config.peer_contract_address().clone().unwrap() == deposit_contract_config.address(),
                            format!(
                                "chain id={} and address={} does not match chain id={} and address={} configured",
                                peer_deposit_contract_config.peer_chain_id().unwrap(),
                                peer_deposit_contract_config.peer_contract_address().clone().unwrap(),
                                chain_config.chain_id(),
                                deposit_contract_config.address(),
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
            }
        }
        Ok(())
    }

    #[flame]
    pub async fn create_from_file(json_file: String) -> Result<MystikoConfig, ValidationError> {
        match RawConfig::create_from_file::<RawMystikoConfig>(json_file.as_str()).await {
            Ok(raw_config) => { MystikoConfig::new(raw_config) }
            Err(err) => { Err(err) }
        }
    }

    pub async fn create_from_raw(raw: RawMystikoConfig) -> Result<MystikoConfig, ValidationError> {
        match raw.validation() {
            Ok(_) => { MystikoConfig::new(raw) }
            Err(err) => { Err(err) }
        }
    }

    #[flame]
    pub async fn create_default_testnet_config() -> Result<MystikoConfig, ValidationError> {
        MystikoConfig::create_from_file(
            "src/json/client/default/testnet.json".to_string()
        ).await
    }

    pub async fn create_default_mainnet_config() -> Result<MystikoConfig, ValidationError> {
        MystikoConfig::create_from_file(
            "src/json/client/default/mainnet.json".to_string()
        ).await
    }
}
