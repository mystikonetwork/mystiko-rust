use crate::common::BridgeType;
use crate::raw::{validate_raw, Validator};
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use typed_builder::TypedBuilder;
use validator::{Validate, ValidationError};

#[derive(
    TypedBuilder, Validate, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, Hash,
)]
#[serde(rename_all = "camelCase")]
pub struct RawAxelarBridgeConfig {
    #[validate(length(min = 1))]
    pub name: String,

    #[serde(rename = "type")]
    #[serde(default = "default_bridge_type")]
    #[builder(default = default_bridge_type())]
    #[validate(custom = "validate_bridge_type")]
    pub bridge_type: BridgeType,
}

impl Validator for RawAxelarBridgeConfig {
    fn validation(&self) -> Result<(), anyhow::Error> {
        validate_raw(self)
    }
}

impl RawAxelarBridgeConfig {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn bridge_type(&self) -> &BridgeType {
        &self.bridge_type
    }
}

fn default_bridge_type() -> BridgeType {
    BridgeType::Axelar
}

fn validate_bridge_type(t: &BridgeType) -> Result<(), ValidationError> {
    if *t == BridgeType::Axelar {
        return Ok(());
    }
    Err(ValidationError::new("bridge type error"))
}
