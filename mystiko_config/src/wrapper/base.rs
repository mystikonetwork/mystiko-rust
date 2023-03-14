use flamer::flame;
use serde::{Deserialize, Serialize};
use crate::errors::AuxDataError;
use crate::raw::base::Validator;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct BaseConfig<R, A = ()>
    where
        R: Validator + Serialize + Clone,
        A: Clone,
{
    pub data: R,
    pub(crate) aux_data: Option<A>,
}

impl<R, A> BaseConfig<R, A> where
    R: Validator + Serialize + Clone,
    A: Clone,
{
    pub fn new(data: R, aux_data: Option<A>) -> Self {
        Self {
            data,
            aux_data,
        }
    }

    pub fn copy_data(&self) -> R {
        self.data.clone()
    }

    pub fn to_json_string(&self) -> String {
        return serde_json::to_string(&self.data).unwrap();
    }

    pub fn mutate(&self, data: Option<R>, aux_data: Option<A>) -> BaseConfig<R, A> {
        let data = match data {
            None => { self.data.clone() }
            Some(value) => { value }
        };
        let aux_data = match aux_data {
            None => {
                self.aux_data.clone()
            }
            Some(value) => { Some(value) }
        };
        BaseConfig::new(data, aux_data)
    }

    #[flame]
    pub fn aux_data_not_empty(&self) -> Result<&A, AuxDataError> {
        match &self.aux_data {
            None => {
                Err(AuxDataError::new(
                    String::from("aux_data has not been specified")
                ))
            }
            Some(value) => {
                Ok(value)
            }
        }
    }
}
