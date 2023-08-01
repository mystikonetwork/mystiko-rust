use crate::data::contract::ContractData;
use crate::data::result::ChainResult;
use crate::data::types::LoadedData;
use crate::filter::ContractFilter;
use anyhow::Result;
use async_trait::async_trait;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use std::fmt::Debug;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone)]
pub enum FetchOption<'a> {
    Chain(&'a ChainFetchOption),
    Contracts(&'a Vec<ContractFetchOption>),
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ChainFetchOption {
    pub config: Arc<MystikoConfig>,
    pub chain_id: u64,
    pub start_block: u64,
    pub end_block: u64,
    pub contract_filter: Option<Arc<Box<dyn ContractFilter>>>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ContractFetchOption {
    pub config: Arc<MystikoConfig>,
    pub chain_id: u64,
    pub address: String,
    pub start_block: u64,
    pub end_block: u64,
}

pub type FetchResult<R> = Result<ChainResult<ContractData<R>>>;

#[async_trait]
pub trait DataFetcher<R: LoadedData>: Send + Sync {
    async fn fetch(&self, option: &FetchOption) -> FetchResult<R>;
}

#[async_trait]
impl<R> DataFetcher<R> for Box<dyn DataFetcher<R>>
where
    R: LoadedData,
{
    async fn fetch(&self, option: &FetchOption) -> FetchResult<R> {
        self.as_ref().fetch(option).await
    }
}
