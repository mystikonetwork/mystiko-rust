use crate::RawCelerBridgeConfig;
use anyhow::Result;
use mystiko_types::BridgeType;
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

    #[cfg(feature = "proto")]
    pub fn to_proto(&self) -> mystiko_protos::config::bridge::v1::BridgeConfig {
        mystiko_protos::config::bridge::v1::BridgeConfig::builder()
            .name(self.name().to_string())
            .bridge_type(Into::<i32>::into(self.bridge_type()))
            .build()
    }

    pub fn validate(&self) -> Result<()> {
        Ok(self.raw.validate()?)
    }
}
