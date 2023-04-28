use crate::provider::wrapper::{JsonRpcClientWrapper, JsonRpcParams};
use anyhow::Result;
use async_trait::async_trait;
use ethers_providers::{JsonRpcClient, ProviderError};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub trait FailoverPolicy: Send + Sync + Debug {
    fn should_failover(&self, error: &ProviderError) -> bool;
}

#[derive(Debug, Default)]
pub struct DefaultFailoverPolicy {}

#[derive(Debug)]
pub struct FailoverProvider<T = Box<dyn JsonRpcClientWrapper>, P = Box<dyn FailoverPolicy>> {
    failover_policy: P,
    providers: Vec<T>,
}

#[derive(Debug)]
pub struct FailoverProviderBuilder<T> {
    failover_policy: Option<Box<dyn FailoverPolicy>>,
    providers: Vec<T>,
}

impl FailoverProvider<Box<dyn JsonRpcClientWrapper>> {
    pub fn dyn_rpc() -> FailoverProviderBuilder<Box<dyn JsonRpcClientWrapper>> {
        Self::builder()
    }
}

impl<T> FailoverProvider<T> {
    pub fn builder() -> FailoverProviderBuilder<T> {
        FailoverProviderBuilder::default()
    }

    pub fn providers(&self) -> &[T] {
        &self.providers
    }
}

impl<T> Default for FailoverProviderBuilder<T> {
    fn default() -> Self {
        Self {
            failover_policy: None,
            providers: Vec::new(),
        }
    }
}

impl<T> FailoverProviderBuilder<T> {
    pub fn failover_policy(mut self, policy: Box<dyn FailoverPolicy>) -> Self {
        self.failover_policy = Some(policy);
        self
    }

    pub fn add_provider(mut self, provider: T) -> Self {
        self.providers.push(provider);
        self
    }

    pub fn add_providers(mut self, providers: impl IntoIterator<Item = T>) -> Self {
        for provider in providers {
            self.providers.push(provider);
        }
        self
    }

    pub fn build(self) -> FailoverProvider<T> {
        FailoverProvider {
            failover_policy: self.failover_policy.unwrap_or(Box::<DefaultFailoverPolicy>::default()),
            providers: self.providers,
        }
    }
}

#[async_trait]
impl<C> JsonRpcClient for FailoverProvider<C>
where
    C: JsonRpcClientWrapper,
{
    type Error = ProviderError;

    async fn request<T, R>(&self, method: &str, params: T) -> Result<R, Self::Error>
    where
        T: Debug + Serialize + Send + Sync,
        R: DeserializeOwned + Send,
    {
        let json_rpc_params = if std::mem::size_of::<T>() == 0 {
            JsonRpcParams::Zst
        } else {
            JsonRpcParams::Value(serde_json::to_value(params)?)
        };
        for (index, provider) in self.providers.iter().enumerate() {
            match provider.request(method, json_rpc_params.clone()).await {
                Ok(resp) => return Ok(serde_json::from_value(resp)?),
                Err(err) => {
                    if index < self.providers.len() - 1 && self.failover_policy.should_failover(&err) {
                        continue;
                    } else {
                        return Err(err);
                    }
                }
            }
        }
        Err(ProviderError::CustomError(String::from("no available provider")))
    }
}

impl FailoverPolicy for DefaultFailoverPolicy {
    fn should_failover(&self, error: &ProviderError) -> bool {
        matches!(
            error,
            ProviderError::JsonRpcClientError(_)
                | ProviderError::HTTPError(_)
                | ProviderError::UnsupportedRPC
                | ProviderError::UnsupportedNodeClient
        )
    }
}
