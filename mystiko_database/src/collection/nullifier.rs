#![forbid(unsafe_code)]
use crate::document::nullifier::Nullifier;
use anyhow::Result;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::{Document, DocumentData, DocumentRawData};
use mystiko_storage::filter::QueryFilter;
use mystiko_storage::formatter::StatementFormatter;
use mystiko_storage::migration::Migration;
use mystiko_storage::storage::Storage;
use std::sync::Arc;

pub struct NullifierCollection<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    collection: Arc<Collection<F, R, S>>,
}

impl<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> NullifierCollection<F, R, S> {
    pub fn new(collection: Arc<Collection<F, R, S>>) -> Self {
        NullifierCollection { collection }
    }

    pub async fn insert(&self, nullifier: &Nullifier) -> Result<Document<Nullifier>> {
        self.collection.insert(nullifier).await
    }

    pub async fn insert_batch(&self, nullifiers: &Vec<Nullifier>) -> Result<Vec<Document<Nullifier>>> {
        self.collection.insert_batch(nullifiers).await
    }

    pub async fn find(&self, filter: QueryFilter) -> Result<Vec<Document<Nullifier>>> {
        self.collection.find::<Nullifier>(Some(filter)).await
    }

    pub async fn find_all(&self) -> Result<Vec<Document<Nullifier>>> {
        self.collection.find::<Nullifier>(None).await
    }

    pub async fn find_one(&self, filter: QueryFilter) -> Result<Option<Document<Nullifier>>> {
        self.collection.find_one(Some(filter)).await
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Document<Nullifier>>> {
        self.collection.find_by_id(id).await
    }

    pub async fn count(&self, filter: QueryFilter) -> Result<u64> {
        self.collection.count::<Nullifier>(Some(filter)).await
    }

    pub async fn count_all(&self) -> Result<u64> {
        self.collection.count::<Nullifier>(None).await
    }

    pub async fn update(&self, nullifier: &Document<Nullifier>) -> Result<Document<Nullifier>> {
        self.collection.update(nullifier).await
    }

    pub async fn update_batch(&self, nullifiers: &Vec<Document<Nullifier>>) -> Result<Vec<Document<Nullifier>>> {
        self.collection.update_batch(nullifiers).await
    }

    pub async fn delete(&self, nullifier: &Document<Nullifier>) -> Result<()> {
        self.collection.delete(nullifier).await
    }

    pub async fn delete_batch(&self, nullifiers: &Vec<Document<Nullifier>>) -> Result<()> {
        self.collection.delete_batch(nullifiers).await
    }

    pub async fn delete_all(&self) -> Result<()> {
        self.collection.delete_by_filter::<Nullifier>(None).await
    }

    pub async fn delete_by_filter(&self, filter: QueryFilter) -> Result<()> {
        self.collection.delete_by_filter::<Nullifier>(Some(filter)).await
    }

    pub async fn migrate(&self) -> Result<Document<Migration>> {
        self.collection.migrate(Nullifier::schema()).await
    }

    pub async fn collection_exists(&self) -> Result<bool> {
        self.collection.collection_exists(Nullifier::schema()).await
    }
}
