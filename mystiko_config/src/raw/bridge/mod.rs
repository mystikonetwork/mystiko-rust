use crate::raw::bridge::axelar::RawAxelarBridgeConfig;
use crate::raw::bridge::celer::RawCelerBridgeConfig;
use crate::raw::bridge::layer_zero::RawLayerZeroBridgeConfig;
use crate::raw::bridge::poly::RawPolyBridgeConfig;
use crate::raw::bridge::tbridge::RawTBridgeConfig;
use crate::types::BridgeType;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use validator::{Validate, ValidationErrors};

pub mod axelar;
pub mod celer;
pub mod layer_zero;
pub mod poly;
pub mod tbridge;

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum RawBridgeConfig {
    Axelar(Arc<RawAxelarBridgeConfig>),
    Celer(Arc<RawCelerBridgeConfig>),
    LayerZero(Arc<RawLayerZeroBridgeConfig>),
    Poly(Arc<RawPolyBridgeConfig>),
    Tbridge(Arc<RawTBridgeConfig>),
}

impl RawBridgeConfig {
    pub fn bridge_type(&self) -> &BridgeType {
        match self {
            RawBridgeConfig::Axelar(conf) => &conf.bridge_type,
            RawBridgeConfig::Celer(conf) => &conf.bridge_type,
            RawBridgeConfig::LayerZero(conf) => &conf.bridge_type,
            RawBridgeConfig::Poly(conf) => &conf.bridge_type,
            RawBridgeConfig::Tbridge(conf) => &conf.bridge_type,
        }
    }
}

impl Validate for RawBridgeConfig {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            RawBridgeConfig::Axelar(conf) => conf.validate(),
            RawBridgeConfig::Celer(conf) => conf.validate(),
            RawBridgeConfig::LayerZero(conf) => conf.validate(),
            RawBridgeConfig::Poly(conf) => conf.validate(),
            RawBridgeConfig::Tbridge(conf) => conf.validate(),
        }
    }
}

impl Hash for RawBridgeConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            RawBridgeConfig::Axelar(conf) => conf.hash(state),
            RawBridgeConfig::Celer(conf) => conf.hash(state),
            RawBridgeConfig::LayerZero(conf) => conf.hash(state),
            RawBridgeConfig::Poly(conf) => conf.hash(state),
            RawBridgeConfig::Tbridge(conf) => conf.hash(state),
        }
    }
}

impl PartialEq for RawBridgeConfig {
    fn eq(&self, other: &Self) -> bool {
        self.bridge_type() == other.bridge_type()
    }
}
