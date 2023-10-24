mod grpc;
mod key;

pub use grpc::*;
pub use key::*;

use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, TxHash};

#[async_trait]
pub trait TransactionSigner: Send + Sync {
    type Error;
    async fn address(&self) -> Result<Address, Self::Error>;

    async fn send_transaction<T>(&self, chain_id: u64, tx: T) -> Result<TxHash, Self::Error>
    where
        T: Into<TypedTransaction> + Send + Sync + 'static;
}
