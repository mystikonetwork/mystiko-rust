use std::collections::HashMap;
use std::str::FromStr;
use num_bigint::BigInt;
use mystiko_utils::check::check;
use mystiko_utils::convert::from_decimals;
use crate::common::{AssetType, BridgeType};
use crate::raw::contract::base::RawContractConfigTrait;
use crate::raw::contract::deposit::RawDepositContractConfig;
use crate::wrapper::asset::{AssetConfig, MAIN_ASSET_ADDRESS};
use crate::wrapper::circuit::CircuitConfig;
use crate::wrapper::contract::base::ContractConfig;
use crate::wrapper::contract::pool::PoolContractConfig;
use crate::wrapper::mystiko::MystikoConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct AuxData {
    pool_contract_configs: HashMap<String, PoolContractConfig>,
    main_asset_config: AssetConfig,
    asset_configs: HashMap<String, AssetConfig>,
    mystiko_config: Option<MystikoConfig>,
}

impl AuxData {
    pub fn new(
        pool_contract_configs: HashMap<String, PoolContractConfig>,
        main_asset_config: AssetConfig,
        asset_configs: HashMap<String, AssetConfig>,
        mystiko_config: Option<MystikoConfig>,
    ) -> Self {
        Self {
            pool_contract_configs,
            main_asset_config,
            asset_configs,
            mystiko_config,
        }
    }

    fn pool_contract_getter(&self, address: &str) -> Option<&PoolContractConfig> {
        self.pool_contract_configs.get(address)
    }

