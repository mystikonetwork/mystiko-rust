use crate::common::BridgeType;
use crate::raw::base::Validator;
use crate::raw::bridge::base::{RawBridgeConfig, RawBridgeConfigTrait};
use crate::raw::chain::EXPLORER_DEFAULT_PREFIX;
use serde::{Deserialize, Deserializer, Serialize};
use std::hash::{Hash, Hasher};
use validator::{Validate, ValidationError};

#[derive(Validate, Serialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RawPolyBridgeConfig {
    #[validate]
    #[serde(flatten)]
    pub base: RawBridgeConfig,

    #[serde(rename = "type")]
    #[serde(skip_serializing)]
    #[validate(custom = "validate_bridge_type")]
    pub bridge_type: BridgeType,

    #[validate(url)]
    pub explorer_url: String,

    #[validate(contains = "%tx%")]
    pub explorer_prefix: String,

    #[validate(url)]
    pub api_url: String,

    #[validate(contains = "%tx%")]
    pub api_prefix: String,
}

impl RawPolyBridgeConfig {
    pub fn new(
        name: String,
        explorer_url: String,
        explorer_prefix: String,
        api_url: String,
        api_prefix: String,
    ) -> Self {
        let bridge_type = default_bridge_type();
        Self {
            base: RawBridgeConfig::builder()
                .name(name)
                .bridge_type(bridge_type.clone())
                .build(),
            bridge_type,
            explorer_url,
            explorer_prefix,
            api_url,
            api_prefix,
        }
    }
}

impl Validator for RawPolyBridgeConfig {
    fn validation(&self) -> Result<(), anyhow::Error> {
        self.base.base.validate_object(self)
    }
}

impl RawBridgeConfigTrait for RawPolyBridgeConfig {
    fn name(&self) -> &String {
        &self.base.name
    }

    fn bridge_type(&self) -> &BridgeType {
        &self.bridge_type
    }
}

impl Hash for RawPolyBridgeConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bridge_type.hash(state)
    }
}

impl<'de> Deserialize<'de> for RawPolyBridgeConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Inner {
            #[serde(rename = "type")]
            bridge_type: Option<BridgeType>,
            name: String,
            explorer_url: String,
            explorer_prefix: Option<String>,
            api_url: String,
            api_prefix: String,
        }
        let inner = Inner::deserialize(deserializer)?;
        let bridge_type = inner.bridge_type.unwrap_or(BridgeType::Poly);
        let base_bridge_type = bridge_type.clone();
        let explorer_prefix = inner
            .explorer_prefix
            .unwrap_or_else(|| EXPLORER_DEFAULT_PREFIX.to_string());
        Ok(Self {
            base: RawBridgeConfig {
                base: Default::default(),
                bridge_type: base_bridge_type,
                name: inner.name,
            },
            bridge_type,
            explorer_url: inner.explorer_url,
            explorer_prefix,
            api_url: inner.api_url,
            api_prefix: inner.api_prefix,
        })
    }
}

fn default_bridge_type() -> BridgeType {
    BridgeType::Poly
}

fn validate_bridge_type(t: &BridgeType) -> Result<(), ValidationError> {
    if *t == BridgeType::Poly {
        return Ok(());
    }
    Err(ValidationError::new("bridge type error"))
}
