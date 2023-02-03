use serde::Serialize;
use crate::common::ContractType;
use crate::raw::contract::base::RawContractConfigTrait;
use crate::wrapper::base::BaseConfig;

pub struct ContractConfig<T, A = ()>
    where
        T: RawContractConfigTrait + Serialize {
    pub base: BaseConfig<T, A>,
}

impl<T, A> ContractConfig<T, A> where
    T: RawContractConfigTrait + Serialize {
    pub fn new(data: T, aux_data: Option<A>) -> Self {
        Self { base: BaseConfig::new(data, aux_data) }
    }

    pub fn version(&self) -> &u32 {
        &self.base.data.version()
    }

    pub fn name(&self) -> &str {
        &self.base.data.name()
    }

    pub fn address(&self) -> &str {
        &self.base.data.address()
    }

    pub fn contract_type(&self) -> &ContractType {
        &self.base.data.contract_type()
    }

    pub fn start_block(&self) -> &u32 {
        &self.base.data.start_block()
    }

    pub fn event_filter_size(&self) -> &Option<u32> {
        &self.base.data.event_filter_size()
    }

    pub fn indexer_filter_size(&self) -> &Option<u32> {
        &self.base.data.indexer_filter_size()
    }

    pub fn mutate(&self, data: Option<T>, aux_data: Option<A>) -> Self {
        let mut d: T;
        let mut a: A;
        match data {
            None => {
                d = self.base.data.clone();
            }
            Some(value) => {
                d = value;
            }
        }
        match aux_data {
            None => {
                a = self.base.aux_data.clone();
            }
            Some(value) => {
                a = value;
            }
        }

        ContractConfig::new(d, a)
    }
}