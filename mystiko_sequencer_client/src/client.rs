use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::Address;

#[async_trait]
pub trait SequencerClient<D, R>: Send + Sync {
    async fn chain_loaded_block(&mut self, chain_id: u64) -> Result<u64>;

    async fn contract_loaded_block(&mut self, chain_id: u64, contract_address: &Address) -> Result<u64>;

    async fn fetch_chain(&mut self, request: D) -> Result<R>;
}

#[async_trait]
impl<D, R, T> SequencerClient<D, R> for Box<T>
where
    T: SequencerClient<D, R>,
    D: Send + 'static,
    R: Send + 'static,
{
    async fn chain_loaded_block(&mut self, chain_id: u64) -> Result<u64> {
        self.chain_loaded_block(chain_id).await
    }

    async fn contract_loaded_block(&mut self, chain_id: u64, contract_address: &Address) -> Result<u64> {
        self.contract_loaded_block(chain_id, contract_address).await
    }

    async fn fetch_chain(&mut self, request: D) -> Result<R> {
        self.fetch_chain(request).await
    }
}
