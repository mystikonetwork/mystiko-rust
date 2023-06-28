use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, TxHash};
use std::fmt::Debug;

#[async_trait]
pub trait Signer: Send + Sync + Debug {
    async fn switch_chain(&mut self, chain_id: u64) -> Result<u64>;
    async fn chain_id(&self) -> Result<u64>;
    async fn accounts(&self) -> Result<Vec<Address>>;
    async fn send_transaction<T: Into<TypedTransaction> + Send + Sync>(&self, tx: T) -> Result<TxHash>;
}
