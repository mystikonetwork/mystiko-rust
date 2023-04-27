#![forbid(unsafe_code)]
use crate::document::commitment::Commitment;
use anyhow::Result;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::{Document, DocumentData, DocumentRawData};
use mystiko_storage::filter::QueryFilter;
use mystiko_storage::formatter::StatementFormatter;
use mystiko_storage::migration::Migration;
use mystiko_storage::storage::Storage;
use std::sync::Arc;

pub struct CommitmentCollection<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    collection: Arc<Collection<F, R, S>>,
}

impl<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> CommitmentCollection<F, R, S> {
    pub fn new(collection: Arc<Collection<F, R, S>>) -> Self {
        CommitmentCollection { collection }
    }

    pub async fn insert(&self, commitment: &Commitment) -> Result<Document<Commitment>> {
        self.collection.insert(commitment).await
    }

    pub async fn insert_batch(
        &self,
        commitments: &Vec<Commitment>,
    ) -> Result<Vec<Document<Commitment>>> {
        self.collection.insert_batch(commitments).await
    }

    pub async fn find(&self, filter: QueryFilter) -> Result<Vec<Document<Commitment>>> {
        self.collection.find::<Commitment>(Some(filter)).await
    }

    pub async fn find_all(&self) -> Result<Vec<Document<Commitment>>> {
        self.collection.find::<Commitment>(None).await
    }

    pub async fn find_one(&self, filter: QueryFilter) -> Result<Option<Document<Commitment>>> {
        self.collection.find_one(Some(filter)).await
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Document<Commitment>>> {
        self.collection.find_by_id(id).await
    }

    pub async fn count(&self, filter: QueryFilter) -> Result<u64> {
        self.collection.count::<Commitment>(Some(filter)).await
    }

    pub async fn count_all(&self) -> Result<u64> {
        self.collection.count::<Commitment>(None).await
    }

    pub async fn update(&self, commitment: &Document<Commitment>) -> Result<Document<Commitment>> {
        self.collection.update(commitment).await
    }

    pub async fn update_batch(
        &self,
        commitments: &Vec<Document<Commitment>>,
    ) -> Result<Vec<Document<Commitment>>> {
        self.collection.update_batch(commitments).await
    }

    pub async fn delete(&self, commitment: &Document<Commitment>) -> Result<()> {
        self.collection.delete(commitment).await
    }

    pub async fn delete_batch(&self, commitments: &Vec<Document<Commitment>>) -> Result<()> {
        self.collection.delete_batch(commitments).await
    }

    pub async fn delete_all(&self) -> Result<()> {
        self.collection.delete_by_filter::<Commitment>(None).await
    }

    pub async fn delete_by_filter(&self, filter: QueryFilter) -> Result<()> {
        self.collection
            .delete_by_filter::<Commitment>(Some(filter))
            .await
    }

    pub async fn migrate(&self) -> Result<Document<Migration>> {
        self.collection.migrate(Commitment::schema()).await
    }

    pub async fn collection_exists(&self) -> Result<bool> {
        self.collection
            .collection_exists(Commitment::schema())
            .await
    }
}
