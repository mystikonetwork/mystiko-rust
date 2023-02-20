use std::hash::{Hash};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::raw::bridge::axelar::RawAxelarBridgeConfig;
use crate::raw::bridge::celer::RawCelerBridgeConfig;
use crate::raw::bridge::layer_zero::RawLayerZeroBridgeConfig;
use crate::raw::bridge::poly::RawPolyBridgeConfig;
use crate::raw::bridge::tbridge::RawTBridgeConfig;
use crate::raw::chain::RawChainConfig;
use crate::raw::circuit::RawCircuitConfig;
use crate::raw::indexer::RawIndexerConfig;
use crate::raw::base::{RawConfig, Validator};
use crate::raw::validator::{is_sem_ver, array_unique, validate_nested_vec};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum RawBridgeConfigType {
    Axelar(RawAxelarBridgeConfig),
    Celer(RawCelerBridgeConfig),
    LayerZero(RawLayerZeroBridgeConfig),
    Poly(RawPolyBridgeConfig),
    Tbridge(RawTBridgeConfig),
}

impl Validator for RawBridgeConfigType {
    fn validation(&self) {
        match self {
            RawBridgeConfigType::Axelar(c) => { c.validation() }
            RawBridgeConfigType::Celer(c) => { c.validation() }
            RawBridgeConfigType::LayerZero(c) => { c.validation() }
            RawBridgeConfigType::Poly(c) => { c.validation() }
            RawBridgeConfigType::Tbridge(c) => { c.validation() }
        }
    }
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMystikoConfig {
    #[serde(default)]
    pub base: RawConfig,

    #[validate(custom = "is_sem_ver")]
    pub version: String,

    #[validate(
    custom(function = "array_unique"),
    custom(function = "validate_nested_vec")
    )]
    pub chains: Vec<RawChainConfig>,

    #[validate(
    custom(function = "array_unique"),
    custom(function = "validate_nested_vec")
    )]
    pub bridges: Vec<RawBridgeConfigType>,

    #[validate(
    custom(function = "array_unique"),
    custom(function = "validate_nested_vec")
    )]
    pub circuits: Vec<RawCircuitConfig>,

    #[validate]
    pub indexer: Option<RawIndexerConfig>,
}

impl Validator for RawMystikoConfig {
    fn validation(&self) {
        self.base.validate_object(self);
        for bridge in &self.bridges {
            bridge.validation()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::raw::base::{RawConfig, Validator};
    use crate::raw::bridge::tbridge::RawTBridgeConfig;
    use crate::raw::indexer::RawIndexerConfig;
    use crate::raw::mystiko::{RawBridgeConfigType, RawMystikoConfig};

    async fn default_config() -> RawMystikoConfig {
        RawConfig::create_from_file::<RawMystikoConfig>(
            "src/tests/files/mystiko.valid.json"
        ).await
    }

    #[tokio::test]
    async fn test_valid_success() {
        let config = default_config().await;
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_version_0() {
        let mut config = default_config().await;
        config.version = String::from("");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_version_1() {
        let mut config = default_config().await;
        config.version = String::from("wrong version");
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_chains() {
        let mut config = default_config().await;
        config.chains.append(&mut config.chains.clone());
        config.validation()
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_bridges_0() {
        let mut config = default_config().await;
        config.bridges.append(&mut config.bridges.clone());
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_bridges_1() {
        let mut config = default_config().await;
        let bridge_config = RawTBridgeConfig::new("".to_string());
        config.bridges.push(RawBridgeConfigType::Tbridge(bridge_config));
        println!("{:?}", config.bridges);
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_circuits_0() {
        let mut config = default_config().await;
        config.circuits.append(&mut config.circuits.clone());
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_circuits_1() {
        let mut config = default_config().await;
        let mut circuit_configs = config.circuits;
        circuit_configs[0].name = "".to_string();
        config.circuits = circuit_configs;
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_indexer() {
        let mut config = default_config().await;
        config.indexer = Some(RawIndexerConfig::new("not a url".to_string(), 1000));
        config.validation();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_import_invalid_json_file() {
        let _file_config =
            RawConfig::create_from_file::<RawMystikoConfig>("src/tests/files/mystiko.invalid.json").await;
    }
}
