use serde::Serialize;
use crate::common::BridgeType;
use crate::raw::bridge::base::RawBridgeConfigTrait;
use crate::wrapper::base::BaseConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct BridgeConfig<T, A = ()>
    where
        T: RawBridgeConfigTrait + Serialize + Clone,
        A: Clone,
{
    pub base: BaseConfig<T, A>,
}

impl<T, A> BridgeConfig<T, A> where
    T: RawBridgeConfigTrait + Serialize + Clone,
    A: Clone
{
    pub fn new(data: T, aux_data: Option<A>) -> Self {
        Self { base: BaseConfig::new(data, aux_data) }
    }

    pub fn name(&self) -> &String {
        &self.base.data.name()
    }

    pub fn bridge_type(&self) -> &BridgeType {
        &self.base.data.bridge_type()
    }

    pub fn mutate(&self, data: Option<T>, aux_data: Option<A>) -> BridgeConfig<T, A> {
        let data = match data {
            None => { self.base.data.clone() }
            Some(value) => { value }
        };
        let aux_data = match aux_data {
            None => {
                self.base.aux_data.clone()
            }
            Some(value) => { Some(value) }
        };
        BridgeConfig::<T, A>::new(data, aux_data)
    }
}
