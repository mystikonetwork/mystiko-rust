use crate::provider::factory::{
    DefaultProviderFactory, Provider, ProviderFactory, ProvidersOptions,
};
use anyhow::{Error, Result};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::Mutex;
use typed_builder::TypedBuilder;

pub trait ChainProvidersOptions: Debug {
    fn providers_options(&self, chain_id: u32) -> Option<ProvidersOptions>;
}

#[derive(Debug, TypedBuilder)]
pub struct ProviderPool {
    chain_providers_options: Box<dyn ChainProvidersOptions>,
    #[builder(default = default_provider_factory())]
    provider_factory: Box<dyn ProviderFactory>,
    #[builder(default = default_providers_map(), setter(skip))]
    providers: Mutex<HashMap<u32, Arc<Provider>>>,
}

impl ProviderPool {
    pub async fn get_provider(&self, chain_id: u32) -> Option<Arc<Provider>> {
        let providers = self.providers.lock().await;
        providers.get(&chain_id).cloned()
    }

    pub async fn get_or_create_provider(&mut self, chain_id: u32) -> Result<Arc<Provider>> {
        let mut providers = self.providers.lock().await;
        if let Some(provider) = providers.get(&chain_id) {
            return Ok(provider.clone());
        }
        if let Some(providers_options) = self.chain_providers_options.providers_options(chain_id) {
            let provider: Arc<Provider> = Arc::new(
                self.provider_factory
                    .create_provider(providers_options)
                    .await?,
            );
            providers.insert(chain_id, provider.clone());
            return Ok(provider);
        }
        Err(Error::msg(format!(
            "No provider configuration found for chain id {}",
            chain_id
        )))
    }

    pub async fn has_provider(&self, chain_id: u32) -> bool {
        self.get_provider(chain_id).await.is_some()
    }

    pub async fn check_provider(&self, chain_id: u32) -> Result<Arc<Provider>> {
        match self.get_provider(chain_id).await {
            Some(provider) => Ok(provider),
            None => Err(Error::msg(format!(
                "No provider found for chain id {}",
                chain_id
            ))),
        }
    }

    pub async fn set_provider(&mut self, chain_id: u32, provider: Arc<Provider>) {
        let mut providers = self.providers.lock().await;
        providers.insert(chain_id, provider);
    }

    pub async fn delete_provider(&mut self, chain_id: u32) -> Option<Arc<Provider>> {
        let mut providers = self.providers.lock().await;
        if providers.contains_key(&chain_id) {
            providers.remove(&chain_id)
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

fn default_providers_map() -> Mutex<HashMap<u32, Arc<Provider>>> {
    Mutex::new(HashMap::new())
}
