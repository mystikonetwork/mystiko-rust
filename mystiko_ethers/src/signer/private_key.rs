use crate::provider::pool::ProviderPool;
use crate::signer::common;
use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, TxHash};
use ethers_middleware::MiddlewareBuilder;
use ethers_providers::Middleware;
use ethers_signers::{LocalWallet, Signer};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct PrivateKeySigner {
    wallet: LocalWallet,
    providers: Arc<RwLock<ProviderPool>>,
}

impl PrivateKeySigner {
    pub fn new(private_key: &str, providers: Arc<RwLock<ProviderPool>>) -> Result<Self> {
        Ok(Self {
            wallet: private_key.parse::<LocalWallet>()?,
            providers,
        })
    }
}

#[async_trait]
impl common::Signer for PrivateKeySigner {
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
        let provider = self
            .providers
            .write()
            .await
            .get_or_create_provider(self.wallet.chain_id())
            .await?;
        let client = provider.with_signer(self.wallet.clone());
        let resp = client.send_transaction(tx, None).await?;
        Ok(resp.tx_hash())
    }
}