    fn deposit_contract_getter(&self, chain_id: u32, address: String) -> Option<&DepositContractConfig> {
        match &self.mystiko_config {
            None => { None }
            Some(conf) => {
                return conf.get_deposit_contract_config_by_address(chain_id, address);
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DepositContractConfig {
    pub base: ContractConfig<RawDepositContractConfig, AuxData>,
    bridge_fee_asset_config: Option<AssetConfig>,
    executor_fee_asset_config: Option<AssetConfig>,
}

impl DepositContractConfig {
    pub fn new(data: RawDepositContractConfig, aux_data: Option<AuxData>) -> Self {
        let contract_config = ContractConfig::new(data, aux_data);
        let bridge_fee_asset_config = DepositContractConfig::init_bridge_fee_asset_config(
            &contract_config,
            contract_config.base.aux_data_not_empty().asset_configs,
        );
        let executor_fee_asset_config = DepositContractConfig::init_executor_fee_asset_config(
            &contract_config,
            contract_config.base.aux_data_not_empty().asset_configs,
        );
        let instance = Self {
            base: contract_config,
            bridge_fee_asset_config,
            executor_fee_asset_config,
        };
        instance.validate();
        instance
    }

    pub fn bridge_type(&self) -> BridgeType {
        self.base.base.data.bridge_type.clone()
    }

    pub fn pool_address(&self) -> &String {
        &self.base.base.data.pool_address
    }

    pub fn pool_contract(&self) -> PoolContractConfig {
        let aux_data = self.base.base.aux_data_not_empty();
        let pool_contract_config =
            aux_data.pool_contract_getter(self.pool_address());
        pool_contract_config.expect(
            format!(
                "no poolContract definition found for deposit contract={:?}", self.base.base.data.address()
            ).as_str()
        ).clone()
    }

    pub fn disabled(&self) -> bool {
        self.base.base.data.disabled
    }

    pub fn min_amount(&self) -> BigInt {
        BigInt::from_str(
            &self.base.base.data.min_amount
        ).unwrap()
    }

    pub fn min_amount_number(&self) -> f64 {
        from_decimals::<&String>(
            &self.base.base.data.min_amount,
            Some(self.asset_decimals()),
        )
    }

    pub fn max_amount(&self) -> BigInt {
        BigInt::from_str(
            &self.base.base.data.max_amount
        ).unwrap()
    }

    pub fn max_amount_number(&self) -> f64 {
        from_decimals::<&String>(
            &self.base.base.data.max_amount,
            Some(self.asset_decimals()),
        )
    }

    pub fn min_bridge_fee(&self) -> BigInt {
        BigInt::from_str(
            &self.base.base.data.min_bridge_fee
        ).unwrap()
    }

    pub fn min_bridge_fee_number(&self) -> f64 {
        from_decimals::<&String>(
            &self.base.base.data.min_bridge_fee,
            Some(self.bridge_fee_asset().asset_decimals()),
        )
    }

    pub fn min_executor_fee(&self) -> BigInt {
        BigInt::from_str(
            &self.base.base.data.min_executor_fee
        ).unwrap()
    }

    pub fn min_executor_fee_number(&self) -> f64 {
        from_decimals::<&String>(
            &self.base.base.data.min_executor_fee,
            Some(self.executor_fee_asset().asset_decimals()),
        )
    }

    pub fn asset(&self) -> AssetConfig {
        self.pool_contract().asset()
    }

    pub fn asset_type(&self) -> AssetType {
        self.pool_contract().asset_type()
    }

    pub fn asset_symbol(&self) -> String {
        self.pool_contract().asset_symbol().to_owned()
    }

    pub fn asset_decimals(&self) -> u32 {
        self.pool_contract().asset_decimals()
    }

    pub fn asset_address(&self) -> Option<String> {
        self.pool_contract().asset_address()
    }

    pub fn recommended_amounts(&self) -> Vec<BigInt> {
        self.pool_contract().recommended_amounts()
    }

    pub fn recommended_amounts_number(&self) -> Vec<f64> {
        self.pool_contract().recommended_amounts_number()
    }

    pub fn min_rollup_fee(&self) -> BigInt {
        if self.peer_contract().is_some() {
            return self.peer_contract().unwrap().pool_contract().min_rollup_fee();
        }
        self.pool_contract().min_rollup_fee()
    }

    pub fn min_rollup_fee_number(&self) -> f64 {
        if self.peer_contract().is_some() {
            return self.peer_contract().unwrap().pool_contract().min_rollup_fee_number();
        }
        self.pool_contract().min_rollup_fee_number()
    }

    pub fn circuits(&self) -> Vec<CircuitConfig> {
        self.pool_contract().circuits()
    }

    pub fn peer_chain_id(&self) -> &Option<u32> {
        &self.base.base.data.peer_chain_id
    }

    pub fn peer_contract_address(&self) -> &Option<String> {
        &self.base.base.data.peer_contract_address
    }

    pub fn peer_contract(&self) -> Option<DepositContractConfig> {
        let peer_chain_id = &self.peer_chain_id();
        let peer_contract_address = &self.peer_contract_address();
        println!("{:?}", peer_chain_id);
        println!("{:?}", peer_contract_address);
        match peer_chain_id {
            Some(peer_chain_id) => {
                match peer_contract_address {
                    Some(peer_contract_address) => {
                        self.base.base.aux_data_not_empty().deposit_contract_getter(
                            *peer_chain_id,
                            peer_contract_address.clone(),
                        ).cloned()
                    }
                    None => { None }
                }
            }
            None => { None }
        }
    }

    pub fn bridge_fee_asset(&self) -> AssetConfig {
        match &self.bridge_fee_asset_config {
            None => { self.base.base.aux_data_not_empty().main_asset_config }
            Some(value) => { value.clone() }
        }
    }

    pub fn executor_fee_asset(&self) -> AssetConfig {
        match &self.executor_fee_asset_config {
            None => { self.asset() }
            Some(value) => { value.clone() }
        }
    }

    pub fn service_fee(&self) -> u32 {
        self.base.base.data.service_fee
    }

    pub fn service_fee_divider(&self) -> u32 {
        self.base.base.data.service_fee_divider
    }

    pub fn mutate(&self, data: Option<RawDepositContractConfig>, aux_data: Option<AuxData>) -> Self {
        let data = match data {
            None => { self.base.base.data.clone() }
            Some(value) => { value }
        };
        let aux_data = match aux_data {
            None => {
                self.base.base.aux_data.clone()
            }
            Some(_) => { aux_data }
        };
        DepositContractConfig::new(data, aux_data)
    }

    fn validate(&self) {
        check(
            self.max_amount().ge(&self.min_amount()),
            format!(
                "deposit contract={} maxAmount is less than minAmount", &self.base.base.data.address()
            ).as_str(),
        );
        if self.bridge_type().eq(&BridgeType::Loop) {
            check(
                self.peer_chain_id().is_none(),
                format!(
                    "deposit contract={} peerChainId should be undefined when bridge type={:?}",
                    &self.base.base.data.address(), &self.bridge_type()
                ).as_str(),
            );
            check(
                self.peer_contract_address().is_none(),
                format!(
                    "deposit contract={} peerContractAddress should be undefined when bridge type={:?}",
                    &self.base.base.data.address(), &self.bridge_type()
                ).as_str(),
            );
        } else {
            check(
                self.peer_chain_id().is_some(),
                format!(
                    "deposit contract={} peerChainId should not be undefined when bridge type={:?}",
                    &self.base.base.data.address(), &self.bridge_type()
                ).as_str(),
            );
            check(
                self.peer_contract_address().is_some(),
                format!(
                    "deposit contract={} peerContractAddress should not be undefined when bridge type={:?}",
                    &self.base.base.data.address(), &self.bridge_type()
                ).as_str(),
            );
        }
    }

    fn init_bridge_fee_asset_config(
        base: &ContractConfig<RawDepositContractConfig, AuxData>,
        asset_configs: HashMap<String, AssetConfig>,
    ) -> Option<AssetConfig> {
        match &base.base.data.bridge_fee_asset_address {
            None => { None }
            Some(address) => {
                if address.eq(MAIN_ASSET_ADDRESS) {
                    return Some(base.base.aux_data_not_empty().main_asset_config);
                }
                let asset_config = asset_configs.get(address);
                check(
                    asset_config.is_some(),
                    format!(
                        "bridge fee asset address={} config has not been defined for deposit contract address={}",
                        address, base.base.data.address()
                    ).as_str(),
                );
                return Some(asset_config.unwrap().clone());
            }
        }
    }

    fn init_executor_fee_asset_config(
        base: &ContractConfig<RawDepositContractConfig, AuxData>,
        asset_configs: HashMap<String, AssetConfig>,
    ) -> Option<AssetConfig> {
        match &base.base.data.executor_fee_asset_address {
            None => { None }
            Some(address) => {
                if address.eq(MAIN_ASSET_ADDRESS) {
                    return Some(base.base.aux_data_not_empty().main_asset_config);
                }
                let asset_config = asset_configs.get(address);
                check(
                    asset_config.is_some(),
                    format!(
                        "executor fee asset address={} config has not been defined for deposit contract address={}",
                        address, base.base.data.address()
                    ).as_str(),
                );
                return Some(asset_config.unwrap().clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::common::{AssetType, BridgeType, CircuitType};
    use crate::raw::asset::RawAssetConfig;
    use crate::raw::base::RawConfig;
    use crate::raw::contract::base::{RawContractConfigTrait};
    use crate::raw::contract::deposit::RawDepositContractConfig;
    use crate::raw::mystiko::RawMystikoConfig;
    use crate::wrapper::asset::{AssetConfig, MAIN_ASSET_ADDRESS};
    use crate::wrapper::circuit::CircuitConfig;
    use crate::wrapper::contract::deposit::{AuxData, DepositContractConfig};
    use crate::wrapper::contract::pool;
    use crate::wrapper::contract::pool::PoolContractConfig;
    use crate::wrapper::mystiko::MystikoConfig;

    async fn raw_mystiko_config() -> RawMystikoConfig {
        let config =
            RawConfig::create_from_file::<RawMystikoConfig>("src/tests/files/mystiko.valid.json").await;
        config
    }

    async fn circuit_configs() -> (HashMap<String, CircuitConfig>, HashMap<CircuitType, CircuitConfig>) {
        let mut circuit_configs_by_name = HashMap::new();
        let mut default_circuit_configs = HashMap::new();
        let raw_mystiko_config = raw_mystiko_config().await;
        for circuit in raw_mystiko_config.circuits {
            let circuit_config = CircuitConfig::new(circuit.clone());
            circuit_configs_by_name.insert(circuit.name.clone(), circuit_config.clone());
            if circuit.is_default {
                default_circuit_configs.insert(
                    circuit.circuit_type.clone(),
                    circuit_config.clone(),
                );
            }
        }
        (circuit_configs_by_name, default_circuit_configs)
    }

    async fn main_asset_config() -> AssetConfig {
        let raw_mystiko_config = raw_mystiko_config().await;
        let asset_symbol =
            raw_mystiko_config.chains.get(0).unwrap().clone().asset_symbol;
        let asset_decimals =
            raw_mystiko_config.chains.get(0).unwrap().clone().asset_decimals;
        let recommended_amounts =
            raw_mystiko_config.chains.get(0).unwrap().clone().recommended_amounts;
        AssetConfig::new(
            RawAssetConfig::new(
                AssetType::Main,
                asset_symbol,
                asset_decimals,
                "0x0000000000000000000000000000000000000000".to_string(),
                recommended_amounts,
            )
        )
    }

    async fn asset_configs() -> HashMap<String, AssetConfig> {
        let mut asset_configs = HashMap::new();
        let raw_mystiko_config = raw_mystiko_config().await;
        let raw_asset_config =
            raw_mystiko_config.chains.get(0).unwrap().assets.get(0).unwrap();
        asset_configs.insert(
            raw_asset_config.asset_address.clone(),
            AssetConfig::new(raw_asset_config.clone()),
        );
        asset_configs
    }

    async fn default_config() -> (RawDepositContractConfig, DepositContractConfig) {
        let mut raw_config =
            RawConfig::create_from_file::<RawDepositContractConfig>(
                "src/tests/files/contract/deposit.valid.json"
            ).await;
        raw_config.bridge_type = BridgeType::Loop;
        raw_config.peer_chain_id = None;
        raw_config.peer_contract_address = None;
        let raw_mystiko_config = raw_mystiko_config().await;
        let pool_contract =
            raw_mystiko_config.chains.get(0).unwrap().pool_contracts.get(0).unwrap();
        let mut pool_contract_configs = HashMap::new();
        let (
            circuit_configs_by_name,
            default_circuit_configs
        ) = circuit_configs().await;
        pool_contract_configs.insert(
            pool_contract.address().to_string(),
            PoolContractConfig::new(
                pool_contract.clone(),
                Some(
                    pool::AuxData::new(
                        default_circuit_configs,
                        circuit_configs_by_name,
                        main_asset_config().await,
                        asset_configs().await,
                    )
                ),
            ),
        );
        let aux_data = Some(AuxData::new(
            pool_contract_configs,
            main_asset_config().await,
            asset_configs().await,
            None,
        ));
        let config = DepositContractConfig::new(raw_config.clone(), aux_data);
        (raw_config, config)
    }

    #[tokio::test]
    async fn test_equality() {
        let (raw_config, config) = default_config().await;
        let asset_configs = asset_configs().await;
        assert_eq!(config.bridge_type(), raw_config.bridge_type);
        assert_eq!(config.pool_address(), &raw_config.pool_address);
        assert_eq!(config.disabled(), raw_config.disabled);
        assert_eq!(config.min_amount().to_string(), raw_config.min_amount);
        assert_eq!(config.min_amount_number(), 1f64);
        assert_eq!(config.max_amount().to_string(), raw_config.max_amount);
        assert_eq!(config.max_amount_number(), 10f64);
        assert_eq!(config.min_bridge_fee().to_string(), raw_config.min_bridge_fee);
        assert_eq!(config.min_bridge_fee_number(), 2f64);
        assert_eq!(config.min_executor_fee().to_string(), raw_config.min_executor_fee);
        assert_eq!(config.min_executor_fee_number(), 3f64);
        assert_eq!(config.asset(), asset_configs.get("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a").unwrap().clone());
        assert_eq!(config.asset_type(), AssetType::Erc20);
        assert_eq!(config.asset_symbol(), "MTT".to_string());
        assert_eq!(config.asset_decimals(), 16);
        assert_eq!(config.asset_address().unwrap(), "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a".to_string());
        assert_eq!(
            config.recommended_amounts().iter().map(|a| a.to_string()).collect::<Vec<String>>(),
            vec![
                String::from("10000000000000000"),
                String::from("100000000000000000"),
            ]
        );
        assert_eq!(config.recommended_amounts_number(), vec![1f64, 10f64]);
        assert_eq!(config.min_rollup_fee().to_string(), "40000000000000000".to_string());
        assert_eq!(config.min_rollup_fee_number(), 4f64);

        let circuit_names =
            config.circuits().iter().map(|conf| conf.name().clone()).collect::<Vec<String>>();
        assert_eq!(circuit_names.contains(&String::from("zokrates-2.0-rollup1")), true);
        assert_eq!(config.pool_contract().base.base.data.address(), raw_config.pool_address);
        assert_eq!(config.peer_contract().is_none(), true);
        assert_eq!(
            config.bridge_fee_asset(),
            asset_configs.get("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a").unwrap().clone(),
        );
        assert_eq!(
            config.executor_fee_asset(),
            asset_configs.get("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a").unwrap().clone()
        );
        assert_eq!(config.service_fee(), 2);
        assert_eq!(config.service_fee_divider(), 1000);
    }

    #[tokio::test]
    #[should_panic(
    expected = "bridge fee asset address=0xBc28029D248FC60bce0bAC01cF41A53aEEaE06F9 config has not \
    been defined for deposit contract address=0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
    )]
    async fn test_bridge_fee_asset() {
        let (mut raw_config, mut config) = default_config().await;
        let main_asset_config = main_asset_config().await;
        raw_config.bridge_fee_asset_address = None;
        config = config.mutate(Some(raw_config.clone()), None);
        assert_eq!(config.bridge_fee_asset(), main_asset_config);
        assert_eq!(config.min_bridge_fee_number(), 0.02f64);
        raw_config.bridge_fee_asset_address = Some(MAIN_ASSET_ADDRESS.to_string());
        config.mutate(Some(raw_config.clone()), None);
        assert_eq!(config.bridge_fee_asset(), main_asset_config);
        raw_config.bridge_fee_asset_address = Some("0xBc28029D248FC60bce0bAC01cF41A53aEEaE06F9".to_string());
        config.mutate(Some(raw_config), None);
    }

    #[tokio::test]
    #[should_panic(expected = "executor fee asset address=0xBc28029D248FC60bce0bAC01cF41A53aEEaE06F9 \
     config has not been defined for deposit contract address=0x961f315a836542e603a3df2e0dd9d4ecd06ebc67"
    )]
    async fn test_executor_fee_asset() {
        let (mut raw_config, mut config) = default_config().await;
        raw_config.executor_fee_asset_address = None;
        config = config.mutate(Some(raw_config.clone()), None);
        assert_eq!(config.executor_fee_asset(), config.asset());
        raw_config.executor_fee_asset_address = Some(MAIN_ASSET_ADDRESS.to_string());
        config = config.mutate(Some(raw_config.clone()), None);
        assert_eq!(config.min_executor_fee_number(), 0.03f64);
        assert_eq!(config.executor_fee_asset(), main_asset_config().await);
        raw_config.executor_fee_asset_address = Some("0xBc28029D248FC60bce0bAC01cF41A53aEEaE06F9".to_string());
        config.mutate(Some(raw_config), None);
    }

    #[tokio::test]
    async fn test_peer_contract() {
        let raw_mystiko_config = raw_mystiko_config().await;
        let raw_asset_config =
            raw_mystiko_config.chains.get(1).unwrap().assets.get(0).unwrap();
        let (mut raw_config, _) = default_config().await;
        let mut peer_assets_config = HashMap::new();
        peer_assets_config.insert(
            raw_asset_config.asset_address.clone(),
            AssetConfig::new(raw_asset_config.clone()),
        );
        let (circuit_configs_by_name, default_circuit_configs) = circuit_configs().await;
        let mut pool_contract_configs: HashMap<String, PoolContractConfig> = HashMap::new();
        let main_asset_config = main_asset_config().await;
        let pool_contract = raw_mystiko_config.chains.get(1).unwrap().pool_contracts.get(0).unwrap();
        pool_contract_configs.insert(
            pool_contract.address().to_string(),
            PoolContractConfig::new(
                pool_contract.clone(),
                Some(
                    pool::AuxData::new(
                        default_circuit_configs,
                        circuit_configs_by_name,
                        main_asset_config.clone(),
                        peer_assets_config.clone(),
                    )
                ),
            ),
        );
        let asset_configs = asset_configs().await;
        let aux_data = Some(AuxData::new(
            pool_contract_configs,
            main_asset_config.clone(),
            asset_configs.clone(),
            None,
        ));
        let peer_contract_config =
            DepositContractConfig::new(
                raw_mystiko_config.chains.get(1).unwrap().deposit_contracts.get(0).unwrap().clone(),
                aux_data,
            );
        raw_config.bridge_type = BridgeType::Tbridge;
        raw_config.peer_chain_id = Some(97);
        raw_config.peer_contract_address = Some(String::from("0xd791049D0a154bC7860804e1A18ACD148Eb0afD9"));
        let config = DepositContractConfig::new(raw_config.clone(), Some(
            AuxData::new(
                Default::default(),
                main_asset_config.clone(),
                asset_configs.clone(),
                Some(
                    // TODO check
                    MystikoConfig::new(raw_mystiko_config.clone())
                ),
            )
        ));
        assert_eq!(
            config.peer_contract().unwrap().base.base.data.address(),
            "0xd791049D0a154bC7860804e1A18ACD148Eb0afD9"
        );
        assert_eq!(config.min_rollup_fee().to_string(), "12345".to_string());
        assert_eq!(config.min_rollup_fee_number(), 1.2345e-12);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_raw_config_0() {
        let (mut raw_config, _) = default_config().await;
        raw_config.bridge_type = BridgeType::Tbridge;
        DepositContractConfig::new(
            raw_config,
            Some(AuxData::new(
                Default::default(),
                main_asset_config().await,
                asset_configs().await,
                None,
            )),
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_raw_config_1() {
        let (mut raw_config, _) = default_config().await;
        raw_config.bridge_type = BridgeType::Loop;
        raw_config.peer_contract_address = Some(String::from("0xd791049D0a154bC7860804e1A18ACD148Eb0afD9"));
        DepositContractConfig::new(
            raw_config,
            Some(AuxData::new(
                Default::default(),
                main_asset_config().await,
                asset_configs().await,
                None,
            )),
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_raw_config_2() {
        let (mut raw_config, _) = default_config().await;
        raw_config.bridge_type = BridgeType::Loop;
        raw_config.peer_contract_address = None;
        raw_config.peer_chain_id = Some(97);
        DepositContractConfig::new(
            raw_config,
            Some(AuxData::new(
                Default::default(),
                main_asset_config().await,
                asset_configs().await,
                None,
            )),
        );
    }

    #[tokio::test]
    #[should_panic(expected = "no poolContract definition found for deposit contract=\"0x961f315a836542e603a3df2e0dd9d4ecd06ebc67\"")]
    async fn test_invalid_raw_config_3() {
        let (mut raw_config, _) = default_config().await;
        raw_config.bridge_type = BridgeType::Loop;
        raw_config.peer_contract_address = None;
        raw_config.peer_chain_id = None;
        let config = DepositContractConfig::new(
            raw_config,
            Some(AuxData::new(
                Default::default(),
                main_asset_config().await,
                asset_configs().await,
                None,
            )),
        );
        config.pool_contract();
    }

    #[tokio::test]
    #[should_panic(expected = "deposit contract=0x961f315a836542e603a3df2e0dd9d4ecd06ebc67 maxAmount is less than minAmount")]
    async fn test_invalid_max_amount() {
        let (mut raw_config, config) = default_config().await;
        raw_config.max_amount = String::from("10000");
        config.mutate(Some(raw_config.clone()), None);
    }

    #[tokio::test]
    async fn test_copy() {
        let (raw_config, config) = default_config().await;
        assert_eq!(config.base.base.copy_data(), raw_config);
    }

    #[tokio::test]
    async fn test_mutate() {
        let (mut raw_config, config) = default_config().await;
        assert_eq!(config.mutate(None, None), config);
        raw_config.base.name = "another name".to_string();
        let new_config = config.mutate(Some(raw_config.clone()), None);
        assert_eq!(new_config.base.name(), "another name".to_string());
    }

    #[tokio::test]
    async fn test_to_json_string() {
        let (raw_config, config) = default_config().await;
        let json_string = config.base.base.to_json_string();
        let loaded_raw_config =
            RawConfig::create_from_json_string::<RawDepositContractConfig>(json_string.as_str()).await;
        assert_eq!(loaded_raw_config, raw_config);
    }
}
