use crate::provider::failover::FailoverProvider;
use crate::provider::types::ProviderOptions;
use anyhow::Result;
use ethers_providers::{Http, HttpRateLimitRetryPolicy, Provider, RetryClient, RetryClientBuilder};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Client, Url};

pub mod failover;
pub mod types;

pub fn create_http_provider(options: ProviderOptions) -> Result<Provider<RetryClient<Http>>> {
    Ok(Provider::new(create_raw_http_provider(options)?))
}

pub fn create_failover_provider(
    options: Vec<ProviderOptions>,
) -> Result<Provider<FailoverProvider>> {
    let mut failover_provider_builder = FailoverProvider::dyn_rpc();
    for provider_options in options.into_iter() {
        if provider_options.url.starts_with("http") || provider_options.url.starts_with("https") {
            let inner_provider = create_raw_http_provider(provider_options)?;
            failover_provider_builder =
                failover_provider_builder.add_provider(Box::new(inner_provider));
        }
    }
    Ok(Provider::new(failover_provider_builder.build()))
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
        retry_provider_builder =
            retry_provider_builder.compute_units_per_second(compute_units_per_second);
    }
    let retry_provider = if let Some(retry_policy) = options.http_retry_policy {
        retry_provider_builder.build(http_provider, retry_policy)
    } else {
        retry_provider_builder.build(http_provider, Box::new(HttpRateLimitRetryPolicy::default()))
    };
    Ok(retry_provider)
}
