use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::{BridgeType, validate_object};
use crate::raw::base::RawConfigTrait;
use crate::raw::bridge::base::{RawBridgeConfig, RawBridgeConfigTrait};

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawCelerBridgeConfig {
    pub base: RawBridgeConfig,
    pub bridge_type: BridgeType,
}

impl RawConfigTrait for RawCelerBridgeConfig {
    fn validate(&self) -> Result<(), Vec<String>> {
        let result = validate_object(self);
        if result.is_err() {
            return Err(result.unwrap_err());
        }
        Ok(())
    }
}

impl RawBridgeConfigTrait for RawCelerBridgeConfig {
    fn name(&self) -> &String {
        &self.base.name()
    }
}