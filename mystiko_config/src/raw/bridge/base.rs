use crate::common::BridgeType;
use crate::raw::base::{RawConfig, Validator};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::Validate;

pub trait RawBridgeConfigTrait: Validator {
    fn name(&self) -> &String;
    fn bridge_type(&self) -> &BridgeType;
}

#[derive(TypedBuilder, Validate, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct RawBridgeConfig {
    #[serde(default)]
    #[builder(default)]
    pub base: RawConfig,

    #[serde(rename = "type")]
    pub bridge_type: BridgeType,

    #[validate(length(min = 1))]
    pub name: String,
}

impl Validator for RawBridgeConfig {
    fn validation(&self) -> Result<(), anyhow::Error> {
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
