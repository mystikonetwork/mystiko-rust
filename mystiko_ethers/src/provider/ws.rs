use anyhow::Result;
use async_trait::async_trait;
use ethers_providers::{ConnectionDetails, JsonRpcClient, Ws, WsClientError};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::time::Duration;
use tokio::time::timeout;

const DEFAULT_TIMEOUT_MS: u64 = 10000;

#[derive(Debug)]
pub struct WsWithTimeout {
    inner: Ws,
    timeout: Duration,
}

impl WsWithTimeout {
    pub async fn connect(
        conn: impl Into<ConnectionDetails>,
        timeout: Option<Duration>,
    ) -> Result<Self, WsClientError> {
        let inner = Ws::connect(conn).await?;
        Ok(Self {
            inner,
            timeout: timeout.unwrap_or(Duration::from_millis(DEFAULT_TIMEOUT_MS)),
        })
    }

    pub async fn connect_with_reconnects(
        conn: impl Into<ConnectionDetails>,
        reconnects: usize,
        timeout: Option<Duration>,
    ) -> Result<Self, WsClientError> {
        let inner = Ws::connect_with_reconnects(conn, reconnects).await?;
        Ok(Self {
            inner,
            timeout: timeout.unwrap_or(Duration::from_millis(DEFAULT_TIMEOUT_MS)),
        })
    }
}

#[async_trait]
impl JsonRpcClient for WsWithTimeout {
    type Error = WsClientError;

    async fn request<T, R>(&self, method: &str, params: T) -> Result<R, Self::Error>
    where
        T: Debug + Serialize + Send + Sync,
        R: DeserializeOwned + Send,
    {
        let timed_result = timeout(self.timeout, self.inner.request::<T, R>(method, params)).await;
        match timed_result {
            Ok(result) => result,
            Err(_) => {
                let timeout_error = std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    format!("timed out after {:?}", self.timeout),
                );
                Err(WsClientError::InternalError(timeout_error.into()))
            }
        }
    }
}
