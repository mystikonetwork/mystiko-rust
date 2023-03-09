use async_trait::async_trait;
use std::io::Error;

#[async_trait]
pub trait MystikoIndexerClient: Send + Sync {
    async fn ping(&self, message: String) -> Result<String, Error>;
    async fn auth_ping(&self, message: String) -> Result<String, Error>;
}
