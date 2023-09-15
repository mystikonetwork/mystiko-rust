use crate::RawTBridgeConfig;
use anyhow::Result;
use mystiko_types::BridgeType;
use std::sync::Arc;
use validator::Validate;

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

    #[cfg(feature = "proto")]
    pub fn to_proto(&self) -> mystiko_protos::config::bridge::v1::BridgeConfig {
        mystiko_protos::config::bridge::v1::BridgeConfig::builder()
            .name(self.name().to_string())
            .bridge_type(Into::<mystiko_protos::common::v1::BridgeType>::into(self.bridge_type()))
            .build()
    }

    pub fn validate(&self) -> Result<()> {
        Ok(self.raw.validate()?)
    }
}
