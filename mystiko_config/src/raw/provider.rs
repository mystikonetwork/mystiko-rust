use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::validate_object;
use crate::raw::base::{RawConfig, RawConfigTrait};

#[derive(Validate, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RawProviderConfig {
    #[serde(default)]
    pub base: RawConfig,

    #[validate(url)]
    pub url: String,

    #[validate(range(min = 1))]
    pub timeout_ms: u32,

    #[validate(range(min = 1))]
    pub max_try_count: u32,
}

impl RawProviderConfig {
    pub fn new(url: String, timeout_ms: u32, max_try_count: u32) -> Self {
        Self { base: RawConfig::default(), url, timeout_ms, max_try_count }
    }
}

impl RawConfigTrait for RawProviderConfig {
    fn validate(&self) {
        self.base.validate_object(self)
    }
}