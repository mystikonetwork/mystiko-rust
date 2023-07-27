use std::fmt::Debug;

use anyhow::Error;

use typed_builder::TypedBuilder;

pub trait RetryPolicy: Debug {
    fn new(max_retry_times: u64) -> Self
        where
            Self: Sized;
    fn is_retryable(&self, error: &Error, current_retry_time: u64) -> bool;
}

#[derive(Debug, TypedBuilder)]
pub struct DefaultRetryPolicy {
    max_retry_times: u64,
}

impl RetryPolicy for DefaultRetryPolicy {
    fn new(max_retry_times: u64) -> Self {
        Self { max_retry_times }
    }
    fn is_retryable(&self, error: &anyhow::Error, current_retry_time: u64) -> bool {
        if current_retry_time >= self.max_retry_times {
            return false;
        }
        let result = format!("{}", error);
        if result.to_lowercase().contains("rate limit") {
            return true;
        }
        false
    }
}

impl Default for DefaultRetryPolicy {
    fn default() -> Self {
        Self::new(5)
    }
}