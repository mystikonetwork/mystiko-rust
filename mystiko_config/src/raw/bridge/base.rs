use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::{BridgeType, validate_object};
use crate::raw::base::{RawConfig, RawConfigTrait};

pub trait RawBridgeConfigTrait: RawConfigTrait {
    fn name(&self) -> &String;
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawBridgeConfig {
    pub base: RawConfig,
    name: String,
    bridge_type: BridgeType,
}

impl RawConfigTrait for RawBridgeConfig {
    fn validate(&self) {
        self.base.validate_object(self)
    }
}

impl RawBridgeConfigTrait for RawBridgeConfig {
    fn name(&self) -> &String {
        &self.name
    }
}