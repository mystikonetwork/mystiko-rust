use crate::raw::base::Validator;
use anyhow::bail;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Serialize, Debug, Clone, PartialEq, Default)]
pub struct BaseConfig<R, A = ()>
where
    R: Validator + Serialize + Clone,
    A: Clone,
{
    pub data: R,
    #[builder(default = None)]
    pub(crate) aux_data: Option<A>,
}

impl<R, A> BaseConfig<R, A>
where
    R: Validator + Serialize + Clone,
    A: Clone,
{
    pub fn copy_data(&self) -> R {
        self.data.clone()
    }

    pub fn to_json_string(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string(&self.data)?)
    }

    pub fn mutate(&self, data: Option<&R>, aux_data: Option<&A>) -> BaseConfig<R, A> {
        let data = match data {
            None => self.data.clone(),
            Some(value) => value.clone(),
        };
        let aux_data = match aux_data {
            None => self.aux_data.clone(),
            Some(value) => Some(value.clone()),
        };
        BaseConfig::builder().data(data).aux_data(aux_data).build()
    }

    pub fn aux_data_not_empty(&self) -> Result<&A, anyhow::Error> {
        match &self.aux_data {
            None => {
                bail!("aux_data has not been specified")
            }
            Some(value) => Ok(value),
        }
    }
}
