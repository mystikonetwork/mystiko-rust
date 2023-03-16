use crate::common::{AssetType, BridgeType, CircuitType};
use crate::errors::ValidationError;
use crate::raw::contract::base::RawContractConfigTrait;
use crate::raw::contract::pool::RawPoolContractConfig;
use crate::wrapper::asset::AssetConfig;
use crate::wrapper::circuit::CircuitConfig;
use crate::wrapper::contract::base::ContractConfig;
use flamer::flame;
use mystiko_utils::check::check;
use mystiko_utils::convert::from_decimals;
use num_bigint::BigInt;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct AuxData {
    default_circuit_configs: Rc<HashMap<CircuitType, CircuitConfig>>,
    circuit_configs_by_name: Rc<HashMap<String, CircuitConfig>>,
    main_asset_config: Rc<AssetConfig>,
    asset_configs: Rc<HashMap<String, AssetConfig>>,
}

impl AuxData {
    pub fn new(
        default_circuit_configs: Rc<HashMap<CircuitType, CircuitConfig>>,
        circuit_configs_by_name: Rc<HashMap<String, CircuitConfig>>,
        main_asset_config: Rc<AssetConfig>,
        asset_configs: Rc<HashMap<String, AssetConfig>>,
    ) -> Self {
        Self {
            default_circuit_configs,
            circuit_configs_by_name,
            main_asset_config,
            asset_configs,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PoolContractConfig {
    base: ContractConfig<RawPoolContractConfig, AuxData>,
    pub circuit_configs: HashMap<CircuitType, CircuitConfig>,
    pub main_asset_config: Rc<AssetConfig>,
    pub asset_config: Option<AssetConfig>,
}

impl PoolContractConfig {
    #[flame]
    pub fn new(
        data: RawPoolContractConfig,
        aux_data: Option<AuxData>,
    ) -> Result<Self, ValidationError> {
        let contract_config = ContractConfig::new(data, aux_data);
        let aux_data = contract_config.base.aux_data_not_empty().unwrap();
        let asset_config = PoolContractConfig::init_asset_config(&contract_config, aux_data);
        if asset_config.is_err() {
            return Err(ValidationError::new(vec![asset_config
                .unwrap_err()
                .to_string()]));
        }
        let config = Self {
            base: contract_config.clone(),
            circuit_configs: PoolContractConfig::init_circuits_configs(&contract_config, aux_data),
            main_asset_config: aux_data.main_asset_config.clone(),
            asset_config: asset_config.unwrap(),
        };
        let validate = config.validate();
        match validate {
            Ok(_) => Ok(config),
            Err(err) => Err(err),
        }
    }

    pub fn version(&self) -> &u32 {
        self.base.version()
    }

    pub fn address(&self) -> &str {
        self.base.base.data.address()
    }

    pub fn copy_data(&self) -> RawPoolContractConfig {
        self.base.base.copy_data()
    }

    pub fn to_json_string(&self) -> String {
        self.base.base.to_json_string()
    }

    pub fn indexer_filter_size(&self) -> &Option<u64> {
        self.base.indexer_filter_size()
    }

    pub fn event_filter_size(&self) -> &Option<u64> {
        self.base.event_filter_size()
    }

    pub fn pool_name(&self) -> &str {
        &self.base.base.data.pool_name
    }

    pub fn bridge_type(&self) -> &BridgeType {
        &self.base.base.data.bridge_type
    }

    pub fn asset(&self) -> Rc<AssetConfig> {
        match &self.asset_config {
            None => self.main_asset_config.clone(),
            Some(value) => Rc::new(value.clone()),
        }
    }

    pub fn asset_type(&self) -> AssetType {
        self.asset().asset_type().clone()
    }

    pub fn asset_symbol(&self) -> String {
        self.asset().asset_symbol()
    }

    pub fn asset_decimals(&self) -> u32 {
        self.asset().asset_decimals()
    }

    pub fn asset_address(&self) -> Option<String> {
        self.base.base.data.asset_address.clone()
    }

    pub fn recommended_amounts(&self) -> Vec<BigInt> {
        self.asset().recommended_amounts()
    }

    pub fn recommended_amounts_number(&self) -> Vec<f64> {
        self.asset().recommended_amounts_number()
    }

    pub fn min_rollup_fee(&self) -> BigInt {
        BigInt::from_str(&self.base.base.data.min_rollup_fee).unwrap()
    }

    pub fn min_rollup_fee_number(&self) -> f64 {
        from_decimals(self.min_rollup_fee(), Some(self.asset_decimals()))
    }

    pub fn circuits(&self) -> Vec<CircuitConfig> {
        self.circuit_configs.values().cloned().collect()
    }

    pub fn circuit_config(&self, t: CircuitType) -> Option<&CircuitConfig> {
        self.circuit_configs.get(&t)
    }

    pub fn mutate(
        &self,
        data: Option<RawPoolContractConfig>,
        aux_data: Option<AuxData>,
    ) -> Result<Self, ValidationError> {
        let aux_data = match aux_data {
            None => self.aux_data().clone(),
            Some(_) => aux_data,
        };
        let data = match data {
            None => self.data().clone(),
            Some(value) => value,
        };
        PoolContractConfig::new(data, aux_data)
    }

    fn data(&self) -> &RawPoolContractConfig {
        &self.base.base.data
    }

    fn aux_data(&self) -> &Option<AuxData> {
        &self.base.base.aux_data
    }

    #[flame]
    fn init_circuits_configs(
        base: &ContractConfig<RawPoolContractConfig, AuxData>,
        aux_data: &AuxData,
    ) -> HashMap<CircuitType, CircuitConfig> {
        let mut circuit_configs: HashMap<CircuitType, CircuitConfig> = HashMap::new();
        for circuit_conf in (*aux_data.default_circuit_configs).values() {
            circuit_configs.insert(*circuit_conf.circuit_type(), circuit_conf.clone());
        }
        for circuit_name in &base.base.data.circuits {
            let circuit_conf = aux_data.circuit_configs_by_name.get(circuit_name);
            match circuit_conf {
                None => {}
                Some(value) => {
                    circuit_configs.insert(*value.circuit_type(), value.clone());
                }
            }
        }
        circuit_configs
    }

    #[flame]
    fn init_asset_config(
        base: &ContractConfig<RawPoolContractConfig, AuxData>,
        aux_data: &AuxData,
    ) -> Result<Option<AssetConfig>, Box<dyn Error>> {
        match &base.base.data.asset_address {
            None => Ok(None),
            Some(value) => {
                if let Some(asset_config) = aux_data.asset_configs.get(value) {
                    Ok(Some(asset_config.clone()))
                } else {
                    Err(
                        format!(
                            "asset address={:?} config has not been defined for pool contract address={:?}",
                            value, base.address()
                        )
                    )?
                }
            }
        }
    }

    #[flame]
    fn validate(&self) -> Result<(), ValidationError> {
        if self.asset_type() == AssetType::Main {
            let check_result = check(
                self.asset_address().is_none(),
                format!(
                    "pool contract={} asset address should be null when asset type={:?}",
                    self.address(),
                    self.asset_type()
                )
                .as_str(),
            );
            if check_result.is_err() {
                return Err(ValidationError::new(vec![check_result
                    .unwrap_err()
                    .to_string()]));
            }
        } else {
            let check_result = check(
                self.asset_address().is_some(),
                format!(
                    "pool contract={} asset address should not be null when asset type={:?}",
                    self.address(),
                    self.asset_type()
                )
                .as_str(),
            );
            if check_result.is_err() {
                return Err(ValidationError::new(vec![check_result
                    .unwrap_err()
                    .to_string()]));
            }
        }
        Ok(())
    }
}
