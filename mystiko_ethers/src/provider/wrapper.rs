use async_trait::async_trait;
use ethers_providers::{JsonRpcClient, ProviderError};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum JsonRpcParams {
    Value(Value),
    Zst,
}

#[async_trait]
pub trait JsonRpcClientWrapper: Send + Sync + Debug {
    async fn request(&self, method: &str, params: JsonRpcParams) -> Result<Value, ProviderError>;
}

#[derive(Debug)]
pub struct ProviderWrapper<T> {
    inner: T,
}

#[async_trait]
impl<C: JsonRpcClient> JsonRpcClientWrapper for C {
    async fn request(&self, method: &str, params: JsonRpcParams) -> Result<Value, ProviderError> {
        let fut = if let JsonRpcParams::Value(params) = params {
            JsonRpcClient::request(self, method, params)
        } else {
            JsonRpcClient::request(self, method, ())
        };

        Ok(fut.await.map_err(C::Error::into)?)
    }
}

#[async_trait]
impl JsonRpcClientWrapper for Box<dyn JsonRpcClientWrapper> {
    async fn request(&self, method: &str, params: JsonRpcParams) -> Result<Value, ProviderError> {
        self.as_ref().request(method, params).await
    }
}

impl<T> ProviderWrapper<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl<P> JsonRpcClient for ProviderWrapper<P>
where
    P: JsonRpcClientWrapper,
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
        match self.inner.request(method, json_rpc_params).await {
            Ok(value) => Ok(serde_json::from_value(value)?),
            Err(err) => Err(err),
        }
    }
}
