use crate::raw::bridge::axelar::RawAxelarBridgeConfig;
use crate::types::BridgeType;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AxelarBridgeConfig {
    raw: Arc<RawAxelarBridgeConfig>,
}

impl AxelarBridgeConfig {
    pub fn new(raw: Arc<RawAxelarBridgeConfig>) -> Self {
        AxelarBridgeConfig { raw }
    }

    pub fn name(&self) -> &str {
        &self.raw.name
    }

    pub fn bridge_type(&self) -> &BridgeType {
        &self.raw.bridge_type
    }
}
