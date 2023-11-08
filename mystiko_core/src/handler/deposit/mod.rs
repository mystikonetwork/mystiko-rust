mod v1;

pub use v1::*;

use crate::TransactionSigner;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait DepositHandler<D, QO, Q, C, DS, S>: Send + Sync {
    type Error;

    async fn quote(&self, options: QO) -> Result<Q, Self::Error>;

    async fn summary(&self, options: C) -> Result<DS, Self::Error>;

    async fn create(&self, options: C) -> Result<D, Self::Error>;

    async fn send(&self, options: S) -> Result<D, Self::Error>;

    async fn send_with_signer<Signer>(&self, options: S, signer: Arc<Signer>) -> Result<D, Self::Error>
    where
        Signer: TransactionSigner + 'static;
}
