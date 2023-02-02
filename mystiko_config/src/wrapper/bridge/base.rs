use serde::Serialize;
use crate::raw::bridge::base::RawBridgeConfigTrait;
use crate::wrapper::base::BaseConfig;

pub struct BridgeConfig<T, A = ()>
    where
        T: RawBridgeConfigTrait + Serialize,
{
    pub base: BaseConfig<T, A>,
}

impl<T, A> BridgeConfig<T, A> where
    T: RawBridgeConfigTrait + Serialize, {
    pub fn new(data: T, aux_data: Option<A>) -> Self {
        Self { base: BaseConfig::new(data, aux_data) }
    }

    pub fn name(&self) -> &String {
        &self.base.data.name()
    }
}

