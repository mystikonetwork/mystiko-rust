use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::validate_object;
use crate::raw::base::{RawConfig, RawConfigTrait};

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawIndexerConfig {
    base: RawConfig,
    url: String,
    timeout_ms: u32,
}

impl RawConfigTrait for RawIndexerConfig {
    fn validate(&self) {
        self.base.validate_object(self)
    }
}