use crate::wrapper::base::BaseConfig;
use crate::common::CircuitType;
use crate::raw::circuit::RawCircuitConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct CircuitConfig<'a> {
    base: BaseConfig<&'a RawCircuitConfig>,
}

impl<'a> CircuitConfig<'a> {
    pub fn new(data: &'a RawCircuitConfig) -> Self {
        Self {
            base: BaseConfig::builder()
                .data(data)
                .build(),
        }
    }

    pub fn data(&self) -> &'a RawCircuitConfig {
        &self.base.data
    }

    pub fn copy_data(&self) -> RawCircuitConfig {
        self.base.copy_data().clone()
    }

    pub fn to_json_string(&self) -> anyhow::Result<String> {
        self.base.to_json_string()
    }

    pub fn name(&self) -> &String {
        &self.base.data.name
    }

    pub fn circuit_type(&self) -> &CircuitType {
        &self.base.data.circuit_type
    }

    pub fn is_default(&self) -> bool {
        self.base.data.is_default
    }

    pub fn program_file(&self) -> &Vec<String> {
        &self.base.data.program_file
    }

    pub fn abi_file(&self) -> &Vec<String> {
        &self.base.data.abi_file
    }

    pub fn proving_key_file(&self) -> &Vec<String> {
        &self.base.data.proving_key_file
    }

    pub fn verifying_key_file(&self) -> &Vec<String> {
        &self.base.data.verifying_key_file
    }

    pub fn mutate(&self, data: Option<&'a RawCircuitConfig>) -> Self {
        let data = match data {
            None => self.data(),
            Some(value) => value,
        };
        CircuitConfig::new(data)
    }
}