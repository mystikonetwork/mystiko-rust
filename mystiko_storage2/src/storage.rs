use crate::document::{Document, DocumentData};
use crate::error::StorageError;
use crate::formatter::{CountStatement, Statement};
use anyhow::Result;
use async_trait::async_trait;
use std::fmt::Debug;
use ulid::Ulid;

#[async_trait]
pub trait Storage: Send + Sync + Debug {
    async fn uuid(&self) -> Result<String, StorageError> {
        Ok(Ulid::new().to_string())
    }
    async fn execute(&self, statement: Statement) -> Result<(), StorageError>;
    async fn execute_batch(&self, statements: Vec<Statement>) -> Result<(), StorageError>;
    async fn query<T: DocumentData>(&self, statement: Statement) -> Result<Vec<Document<T>>, StorageError>;
    async fn count(&self, statement: CountStatement) -> Result<u64, StorageError>;
    async fn collection_exists(&self, collection: &str) -> Result<bool, StorageError>;
}
