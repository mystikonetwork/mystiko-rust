#![forbid(unsafe_code)]
use crate::document::DocumentRawData;
use anyhow::Result;
use async_trait::async_trait;
use ulid::Ulid;

#[async_trait]
pub trait Storage<R: DocumentRawData>: Send + Sync {
    async fn uuid(&self) -> Result<String> {
        Ok(Ulid::new().to_string())
    }
    async fn execute(&self, statement: String) -> Result<()>;
    async fn query(&self, statement: String) -> Result<Vec<R>>;
    async fn collection_exists(&self, collection: &str) -> Result<bool>;
}
