use crate::provider::failover::FailoverProvider;
use crate::provider::types::{ProviderOptions, QuorumProviderOptions};
use crate::provider::wrapper::{JsonRpcClientWrapper, ProviderWrapper};
use crate::provider::ws::WsWithTimeout;
use anyhow::Result;
use async_trait::async_trait;
use ethers_providers::{
    Http, HttpRateLimitRetryPolicy, Quorum, QuorumProvider, RetryClient, RetryClientBuilder, WeightedProvider,
};
use lazy_static::lazy_static;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Client, Url};
use std::fmt::Debug;

lazy_static! {
    pub static ref HTTP_REGEX: regex::Regex = regex::Regex::new(r"^http(s)?://").unwrap();
    pub static ref WS_REGEX: regex::Regex = regex::Regex::new(r"^ws(s)?://").unwrap();
}

pub type Provider = ethers_providers::Provider<ProviderWrapper<Box<dyn JsonRpcClientWrapper>>>;

pub enum ProvidersOptions {
    Failover(Vec<ProviderOptions>),
    Quorum(Vec<ProviderOptions>, QuorumProviderOptions),
    Http(ProviderOptions),
    Ws(ProviderOptions),
}

#[async_trait]
pub trait ProviderFactory: Debug + Send + Sync {
    async fn create_provider(&self, providers_options: ProvidersOptions) -> Result<Provider>;
}

#[derive(Debug, Default)]
pub struct DefaultProviderFactory;

impl DefaultProviderFactory {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl ProviderFactory for DefaultProviderFactory {
    async fn create_provider(&self, providers_options: ProvidersOptions) -> Result<Provider> {
        match providers_options {
            ProvidersOptions::Failover(options) => {
                let failover_provider = create_raw_failover_provider(options).await?;
                Ok(ethers_providers::Provider::new(ProviderWrapper::new(Box::new(
                    failover_provider,
                ))))
            }
            ProvidersOptions::Quorum(options, quorum_options) => {
                let quorum_provider = create_raw_quorum_provider(options, Some(quorum_options)).await?;
                Ok(ethers_providers::Provider::new(ProviderWrapper::new(Box::new(
                    quorum_provider,
                ))))
            }
            ProvidersOptions::Http(options) => {
                let http_provider = create_raw_http_provider(options)?;
                Ok(ethers_providers::Provider::new(ProviderWrapper::new(Box::new(
                    http_provider,
                ))))
            }
            ProvidersOptions::Ws(options) => {
                let ws_provider = create_raw_ws_provider(options).await?;
                Ok(ethers_providers::Provider::new(ProviderWrapper::new(Box::new(
                    ws_provider,
                ))))
            }
        }
    }
}

async fn create_raw_failover_provider(options: Vec<ProviderOptions>) -> Result<FailoverProvider> {
    let mut failover_provider_builder = FailoverProvider::dyn_rpc();
    for provider_options in options.into_iter() {
        if HTTP_REGEX.is_match(&provider_options.url) {
            let inner_provider = create_raw_http_provider(provider_options)?;
            failover_provider_builder = failover_provider_builder.add_provider(Box::new(inner_provider));
        } else if WS_REGEX.is_match(&provider_options.url) {
            let inner_provider = create_raw_ws_provider(provider_options).await?;
            failover_provider_builder = failover_provider_builder.add_provider(Box::new(inner_provider));
        }
    }
    Ok(failover_provider_builder.build())
}

async fn create_raw_quorum_provider(
    providers_options: Vec<ProviderOptions>,
    quorum_options: Option<QuorumProviderOptions>,
) -> Result<QuorumProvider> {
    let quorum_options = quorum_options.unwrap_or_default();
    let mut builder = QuorumProvider::dyn_rpc();
    for provider_options in providers_options.into_iter() {
        let weight = provider_options.quorum_weight.unwrap_or(1);
        if HTTP_REGEX.is_match(&provider_options.url) {
            let inner_provider = create_raw_http_provider(provider_options)?;
            builder = builder.add_provider(WeightedProvider::with_weight(Box::new(inner_provider), weight));
        } else if WS_REGEX.is_match(&provider_options.url) {
            let inner_provider = create_raw_ws_provider(provider_options).await?;
            builder = builder.add_provider(WeightedProvider::with_weight(Box::new(inner_provider), weight));
        }
    }
    builder = builder.quorum(quorum_options.quorum.unwrap_or(Quorum::Majority));
    Ok(builder.build())
}

fn create_raw_http_provider(options: ProviderOptions) -> Result<RetryClient<Http>> {
    let mut client_builder = Client::builder();
    if let Some(authorization) = &options.authorization {
        let mut auth_value = HeaderValue::from_str(&authorization.to_string())?;
        auth_value.set_sensitive(true);
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, auth_value);
        client_builder = client_builder.default_headers(headers);
    }
    if let Some(request_timeout) = options.request_timeout {
        client_builder = client_builder.timeout(request_timeout);
    }
    let client = client_builder.build()?;
    let http_provider = Http::new_with_client(Url::parse(&options.url)?, client);
    let mut retry_provider_builder = RetryClientBuilder::default();
    if let Some(timeout_retries) = options.timeout_retries {
        retry_provider_builder = retry_provider_builder.timeout_retries(timeout_retries);
    }
    if let Some(rate_limit_retries) = options.rate_limit_retries {
        retry_provider_builder = retry_provider_builder.rate_limit_retries(rate_limit_retries);
    }
    if let Some(initial_backoff) = options.initial_backoff {
        retry_provider_builder = retry_provider_builder.initial_backoff(initial_backoff);
    }
    if let Some(compute_units_per_second) = options.compute_units_per_second {
        retry_provider_builder = retry_provider_builder.compute_units_per_second(compute_units_per_second);
    }
    let provider = if let Some(retry_policy) = options.http_retry_policy {
        retry_provider_builder.build(http_provider, retry_policy)
    } else {
        retry_provider_builder.build(http_provider, Box::<HttpRateLimitRetryPolicy>::default())
    };
    Ok(provider)
}

async fn create_raw_ws_provider(options: ProviderOptions) -> Result<WsWithTimeout> {
    Ok(WsWithTimeout::connect(&options.url, options.request_timeout).await?)
}
