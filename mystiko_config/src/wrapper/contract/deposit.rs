use crate::common::{AssetType, BridgeType};
use crate::errors::ValidationError;
use crate::raw::contract::base::RawContractConfigTrait;
use crate::raw::contract::deposit::RawDepositContractConfig;
use crate::wrapper::asset::{AssetConfig, MAIN_ASSET_ADDRESS};
use crate::wrapper::chain::ChainConfig;
use crate::wrapper::circuit::CircuitConfig;
use crate::wrapper::contract::base::ContractConfig;
use crate::wrapper::contract::pool::PoolContractConfig;
use mystiko_utils::check::check;
use mystiko_utils::convert::from_decimals;
use num_bigint::BigInt;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AuxData {
    pool_contract_configs: Rc<HashMap<String, PoolContractConfig>>,
    main_asset_config: Rc<AssetConfig>,
    asset_configs: Rc<HashMap<String, AssetConfig>>,
    chain_configs: Rc<Option<HashMap<u32, ChainConfig>>>,
}

impl AuxData {
    pub fn new(
        pool_contract_configs: Rc<HashMap<String, PoolContractConfig>>,
        main_asset_config: Rc<AssetConfig>,
        asset_configs: Rc<HashMap<String, AssetConfig>>,
        chain_configs: Rc<Option<HashMap<u32, ChainConfig>>>,
    ) -> Self {
        Self {
            pool_contract_configs,
            main_asset_config,
            asset_configs,
            chain_configs,
        }
    }

    fn pool_contract_getter(&self, address: &str) -> Option<&PoolContractConfig> {
        self.pool_contract_configs.get(address)
    }

