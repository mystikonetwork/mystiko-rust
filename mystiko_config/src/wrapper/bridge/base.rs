use serde::Serialize;
use crate::raw::bridge::base::RawBridgeConfigTrait;
use crate::wrapper::base::BaseConfig;

#[derive(Clone, Debug)]
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
}

