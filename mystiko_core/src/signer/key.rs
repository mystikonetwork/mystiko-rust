use crate::TransactionSigner;
use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, TxHash};
use ethers_middleware::MiddlewareBuilder;
use ethers_providers::Middleware;
use ethers_signers::{LocalWallet, Signer};
use mystiko_ethers::Providers;
use std::sync::Arc;
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

impl<P> PrivateKeySigner<P>
where
    P: Providers,
{
    pub fn new<O>(options: O) -> Result<Self>
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
    async fn address(&self) -> Result<Address> {
        Ok(self.wallet.address())
    }

    async fn send_transaction(&self, chain_id: u64, tx: TypedTransaction) -> Result<TxHash> {
        let wallet = self.wallet.clone().with_chain_id(chain_id);
        let provider = self.providers.get_provider(chain_id).await?;
        let client = provider.with_signer(wallet);
        let resp = client.send_transaction(tx, None).await?;
        Ok(resp.tx_hash())
    }
}
