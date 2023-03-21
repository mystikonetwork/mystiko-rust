use crate::common::CircuitType;
use crate::raw::mystiko::RawMystikoConfig;
use crate::wrapper::base::BaseConfig;
use crate::wrapper::chain::{AuxData, ChainConfig};
use crate::wrapper::circuit::CircuitConfig;
use anyhow::ensure;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub struct MystikoConfig<'a> {
    base: BaseConfig<RawMystikoConfig>,
    default_circuit_configs: HashMap<CircuitType, CircuitConfig>,
    circuit_configs_by_name: HashMap<String, CircuitConfig>,
    chain_configs: HashMap<u32, ChainConfig<'a>>,
}

impl<'a> MystikoConfig<'a> {
    pub fn new(data: RawMystikoConfig) -> anyhow::Result<Self> {
        let base = BaseConfig::builder().data(data).build();

        let mut config = Self {
            base,
            default_circuit_configs: Default::default(),
            circuit_configs_by_name: Default::default(),
            chain_configs: Default::default(),
        };

        config.init_circuit_configs()?;
        config.init_chain_configs()?;

        Ok(config)
    }

    pub fn data(&self) -> &RawMystikoConfig {
        &self.base.data
    }

    fn init_circuit_configs(&mut self) -> anyhow::Result<()> {
        let circuits = &self.base.data.circuits;
        for raw in circuits {
            let circuit_config = CircuitConfig::new(raw.clone());
            if raw.is_default {
                ensure!(
                    !self
                        .default_circuit_configs
                        .contains_key(circuit_config.circuit_type()),
                    "duplicate default circuit type={:?} definition",
                    circuit_config.circuit_type()
                );
                self.default_circuit_configs
                    .insert(*circuit_config.circuit_type(), circuit_config.clone());
            }
            self.circuit_configs_by_name
                .insert(circuit_config.name().clone(), circuit_config);
        }

        Ok(())
    }

    fn init_chain_configs(&mut self) -> anyhow::Result<()> {
        let mut aux_data_chain_configs = HashMap::new();
        let chains = &self.base.data.chains;

        for raw in chains {
            let aux_chain_config = ChainConfig::new(
                raw.clone(),
                // only use for get deposit contract, circuit configs is none
                Some(AuxData::default()),
            )?;
            aux_data_chain_configs.insert(raw.chain_id, aux_chain_config);
        }

        for raw in chains {
            let aux_data = AuxData::new(
                Some(&self.default_circuit_configs),
                Some(&self.circuit_configs_by_name),
                Some(aux_data_chain_configs.clone()),
            );
            let chain_config = ChainConfig::new(raw.clone(), Some(aux_data))?;
            self.chain_configs.insert(raw.chain_id, chain_config);
        }

        Ok(())
    }
}
