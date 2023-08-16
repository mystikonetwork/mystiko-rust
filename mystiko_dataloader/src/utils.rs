use anyhow::Result;
use mystiko_ethers::provider::factory::Provider;
use mystiko_ethers::provider::pool::Providers;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn get_provider<P>(providers: &RwLock<P>, chain_id: u64) -> Result<Arc<Provider>>
where
    P: Providers,
{
    let mut optional_provider: Option<Arc<Provider>> = None;
    {
        optional_provider = providers.read().await.get_provider(chain_id);
    }
    let provider = if let Some(provider) = optional_provider {
        provider
    } else {
        providers.write().await.get_or_create_provider(chain_id).await?
    };
    Ok(provider)
}
