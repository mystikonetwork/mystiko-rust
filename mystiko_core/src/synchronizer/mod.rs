mod handler;

pub use handler::*;

use async_trait::async_trait;

#[async_trait]
pub trait SynchronizerHandler<O, S>: Send + Sync {
    type Error;

    async fn chain_synced_block(&self, chain_id: u64) -> Result<Option<u64>, Self::Error>;

    async fn contract_synced_block(&self, chain_id: u64, contract_address: &str) -> Result<Option<u64>, Self::Error>;

    async fn status(&self, with_contracts: bool) -> Result<S, Self::Error>;

    async fn sync(&self, sync_option: O) -> Result<(), Self::Error>;
}
