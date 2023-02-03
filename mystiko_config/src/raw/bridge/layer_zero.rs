use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::BridgeType;
use crate::raw::base::RawConfig;
use crate::raw::bridge::base::{RawBridgeConfig, RawBridgeConfigTrait};

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawLayerZeroBridgeConfig {
    base: RawBridgeConfig,
    pub bridge_type: BridgeType,
}

impl RawConfig for RawLayerZeroBridgeConfig {}

impl RawBridgeConfigTrait for RawLayerZeroBridgeConfig {
    fn name(&self) -> &String {
        &self.base.name()
    }
}