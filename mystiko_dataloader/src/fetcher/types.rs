use crate::data::contract::ContractData;
use crate::data::raw::RawData;
use crate::filter::ContractFilter;
use anyhow::Result;
use async_trait::async_trait;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use std::fmt::Debug;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ChainFetchOption {
    pub config: Arc<MystikoConfig>,
    pub chain_id: u64,
    pub start_block: u64,
    pub end_block: u64,
    #[builder(setter(strip_option))]
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

pub type FetchResult<R> = Result<Vec<Result<ContractData<R>>>>;

#[async_trait]
pub trait DataFetcher<R: RawData>: Send + Sync {
    async fn fetch_chain(&self, option: &ChainFetchOption) -> FetchResult<R>;

    async fn fetch_contracts(&self, options: &[ContractFetchOption]) -> FetchResult<R>;
}

#[async_trait]
impl<R> DataFetcher<R> for Box<dyn DataFetcher<R>>
where
    R: RawData,
{
    async fn fetch_chain(&self, option: &ChainFetchOption) -> FetchResult<R> {
        Ok(self.as_ref().fetch_chain(option).await?)
    }

    async fn fetch_contracts(&self, options: &[ContractFetchOption]) -> FetchResult<R> {
        Ok(self.as_ref().fetch_contracts(options).await?)
    }
}
