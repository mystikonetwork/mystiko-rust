use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::{BridgeType};
use crate::raw::base::{RawConfig, Validator};

pub trait RawBridgeConfigTrait: Validator {
    fn name(&self) -> &String;
    fn bridge_type(&self) -> &BridgeType;
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct RawBridgeConfig {
    #[serde(default)]
    pub base: RawConfig,

    #[serde(rename = "type")]
    pub bridge_type: BridgeType,

    #[validate(length(min = 1))]
    pub name: String,
}

impl RawBridgeConfig {
    pub fn new(
        name: String,
        bridge_type: BridgeType,
    ) -> Self {
        Self {
            base: RawConfig::default(),
            bridge_type,
            name,
        }
    }
}

impl Validator for RawBridgeConfig {
    fn validation(&self) {
        self.base.validate_object(self)
    }
}

impl RawBridgeConfigTrait for RawBridgeConfig {
    fn name(&self) -> &String {
        &self.name
    }

    fn bridge_type(&self) -> &BridgeType {
        &self.bridge_type
    }
}

#[cfg(test)]
mod tests {
    use crate::common::BridgeType;
    use crate::raw::base::{RawConfig, Validator};
    use crate::raw::bridge::base::RawBridgeConfig;

    async fn default_config() -> RawBridgeConfig {
        RawConfig::create_from_object::<RawBridgeConfig>(
            RawBridgeConfig::new("TBridge config".to_string(), BridgeType::Tbridge)
        ).await
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_name() {
        let mut config = default_config().await;
        config.name = "".to_string();
        config.validation();
    }
}