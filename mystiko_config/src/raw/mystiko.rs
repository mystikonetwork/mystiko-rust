use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};
use crate::raw::bridge::axelar::RawAxelarBridgeConfig;
use crate::raw::bridge::celer::RawCelerBridgeConfig;
use crate::raw::bridge::layer_zero::RawLayerZeroBridgeConfig;
use crate::raw::bridge::poly::RawPolyBridgeConfig;
use crate::raw::bridge::tbridge::RawTBridgeConfig;
use crate::raw::chain::RawChainConfig;
use crate::raw::circuit::RawCircuitConfig;
use crate::raw::indexer::RawIndexerConfig;
use crate::common::validate_object;
use crate::raw::base::{RawConfig, RawConfigTrait};
use crate::raw::validator::{is_sem_ver, array_unique, validate_nested_vec};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RawBridgeConfigType {
    RawAxelarBridgeConfig(RawAxelarBridgeConfig),
    RawCelerBridgeConfig(RawCelerBridgeConfig),
    RawLayerZeroBridgeConfig(RawLayerZeroBridgeConfig),
    RawPolyBridgeConfig(RawPolyBridgeConfig),
    RawTBridgeConfig(RawTBridgeConfig),
}

impl RawConfigTrait for RawBridgeConfigType {
    fn validate(&self) {
        match self {
            RawBridgeConfigType::RawAxelarBridgeConfig(c) => { c.validate() }
            RawBridgeConfigType::RawCelerBridgeConfig(c) => { c.validate() }
            RawBridgeConfigType::RawLayerZeroBridgeConfig(c) => { c.validate() }
            RawBridgeConfigType::RawPolyBridgeConfig(c) => { c.validate() }
            RawBridgeConfigType::RawTBridgeConfig(c) => { c.validate() }
        }
    }
}

#[derive(validator::Validate, Serialize, Deserialize, Debug, Clone)]
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

    pub indexer: Option<RawIndexerConfig>,
}

impl RawConfigTrait for RawMystikoConfig {
    fn validate(&self) {
        self.base.validate_object(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::raw::base::{RawConfig, RawConfigTrait};
    use crate::raw::mystiko::RawMystikoConfig;

    async fn default_config() -> RawMystikoConfig {
        RawConfig::create_from_file::<RawMystikoConfig>(
            "src/tests/files/mystiko.valid.json"
        ).await
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_version_0() {
        let mut config = default_config().await;
        config.version = String::from("");
        config.validate();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_version_1() {
        let mut config = default_config().await;
        config.version = String::from("wrong version");
        config.validate();
    }

    //TODO continue impl
}
