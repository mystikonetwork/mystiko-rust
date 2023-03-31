use crate::raw::circuit::RawCircuitConfig;
use crate::types::CircuitType;
use anyhow::Result;
use std::sync::Arc;
use validator::Validate;

#[derive(Clone, Debug)]
pub struct CircuitConfig {
    raw: Arc<RawCircuitConfig>,
}

impl CircuitConfig {
    pub fn new(raw: Arc<RawCircuitConfig>) -> Self {
        CircuitConfig { raw }
    }

    pub fn name(&self) -> &str {
        &self.raw.name
    }

    pub fn circuit_type(&self) -> &CircuitType {
        &self.raw.circuit_type
    }

    pub fn is_default(&self) -> bool {
        self.raw.is_default
    }

    pub fn program_file(&self) -> &Vec<String> {
        &self.raw.program_file
    }

    pub fn abi_file(&self) -> &Vec<String> {
        &self.raw.abi_file
    }

    pub fn proving_key_file(&self) -> &Vec<String> {
        &self.raw.proving_key_file
    }

    pub fn verifying_key_file(&self) -> &Vec<String> {
        &self.raw.verifying_key_file
    }

    pub fn validate(&self) -> Result<()> {
        Ok(self.raw.validate()?)
    }
}
