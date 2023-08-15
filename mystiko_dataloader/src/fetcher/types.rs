use crate::data::ChainResult;
use crate::data::ContractData;
use crate::data::DataType;
use crate::data::LoadedData;
use crate::fetcher::FetcherError;
use async_trait::async_trait;
use mystiko_config::wrapper::contract::ContractConfig;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use std::fmt::Debug;
use std::sync::Arc;
use typed_builder::TypedBuilder;

pub type Result<T> = anyhow::Result<T, FetcherError>;

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

#[derive(Clone, Debug, TypedBuilder)]
pub(crate) struct FetcherLogOptions {
    pub(crate) chain_id: u64,
    pub(crate) address: String,
    pub(crate) start_block: u64,
    pub(crate) end_block: u64,
    pub(crate) data_type: DataType,
}

impl ToString for FetcherLogOptions {
    fn to_string(&self) -> String {
        format!(
            "IndexerFetcher[type={:?}, chain_id={}, address={}, from_block={}, to_block={}]",
            self.data_type, self.chain_id, self.address, self.start_block, self.end_block
        )
    }
}
