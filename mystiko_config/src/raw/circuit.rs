use crate::common::CircuitType;
use crate::raw::base::{RawConfig, Validator};
use crate::raw::validator::string_vec_each_not_empty;
use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use typed_builder::TypedBuilder;
use validator::Validate;

#[derive(TypedBuilder, Validate, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RawCircuitConfig {
    #[serde(default)]
    #[builder(default)]
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

impl Validator for RawCircuitConfig {
    fn validation(&self) -> Result<(), Error> {
        self.base.validate_object::<RawCircuitConfig>(self)
    }
}

impl Hash for RawCircuitConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

fn default_is_default() -> bool {
    false
}
