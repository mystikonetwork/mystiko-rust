use crate::errors::EtherScanError;
use std::fmt::Debug;
use typed_builder::TypedBuilder;

pub trait RetryPolicy: Debug {
    fn is_retryable(&self, error: &EtherScanError, current_retry_time: u64) -> bool;
}

#[derive(Debug, TypedBuilder)]
pub struct DefaultRetryPolicy {
    max_retry_times: u64,
}

impl RetryPolicy for DefaultRetryPolicy {
    fn is_retryable(&self, error: &EtherScanError, current_retry_time: u64) -> bool {
        if current_retry_time >= self.max_retry_times {
            return false;
        }
        let result = error.to_string();
        if result.to_lowercase().contains("rate limit") {
            return true;
        }
        false
    }
}

impl Default for DefaultRetryPolicy {
    fn default() -> Self {
        DefaultRetryPolicy { max_retry_times: 5 }
    }
}
