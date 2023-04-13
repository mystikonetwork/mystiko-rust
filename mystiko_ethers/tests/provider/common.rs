use async_trait::async_trait;
use ethers_providers::{JsonRpcClient, ProviderError};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::fmt::Debug;

#[derive(Debug)]
pub struct TestProvider {
    pub error: Option<fn() -> ProviderError>,
    pub result: Value,
}

#[async_trait]
impl JsonRpcClient for TestProvider {
    type Error = ProviderError;

    async fn request<T, R>(&self, _method: &str, _params: T) -> Result<R, Self::Error>
    where
        T: Debug + Serialize + Send + Sync,
        R: DeserializeOwned + Send,
    {
        if let Some(error) = &self.error {
            Err(error())
        } else {
            Ok(serde_json::from_value(self.result.clone())?)
        }
    }
}

impl Default for TestProvider {
    fn default() -> Self {
        Self {
            error: None,
            result: serde_json::from_str("null").unwrap(),
        }
    }
}
