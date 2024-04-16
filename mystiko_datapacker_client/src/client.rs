use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::Address;
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
pub trait DataPackerClient<D, M>: Send + Sync {
    async fn query_chain(&self, query: &ChainQuery) -> Result<ChainResponse<D>>;

    async fn query_chain_loaded_block(&self, chain_id: u64) -> Result<u64>;

    async fn query_merkle_tree(&self, chain_id: u64, address: &Address) -> Result<Option<M>>;
}

#[async_trait]
impl<D, M, T> DataPackerClient<D, M> for Box<T>
where
    T: DataPackerClient<D, M>,
{
    async fn query_chain(&self, query: &ChainQuery) -> Result<ChainResponse<D>> {
        self.as_ref().query_chain(query).await
    }

    async fn query_chain_loaded_block(&self, chain_id: u64) -> Result<u64> {
        self.as_ref().query_chain_loaded_block(chain_id).await
    }

    async fn query_merkle_tree(&self, chain_id: u64, address: &Address) -> Result<Option<M>> {
        self.as_ref().query_merkle_tree(chain_id, address).await
    }
}
