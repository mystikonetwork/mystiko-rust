use crate::common::BridgeType;
use crate::raw::base::Validator;
use crate::raw::bridge::base::{RawBridgeConfig, RawBridgeConfigTrait};
use serde::{Deserialize, Deserializer, Serialize};
use std::hash::{Hash, Hasher};
use validator::{Validate, ValidationError};

#[derive(Validate, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawAxelarBridgeConfig {
    #[validate]
    #[serde(flatten)]
    pub base: RawBridgeConfig,

    #[serde(rename = "type")]
    #[serde(skip_serializing)]
    #[validate(custom = "validate_bridge_type")]
    pub bridge_type: BridgeType,
}

impl RawAxelarBridgeConfig {
    pub fn new(name: String) -> Self {
        let bridge_type = default_bridge_type();
        Self {
            base: RawBridgeConfig::builder()
                .name(name)
                .bridge_type(bridge_type.clone())
                .build(),
            bridge_type,
        }
    }
}

impl Validator for RawAxelarBridgeConfig {
    fn validation(&self) -> Result<(), anyhow::Error> {
        self.base.base.validate_object(self)
    }
}

impl RawBridgeConfigTrait for RawAxelarBridgeConfig {
    fn name(&self) -> &String {
        &self.base.name
    }

    fn bridge_type(&self) -> &BridgeType {
        &self.bridge_type
    }
}

impl Hash for RawAxelarBridgeConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bridge_type.hash(state)
    }
}

impl<'de> Deserialize<'de> for RawAxelarBridgeConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Inner {
            #[serde(rename = "type")]
            bridge_type: Option<BridgeType>,
            name: String,
        }
        let inner = Inner::deserialize(deserializer)?;
        let bridge_type = inner.bridge_type.unwrap_or(BridgeType::Axelar);
        let base_bridge_type = bridge_type.clone();
        Ok(Self {
            base: RawBridgeConfig {
                base: Default::default(),
                bridge_type: base_bridge_type,
                name: inner.name,
            },
            bridge_type,
        })
    }
}

fn default_bridge_type() -> BridgeType {
    BridgeType::Axelar
}

fn validate_bridge_type(t: &BridgeType) -> Result<(), ValidationError> {
    if *t == BridgeType::Axelar {
        return Ok(());
    }
    Err(ValidationError::new("bridge type error"))
}
