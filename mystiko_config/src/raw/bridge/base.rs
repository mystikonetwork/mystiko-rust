use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::{BridgeType, validate_object};
use crate::raw::base::{RawConfig, RawConfigTrait};

pub trait RawBridgeConfigTrait: RawConfigTrait {
    fn name(&self) -> &String;
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RawBridgeConfig {
    #[serde(default)]
    pub base: RawConfig,

    #[validate(length(min = 1))]
    pub name: String,
}

impl RawBridgeConfig {
    pub fn new(
        name: String,
    ) -> Self {
        Self {
            base: RawConfig::default(),
            name,
        }
    }
}

impl RawConfigTrait for RawBridgeConfig {
    fn validation(&self) {
        self.base.validate_object(self)
    }
}

impl RawBridgeConfigTrait for RawBridgeConfig {
    fn name(&self) -> &String {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use crate::raw::base::{RawConfig, RawConfigTrait};
    use crate::raw::bridge::base::RawBridgeConfig;

    async fn default_config() -> RawBridgeConfig {
        RawConfig::create_from_object::<RawBridgeConfig>(
            RawBridgeConfig::new("TBridge config".to_string())
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