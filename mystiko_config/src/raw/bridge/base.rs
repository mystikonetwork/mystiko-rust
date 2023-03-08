use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::{BridgeType};
use crate::raw::base::{RawConfig, Validator};

pub trait RawBridgeConfigTrait: Validator {
    fn name(&self) -> &String;
    fn bridge_type(&self) -> &BridgeType;
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct RawBridgeConfig {
    #[serde(default)]
    pub base: RawConfig,

    #[serde(rename = "type")]
    pub bridge_type: BridgeType,

    #[validate(length(min = 1))]
    pub name: String,
}

impl RawBridgeConfig {
    pub fn new(
        name: String,
        bridge_type: BridgeType,
    ) -> Self {
        Self {
            base: RawConfig::default(),
            bridge_type,
            name,
        }
    }
}

impl Validator for RawBridgeConfig {
    fn validation(&self) {
        self.base.validate_object(self)
    }
}

impl RawBridgeConfigTrait for RawBridgeConfig {
    fn name(&self) -> &String {
        &self.name
    }

    fn bridge_type(&self) -> &BridgeType {
        &self.bridge_type
    }
}
