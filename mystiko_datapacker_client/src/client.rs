use anyhow::Result;
use async_trait::async_trait;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct ContractQuery {
    pub contract_address: String,
    pub target_block: u64,
    #[builder(default, setter(strip_option))]
    pub start_block: Option<u64>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct ChainQuery {
    pub chain_id: u64,
    pub target_block: u64,
    #[builder(default, setter(strip_option))]
    pub start_block: Option<u64>,
    #[builder(default, setter(strip_option))]
    pub contracts: Option<Vec<ContractQuery>>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct ChainResponse<D> {
    pub chain_id: u64,
    #[builder(default)]
    pub chain_data: Option<D>,
}

#[async_trait]
pub trait DataPackerClient<D>: Send + Sync {
    type Err;

    async fn query_chain(&self, query: &ChainQuery) -> Result<ChainResponse<D>, Self::Err>;
}

#[async_trait]
impl<D, T> DataPackerClient<D> for Box<T>
where
    T: DataPackerClient<D>,
{
    type Err = T::Err;

    async fn query_chain(&self, query: &ChainQuery) -> Result<ChainResponse<D>, Self::Err> {
        self.as_ref().query_chain(query).await
    }
}
