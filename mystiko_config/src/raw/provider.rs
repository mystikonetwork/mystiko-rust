use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::raw::base::RawConfig;

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawProviderConfig {
    pub url: String,
    pub timeout_ms: u32,
    pub max_try_count: u32,
}

impl RawConfig for RawProviderConfig {}