extern crate mystiko_etherscan_client;

use mystiko_etherscan_client::{DefaultRetryPolicy, EtherScanError, RetryPolicy};

#[test]
fn test_is_retryable() {
    let retry_policy = DefaultRetryPolicy::default();

    // current_retry_time >= self.max_retry_times
    let current_retry_times = 5;
    let error = EtherScanError::ResponseError(String::new());
    assert!(!retry_policy.is_retryable(&error, current_retry_times));

    let retry_policy = DefaultRetryPolicy::builder().max_retry_times(6).build();
    //Error type not ResponseError
    let error = EtherScanError::UnknownError(String::new());
    assert!(!retry_policy.is_retryable(&error, current_retry_times));

    //Error not contains "rate limit"
    let error = EtherScanError::ResponseError(String::new());
    assert!(!retry_policy.is_retryable(&error, current_retry_times));

    //is_retryable return true
    let retry_policy = DefaultRetryPolicy::new(7);
    let error = EtherScanError::ResponseError(String::from(
        "Max rate limit reached, please use API Key for higher rate limit",
    ));
    assert!(retry_policy.is_retryable(&error, current_retry_times));
}
