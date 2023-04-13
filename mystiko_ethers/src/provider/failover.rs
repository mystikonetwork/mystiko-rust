use crate::provider::failover::FailoverProviderError::TimeoutError;
use anyhow::Result;
use async_trait::async_trait;
use ethers_providers::{
    Http, HttpClientError, JsonRpcClient, JsonRpcError, Provider, ProviderError, RpcError, Ws,
    WsClientError,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::str::FromStr;
use std::time::Duration;
use thiserror::Error;
use tokio::time::timeout;

pub const DEFAULT_TIMEOUT_MS: u32 = 10000;

#[derive(Clone, Debug)]
pub enum RawProvider {
    Http(Http),
    Websocket(Ws),
}

#[derive(Clone, Debug)]
pub struct Failover {
    providers: Vec<(RawProvider, Duration)>,
}

#[derive(Clone, Debug, Default)]
pub struct FailoverProviderBuilder {
    providers: Vec<(String, Duration)>,
}

#[derive(Error, Debug)]
pub enum FailoverProviderError {
    #[error(transparent)]
    HttpProviderError(#[from] HttpClientError),
    #[error(transparent)]
    WebsocketProviderError(#[from] WsClientError),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("timed out after {0:?}")]
    TimeoutError(Duration),
    #[error("no provider is available")]
    NoProviderError,
}

impl FailoverProviderError {
    pub fn is_retryable(&self) -> bool {
        match self {
            FailoverProviderError::HttpProviderError(error) => is_http_error_retryable(error),
            FailoverProviderError::WebsocketProviderError(error) => is_ws_error_retryable(error),
            FailoverProviderError::TimeoutError(_) => true,
            _ => false,
        }
    }
}

impl RpcError for FailoverProviderError {
    fn as_error_response(&self) -> Option<&JsonRpcError> {
        match self {
            FailoverProviderError::HttpProviderError(error) => error.as_error_response(),
            FailoverProviderError::WebsocketProviderError(error) => error.as_error_response(),
            FailoverProviderError::SerdeJsonError(_) => None,
            FailoverProviderError::TimeoutError(_) => None,
            FailoverProviderError::NoProviderError => None,
        }
    }

    fn as_serde_error(&self) -> Option<&serde_json::error::Error> {
        todo!()
    }
}

impl From<FailoverProviderError> for ProviderError {
    fn from(value: FailoverProviderError) -> Self {
        match value {
            FailoverProviderError::HttpProviderError(error) => ProviderError::from(error),
            FailoverProviderError::WebsocketProviderError(error) => ProviderError::from(error),
            FailoverProviderError::SerdeJsonError(error) => ProviderError::SerdeJson(error),
            FailoverProviderError::TimeoutError(error) => {
                ProviderError::CustomError(format!("timed out after {:?}", error))
            }
            FailoverProviderError::NoProviderError => {
                ProviderError::CustomError(String::from("no provider is available"))
            }
        }
    }
}

impl Failover {
    pub fn builder() -> FailoverProviderBuilder {
        FailoverProviderBuilder::default()
    }

    fn new(providers: Vec<(RawProvider, Duration)>) -> Self {
        Self { providers }
    }

    async fn request_http<T, R>(
        &self,
        raw_provider: &Http,
        timeout_duration: &Duration,
        method: &str,
        params: T,
    ) -> Result<R, FailoverProviderError>
    where
        T: Debug + Serialize + Send + Sync,
        R: DeserializeOwned + Send,
    {
        match timeout(
            *timeout_duration,
            raw_provider.request::<T, R>(method, params),
        )
        .await
        {
            Ok(result) => Ok(result?),
            Err(_) => Err(TimeoutError(*timeout_duration)),
        }
    }

    async fn request_ws<T, R>(
        &self,
        raw_provider: &Ws,
        timeout_duration: &Duration,
        method: &str,
        params: T,
    ) -> Result<R, FailoverProviderError>
    where
        T: Debug + Serialize + Send + Sync,
        R: DeserializeOwned + Send,
    {
        match timeout(
            *timeout_duration,
            raw_provider.request::<T, R>(method, params),
        )
        .await
        {
            Ok(result) => Ok(result?),
            Err(_) => Err(TimeoutError(*timeout_duration)),
        }
    }

    async fn request_raw<T, R>(
        &self,
        raw_provider: &RawProvider,
        timeout_duration: &Duration,
        method: &str,
        params: T,
    ) -> Result<R, FailoverProviderError>
    where
        T: Debug + Serialize + Send + Sync,
        R: DeserializeOwned + Send,
    {
        match raw_provider {
            RawProvider::Http(http_provider) => {
                self.request_http(http_provider, timeout_duration, method, params)
                    .await
            }
            RawProvider::Websocket(ws_provider) => {
                self.request_ws(ws_provider, timeout_duration, method, params)
                    .await
            }
        }
    }
}

#[async_trait]
impl JsonRpcClient for Failover {
    type Error = FailoverProviderError;

    async fn request<T, R>(&self, method: &str, params: T) -> Result<R, Self::Error>
    where
        T: Debug + Serialize + Send + Sync,
        R: DeserializeOwned + Send,
    {
        let params_value = serde_json::to_value(params)?;
        for (index, (raw_provider, timeout_duration)) in self.providers.iter().enumerate() {
            let result: Result<R, Self::Error> = self
                .request_raw::<serde_json::Value, R>(
                    raw_provider,
                    timeout_duration,
                    method,
                    params_value.clone(),
                )
                .await;
            match result {
                Ok(response) => {
                    return Ok(response);
                }
                Err(error) => {
                    if index < self.providers.len() - 1 && error.is_retryable() {
                        continue;
                    } else {
                        return Err(error);
                    }
                }
            }
        }
        Err(FailoverProviderError::NoProviderError)
    }
}

impl FailoverProviderBuilder {
    pub fn provider(mut self, url: &str, timeout_ms: Option<u32>) -> Self {
        self.providers.push((
            url.to_string(),
            Duration::from_millis(timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS) as u64),
        ));
        self
    }

    pub fn providers(mut self, provider_urls: &[(&str, Option<u32>)]) -> Self {
        for (provider_url, timeout_ms) in provider_urls {
            self.providers.push((
                provider_url.to_string(),
                Duration::from_millis(timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS) as u64),
            ));
        }
        self
    }

    pub async fn build(self) -> Result<Provider<Failover>> {
        let mut providers: Vec<(RawProvider, Duration)> = vec![];
        for (provider_url, duration) in self.providers.iter() {
            if provider_url.starts_with("http") || provider_url.starts_with("https") {
                providers.push((
                    RawProvider::Http(Http::from_str(provider_url.as_str())?),
                    *duration,
                ));
            } else if provider_url.starts_with("ws") || provider_url.starts_with("wss") {
                providers.push((
                    RawProvider::Websocket(Ws::connect(provider_url.as_str()).await?),
                    *duration,
                ));
            }
        }
        Ok(Provider::<Failover>::new(Failover::new(providers)))
    }
}

fn is_http_error_retryable(error: &HttpClientError) -> bool {
    match error {
        HttpClientError::ReqwestError(_) => true,
        HttpClientError::JsonRpcError(json_rpc_error) => !json_rpc_error.is_revert(),
        _ => false,
    }
}

fn is_ws_error_retryable(error: &WsClientError) -> bool {
    match error {
        WsClientError::JsonRpcError(json_rpc_error) => !json_rpc_error.is_revert(),
        _ => false,
    }
}
