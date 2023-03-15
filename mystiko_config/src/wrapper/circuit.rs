use crate::common::CircuitType;
use crate::raw::circuit::RawCircuitConfig;
use crate::wrapper::base::BaseConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct CircuitConfig {
    base: BaseConfig<RawCircuitConfig>,
}

impl CircuitConfig {
    pub fn new(data: RawCircuitConfig) -> Self {
        Self {
            base: BaseConfig::new(data, None),
        }
    }

    pub fn data(&self) -> &RawCircuitConfig {
        &self.base.data
    }

    pub fn copy_data(&self) -> RawCircuitConfig {
        self.base.copy_data()
    }

    pub fn to_json_string(&self) -> String {
        self.base.to_json_string()
    }

    pub fn name(&self) -> &String {
        return &self.base.data.name;
    }

    pub fn circuit_type(&self) -> &CircuitType {
        return &self.base.data.circuit_type;
    }

    pub fn is_default(&self) -> bool {
        self.base.data.is_default
    }

    pub fn program_file(&self) -> Vec<String> {
        self.base.data.program_file.clone()
    }

    pub fn abi_file(&self) -> Vec<String> {
        self.base.data.abi_file.clone()
    }

    pub fn proving_key_file(&self) -> Vec<String> {
        self.base.data.proving_key_file.clone()
    }

    pub fn verifying_key_file(&self) -> Vec<String> {
        self.base.data.verifying_key_file.clone()
    }

    pub fn mutate(&self, data: Option<RawCircuitConfig>) -> Self {
        let data = match data {
            None => self.data().clone(),
            Some(value) => value,
        };
        CircuitConfig::new(data)
    }
}
