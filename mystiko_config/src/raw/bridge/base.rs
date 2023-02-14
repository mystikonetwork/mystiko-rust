use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::{BridgeType, validate_object};
use crate::raw::base::{RawConfig, RawConfigTrait};

pub trait RawBridgeConfigTrait: RawConfigTrait {
    fn name(&self) -> &String;
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RawBridgeConfig {
    #[serde(default)]
    pub base: RawConfig,

    #[validate(length(min = 1))]
    name: String,
}

impl RawBridgeConfig {
    pub fn new(
        name: String,
    ) -> Self {
        Self {
            base: RawConfig::default(),
            name,
        }
    }
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