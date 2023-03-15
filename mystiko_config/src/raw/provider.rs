use crate::errors::ValidationError;
use crate::raw::base::{RawConfig, Validator};
use serde::{Deserialize, Serialize};
use validator::Validate;

fn default_timeout_ms() -> u32 {
    5000
}

fn default_max_try_count() -> u32 {
    2
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawProviderConfig {
    #[serde(default)]
    pub base: RawConfig,

    #[validate(url)]
    pub url: String,

    #[validate(range(min = 1))]
    #[serde(default = "default_timeout_ms")]
    pub timeout_ms: u32,

    #[validate(range(min = 1))]
    #[serde(default = "default_max_try_count")]
    pub max_try_count: u32,
}

impl RawProviderConfig {
    pub fn new(url: String, timeout_ms: Option<u32>, max_try_count: Option<u32>) -> Self {
        let timeout_ms = match timeout_ms {
            None => default_timeout_ms(),
            Some(value) => value,
        };
        let max_try_count = match max_try_count {
            None => default_max_try_count(),
            Some(value) => value,
        };
        Self {
            base: RawConfig::default(),
            url,
            timeout_ms,
            max_try_count,
        }
    }
}

impl Validator for RawProviderConfig {
    fn validation(&self) -> Result<(), ValidationError> {
        self.base.validate_object(self)
    }
}
