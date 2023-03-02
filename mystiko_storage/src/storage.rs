#![forbid(unsafe_code)]
use crate::document::{Document, DocumentData};
use async_trait::async_trait;
use std::io::Error;
use ulid::Ulid;

#[async_trait]
pub trait Storage: Send + Sync {
    async fn uuid(&mut self) -> Result<String, Error> {
        Ok(Ulid::new().to_string())
    }
    async fn execute(&mut self, statement: String) -> Result<(), Error>;
    async fn query<D: DocumentData>(
        &mut self,
        statement: String,
    ) -> Result<Vec<Document<D>>, Error>;
    async fn collection_exists(&mut self, collection: &str) -> Result<bool, Error>;
}
