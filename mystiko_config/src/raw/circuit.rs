use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::{CircuitType, validate_object};
use crate::raw::base::{RawConfig, RawConfigTrait};

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawCircuitConfig {
    pub base: RawConfig,
    pub name: String,
    pub circuit_type: CircuitType,
    pub is_default: bool,
    pub program_file: Vec<String>,
    pub abi_file: Vec<String>,
    pub proving_key_file: Vec<String>,
    pub verifying_key_file: Vec<String>,
}

impl RawConfigTrait for RawCircuitConfig {
    fn validate(&self) {
        self.base.validate_object(self)
    }
}