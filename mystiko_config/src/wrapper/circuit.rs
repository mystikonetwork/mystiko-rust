use crate::common::CircuitType;
use crate::raw::circuit::RawCircuitConfig;
use crate::wrapper::base::BaseConfig;

#[derive(Clone, Debug)]
pub struct CircuitConfig {
    base: BaseConfig<RawCircuitConfig>,
}

impl CircuitConfig {
    pub fn new(data: RawCircuitConfig) -> Self {
        Self {
            base: BaseConfig::new(data, None)
        }
    }

    pub fn name(&self) -> &String {
        return &self.base.data.name;
    }

    pub fn circuit_type(&self) -> &CircuitType {
        return &self.base.data.circuit_type;
    }
}

