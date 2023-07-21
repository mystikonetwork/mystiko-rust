use crate::data::chain::ChainData;
use crate::data::raw::RawData;
use anyhow::Result;
use async_trait::async_trait;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use std::fmt::Debug;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ValidateOption {
    pub config: Arc<MystikoConfig>,
}

#[async_trait]
pub trait DataValidator<R: RawData>: Send + Sync {
    async fn validate(&self, data: &ChainData<R>, option: &ValidateOption) -> Result<bool>;
}

#[async_trait]
impl<R> DataValidator<R> for Box<dyn DataValidator<R>>
where
    R: RawData,
{
    async fn validate(&self, data: &ChainData<R>, option: &ValidateOption) -> Result<bool> {
        Ok(self.as_ref().validate(data, option).await?)
    }
}
