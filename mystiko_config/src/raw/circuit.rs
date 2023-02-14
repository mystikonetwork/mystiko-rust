use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::{CircuitType, validate_object};
use crate::raw::base::{RawConfig, RawConfigTrait};
use crate::raw::validator::{string_vec_each_not_empty};

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawCircuitConfig {
    pub base: RawConfig,
    pub name: String,
    pub circuit_type: CircuitType,
    pub is_default: bool,

    #[validate(custom = "string_vec_each_not_empty")]
    pub program_file: Vec<String>,

    #[validate(custom = "string_vec_each_not_empty")]
    pub abi_file: Vec<String>,

    #[validate(custom = "string_vec_each_not_empty")]
    pub proving_key_file: Vec<String>,

    #[validate(custom = "string_vec_each_not_empty")]
    pub verifying_key_file: Vec<String>,
}

impl RawConfigTrait for RawCircuitConfig {
    fn validate(&self) {
        self.base.validate_object(self)
    }
}