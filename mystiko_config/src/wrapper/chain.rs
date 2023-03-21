use crate::common::CircuitType;
use crate::raw::chain::RawChainConfig;
use crate::wrapper::base::BaseConfig;
use crate::wrapper::circuit::CircuitConfig;
use std::collections::HashMap;
use validator::Validate;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AuxData<'a> {
    default_circuit_configs: Option<&'a HashMap<CircuitType, CircuitConfig>>,
    circuit_configs_by_name: Option<&'a HashMap<String, CircuitConfig>>,
    chain_configs: Option<HashMap<u32, ChainConfig<'a>>>,
}

impl<'a> AuxData<'a> {
    pub fn new(
        default_circuit_configs: Option<&'a HashMap<CircuitType, CircuitConfig>>,
        circuit_configs_by_name: Option<&'a HashMap<String, CircuitConfig>>,
        chain_configs: Option<HashMap<u32, ChainConfig<'a>>>,
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

impl<'a> ChainConfig<'a> {
    pub fn new(data: RawChainConfig, aux_data: Option<AuxData<'a>>) -> anyhow::Result<Self> {
        let base_config = BaseConfig::builder().data(data).aux_data(aux_data).build();
        Ok(Self { base: base_config })
    }
}
