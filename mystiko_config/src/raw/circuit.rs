use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::CircuitType;
use crate::raw::base::RawConfig;

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct RawCircuitConfig {
    pub name: String,
    pub circuit_type: CircuitType,
    pub is_default: bool,
    pub program_file: Vec<String>,
    pub abi_file: Vec<String>,
    pub proving_key_file: Vec<String>,
    pub verifying_key_file: Vec<String>,
}

impl RawConfig for RawCircuitConfig {}