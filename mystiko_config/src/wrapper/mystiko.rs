use std::collections::HashMap;
use std::rc::Rc;
use anyhow::ensure;
use crate::common::CircuitType;
use crate::raw::mystiko::RawMystikoConfig;
use crate::wrapper::base::BaseConfig;
use crate::wrapper::circuit::CircuitConfig;

#[derive(Clone, PartialEq, Debug)]
pub struct MystikoConfig {
    base: BaseConfig<RawMystikoConfig>,
    default_circuit_configs: HashMap<CircuitType, Rc<CircuitConfig>>,
    circuit_configs_by_name: HashMap<String, Rc<CircuitConfig>>,
}

impl MystikoConfig {
    pub fn new(data: RawMystikoConfig) -> anyhow::Result<Self> {
        let base = BaseConfig::builder()
            .data(data)
            .build();
        let mut config = Self {
            base,
            default_circuit_configs: Default::default(),
            circuit_configs_by_name: Default::default(),
        };
        config.init_circuit_configs()?;

        Ok(config)
    }

    pub fn data(&self) -> &RawMystikoConfig {
        &self.base.data
    }

    fn init_circuit_configs(&mut self) -> anyhow::Result<()> {
        let mut default_circuit_configs = HashMap::new();
        let mut circuit_config_by_names = HashMap::new();

        let circuits = &self.base.data.circuits;
        for raw in circuits {
            let circuit_config = Rc::new(
                CircuitConfig::new(raw.clone())
            );
            if raw.is_default {
                ensure!(
                    !default_circuit_configs.contains_key(circuit_config.circuit_type()),
                    "duplicate default circuit type={:?} definition",
                    circuit_config.circuit_type()
                );
                default_circuit_configs.insert(
                    *circuit_config.circuit_type(),
                    circuit_config.clone(),
                );
            }
            circuit_config_by_names.insert(
                circuit_config.name().clone(),
                circuit_config,
            );
        }

        self.default_circuit_configs = default_circuit_configs;
        self.circuit_configs_by_name = circuit_config_by_names;

        Ok(())
    }
}