use std::collections::HashMap;
use std::str::FromStr;
use num_bigint::BigInt;
use mystiko_utils::convert::from_decimals;
use crate::common::{AssetType, BridgeType, CircuitType};
use crate::raw::contract::base::RawContractConfigTrait;
use crate::raw::contract::pool::RawPoolContractConfig;
use crate::wrapper::asset::AssetConfig;
use crate::wrapper::circuit::CircuitConfig;
use crate::wrapper::contract::base::ContractConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct AuxData {
    default_circuit_configs: HashMap<CircuitType, CircuitConfig>,
    circuit_configs_by_name: HashMap<String, CircuitConfig>,
    main_asset_config: AssetConfig,
    asset_configs: HashMap<String, AssetConfig>,
}

impl AuxData {
    pub fn new(
        default_circuit_configs: HashMap<CircuitType, CircuitConfig>,
        circuit_configs_by_name: HashMap<String, CircuitConfig>,
        main_asset_config: AssetConfig,
        asset_configs: HashMap<String, AssetConfig>,
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
    pub base: ContractConfig<RawPoolContractConfig, AuxData>,
    pub circuit_configs: HashMap<CircuitType, CircuitConfig>,
    pub main_asset_config: AssetConfig,
    pub asset_config: Option<AssetConfig>,
}

impl PoolContractConfig {
    pub fn new(data: RawPoolContractConfig, aux_data: Option<AuxData>) -> Self {
        let contract_config = ContractConfig::new(data, aux_data);
        let config = Self {
            base: contract_config.clone(),
            circuit_configs: PoolContractConfig::init_circuits_configs(
                &contract_config,
                contract_config.base.aux_data_not_empty().default_circuit_configs,
                contract_config.base.aux_data_not_empty().circuit_configs_by_name,
            ),
            main_asset_config: contract_config.base.aux_data_not_empty().main_asset_config,
            asset_config: PoolContractConfig::init_asset_config(
                &contract_config,
                contract_config.base.aux_data_not_empty().asset_configs,
            ),
        };
        config.validate();
        config
    }

    pub fn pool_name(&self) -> &str {
        &self.base.base.data.pool_name
    }

    pub fn bridge_type(&self) -> &BridgeType {
        &self.base.base.data.bridge_type
    }

