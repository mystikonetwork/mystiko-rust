use crate::data::chain::ChainData;
use crate::data::types::LoadedData;
use anyhow::Result;
use async_trait::async_trait;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use std::fmt::Debug;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct HandleOption {
    pub config: Arc<MystikoConfig>,
}

#[async_trait]
pub trait DataHandler<R: LoadedData>: Send + Sync {
    async fn handle(&self, data: &ChainData<R>, option: &HandleOption) -> Result<()>;
}

#[async_trait]
impl<R> DataHandler<R> for Box<dyn DataHandler<R>>
where
    R: LoadedData,
{
    async fn handle(&self, data: &ChainData<R>, option: &HandleOption) -> Result<()> {
        self.as_ref().handle(data, option).await
    }
}
