use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::BridgeType;
use crate::raw::base::RawConfig;
use crate::raw::bridge::base::{RawBridgeConfig, RawBridgeConfigTrait};

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawPolyBridgeConfig {
    base: RawBridgeConfig,
    pub bridge_type: BridgeType,
    pub explorer_url: String,
    pub explorer_prefix: String,
    pub api_url: String,
    pub api_prefix: String,
}

impl RawConfig for RawPolyBridgeConfig {}

impl RawBridgeConfigTrait for RawPolyBridgeConfig {
    fn name(&self) -> &String {
        &self.base.name()
    }
}