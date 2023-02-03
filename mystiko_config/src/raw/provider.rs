use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::raw::base::RawConfig;

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawProviderConfig {
    url: String,
    timeout_ms: u32,
    max_try_count: u32,
}

impl RawConfig for RawProviderConfig {}