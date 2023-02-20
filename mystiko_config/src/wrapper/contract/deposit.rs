use std::collections::HashMap;
use std::str::FromStr;
use std::thread::sleep;
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

#[derive(Clone, Debug)]
pub struct AuxData {
    deposit_contract_configs: HashMap<u32, Vec<DepositContractConfig>>,
    pool_contract_configs: HashMap<String, PoolContractConfig>,
    main_asset_config: AssetConfig,
    asset_configs: HashMap<String, AssetConfig>,
}

impl AuxData {
    pub fn new(
        deposit_contract_configs: HashMap<u32, Vec<DepositContractConfig>>,
        pool_contract_configs: HashMap<String, PoolContractConfig>,
        main_asset_config: AssetConfig,
        asset_configs: HashMap<String, AssetConfig>,
    ) -> Self {
        Self { deposit_contract_configs, pool_contract_configs, main_asset_config, asset_configs }
    }

    fn pool_contract_getter(&self, address: &str) -> Option<&PoolContractConfig> {
        self.pool_contract_configs.get(address)
    }

    fn deposit_contract_getter(&self, chain_id: u32, address: String) -> Option<DepositContractConfig> {
        let deposit_configs = self.deposit_contract_configs.get(&chain_id);
        if deposit_configs.is_some() {
            for config in deposit_configs.unwrap() {
                if config.base.base.data.address() == address {
                    return Some(config.clone());
                }
            }
        }
        None
    }
}

#[derive(Clone, Debug)]
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
        let pool_contract_config = aux_data.pool_contract_getter(
            &self.base.base.data.pool_address
        );
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
        self.pool_contract().asset_address().clone()
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
        match peer_chain_id {
            None => { None }
            Some(peer_chain_id) => {
                match peer_contract_address {
                    None => { None }
                    Some(peer_contract_address) => {
                        self.base.base.aux_data_not_empty().deposit_contract_getter(
                            *peer_chain_id,
                            peer_contract_address.clone(),
                        )
                    }
                }
            }
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
