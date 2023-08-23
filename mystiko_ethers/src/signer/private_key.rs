use crate::provider::pool::Providers;
use crate::signer::common;
use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, TxHash};
use ethers_middleware::MiddlewareBuilder;
use ethers_providers::Middleware;
use ethers_signers::{LocalWallet, Signer};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PrivateKeySigner<P: Providers = Box<dyn Providers>> {
    wallet: LocalWallet,
    providers: Arc<P>,
}

impl<P> PrivateKeySigner<P>
where
    P: Providers,
{
    pub fn new(private_key: &str, providers: Arc<P>) -> Result<Self> {
        Ok(Self {
            wallet: private_key.parse::<LocalWallet>()?,
            providers,
        })
    }
}

#[async_trait]
impl<P> common::Signer for PrivateKeySigner<P>
where
    P: Providers,
{
    async fn switch_chain(&mut self, chain_id: u64) -> Result<u64> {
        self.wallet = self.wallet.clone().with_chain_id(chain_id);
        Ok(chain_id)
    }

    async fn chain_id(&self) -> Result<u64> {
        Ok(self.wallet.chain_id())
    }

    async fn accounts(&self) -> Result<Vec<Address>> {
        Ok(vec![self.wallet.address()])
    }

    async fn send_transaction<T: Into<TypedTransaction> + Send + Sync>(&self, tx: T) -> Result<TxHash> {
        let provider = self.providers.get_provider(self.wallet.chain_id()).await?;
        let client = provider.with_signer(self.wallet.clone());
        let resp = client.send_transaction(tx, None).await?;
        Ok(resp.tx_hash())
    }
}
