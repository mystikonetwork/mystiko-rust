use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::BridgeType;
use crate::raw::base::RawConfig;

pub trait RawBridgeConfigTrait: RawConfig {
    fn name(&self) -> &String;
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawBridgeConfig {
    name: String,
    bridge_type: BridgeType,
}

impl RawConfig for RawBridgeConfig {}

impl RawBridgeConfigTrait for RawBridgeConfig {
    fn name(&self) -> &String {
        &self.name
    }
}