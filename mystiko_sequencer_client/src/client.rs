use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::Address;

#[async_trait]
pub trait SequencerClient<D, R>: Send + Sync {
    type Error;

    async fn chain_loaded_block(&self, chain_id: u64) -> Result<u64, Self::Error>;

    async fn contract_loaded_block(&self, chain_id: u64, contract_address: &Address) -> Result<u64, Self::Error>;

    async fn fetch_chain(&self, request: D) -> Result<R, Self::Error>;

    async fn health_check(&self) -> Result<(), Self::Error>;
}
