use crate::raw::base::{RawConfig, Validator};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::Validate;

#[derive(Validate, Serialize, Deserialize, Debug, Clone, PartialEq, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RawIndexerConfig {
    #[serde(default)]
    #[builder(default)]
    pub base: RawConfig,

    #[validate(url)]
    pub url: String,

    #[validate(range(min = 1))]
    #[serde(default = "default_timeout_ms")]
    pub timeout_ms: u32,
}

impl Validator for RawIndexerConfig {
    fn validation(&self) -> Result<(), anyhow::Error> {
        self.base.validate_object(self)
    }
}

fn default_timeout_ms() -> u32 {
    15000
}
