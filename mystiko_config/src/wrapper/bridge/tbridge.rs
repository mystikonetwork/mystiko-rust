use crate::raw::bridge::tbridge::RawTBridgeConfig;
use crate::types::BridgeType;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct TBridgeConfig {
    raw: Arc<RawTBridgeConfig>,
}

impl TBridgeConfig {
    pub fn new(raw: Arc<RawTBridgeConfig>) -> Self {
        TBridgeConfig { raw }
    }

    pub fn name(&self) -> &str {
        &self.raw.name
    }

    pub fn bridge_type(&self) -> &BridgeType {
        &self.raw.bridge_type
    }
}
