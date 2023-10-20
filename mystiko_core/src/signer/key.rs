use crate::PrivateKeySignerError::GetProviderError;
use crate::TransactionSigner;
use async_trait::async_trait;
use ethers_core::types::{Address, TransactionRequest, TxHash};
use ethers_middleware::signer::SignerMiddlewareError;
use ethers_middleware::MiddlewareBuilder;
use ethers_providers::Middleware;
use ethers_signers::{LocalWallet, Signer, WalletError};
use mystiko_ethers::{Provider, Providers};
use std::sync::Arc;
use thiserror::Error;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PrivateKeySigner<P: Providers> {
    wallet: LocalWallet,
    providers: Arc<P>,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PrivateKeySignerOptions<P: Providers> {
    private_key: String,
    providers: Arc<P>,
}

#[derive(Debug, Error)]
pub enum PrivateKeySignerError {
    #[error(transparent)]
    WalletError(#[from] WalletError),
    #[error(transparent)]
    SignerMiddlewareError(#[from] SignerMiddlewareError<Arc<Provider>, LocalWallet>),
    #[error("failed to get provider of chain_id={0}: {1}")]
    GetProviderError(u64, anyhow::Error),
}

impl<P> PrivateKeySigner<P>
where
    P: Providers,
{
    pub fn new<O>(options: O) -> Result<Self, PrivateKeySignerError>
    where
        O: Into<PrivateKeySignerOptions<P>>,
    {
        let options = options.into();
        Ok(PrivateKeySigner::<P>::builder()
            .wallet(options.private_key.parse::<LocalWallet>()?)
            .providers(options.providers)
            .build())
    }
}

#[async_trait]
impl<P> TransactionSigner for PrivateKeySigner<P>
where
    P: Providers,
{
    type Error = PrivateKeySignerError;

    async fn address(&self) -> Result<Address, Self::Error> {
        Ok(self.wallet.address())
    }

    async fn send_transaction<T>(&self, chain_id: u64, tx: T) -> Result<TxHash, Self::Error>
    where
        T: Into<TransactionRequest> + Send + Sync,
    {
        let wallet = self.wallet.clone().with_chain_id(chain_id);
        let provider = self
            .providers
            .get_provider(chain_id)
            .await
            .map_err(|err| GetProviderError(chain_id, err))?;
        let client = provider.with_signer(wallet);
        let resp = client.send_transaction(tx.into(), None).await?;
        Ok(resp.tx_hash())
    }
}
