use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::validate_object;
use crate::raw::base::RawConfig;

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawProviderConfig {
    pub url: String,
    pub timeout_ms: u32,
    pub max_try_count: u32,
}

impl RawConfig for RawProviderConfig {
    fn validate(&self) -> Result<(), Vec<String>> {
        let result = validate_object(self);
        if result.is_err() {
            return Err(result.unwrap_err());
        }
        Ok(())
    }
}