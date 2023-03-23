use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
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

impl Hash for RawIndexerConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state)
    }
}

fn default_timeout_ms() -> u32 {
    15000
}
