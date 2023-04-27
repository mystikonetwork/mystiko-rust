use anyhow::Result;
use async_trait::async_trait;
use ethers_providers::{JsonRpcClient, ProviderError};
use futures_util::sink::SinkExt;
use futures_util::StreamExt;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::fmt::Debug;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::time::sleep;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug)]
pub struct TestProvider {
    pub error: Option<fn() -> ProviderError>,
    pub result: Value,
}

#[derive(Debug, Clone)]
pub enum MockWebSocketResponse {
    Normal(String),
    Stall(String, u64),
}

#[derive(Debug)]
pub struct MockWebSocketServer {
    responses: Vec<MockWebSocketResponse>,
}

#[async_trait]
impl JsonRpcClient for TestProvider {
    type Error = ProviderError;

    async fn request<T, R>(&self, _method: &str, _params: T) -> Result<R, Self::Error>
    where
        T: Debug + Serialize + Send + Sync,
        R: DeserializeOwned + Send,
    {
        if let Some(error) = &self.error {
            Err(error())
        } else {
            Ok(serde_json::from_value(self.result.clone())?)
        }
    }
}

impl Default for TestProvider {
    fn default() -> Self {
        Self {
            error: None,
            result: serde_json::from_str("null").unwrap(),
        }
    }
}

impl MockWebSocketServer {
    pub fn new(responses: Vec<MockWebSocketResponse>) -> Self {
        Self { responses }
    }

    pub async fn start(self, port: u16) -> Result<String> {
        let host = String::from("127.0.0.1");
        let listener = TcpListener::bind(format!("{}:{}", host, port)).await?;
        let local_port = listener.local_addr()?.port();
        tokio::spawn(async move {
            self.accept(&listener).await.unwrap();
        });
        Ok(format!("ws://{}:{}", host, local_port))
    }

    async fn accept(&self, listener: &TcpListener) -> Result<()> {
        let mut responses = self.responses.clone();
        let (conn, _) = listener.accept().await?;
        let mut stream = accept_async(conn).await?;
        while let Some(message) = stream.next().await {
            match message? {
                Message::Text(_) => {
                    if !responses.is_empty() {
                        let response = responses.remove(0);
                        match response {
                            MockWebSocketResponse::Normal(result) => {
                                stream.send(Message::Text(result)).await?;
                            }
                            MockWebSocketResponse::Stall(result, delay) => {
                                sleep(Duration::from_millis(delay)).await;
                                stream.send(Message::Text(result)).await?;
                            }
                        }
                    } else {
                        stream
                            .send(Message::Text("No more response".into()))
                            .await?;
                    }
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
        Ok(())
    }
}
