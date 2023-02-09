use serde::{Deserialize, Serialize};
use crate::raw::base::RawConfigTrait;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseConfig<R, A = ()>
    where
        R: RawConfigTrait + Serialize + Clone,
        A: Clone,
{
    pub data: R,
    pub aux_data: Option<A>,
}

impl<R, A> BaseConfig<R, A> where
    R: RawConfigTrait + Serialize + Clone,
    A: Clone,
{
    pub fn new(data: R, aux_data: Option<A>) -> Self {
        Self {
            data,
            aux_data,
        }
    }

    pub fn to_json_string(&self) -> String {
        return serde_json::to_string(&self.data).unwrap();
    }

    pub fn aux_data_not_empty(&self) -> A {
        match &self.aux_data {
            None => {
                panic!("aux_data has not been specified")
            }
            Some(value) => {
                value.clone()
            }
        }
    }
}
