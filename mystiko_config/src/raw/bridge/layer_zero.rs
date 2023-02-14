use std::hash::{Hash, Hasher};
use std::os::macos::raw::stat;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use crate::common::{BridgeType, validate_object};
use crate::raw::base::RawConfigTrait;
use crate::raw::bridge::base::{RawBridgeConfig, RawBridgeConfigTrait};

fn default_bridge_type() -> BridgeType {
    BridgeType::LayerZero
}

fn validate_bridge_type(t: &BridgeType) -> Result<(), ValidationError> {
    if *t == BridgeType::LayerZero {
        return Ok(());
    }
    Err(ValidationError::new("bridge type error"))
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawLayerZeroBridgeConfig {
    #[serde(flatten)]
    base: RawBridgeConfig,

    #[serde(default = "default_bridge_type")]
    #[serde(rename = "type")]
    #[validate(custom = "validate_bridge_type")]
    pub bridge_type: BridgeType,
}

impl RawLayerZeroBridgeConfig {
    pub fn new(name: String) -> Self {
        Self {
            base: RawBridgeConfig::new(name),
            bridge_type: default_bridge_type(),
        }
    }
}

impl RawConfigTrait for RawLayerZeroBridgeConfig {
    fn validate(&self) {
        self.base.base.validate_object(self)
    }
}

impl RawBridgeConfigTrait for RawLayerZeroBridgeConfig {
    fn name(&self) -> &String {
        &self.base.name()
    }
}

impl Hash for RawLayerZeroBridgeConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bridge_type.hash(state)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::BridgeType;
    use crate::raw::base::{RawConfig, RawConfigTrait};
    use crate::raw::bridge::layer_zero::RawLayerZeroBridgeConfig;

    async fn default_config() -> RawLayerZeroBridgeConfig {
        RawConfig::create_from_object::<RawLayerZeroBridgeConfig>(
            RawLayerZeroBridgeConfig::new(String::from("LayerZero Bridge"))
        ).await
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_type() {
        let mut config = default_config().await;
        config.bridge_type = BridgeType::Tbridge;
        config.validate();
    }

    #[tokio::test]
    async fn test_import_valid_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawLayerZeroBridgeConfig>("src/tests/files/bridge/layerZero.valid.json").await;
        assert_eq!(file_config, default_config().await);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_import_invalid_json_file() {
        let file_config =
            RawConfig::create_from_file::<RawLayerZeroBridgeConfig>("src/tests/files/bridge/layerZero.invalid.json").await;
    }
}