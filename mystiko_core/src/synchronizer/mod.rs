mod handler;
mod loader;

pub use handler::*;
pub use loader::*;

use async_trait::async_trait;

#[async_trait]
pub trait SynchronizerHandler<O, S, R>: Send + Sync {
    type Error;

    async fn chain_synced_block(&self, chain_id: u64) -> Result<Option<u64>, Self::Error>;

    async fn contract_synced_block(&self, chain_id: u64, contract_address: &str) -> Result<Option<u64>, Self::Error>;

    async fn status(&self, with_contracts: bool) -> Result<S, Self::Error>;

    async fn sync(&self, sync_option: O) -> Result<(), Self::Error>;

    async fn reset(&self, reset_options: R) -> Result<(), Self::Error>;
}
