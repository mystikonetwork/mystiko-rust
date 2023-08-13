use crate::data::contract::ContractData;
use crate::data::result::ChainResult;
use crate::data::types::LoadedData;
use crate::fetcher::error::FetcherError;
use async_trait::async_trait;
use mystiko_config::wrapper::contract::ContractConfig;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use std::fmt::Debug;
use std::sync::Arc;
use typed_builder::TypedBuilder;

pub type Result<T> = anyhow::Result<T, FetcherError>;

#[derive(Clone, TypedBuilder)]
pub struct LogPrefixOptions {
    pub chain_id: u64,
    pub address: String,
    pub start_block: u64,
    pub end_block: u64,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct FetchOptions {
    pub config: Arc<MystikoConfig>,
    pub chain_id: u64,
    pub start_block: u64,
    pub target_block: u64,
    pub contract_options: Option<Vec<ContractFetchOptions>>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ContractFetchOptions {
    pub contract_config: ContractConfig,
    pub start_block: Option<u64>,
    pub target_block: Option<u64>,
}

pub type FetchResult<R> = Result<ChainResult<ContractData<R>>>;

#[async_trait]
pub trait DataFetcher<R: LoadedData>: Send + Sync {
    async fn fetch(&self, option: &FetchOptions) -> FetchResult<R>;
}

#[async_trait]
impl<R> DataFetcher<R> for Box<dyn DataFetcher<R>>
where
    R: LoadedData,
{
    async fn fetch(&self, option: &FetchOptions) -> FetchResult<R> {
        self.as_ref().fetch(option).await
    }
}
