#[derive(Clone, Debug)]
pub struct RetryPolicy {
    max_retry_times: u64,
}

impl RetryPolicy {
    pub fn new(max_retry_times: u64) -> Self {
        Self { max_retry_times }
    }
    pub fn is_retryable(&self, error: &anyhow::Error, current_retry_time: u64) -> bool {
        if current_retry_time >= self.max_retry_times {
            //todo log
            return false;
        }
        if let Some(result) = error.downcast_ref::<String>() {
            if result.to_lowercase().contains("rate limit") {
                return true;
            }
        }
        false
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self::new(5)
    }
}
