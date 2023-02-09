use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::{BridgeType, validate_object};
use crate::raw::base::RawConfigTrait;

pub trait RawBridgeConfigTrait: RawConfigTrait {
    fn name(&self) -> &String;
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawBridgeConfig {
    name: String,
    bridge_type: BridgeType,
}

impl RawConfigTrait for RawBridgeConfig {
    fn validate(&self) -> Result<(), Vec<String>> {
        let result = validate_object(self);
        if result.is_err() {
            return Err(result.unwrap_err());
        }
        Ok(())
    }
}

impl RawBridgeConfigTrait for RawBridgeConfig {
    fn name(&self) -> &String {
        &self.name
    }
}