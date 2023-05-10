#![forbid(unsafe_code)]

use crate::document::DocumentRawData;
use crate::error::StorageError;
use anyhow::Result;
use async_trait::async_trait;
use std::fmt::Debug;
use ulid::Ulid;

#[async_trait]
pub trait Storage<R: DocumentRawData>: Send + Sync + Debug {
    async fn uuid(&self) -> Result<String, StorageError> {
        Ok(Ulid::new().to_string())
    }
    async fn execute(&self, statement: String) -> Result<(), StorageError>;
    async fn query(&self, statement: String) -> Result<Vec<R>, StorageError>;
    async fn collection_exists(&self, collection: &str) -> Result<bool, StorageError>;
}
