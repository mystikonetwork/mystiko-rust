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
pub struct RawLayerZeroBridgeConfig {
    #[validate(length(min = 1))]
    pub name: String,

    #[serde(rename = "type")]
    #[serde(default = "default_bridge_type")]
    #[builder(default = default_bridge_type())]
    #[validate(custom = "validate_bridge_type")]
    pub bridge_type: BridgeType,
}

impl RawLayerZeroBridgeConfig {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn bridge_type(&self) -> &BridgeType {
        &self.bridge_type
    }
}

impl Validator for RawLayerZeroBridgeConfig {
    fn validation(&self) -> Result<(), anyhow::Error> {
        validate_raw(self)
    }
}

fn default_bridge_type() -> BridgeType {
    BridgeType::LayerZero
}

fn validate_bridge_type(t: &BridgeType) -> Result<(), ValidationError> {
    if *t == BridgeType::LayerZero {
        return Ok(());
    }
    Err(ValidationError::new("bridge type error"))
}
