use crate::raw::base::{RawConfig, Validator};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::Validate;

#[derive(TypedBuilder, Validate, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawProviderConfig {
    #[serde(default)]
    #[builder(default)]
    pub base: RawConfig,

    #[validate(url)]
    pub url: String,

    #[validate(range(min = 1))]
    #[serde(default = "default_timeout_ms")]
    #[builder(default = default_timeout_ms())]
    pub timeout_ms: u32,

    #[validate(range(min = 1))]
    #[serde(default = "default_max_try_count")]
    #[builder(default = default_max_try_count())]
    pub max_try_count: u32,
}

impl Validator for RawProviderConfig {
    fn validation(&self) -> Result<(), anyhow::Error> {
        self.base.validate_object(self)
    }
}

fn default_timeout_ms() -> u32 {
    5000
}

fn default_max_try_count() -> u32 {
    2
}
