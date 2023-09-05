use crate::{DefaultProviderFactory, Provider, ProviderFactory, ProvidersOptions};
use anyhow::{Error, Result};
use async_trait::async_trait;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::RwLock;
use typed_builder::TypedBuilder;

#[async_trait]
pub trait ChainProvidersOptions: Debug + Send + Sync {
    async fn providers_options(&self, chain_id: u64) -> Result<Option<ProvidersOptions>>;
}

#[async_trait]
pub trait Providers: Debug + Send + Sync {
    async fn get_provider(&self, chain_id: u64) -> Result<Arc<Provider>>;

    async fn has_provider(&self, chain_id: u64) -> bool;

    async fn set_provider(&self, chain_id: u64, provider: Arc<Provider>) -> Option<Arc<Provider>>;

    async fn delete_provider(&self, chain_id: u64) -> Option<Arc<Provider>>;
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProviderPool<
    O: ChainProvidersOptions = Box<dyn ChainProvidersOptions>,
    F: ProviderFactory = Box<dyn ProviderFactory>,
> {
    chain_providers_options: O,
    #[builder(default)]
    provider_factory: Option<F>,
    #[builder(default, setter(skip))]
    providers: RwLock<HashMap<u64, Arc<Provider>>>,
}

impl<O, F> ProviderPool<O, F>
where
    O: ChainProvidersOptions,
    F: ProviderFactory,
{
    pub fn set_provider_factory(&mut self, provider_factory: F) {
        self.provider_factory = Some(provider_factory);
    }
}

#[async_trait]
impl<O, F> Providers for ProviderPool<O, F>
where
    O: ChainProvidersOptions,
    F: ProviderFactory,
{
    async fn get_provider(&self, chain_id: u64) -> Result<Arc<Provider>> {
        let provider = self.providers.read().await.get(&chain_id).cloned();
        if let Some(provider) = provider {
            return Ok(provider);
        }
        let mut write_guard = self.providers.write().await;
        if let Some(provider) = write_guard.get(&chain_id).cloned() {
            return Ok(provider);
        }
        if let Some(options) = self.chain_providers_options.providers_options(chain_id).await? {
            let provider = if let Some(factory) = &self.provider_factory {
                Arc::new(factory.create_provider(options).await?)
            } else {
                Arc::new(DefaultProviderFactory.create_provider(options).await?)
            };
            write_guard.insert(chain_id, provider.clone());
            Ok(provider)
        } else {
            Err(Error::msg(format!("No provider options for chain id {}", chain_id)))
        }
    }

    async fn has_provider(&self, chain_id: u64) -> bool {
        self.providers.read().await.contains_key(&chain_id)
    }

    async fn set_provider(&self, chain_id: u64, provider: Arc<Provider>) -> Option<Arc<Provider>> {
        self.providers.write().await.insert(chain_id, provider)
    }

    async fn delete_provider(&self, chain_id: u64) -> Option<Arc<Provider>> {
        self.providers.write().await.remove(&chain_id)
    }
}

#[async_trait]
impl ChainProvidersOptions for Box<dyn ChainProvidersOptions> {
    async fn providers_options(&self, chain_id: u64) -> Result<Option<ProvidersOptions>> {
        self.as_ref().providers_options(chain_id).await
    }
}

#[async_trait]
impl Providers for Box<dyn Providers> {
    async fn get_provider(&self, chain_id: u64) -> Result<Arc<Provider>> {
        self.as_ref().get_provider(chain_id).await
    }

    async fn has_provider(&self, chain_id: u64) -> bool {
        self.as_ref().has_provider(chain_id).await
    }

    async fn set_provider(&self, chain_id: u64, provider: Arc<Provider>) -> Option<Arc<Provider>> {
        self.as_ref().set_provider(chain_id, provider).await
    }

    async fn delete_provider(&self, chain_id: u64) -> Option<Arc<Provider>> {
        self.as_ref().delete_provider(chain_id).await
    }
}
