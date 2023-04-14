use ethers_providers::{Authorization, HttpClientError, RetryPolicy};
use std::time::Duration;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct ProviderOptions {
    pub url: String,
    #[builder(default, setter(strip_option))]
    pub request_timeout: Option<Duration>,
    #[builder(default, setter(strip_option))]
    pub authorization: Option<Authorization>,
    #[builder(default, setter(strip_option))]
    pub timeout_retries: Option<u32>,
    #[builder(default, setter(strip_option))]
    pub rate_limit_retries: Option<u32>,
    #[builder(default, setter(strip_option))]
    pub initial_backoff: Option<Duration>,
    #[builder(default, setter(strip_option))]
    pub compute_units_per_second: Option<u64>,
    #[builder(default, setter(strip_option))]
    pub http_retry_policy: Option<Box<dyn RetryPolicy<HttpClientError>>>,
}
