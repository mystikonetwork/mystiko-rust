use std::collections::HashMap;
use strum::IntoEnumIterator;
use mystiko_utils::check::check;
use crate::common::{BridgeType, CircuitType};
use crate::raw::base::{RawConfig, Validator};
use crate::raw::contract::base::RawContractConfigTrait;
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
    pub fn new(data: RawMystikoConfig) -> Self {
        let base = BaseConfig::new(data, None);
        let (
            default_circuit_configs,
            circuit_configs_by_name
        ) = MystikoConfig::init_circuit_configs(&base);
        let mut config = Self {
            base: base.clone(),
            default_circuit_configs: default_circuit_configs.clone(),
            circuit_configs_by_name: circuit_configs_by_name.clone(),
            bridge_configs: MystikoConfig::init_bridge_configs(&base),
            chain_configs: Default::default(),
            indexer_config: MystikoConfig::init_indexer_config(&base),
        };
        config.init_chain_configs(default_circuit_configs, circuit_configs_by_name);
        config.validate();
        config
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

    pub fn get_asset_symbols(&self, chain_id: u32, peer_chain_id: u32) -> Vec<String> {
        match self.get_chain_config(chain_id) {
            None => { vec![] }
            Some(config) => {
                config.get_asset_symbols(peer_chain_id)
            }
        }
    }

    pub fn get_bridges(&self, chain_id: u32, peer_chain_id: u32, asset_symbol: &str) -> Vec<BridgeConfigType> {
        let mut bridges: Vec<BridgeConfigType> = Vec::new();
        let chain_config = self.get_chain_config(chain_id);
        if chain_config.is_some() {
            for bridge_type in chain_config.unwrap().get_bridges(peer_chain_id, asset_symbol) {
                let bridge_config = self.get_bridge_config(bridge_type);
                if bridge_config.is_some() {
                    bridges.push(bridge_config.unwrap().clone());
                }
            }
        }

        bridges
    }

    pub fn get_deposit_contract_config(
        &self,
        chain_id: u32,
        peer_chain_id: u32,
        asset_symbol: &str,
        bridge_type: BridgeType,
    ) -> Option<DepositContractConfig> {
        match self.get_chain_config(chain_id) {
            None => { None }
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
        println!("{}", chain_id);
        for (x, _) in &self.chain_configs {
            println!("{}", x)
        }
        let chain_config = self.get_chain_config(chain_id).clone();
        match chain_config {
            Some(config) => {
                // TODO problem
                println!("chain config found~");
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

    pub fn mutate(&self, data: Option<RawMystikoConfig>) -> Self {
        match data {
            None => {
                MystikoConfig::new(self.base.data.clone())
            }
            Some(config) => {
                MystikoConfig::new(config)
            }
        }
    }

    fn init_circuit_configs(base: &BaseConfig<RawMystikoConfig>) -> (HashMap<CircuitType, CircuitConfig>, HashMap<String, CircuitConfig>) {
        let mut default_circuit_configs: HashMap<CircuitType, CircuitConfig> = HashMap::new();
        let mut circuit_config_by_names: HashMap<String, CircuitConfig> = HashMap::new();

        let circuits = &base.data.circuits;
        for raw in circuits {
            let circuit_config = CircuitConfig::new(raw.clone());
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
        for chain in &base.data.chains {
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

    fn init_bridge_configs(base: &BaseConfig<RawMystikoConfig>) -> HashMap<BridgeType, BridgeConfigType> {
        let mut bridge_configs: HashMap<BridgeType, BridgeConfigType> = HashMap::new();
        for bridge in &base.data.bridges {
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

        bridge_configs
    }

    fn init_chain_configs(
        &mut self,
        default_circuit_configs: HashMap<CircuitType, CircuitConfig>,
        circuit_configs_by_name: HashMap<String, CircuitConfig>,
    ) {
        let mut chain_configs: HashMap<u32, ChainConfig> = HashMap::new();
        for raw in &self.base.data.chains {
            chain_configs.insert(
                raw.chain_id,
                ChainConfig::new(
                    raw.clone(),
                    Some(
                        AuxData::new(
                            default_circuit_configs.clone(),
                            circuit_configs_by_name.clone(),
                            Some(self.clone()),
                        )
                    ),
                ),
            );
        }
        self.chain_configs = chain_configs;
    }

    fn init_indexer_config(base: &BaseConfig<RawMystikoConfig>) -> Option<IndexerConfig> {
        match &base.data.indexer {
            Some(config) => {
                Some(IndexerConfig::new(config.clone()))
            }
            None => {
                None
            }
        }
    }

    fn validate(&self) {
        for (_, chain_config) in &self.chain_configs {
            for deposit_contract_config in chain_config.deposit_contracts_with_disabled() {
                if deposit_contract_config.bridge_type() != BridgeType::Loop {
                    check(
                        self.bridge_configs.contains_key(&deposit_contract_config.bridge_type()),
                        format!(
                            "bridge type = {:?} definition does not exist", deposit_contract_config.bridge_type()
                        ).as_str(),
                    );
                    if deposit_contract_config.peer_chain_id().is_some() &&
                        deposit_contract_config.peer_contract_address().is_some() {
                        let peer_chain_id = deposit_contract_config.peer_chain_id().unwrap();
                        let peer_contract_address = deposit_contract_config.peer_contract_address().clone().unwrap();
                        let peer_chain_config = self.chain_configs.get(&peer_chain_id);
                        if peer_chain_config.is_none() {
                            panic!(
                                "no corresponding peer chain id={} definition for deposit contract {} peer chain configuration",
                                peer_chain_id, deposit_contract_config.base.base.data.address()
                            );
                        }
                        let peer_chain_config = peer_chain_config.unwrap();
                        let peer_deposit_contract_config =
                            peer_chain_config.get_deposit_contract_by_address(peer_contract_address.clone());
                        if peer_deposit_contract_config.is_none() {
                            panic!(
                                "no corresponding peer deposit contract chain id={} and address={}  \
                                definition for deposit contract address={} peer chain configuration",
                                peer_chain_id, peer_contract_address, deposit_contract_config.base.base.data.address()
                            );
                        }
                        let peer_deposit_contract_config = peer_deposit_contract_config.unwrap();
                        check(
                            peer_deposit_contract_config.bridge_type() == deposit_contract_config.bridge_type(),
                            format!(
                                "bridge type mismatch for chain id={} address={} vs chain id={} and address={}",
                                peer_chain_id,
                                peer_contract_address,
                                chain_config.chain_id(),
                                deposit_contract_config.base.base.data.address()
                            ).as_str(),
                        );
                        check(
                            &peer_deposit_contract_config.peer_chain_id().unwrap() == chain_config.chain_id() &&
                                peer_deposit_contract_config.peer_contract_address().clone().unwrap() == deposit_contract_config.base.base.data.address(),
                            format!(
                                "chain id={} and address={} does not match chain id={} and address={} configured",
                                peer_deposit_contract_config.peer_chain_id().unwrap(),
                                peer_deposit_contract_config.peer_contract_address().clone().unwrap(),
                                chain_config.chain_id(),
                                deposit_contract_config.base.base.data.address(),
                            ).as_str(),
                        );
                    }
                }
            }
        }
    }

    pub async fn create_from_file(json_file: String) -> MystikoConfig {
        let raw_config =
            RawConfig::create_from_file::<RawMystikoConfig>(json_file.as_str()).await;
        MystikoConfig::new(raw_config)
    }

    pub async fn create_from_raw(raw: RawMystikoConfig) -> MystikoConfig {
        raw.validation();
        MystikoConfig::new(raw)
    }

    // TODO implement
    pub async fn create_default_testnet_config() {}

    // TODO implement
    pub async fn create_default_mainnet_config() {}
}

#[cfg(test)]
mod tests {
    use crate::common::{BridgeType, CircuitType};
    use crate::raw::base::RawConfig;
    use crate::raw::bridge::celer::RawCelerBridgeConfig;
    use crate::raw::bridge::poly::RawPolyBridgeConfig;
    use crate::raw::bridge::tbridge::RawTBridgeConfig;
    use crate::raw::chain::RawChainConfig;
    use crate::raw::circuit::RawCircuitConfig;
    use crate::raw::mystiko::{RawBridgeConfigType, RawMystikoConfig};
    use crate::wrapper::mystiko::{BridgeConfigType, MystikoConfig};

    async fn raw_config() -> RawMystikoConfig {
        RawConfig::create_from_file::<RawMystikoConfig>(
            "src/tests/files/mystiko.valid01.json"
        ).await
    }

    async fn config() -> MystikoConfig {
        MystikoConfig::create_from_raw(
            raw_config().await
        ).await
    }

    #[tokio::test]
    async fn test_equality() {
        let raw_config = raw_config().await;
        let config = config().await;
        assert_eq!(config.version(), raw_config.version);

        let a =
            config.chains().iter().map(|conf| conf.base.copy_data()).collect::<Vec<RawChainConfig>>();
        let b = raw_config.chains;
        assert_eq!(a.len(), b.len());
        for x in a {
            assert_eq!(b.contains(&x), true);
        }

        let a =
            config.circuits().iter().map(|conf| conf.base.copy_data()).collect::<Vec<RawCircuitConfig>>();
        let b = raw_config.circuits;
        assert_eq!(a.len(), b.len());
        for x in a {
            assert_eq!(b.contains(&x), true);
        }

        let mut a: Vec<RawBridgeConfigType> = Vec::new();
        let b = raw_config.bridges;
        let bridges = config.bridges();
        for bridge in bridges {
            match bridge {
                BridgeConfigType::AxelarBridgeConfig(v) => {
                    a.push(RawBridgeConfigType::Axelar(v.base.base.copy_data()));
                }
                BridgeConfigType::CelerBridgeConfig(v) => {
                    a.push(RawBridgeConfigType::Celer(v.base.base.copy_data()));
                }
                BridgeConfigType::PolyBridgeConfig(v) => {
                    a.push(RawBridgeConfigType::Poly(v.base.base.copy_data()));
                }
                BridgeConfigType::LayerZeroBridgeConfig(v) => {
                    a.push(RawBridgeConfigType::LayerZero(v.base.base.copy_data()));
                }
                BridgeConfigType::TBridgeConfig(v) => {
                    a.push(RawBridgeConfigType::Tbridge(v.base.base.copy_data()));
                }
            }
        }
        assert_eq!(a.len(), b.len());
        for x in a {
            assert_eq!(b.contains(&x), true);
        }

        assert_eq!(config.indexer().is_none(), true);
    }

    #[tokio::test]
    async fn test_get_chain_config() {
        let raw_config = raw_config().await;
        let config = config().await;
        assert_eq!(
            config.get_chain_config(3).unwrap().base.copy_data(),
            raw_config.chains[0]
        );
        assert_eq!(
            config.get_chain_config(97).unwrap().base.copy_data(),
            raw_config.chains[1]
        );
        assert_eq!(
            config.get_chain_config(1024).is_none(),
            true
        );
    }

    #[tokio::test]
    async fn test_get_peer_chain_configs() {
        let config = config().await;
        let mut a =
            config.get_peer_chain_configs(3).iter().map(
                |conf| conf.chain_id().clone()
            ).collect::<Vec<u32>>();
        a.sort();
        assert_eq!(a, vec![3, 97]);
        assert_eq!(
            config.get_peer_chain_configs(97).iter().map(
                |conf| conf.chain_id().clone()
            ).collect::<Vec<u32>>(),
            vec![3]
        );
        assert_eq!(config.get_peer_chain_configs(1024).len(), 0);
    }

    #[tokio::test]
    async fn test_get_asset_symbols() {
        let config = config().await;
        assert_eq!(
            config.get_asset_symbols(3, 97),
            vec!["MTT"]
        );
        assert_eq!(
            config.get_asset_symbols(3, 3),
            vec!["ETH"]
        );
        assert_eq!(
            config.get_asset_symbols(97, 3),
            vec!["MTT"]
        );
        assert_eq!(
            config.get_asset_symbols(97, 97).len(),
            0
        );
        assert_eq!(
            config.get_asset_symbols(3, 1024).len(),
            0
        );
        assert_eq!(
            config.get_asset_symbols(1024, 97).len(),
            0
        );
    }

    #[tokio::test]
    async fn test_get_bridges() {
        let config = config().await;
        let a =
            config.get_bridges(3, 97, "MTT").iter()
                .map(|conf| match conf {
                    BridgeConfigType::AxelarBridgeConfig(v) => { v.base.bridge_type().clone() }
                    BridgeConfigType::CelerBridgeConfig(v) => { v.base.bridge_type().clone() }
                    BridgeConfigType::PolyBridgeConfig(v) => { v.base.bridge_type().clone() }
                    BridgeConfigType::LayerZeroBridgeConfig(v) => { v.base.bridge_type().clone() }
                    BridgeConfigType::TBridgeConfig(v) => { v.base.bridge_type().clone() }
                }).collect::<Vec<BridgeType>>();
        let b = vec![
            BridgeType::Axelar,
            BridgeType::Celer,
            BridgeType::LayerZero,
            BridgeType::Tbridge,
        ];
        assert_eq!(a.len(), b.len());
        for x in a {
            assert_eq!(b.contains(&x), true);
        }
        assert_eq!(
            config.get_bridges(1024, 97, "MTT").len(),
            0
        );
        assert_eq!(
            config.get_bridges(3, 1024, "MTT").len(),
            0
        );
        assert_eq!(
            config.get_bridges(3, 97, "ETH").len(),
            0
        );
    }

    #[tokio::test]
    async fn test_get_deposit_contract_config() {
        let config = config().await;
        assert_eq!(
            &config.get_deposit_contract_config(
                3,
                97,
                "MTT",
                BridgeType::Celer,
            ).unwrap(),
            config.get_deposit_contract_config_by_address(
                3,
                "0xe6394a06905d83B19Dbd51804Ca84677a2054FA6".to_string(),
            ).unwrap()
        );
        assert_eq!(
            &config.get_deposit_contract_config(
                3,
                97,
                "MTT",
                BridgeType::Tbridge,
            ).unwrap(),
            config.get_deposit_contract_config_by_address(
                3,
                "0xbF5605f5Ed6d18ed957cBA80dbA8838dFcb9A69f".to_string(),
            ).unwrap()
        );
        assert_eq!(
            &config.get_deposit_contract_config(
                3,
                3,
                "ETH",
                BridgeType::Loop,
            ).unwrap(),
            config.get_deposit_contract_config_by_address(
                3,
                "0x390d485f4d43212d4ae8cdd967a711514ed5a54f".to_string(),
            ).unwrap()
        );
        assert_eq!(
            &config.get_deposit_contract_config(
                97,
                3,
                "MTT",
                BridgeType::Celer,
            ).unwrap(),
            config.get_deposit_contract_config_by_address(
                97,
                "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".to_string(),
            ).unwrap()
        );
        assert_eq!(
            &config.get_deposit_contract_config(
                97,
                3,
                "MTT",
                BridgeType::Tbridge,
            ).unwrap(),
            config.get_deposit_contract_config_by_address(
                97,
                "0x9C33eaCc2F50E39940D3AfaF2c7B8246B681A374".to_string(),
            ).unwrap()
        );
        assert_eq!(
            config.get_deposit_contract_config(
                1024,
                3,
                "MTT",
                BridgeType::Tbridge,
            ).is_none(),
            true
        );
        assert_eq!(
            config.get_deposit_contract_config_by_address(
                1024, "0x9C33eaCc2F50E39940D3AfaF2c7B8246B681A374".to_string(),
            ).is_none(),
            true
        );
        assert_eq!(
            config.get_deposit_contract_config_by_address(
                3, "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67".to_string(),
            ).is_some(),
            true
        );
        assert_eq!(
            config.get_deposit_contract_config_by_address(
                97, "0xd791049D0a154bC7860804e1A18ACD148Eb0afD9".to_string(),
            ).is_some(),
            true
        );
        let deposit_contract_config = config.get_deposit_contract_config_by_address(
            3,
            "0x961f315a836542e603a3df2e0dd9d4ecd06ebc67".to_string(),
        ).unwrap();
        let peer_deposit_contract_config =
            deposit_contract_config.peer_contract().unwrap();
        println!("{:?}", peer_deposit_contract_config);
        // println!("{:?}", deposit_contract_config.unwrap().peer_contract());
        // let a = config.get_deposit_contract_config_by_address(
        //     97,
        //     "0xd791049D0a154bC7860804e1A18ACD148Eb0afD9".to_string(),
        // );
        // println!("{:?}", a);
        // assert_eq!(
        //     &deposit_contract_config.unwrap().peer_contract().unwrap(),
        //     config.get_deposit_contract_config_by_address(
        //         97,
        //         "0xd791049D0a154bC7860804e1A18ACD148Eb0afD9".to_string()
        //     ).unwrap()
        // );
    }

    #[tokio::test]
    async fn test_get_pool_contract_config() {
        let config = config().await;
        assert_eq!(
            config.get_pool_contract_config(
                3,
                "MTT",
                BridgeType::Celer,
                2,
            ),
            config.get_pool_contract_config_by_address(
                3,
                "0x20Eb345870059E688c59e89523442ade33C7c813",
            )
        );
        assert_eq!(
            config.get_pool_contract_config(
                3,
                "MTT",
                BridgeType::Tbridge,
                2,
            ),
            config.get_pool_contract_config_by_address(
                3,
                "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d",
            )
        );
        assert_eq!(
            config.get_pool_contract_config(
                3,
                "ETH",
                BridgeType::Loop,
                2,
            ),
            config.get_pool_contract_config_by_address(
                3,
                "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
            )
        );
        assert_eq!(
            config.get_pool_contract_config(
                97,
                "MTT",
                BridgeType::Celer,
                2,
            ),
            config.get_pool_contract_config_by_address(
                97,
                "0x6B8a4ea37C72F1992626eb9bD48d4aA6aa077c47",
            )
        );
        assert_eq!(
            config.get_pool_contract_config(
                97,
                "MTT",
                BridgeType::Tbridge,
                2,
            ),
            config.get_pool_contract_config_by_address(
                97,
                "0xBe2C9c8a00951662dF3a978b25F448968F0595AE",
            )
        );
        assert_eq!(
            config.get_pool_contract_config(
                1024,
                "MTT",
                BridgeType::Celer,
                2,
            ).is_none(),
            true
        );
        assert_eq!(
            config.get_pool_contract_config_by_address(
                1024,
                "0xBe2C9c8a00951662dF3a978b25F448968F0595AE",
            ).is_none(),
            true
        );
    }

    #[tokio::test]
    async fn test_get_pool_contract_configs() {
        let config = config().await;
        assert_eq!(
            config.get_pool_contract_configs(
                3,
                "MTT",
                BridgeType::Celer,
            ),
            vec![
                config.get_pool_contract_config_by_address(
                    3,
                    "0x20Eb345870059E688c59e89523442ade33C7c813",
                ).unwrap().clone()
            ]
        );
        // TODO episodic error fix
        // assert_eq!(
        //     config.get_pool_contract_configs(
        //         3,
        //         "MTT",
        //         BridgeType::Tbridge,
        //     ),
        //     vec![
        //         config.get_pool_contract_config_by_address(
        //             3,
        //             "0x9b42ec45f6fb6c7d252c66741e960585888de7b6",
        //         ).unwrap().clone(),
        //         config.get_pool_contract_config_by_address(
        //             3,
        //             "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d",
        //         ).unwrap().clone(),
        //     ]
        // );
        assert_eq!(
            config.get_pool_contract_configs(
                3,
                "ETH",
                BridgeType::Loop,
            ),
            vec![
                config.get_pool_contract_config_by_address(
                    3,
                    "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
                ).unwrap().clone()
            ]
        );
        assert_eq!(
            config.get_pool_contract_configs(
                97,
                "MTT",
                BridgeType::Celer,
            ),
            vec![
                config.get_pool_contract_config_by_address(
                    97,
                    "0x6B8a4ea37C72F1992626eb9bD48d4aA6aa077c47",
                ).unwrap().clone()
            ]
        );
        assert_eq!(
            config.get_pool_contract_configs(
                97,
                "MTT",
                BridgeType::Tbridge,
            ),
            vec![
                config.get_pool_contract_config_by_address(
                    97,
                    "0xBe2C9c8a00951662dF3a978b25F448968F0595AE",
                ).unwrap().clone()
            ]
        );
        assert_eq!(
            config.get_pool_contract_configs(
                1024,
                "MTT",
                BridgeType::Celer,
            ).len(),
            0
        );
    }

    #[tokio::test]
    async fn test_get_bridge_config() {
        let config = config().await;
        let raw_config = raw_config().await;

        let poly_bridge_config =
            config.get_bridge_config(BridgeType::Poly).unwrap();
        let copy_data = match poly_bridge_config {
            BridgeConfigType::PolyBridgeConfig(conf) => {
                conf.base.base.copy_data()
            }
            _ => { RawPolyBridgeConfig::default() }
        };
        let conf = match raw_config.bridges.get(0).unwrap() {
            RawBridgeConfigType::Poly(conf) => { conf.clone() }
            _ => { RawPolyBridgeConfig::default() }
        };
        assert_eq!(copy_data, conf);

        let tbridge_config =
            config.get_bridge_config(BridgeType::Tbridge).unwrap();
        let copy_data = match tbridge_config {
            BridgeConfigType::TBridgeConfig(conf) => {
                conf.base.base.copy_data()
            }
            _ => { RawTBridgeConfig::default() }
        };
        let conf = match raw_config.bridges.get(1).unwrap() {
            RawBridgeConfigType::Tbridge(conf) => {
                conf.clone()
            }
            _ => { RawTBridgeConfig::default() }
        };
        assert_eq!(copy_data, conf);

        let celer_config =
            config.get_bridge_config(BridgeType::Celer).unwrap();
        let copy_data = match celer_config {
            BridgeConfigType::CelerBridgeConfig(conf) => {
                conf.base.base.copy_data()
            }
            _ => { RawCelerBridgeConfig::default() }
        };
        let conf = match raw_config.bridges.get(2).unwrap() {
            RawBridgeConfigType::Celer(conf) => {
                conf.clone()
            }
            _ => { RawCelerBridgeConfig::default() }
        };
        assert_eq!(copy_data, conf);
    }

    #[tokio::test]
    async fn test_get_default_circuit_config() {
        let config = config().await;
        let name =
            config.get_default_circuit_config(CircuitType::Rollup1).unwrap().name();
        assert_eq!(name, "zokrates-1.0-rollup1");
    }

    #[tokio::test]
    async fn test_get_circuit_config_by_name() {
        let config = config().await;
        assert_eq!(
            config.get_circuit_config_by_name(
                "zokrates-2.0-rollup1"
            ).unwrap().is_default(),
            false
        );
        assert_eq!(
            config.get_circuit_config_by_name(
                "zokrates-4.0-rollup1"
            ).is_none(),
            true
        );
    }

    #[tokio::test]
    async fn test_create_from_file() {
        let new_config =
            MystikoConfig::create_from_file(
                "src/tests/files/mystiko.valid01.json".to_string()
            ).await;
        let config = config().await;
        assert_eq!(new_config.base.to_json_string(), config.base.to_json_string());
    }

    #[tokio::test]
    #[should_panic(expected = "duplicate default circuit type=TRANSACTION2x2 definition")]
    async fn test_duplicate_circuit_type_default() {
        let mut raw_config = raw_config().await;
        raw_config.circuits.push(
            RawConfig::create_from_object::<RawCircuitConfig>(
                RawCircuitConfig::new(
                    "zokrates-2.0-transaction2x2".to_string(),
                    CircuitType::TRANSACTION2x2,
                    true,
                    vec!["./Transaction2x2.program.gz".to_string()],
                    vec!["./Transaction2x2.abi.json".to_string()],
                    vec!["./Transaction2x2.pkey.gz".to_string()],
                    vec!["./Transaction2x2.vkey.gz".to_string()],
                )
            ).await
        );
        MystikoConfig::create_from_raw(raw_config).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn test_duplicate_circuit_name() {
        let mut raw_config = raw_config().await;
        raw_config.circuits.push(
            RawConfig::create_from_object::<RawCircuitConfig>(
                RawCircuitConfig::new(
                    "zokrates-2.0-transaction2x2".to_string(),
                    CircuitType::TRANSACTION2x2,
                    true,
                    vec!["./Transaction2x2.program.gz".to_string()],
                    vec!["./Transaction2x2.abi.json".to_string()],
                    vec!["./Transaction2x2.pkey.gz".to_string()],
                    vec!["./Transaction2x2.vkey.gz".to_string()],
                )
            ).await
        );
        raw_config.circuits.push(
            RawConfig::create_from_object::<RawCircuitConfig>(
                RawCircuitConfig::new(
                    "zokrates-1.0-transaction2x2".to_string(),
                    CircuitType::TRANSACTION2x2,
                    false,
                    vec!["./Transaction2x2.program.gz".to_string()],
                    vec!["./Transaction2x2.abi.json".to_string()],
                    vec!["./Transaction2x2.pkey.gz".to_string()],
                    vec!["./Transaction2x2.vkey.gz".to_string()],
                )
            ).await
        );
        MystikoConfig::create_from_raw(raw_config).await;
    }
}
