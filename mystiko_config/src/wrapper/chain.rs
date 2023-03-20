use std::collections::HashMap;
use validator::Validate;
use crate::common::CircuitType;
use crate::raw::chain::RawChainConfig;
use crate::wrapper::base::BaseConfig;
use crate::wrapper::circuit::CircuitConfig;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AuxData<'a> {
    default_circuit_configs: Option<&'a HashMap<CircuitType, CircuitConfig>>,
    circuit_configs_by_name: Option<&'a HashMap<String, CircuitConfig>>,
    chain_configs: Option<&'a HashMap<u32, ChainConfig<'a>>>,
}

impl<'a> AuxData<'a> {
    pub fn new(
        default_circuit_configs: Option<&'a HashMap<CircuitType, CircuitConfig>>,
        circuit_configs_by_name: Option<&'a HashMap<String, CircuitConfig>>,
        chain_configs: Option<&'a HashMap<u32, ChainConfig>>,
    ) -> Self {
        Self {
            default_circuit_configs,
            circuit_configs_by_name,
            chain_configs,
        }
    }
}

#[derive(Validate, Clone, Debug, PartialEq)]
pub struct ChainConfig<'a> {
    base: BaseConfig<RawChainConfig, AuxData<'a>>,
}