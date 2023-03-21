use crate::common::BridgeType;
use crate::raw::base::{RawConfig, Validator};
use crate::raw::bridge::axelar::RawAxelarBridgeConfig;
use crate::raw::bridge::celer::RawCelerBridgeConfig;
use crate::raw::bridge::layer_zero::RawLayerZeroBridgeConfig;
use crate::raw::bridge::poly::RawPolyBridgeConfig;
use crate::raw::bridge::tbridge::RawTBridgeConfig;
use crate::raw::chain::RawChainConfig;
use crate::raw::circuit::RawCircuitConfig;
use crate::raw::indexer::RawIndexerConfig;
use crate::raw::validator::{array_unique, is_sem_ver, validate_nested_vec};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum RawBridgeConfigType {
    Axelar(RawAxelarBridgeConfig),
    Celer(RawCelerBridgeConfig),
    LayerZero(RawLayerZeroBridgeConfig),
    Poly(RawPolyBridgeConfig),
    Tbridge(RawTBridgeConfig),
}

impl RawBridgeConfigType {
    pub fn bridge_type(&self) -> &BridgeType {
        match self {
            RawBridgeConfigType::Axelar(conf) => &conf.bridge_type,
            RawBridgeConfigType::Celer(conf) => &conf.bridge_type,
            RawBridgeConfigType::LayerZero(conf) => &conf.bridge_type,
            RawBridgeConfigType::Poly(conf) => &conf.bridge_type,
            RawBridgeConfigType::Tbridge(conf) => &conf.bridge_type,
        }
    }
}

impl Validator for RawBridgeConfigType {
    fn validation(&self) -> Result<(), anyhow::Error> {
        match self {
            RawBridgeConfigType::Axelar(conf) => conf.validation(),
            RawBridgeConfigType::Celer(conf) => conf.validation(),
            RawBridgeConfigType::LayerZero(conf) => conf.validation(),
            RawBridgeConfigType::Poly(conf) => conf.validation(),
            RawBridgeConfigType::Tbridge(conf) => conf.validation(),
        }
    }
}

impl PartialEq for RawBridgeConfigType {
    fn eq(&self, other: &Self) -> bool {
        let type1 = match self {
            RawBridgeConfigType::Axelar(conf) => &conf.bridge_type,
            RawBridgeConfigType::Celer(conf) => &conf.bridge_type,
            RawBridgeConfigType::LayerZero(conf) => &conf.bridge_type,
            RawBridgeConfigType::Poly(conf) => &conf.bridge_type,
            RawBridgeConfigType::Tbridge(conf) => &conf.bridge_type,
        };
        let type2 = match other {
            RawBridgeConfigType::Axelar(conf) => &conf.bridge_type,
            RawBridgeConfigType::Celer(conf) => &conf.bridge_type,
            RawBridgeConfigType::LayerZero(conf) => &conf.bridge_type,
            RawBridgeConfigType::Poly(conf) => &conf.bridge_type,
            RawBridgeConfigType::Tbridge(conf) => &conf.bridge_type,
        };
        type1 == type2
    }
}

impl Hash for RawBridgeConfigType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bridge_type().hash(state)
    }
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    fn validation(&self) -> Result<(), anyhow::Error> {
        let result = self.base.validate_object(self);
        match result {
            Ok(_) => {
                for bridge in &self.bridges {
                    bridge.validation()?
                }
            }
            Err(err) => {
                return Err(err);
            }
        }
        Ok(())
    }
}
