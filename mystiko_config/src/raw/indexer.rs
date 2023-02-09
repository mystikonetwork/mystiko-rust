use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::validate_object;
use crate::raw::base::RawConfigTrait;

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawIndexerConfig {
    url: String,
    timeout_ms: u32,
}

impl RawConfigTrait for RawIndexerConfig {
    fn validate(&self) -> Result<(), Vec<String>> {
        let result = validate_object(self);
        if result.is_err() {
            return Err(result.unwrap_err());
        }
        Ok(())
    }
}