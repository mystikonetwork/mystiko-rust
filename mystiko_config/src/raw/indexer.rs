use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::raw::base::RawConfig;

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawIndexerConfig {
    url: String,
    timeout_ms: u32,
}

impl RawConfig for RawIndexerConfig {}