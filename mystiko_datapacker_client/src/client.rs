use anyhow::Result;
use async_trait::async_trait;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct ChainQuery {
    pub chain_id: u64,
    pub target_block: u64,
    #[builder(default, setter(strip_option))]
    pub start_block: Option<u64>,
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
    async fn query_chain(&self, query: &ChainQuery) -> Result<ChainResponse<D>>;
}

#[async_trait]
impl<D, T> DataPackerClient<D> for Box<T>
where
    T: DataPackerClient<D>,
{
    async fn query_chain(&self, query: &ChainQuery) -> Result<ChainResponse<D>> {
        self.as_ref().query_chain(query).await
    }
}
