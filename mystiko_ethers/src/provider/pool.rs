use crate::provider::factory::{DefaultProviderFactory, Provider, ProviderFactory, ProvidersOptions};
use anyhow::{Error, Result};
use async_trait::async_trait;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[async_trait]
pub trait ChainProvidersOptions: Debug {
    async fn providers_options(&self, chain_id: u64) -> Result<Option<ProvidersOptions>>;
}

#[derive(Debug, TypedBuilder)]
pub struct ProviderPool {
    chain_providers_options: Box<dyn ChainProvidersOptions>,
    #[builder(default = default_provider_factory())]
    provider_factory: Box<dyn ProviderFactory>,
    #[builder(default = default_providers_map(), setter(skip))]
    providers: HashMap<u64, Arc<Provider>>,
}

impl ProviderPool {
    pub fn get_provider(&self, chain_id: u64) -> Option<Arc<Provider>> {
        self.providers.get(&chain_id).cloned()
    }

    pub async fn get_or_create_provider(&mut self, chain_id: u64) -> Result<Arc<Provider>> {
        if let Some(provider) = self.providers.get(&chain_id) {
            return Ok(provider.clone());
        }
        if let Some(providers_options) = self.chain_providers_options.providers_options(chain_id).await? {
            let provider: Arc<Provider> = Arc::new(self.provider_factory.create_provider(providers_options).await?);
            self.providers.insert(chain_id, provider.clone());
            return Ok(provider);
        }
        Err(Error::msg(format!(
            "No provider configuration found for chain id {}",
            chain_id
        )))
    }

    pub fn has_provider(&self, chain_id: u64) -> bool {
        self.get_provider(chain_id).is_some()
    }

    pub fn check_provider(&self, chain_id: u64) -> Result<Arc<Provider>> {
        match self.get_provider(chain_id) {
            Some(provider) => Ok(provider),
            None => Err(Error::msg(format!("No provider found for chain id {}", chain_id))),
        }
    }

    pub fn set_provider(&mut self, chain_id: u64, provider: Arc<Provider>) {
        self.providers.insert(chain_id, provider);
    }

    pub fn delete_provider(&mut self, chain_id: u64) -> Option<Arc<Provider>> {
        if self.providers.contains_key(&chain_id) {
            self.providers.remove(&chain_id)
        } else {
            None
        }
    }

    pub fn set_provider_factory(&mut self, provider_factory: Box<dyn ProviderFactory>) {
        self.provider_factory = provider_factory;
    }
}

fn default_provider_factory() -> Box<dyn ProviderFactory> {
    Box::<DefaultProviderFactory>::default()
}

fn default_providers_map() -> HashMap<u64, Arc<Provider>> {
    HashMap::new()
}
