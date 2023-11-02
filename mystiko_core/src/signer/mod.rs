mod grpc;
mod key;

pub use grpc::*;
pub use key::*;

use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, TxHash};

#[async_trait]
pub trait TransactionSigner: Send + Sync {
    async fn address(&self) -> Result<Address>;

    async fn send_transaction(&self, chain_id: u64, tx: TypedTransaction) -> Result<TxHash>;
}

#[async_trait]
impl TransactionSigner for Box<dyn TransactionSigner> {
    async fn address(&self) -> Result<Address> {
        self.as_ref().address().await
    }

    async fn send_transaction(&self, chain_id: u64, tx: TypedTransaction) -> Result<TxHash> {
        self.as_ref().send_transaction(chain_id, tx).await
    }
}
