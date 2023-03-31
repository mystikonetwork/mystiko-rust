use crate::raw::bridge::celer::RawCelerBridgeConfig;
use crate::types::BridgeType;
use anyhow::Result;
use std::sync::Arc;
use validator::Validate;

#[derive(Clone, Debug)]
pub struct CelerBridgeConfig {
    raw: Arc<RawCelerBridgeConfig>,
}

impl CelerBridgeConfig {
    pub fn new(raw: Arc<RawCelerBridgeConfig>) -> Self {
        CelerBridgeConfig { raw }
    }

    pub fn name(&self) -> &str {
        &self.raw.name
    }

    pub fn bridge_type(&self) -> &BridgeType {
        &self.raw.bridge_type
    }

    pub fn validate(&self) -> Result<()> {
        Ok(self.raw.validate()?)
    }
}