    fn deposit_contract_getter(
        &self,
        chain_id: u32,
        address: String,
    ) -> Option<&DepositContractConfig> {
        match &*self.chain_configs {
            None => None,
            Some(chain_configs) => {
                let chain_config = chain_configs.get(&chain_id);
                match chain_config {
                    None => None,
                    Some(config) => config.get_deposit_contract_by_address(address),
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DepositContractConfig {
    base: ContractConfig<RawDepositContractConfig, AuxData>,
    bridge_fee_asset_config: Option<Rc<AssetConfig>>,
    executor_fee_asset_config: Option<Rc<AssetConfig>>,
}

impl DepositContractConfig {
    pub fn new(
        data: RawDepositContractConfig,
        aux_data: Option<AuxData>,
    ) -> Result<Self, ValidationError> {
        let contract_config = ContractConfig::new(data, aux_data);
        let aux_data = contract_config.base.aux_data_not_empty().unwrap();
        let bridge_fee_asset_config =
            DepositContractConfig::init_bridge_fee_asset_config(&contract_config, aux_data);
        if bridge_fee_asset_config.is_err() {
            return Err(bridge_fee_asset_config.unwrap_err());
        }
        let bridge_fee_asset_config = bridge_fee_asset_config.unwrap();
        let executor_fee_asset_config =
            DepositContractConfig::init_executor_fee_asset_config(&contract_config, aux_data);
        if executor_fee_asset_config.is_err() {
            return Err(executor_fee_asset_config.unwrap_err());
        }
        let executor_fee_asset_config = executor_fee_asset_config.unwrap();
        let instance = Self {
            base: contract_config,
            bridge_fee_asset_config,
            executor_fee_asset_config,
        };
        let validate = instance.validate();
        return match validate {
            Ok(_) => Ok(instance),
            Err(err) => Err(err),
        };
    }

    pub fn bridge_type(&self) -> BridgeType {
        self.base.base.data.bridge_type.clone()
    }

    pub fn pool_address(&self) -> &String {
        &self.base.base.data.pool_address
    }

    pub fn pool_contract(&self) -> Result<PoolContractConfig, Box<dyn Error>> {
        let aux_data = self.base.base.aux_data_not_empty().unwrap();
        let pool_contract_config = aux_data.pool_contract_getter(self.pool_address());
        if pool_contract_config.is_none() {
            return Err(format!(
                "no poolContract definition found for deposit contract={}",
                self.base.base.data.address()
            ))?;
        }
        Ok(pool_contract_config.unwrap().clone())
    }

    pub fn disabled(&self) -> bool {
        self.base.base.data.disabled
    }

    pub fn min_amount(&self) -> BigInt {
        BigInt::from_str(&self.base.base.data.min_amount).unwrap()
    }

    pub fn min_amount_number(&self) -> Result<f64, Box<dyn Error>> {
        let asset_decimals = self.asset_decimals()?;
        Ok(from_decimals::<&String>(
            &self.base.base.data.min_amount,
            Some(asset_decimals),
        ))
    }

    pub fn max_amount(&self) -> BigInt {
        BigInt::from_str(&self.base.base.data.max_amount).unwrap()
    }

    pub fn max_amount_number(&self) -> Result<f64, Box<dyn Error>> {
        let asset_decimals = self.asset_decimals()?;
        Ok(from_decimals::<&String>(
            &self.base.base.data.max_amount,
            Some(asset_decimals),
        ))
    }

    pub fn min_bridge_fee(&self) -> BigInt {
        BigInt::from_str(&self.base.base.data.min_bridge_fee).unwrap()
    }

    pub fn min_bridge_fee_number(&self) -> f64 {
        from_decimals::<&String>(
            &self.base.base.data.min_bridge_fee,
            Some(self.bridge_fee_asset().asset_decimals()),
        )
    }

    pub fn min_executor_fee(&self) -> BigInt {
        BigInt::from_str(&self.base.base.data.min_executor_fee).unwrap()
    }

    pub fn min_executor_fee_number(&self) -> Result<f64, Box<dyn Error>> {
        let executor_fee_asset = self.executor_fee_asset()?;
        Ok(from_decimals::<&String>(
            &self.base.base.data.min_executor_fee,
            Some(executor_fee_asset.asset_decimals()),
        ))
    }

    pub fn asset(&self) -> Result<Rc<AssetConfig>, Box<dyn Error>> {
        let pool_contract = self.pool_contract()?;
        Ok(pool_contract.asset())
    }

    pub fn asset_type(&self) -> Result<AssetType, Box<dyn Error>> {
        let pool_contract = self.pool_contract()?;
        Ok(pool_contract.asset_type())
    }

    pub fn asset_symbol(&self) -> Result<String, Box<dyn Error>> {
        let pool_contract = self.pool_contract()?;
        Ok(pool_contract.asset_symbol().to_owned())
    }

    pub fn asset_decimals(&self) -> Result<u32, Box<dyn Error>> {
        let pool_contract = self.pool_contract()?;
        Ok(pool_contract.asset_decimals())
    }

    pub fn asset_address(&self) -> Result<Option<String>, Box<dyn Error>> {
        let pool_contract = self.pool_contract()?;
        Ok(pool_contract.asset_address())
    }

    pub fn recommended_amounts(&self) -> Result<Vec<BigInt>, Box<dyn Error>> {
        let pool_contract = self.pool_contract()?;
        Ok(pool_contract.recommended_amounts())
    }

    pub fn recommended_amounts_number(&self) -> Result<Vec<f64>, Box<dyn Error>> {
        let pool_contract = self.pool_contract()?;
        Ok(pool_contract.recommended_amounts_number())
    }

    pub fn min_rollup_fee(&self) -> Result<BigInt, Box<dyn Error>> {
        if self.peer_contract().is_some() {
            let pool_contract = self.peer_contract().unwrap().pool_contract()?;
            return Ok(pool_contract.min_rollup_fee());
        }
        let pool_contract = self.pool_contract()?;
        Ok(pool_contract.min_rollup_fee())
    }

    pub fn min_rollup_fee_number(&self) -> Result<f64, Box<dyn Error>> {
        if self.peer_contract().is_some() {
            let pool_contract = self.peer_contract().unwrap().pool_contract()?;
            return Ok(pool_contract.min_rollup_fee_number());
        }
        let pool_contract = self.pool_contract()?;
        Ok(pool_contract.min_rollup_fee_number())
    }

    pub fn circuits(&self) -> Result<Vec<CircuitConfig>, Box<dyn Error>> {
        let pool_contract = self.pool_contract()?;
        Ok(pool_contract.circuits())
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
            Some(peer_chain_id) => match peer_contract_address {
                Some(peer_contract_address) => self
                    .base
                    .base
                    .aux_data_not_empty()
                    .unwrap()
                    .deposit_contract_getter(*peer_chain_id, peer_contract_address.clone())
                    .cloned(),
                None => None,
            },
            None => None,
        }
    }

    pub fn bridge_fee_asset(&self) -> &AssetConfig {
        match &self.bridge_fee_asset_config {
            None => {
                &self
                    .base
                    .base
                    .aux_data_not_empty()
                    .unwrap()
                    .main_asset_config
            }
            Some(value) => value,
        }
    }

    pub fn executor_fee_asset(&self) -> Result<Rc<AssetConfig>, Box<dyn Error>> {
        match &self.executor_fee_asset_config {
            None => self.asset(),
            Some(value) => Ok(value.clone()),
        }
    }

    pub fn service_fee(&self) -> u32 {
        self.base.base.data.service_fee
    }

    pub fn service_fee_divider(&self) -> u32 {
        self.base.base.data.service_fee_divider
    }

    pub fn address(&self) -> &str {
        &self.base.address()
    }

    pub fn event_filter_size(&self) -> &Option<u64> {
        self.base.event_filter_size()
    }

    pub fn indexer_filter_size(&self) -> &Option<u64> {
        self.base.indexer_filter_size()
    }

    pub fn copy_data(&self) -> RawDepositContractConfig {
        self.base.base.copy_data()
    }

    pub fn to_json_string(&self) -> String {
        self.base.base.to_json_string()
    }

    pub fn data(&self) -> &RawDepositContractConfig {
        &self.base.base.data
    }

    pub fn aux_data(&self) -> &Option<AuxData> {
        &self.base.base.aux_data
    }

    pub fn name(&self) -> &str {
        &self.base.name()
    }

    pub fn mutate(
        &self,
        data: Option<RawDepositContractConfig>,
        aux_data: Option<AuxData>,
    ) -> Result<Self, ValidationError> {
        let data = match data {
            None => self.data().clone(),
            Some(value) => value,
        };
        let aux_data = match aux_data {
            None => self.aux_data().clone(),
            Some(_) => aux_data,
        };
        DepositContractConfig::new(data, aux_data)
    }

    fn validate(&self) -> Result<(), ValidationError> {
        let check_result = check(
            self.max_amount().ge(&self.min_amount()),
            format!(
                "deposit contract={} maxAmount is less than minAmount",
                &self.base.base.data.address()
            )
            .as_str(),
        );
        if check_result.is_err() {
            return Err(ValidationError::new(vec![check_result
                .unwrap_err()
                .to_string()]));
        }
        if self.bridge_type().eq(&BridgeType::Loop) {
            let check_result = check(
                self.peer_chain_id().is_none(),
                format!(
                    "deposit contract={} peerChainId should be undefined when bridge type={:?}",
                    &self.address(),
                    &self.bridge_type()
                )
                .as_str(),
            );
            if check_result.is_err() {
                return Err(ValidationError::new(vec![check_result
                    .unwrap_err()
                    .to_string()]));
            }
            let check_result = check(
                self.peer_contract_address().is_none(),
                format!(
                    "deposit contract={} peerContractAddress should be undefined when bridge type={:?}",
                    &self.address(), &self.bridge_type()
                ).as_str(),
            );
            if check_result.is_err() {
                return Err(ValidationError::new(vec![check_result
                    .unwrap_err()
                    .to_string()]));
            }
        } else {
            let check_result = check(
                self.peer_chain_id().is_some(),
                format!(
                    "deposit contract={} peerChainId should not be undefined when bridge type={:?}",
                    &self.address(),
                    &self.bridge_type()
                )
                .as_str(),
            );
            if check_result.is_err() {
                return Err(ValidationError::new(vec![check_result
                    .unwrap_err()
                    .to_string()]));
            }
            let check_result = check(
                self.peer_contract_address().is_some(),
                format!(
                    "deposit contract={} peerContractAddress should not be undefined when bridge type={:?}",
                    &self.address(), &self.bridge_type()
                ).as_str(),
            );
            if check_result.is_err() {
                return Err(ValidationError::new(vec![check_result
                    .unwrap_err()
                    .to_string()]));
            }
        }
        Ok(())
    }

    fn init_bridge_fee_asset_config(
        base: &ContractConfig<RawDepositContractConfig, AuxData>,
        aux_data: &AuxData,
    ) -> Result<Option<Rc<AssetConfig>>, ValidationError> {
        match &base.base.data.bridge_fee_asset_address {
            None => Ok(None),
            Some(address) => {
                if address.eq(MAIN_ASSET_ADDRESS) {
                    return Ok(Some(aux_data.main_asset_config.clone()));
                }
                let asset_config = aux_data.asset_configs.get(address);
                let check_result = check(
                    asset_config.is_some(),
                    format!(
                        "bridge fee asset address={} config has not been defined for deposit contract address={}",
                        address, base.base.data.address()
                    ).as_str(),
                );
                if check_result.is_err() {
                    return Err(ValidationError::new(vec![check_result
                        .unwrap_err()
                        .to_string()]));
                }
                return Ok(Some(Rc::new(asset_config.unwrap().clone())));
            }
        }
    }

    fn init_executor_fee_asset_config(
        base: &ContractConfig<RawDepositContractConfig, AuxData>,
        aux_data: &AuxData,
    ) -> Result<Option<Rc<AssetConfig>>, ValidationError> {
        match &base.base.data.executor_fee_asset_address {
            None => Ok(None),
            Some(address) => {
                if address.eq(MAIN_ASSET_ADDRESS) {
                    return Ok(Some(aux_data.main_asset_config.clone()));
                }
                let asset_config = aux_data.asset_configs.get(address);
                let check_result = check(
                    asset_config.is_some(),
                    format!(
                        "executor fee asset address={} config has not been defined for deposit contract address={}",
                        address, base.base.data.address()
                    ).as_str(),
                );
                if check_result.is_err() {
                    return Err(ValidationError::new(vec![check_result
                        .unwrap_err()
                        .to_string()]));
                }
                return Ok(Some(Rc::new(asset_config.unwrap().clone())));
            }
        }
    }
}
