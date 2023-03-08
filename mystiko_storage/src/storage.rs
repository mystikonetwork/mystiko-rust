#![forbid(unsafe_code)]
use crate::document::DocumentRawData;
use async_trait::async_trait;
use std::io::Error;
use ulid::Ulid;

#[async_trait]
pub trait Storage<R: DocumentRawData>: Send + Sync {
    async fn uuid(&mut self) -> Result<String, Error> {
        Ok(Ulid::new().to_string())
    }
    async fn execute(&mut self, statement: String) -> Result<(), Error>;
    async fn query(&mut self, statement: String) -> Result<Vec<R>, Error>;
    async fn collection_exists(&mut self, collection: &str) -> Result<bool, Error>;
}
