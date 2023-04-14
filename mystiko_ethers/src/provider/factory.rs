use crate::provider::enhanced::EnhancedProvider;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub type Provider = ethers_providers::Provider<EnhancedProvider>;

#[async_trait]
pub trait ProviderFactory {
    async fn get_or_create(&self, chain_id: u32) -> Result<Arc<Provider>>;
}
