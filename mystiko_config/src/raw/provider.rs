use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct RawProviderConfig {
    url: String,
    timeout_ms: u32,
    max_try_count: u32,
}