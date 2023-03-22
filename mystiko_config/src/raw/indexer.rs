use crate::raw::{validate_raw, Validator};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use validator::Validate;

#[derive(TypedBuilder, Validate, Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RawIndexerConfig {
    #[validate(url)]
    pub url: String,

    #[validate(range(min = 1))]
    #[serde(default = "default_timeout_ms")]
    pub timeout_ms: u32,
}

impl Validator for RawIndexerConfig {
    fn validation(&self) -> Result<(), anyhow::Error> {
        validate_raw(self)
    }
}

fn default_timeout_ms() -> u32 {
    15000
}
