mod handler;

pub use handler::*;

use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{TransactionReceipt, TxHash};
use mystiko_types::TransactionType;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct WaitOptions {
    pub chain_id: u64,
    pub tx_hash: TxHash,
    #[builder(default)]
    pub confirmations: Option<u64>,
    #[builder(default)]
    pub timeout_ms: Option<u64>,
    #[builder(default)]
    pub interval_ms: Option<u64>,
}

#[async_trait]
pub trait TransactionHandler<T>: Send + Sync {
    type Error;

    fn create(&self, tx: Option<T>, tx_type: &TransactionType) -> Result<TypedTransaction, Self::Error>;

    async fn wait(&self, options: WaitOptions) -> Result<TransactionReceipt, Self::Error>;
}
