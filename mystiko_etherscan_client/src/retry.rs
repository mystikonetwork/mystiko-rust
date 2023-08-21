use std::fmt::Debug;

use typed_builder::TypedBuilder;

use crate::errors::EtherScanError;

pub trait RetryPolicy: Debug + Send + Sync {
    fn is_retryable(&self, error: &EtherScanError, current_retry_time: u64) -> bool;
}

#[derive(Debug, TypedBuilder)]
pub struct DefaultRetryPolicy {
    max_retry_times: u64,
}

impl DefaultRetryPolicy {
    pub fn new(max_retry_times: u64) -> Self {
        Self { max_retry_times }
    }
}

impl RetryPolicy for DefaultRetryPolicy {
    fn is_retryable(&self, error: &EtherScanError, current_retry_time: u64) -> bool {
        if current_retry_time >= self.max_retry_times {
            return false;
        }
        if let EtherScanError::ResponseError(response_error) = error {
            if response_error.to_string().to_lowercase().contains("rate limit") {
                return true;
            }
        }
        false
    }
}

impl Default for DefaultRetryPolicy {
    fn default() -> Self {
        Self::new(5)
    }
}
