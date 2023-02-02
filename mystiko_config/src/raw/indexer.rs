use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct RawIndexerConfig {
    url: String,
    timeout_ms: u32,
}