    pub fn asset(&self) -> AssetConfig {
        match &self.asset_config {
            None => {
                self.main_asset_config.clone()
            }
            Some(value) => {
                value.clone()
            }
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
        BigInt::from_str(
            &self.base.base.data.min_rollup_fee,
        ).unwrap()
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

    pub fn mutate(&self, data: Option<RawPoolContractConfig>, aux_data: Option<AuxData>) -> Self {
        let aux_data = match aux_data {
            None => { self.base.base.aux_data.clone() }
            Some(_) => { aux_data }
        };
        let data = match data {
            None => { self.base.base.data.clone() }
            Some(value) => { value }
        };
        PoolContractConfig::new(data, aux_data)
    }

    fn init_circuits_configs(
        base: &ContractConfig<RawPoolContractConfig, AuxData>,
        default_circuit_configs: HashMap<CircuitType, CircuitConfig>,
        circuit_configs_by_name: HashMap<String, CircuitConfig>,
    ) -> HashMap<CircuitType, CircuitConfig> {
        let mut circuit_configs: HashMap<CircuitType, CircuitConfig> = HashMap::new();
        for (_, circuit_conf) in default_circuit_configs {
            circuit_configs.insert(
                circuit_conf.circuit_type().clone(),
                circuit_conf,
            );
        }
        for circuit_name in &base.base.data.circuits {
            let circuit_conf = circuit_configs_by_name.get(circuit_name);
            match circuit_conf {
                None => {}
                Some(value) => {
                    circuit_configs.insert(
                        value.circuit_type().clone(),
                        value.clone(),
                    );
                }
            }
        }
        circuit_configs
    }

    fn init_asset_config(
        base: &ContractConfig<RawPoolContractConfig, AuxData>,
        asset_configs: HashMap<String, AssetConfig>,
    ) -> Option<AssetConfig> {
        match &base.base.data.asset_address {
            None => {
                None
            }
            Some(value) => {
                let asset_config = asset_configs.get(value);
                Some(
                    asset_config.expect(
                        format!(
                            "asset address={:?} config has not been defined for pool contract address={:?}",
                            value,
                            base.base.data.address()
                        ).as_str()
                    ).clone()
                )
            }
        }
    }

    fn validate(&self) {}
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use serde_json::Value::Array;
    use crate::common::{AssetType, CircuitType};
    use crate::raw::asset::RawAssetConfig;
    use crate::raw::base::RawConfig;
    use crate::raw::contract::pool::RawPoolContractConfig;
    use crate::raw::mystiko::RawMystikoConfig;
    use crate::wrapper::asset::AssetConfig;
    use crate::wrapper::circuit::CircuitConfig;
    use crate::wrapper::contract::pool::{AuxData, PoolContractConfig};

    async fn raw_mystiko_config() -> RawMystikoConfig {
        RawConfig::create_from_file::<RawMystikoConfig>("src/tests/files/mystiko.valid.json").await
    }

    async fn circuit_configs_by_name() -> HashMap<String, CircuitConfig> {
        let raw_mystiko_config = raw_mystiko_config().await;
        let mut configs = HashMap::new();
        for circuit in raw_mystiko_config.circuits {
            let circuit_config = CircuitConfig::new(circuit.clone());
            configs.insert(circuit.name.clone(), circuit_config);
        }
        configs
    }

    async fn default_circuit_configs(circuit_configs_by_name: &HashMap<String, CircuitConfig>) -> HashMap<CircuitType, CircuitConfig> {
        let mut default_configs = HashMap::new();
        for (_, circuit_config) in circuit_configs_by_name {
            if circuit_config.is_default() {
                default_configs.insert(circuit_config.circuit_type().clone(), circuit_config.clone());
            }
        }
        default_configs
    }

    async fn raw_config() -> RawPoolContractConfig {
        let circuit_configs_by_name = circuit_configs_by_name().await;
        let default_circuit_configs = default_circuit_configs(&circuit_configs_by_name);
        RawConfig::create_from_file::<RawPoolContractConfig>("src/tests/files/contract/pool.valid.json").await
    }

    async fn main_asset_config() -> AssetConfig {
        let raw_mystiko_config = raw_mystiko_config().await;
        AssetConfig::new(RawAssetConfig::new(
            AssetType::Main,
            raw_mystiko_config.chains.get(0).unwrap().asset_symbol.clone(),
            raw_mystiko_config.chains.get(0).unwrap().asset_decimals.clone(),
            "0x0000000000000000000000000000000000000000".to_string(),
            raw_mystiko_config.chains.get(0).unwrap().recommended_amounts.clone(),
        ))
    }

    async fn asset_configs() -> HashMap<String, AssetConfig> {
        let raw_mystiko_config = raw_mystiko_config().await;
        HashMap::from_iter(
            [(
                raw_mystiko_config.chains.get(0).unwrap().assets.get(0).unwrap().asset_address.clone(),
                AssetConfig::new(raw_mystiko_config.chains.get(0).unwrap().assets.get(0).unwrap().clone())
            )]
        )
    }

    async fn config() -> PoolContractConfig {
        let raw_config = raw_config().await;
        let circuit_configs_by_names = circuit_configs_by_name().await;
        PoolContractConfig::new(raw_config, Some(
            AuxData::new(
                default_circuit_configs(&circuit_configs_by_names).await,
                circuit_configs_by_names,
                main_asset_config().await,
                asset_configs().await,
            )
        ))
    }

    #[tokio::test]
    async fn test_equality() {
        let config = config().await;
        let raw_config = raw_config().await;
        let asset_configs = asset_configs().await;
        let circuit_configs_by_name =
            circuit_configs_by_name().await;
        let default_circuit_configs =
            default_circuit_configs(&circuit_configs_by_name).await;
        assert_eq!(config.pool_name(), &raw_config.pool_name);
        assert_eq!(config.bridge_type(), &raw_config.bridge_type);
        assert_eq!(&config.asset(), asset_configs.get("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a").unwrap());
        assert_eq!(&config.asset_type(), config.asset().asset_type());
        assert_eq!(config.asset_symbol(), config.asset().asset_symbol());
        assert_eq!(config.asset_decimals(), config.asset().asset_decimals());
        assert_eq!(config.asset_address().unwrap(), raw_config.asset_address.unwrap());
        assert_eq!(config.recommended_amounts(), config.asset().recommended_amounts());
        assert_eq!(config.recommended_amounts_number(), config.asset().recommended_amounts_number());
        assert_eq!(config.min_rollup_fee().to_string(), raw_config.min_rollup_fee);
        assert_eq!(config.min_rollup_fee_number(), 12f64);
    }

    #[tokio::test]
    async fn test_asset_address_is_none() {
        let mut raw_config = raw_config().await;
        raw_config.asset_address = None;
        let circuit_configs_by_name =
            circuit_configs_by_name().await;
        let default_circuit_configs =
            default_circuit_configs(&circuit_configs_by_name).await;
        let main_asset_config = main_asset_config().await;
        let asset_configs = asset_configs().await;
        let config = PoolContractConfig::new(raw_config, Some(
            AuxData::new(
                default_circuit_configs,
                circuit_configs_by_name,
                main_asset_config.clone(),
                asset_configs,
            )
        ));
        assert_eq!(config.asset(), main_asset_config);
    }

    #[tokio::test]
    async fn test_circuit_overwrite() {
        let mut config = config().await;
        assert_eq!(config.circuit_config(CircuitType::Rollup1).unwrap().name(), "zokrates-1.0-rollup1");
        let mut raw_config = raw_config().await;
        raw_config.circuits = vec![String::from("zokrates-2.0-rollup1")];
        let circuit_configs_by_name =
            circuit_configs_by_name().await;
        let default_circuit_configs =
            default_circuit_configs(&circuit_configs_by_name).await;
        let main_asset_config = main_asset_config().await;
        let asset_configs = asset_configs().await;
        config = PoolContractConfig::new(
            raw_config,
            Some(
                AuxData::new(
                    default_circuit_configs,
                    circuit_configs_by_name,
                    main_asset_config,
                    asset_configs,
                ),
            ),
        );
        assert_eq!(config.circuit_config(CircuitType::Rollup1).unwrap().name(), "zokrates-2.0-rollup1");
        assert_eq!(config.circuit_config(CircuitType::Rollup4).unwrap().name(), "zokrates-1.0-rollup4");
    }

    #[tokio::test]
    async fn test_copy() {
        let config = config().await;
        let raw_config = raw_config().await;
        let circuit_configs_by_name =
            circuit_configs_by_name().await;
        let default_circuit_configs =
            default_circuit_configs(&circuit_configs_by_name).await;
        let main_asset_config = main_asset_config().await;
        let asset_configs = asset_configs().await;
        assert_eq!(
            PoolContractConfig::new(raw_config, Some(
                AuxData::new(
                    default_circuit_configs,
                    circuit_configs_by_name,
                    main_asset_config,
                    asset_configs,
                )
            )),
            config
        )
    }

    #[tokio::test]
    async fn test_mutate() {
        let config = config().await;
        assert_eq!(config.mutate(None, None), config);
        let mut raw_config = raw_config().await;
        raw_config.base.name = "another name".to_string();
        let circuit_configs_by_name =
            circuit_configs_by_name().await;
        let default_circuit_configs =
            default_circuit_configs(&circuit_configs_by_name).await;
        let main_asset_config = main_asset_config().await;
        let asset_configs = asset_configs().await;
        let new_config = config.mutate(Some(raw_config.clone()), Some(
            AuxData::new(
                default_circuit_configs,
                circuit_configs_by_name,
                main_asset_config,
                asset_configs,
            )
        ));
        assert_eq!(new_config.base.base.copy_data(), raw_config);
    }

    #[tokio::test]
    async fn test_to_json_string() {
        let config = config().await;
        let json_string = config.base.base.to_json_string();
        let loaded_raw_config =
            RawConfig::create_from_json_string::<RawPoolContractConfig>(json_string.as_str()).await;
        assert_eq!(loaded_raw_config, raw_config().await);
    }
}
