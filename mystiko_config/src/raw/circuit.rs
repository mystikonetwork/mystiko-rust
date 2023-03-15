use crate::common::CircuitType;
use crate::errors::ValidationError;
use crate::raw::base::{RawConfig, Validator};
use crate::raw::validator::string_vec_each_not_empty;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use validator::Validate;

fn default_is_default() -> bool {
    false
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawCircuitConfig {
    #[serde(default)]
    pub base: RawConfig,

    #[validate(length(min = 1))]
    pub name: String,

    #[serde(rename = "type")]
    pub circuit_type: CircuitType,

    #[serde(default = "default_is_default")]
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

impl RawCircuitConfig {
    pub fn new(
        name: String,
        circuit_type: CircuitType,
        is_default: bool,
        program_file: Vec<String>,
        abi_file: Vec<String>,
        proving_key_file: Vec<String>,
        verifying_key_file: Vec<String>,
    ) -> Self {
        Self {
            base: RawConfig::default(),
            name,
            circuit_type,
            is_default,
            program_file,
            abi_file,
            proving_key_file,
            verifying_key_file,
        }
    }
}

impl Validator for RawCircuitConfig {
    fn validation(&self) -> Result<(), ValidationError> {
        self.base.validate_object(self)
    }
}

impl Hash for RawCircuitConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}
