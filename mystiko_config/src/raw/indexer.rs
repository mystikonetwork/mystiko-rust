use crate::errors::ValidationError;
use crate::raw::base::{RawConfig, Validator};
use serde::{Deserialize, Serialize};
use validator::Validate;

fn default_timeout_ms() -> u32 {
    15000
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RawIndexerConfig {
    #[serde(default)]
    pub base: RawConfig,

    #[validate(url)]
    pub url: String,

    #[validate(range(min = 1))]
    #[serde(default = "default_timeout_ms")]
    pub timeout_ms: u32,
}

impl RawIndexerConfig {
    pub fn new(url: String, timeout_ms: u32) -> Self {
        Self {
            base: RawConfig::default(),
            url,
            timeout_ms,
        }
    }
}

impl Validator for RawIndexerConfig {
    fn validation(&self) -> Result<(), ValidationError> {
        self.base.validate_object(self)
    }
}
