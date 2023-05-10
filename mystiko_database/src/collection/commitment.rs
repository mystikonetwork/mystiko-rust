#![forbid(unsafe_code)]
use crate::document::commitment::Commitment;
use crate::error::DatabaseError;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::{Document, DocumentData, DocumentRawData};
use mystiko_storage::filter::QueryFilter;
use mystiko_storage::formatter::StatementFormatter;
use mystiko_storage::migration::Migration;
use mystiko_storage::storage::Storage;
use std::sync::Arc;

pub type Result<T> = anyhow::Result<T, DatabaseError>;

#[derive(Debug)]
pub struct CommitmentCollection<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    collection: Arc<Collection<F, R, S>>,
}

impl<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> CommitmentCollection<F, R, S> {
    pub fn new(collection: Arc<Collection<F, R, S>>) -> Self {
        CommitmentCollection { collection }
    }

    pub async fn insert(&self, commitment: &Commitment) -> Result<Document<Commitment>> {
        self.collection
            .insert(commitment)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn insert_batch(&self, commitments: &Vec<Commitment>) -> Result<Vec<Document<Commitment>>> {
        self.collection
            .insert_batch(commitments)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn find<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<Vec<Document<Commitment>>> {
        self.collection
            .find::<Commitment, Q>(Some(filter))
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn find_all(&self) -> Result<Vec<Document<Commitment>>> {
        self.collection
            .find::<Commitment, QueryFilter>(None)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn find_one<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<Option<Document<Commitment>>> {
        self.collection
            .find_one(Some(filter))
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Document<Commitment>>> {
        self.collection
            .find_by_id(id)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn count<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<u64> {
        self.collection
            .count::<Commitment, Q>(Some(filter))
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn count_all(&self) -> Result<u64> {
        self.collection
            .count::<Commitment, QueryFilter>(None)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn update(&self, commitment: &Document<Commitment>) -> Result<Document<Commitment>> {
        self.collection
            .update(commitment)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn update_batch(&self, commitments: &Vec<Document<Commitment>>) -> Result<Vec<Document<Commitment>>> {
        self.collection
            .update_batch(commitments)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn delete(&self, commitment: &Document<Commitment>) -> Result<()> {
        self.collection
            .delete(commitment)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn delete_batch(&self, commitments: &Vec<Document<Commitment>>) -> Result<()> {
        self.collection
            .delete_batch(commitments)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn delete_all(&self) -> Result<()> {
        self.collection
            .delete_by_filter::<Commitment, QueryFilter>(None)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn delete_by_filter<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<()> {
        self.collection
            .delete_by_filter::<Commitment, Q>(Some(filter))
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn migrate(&self) -> Result<Document<Migration>> {
        self.collection
            .migrate(Commitment::schema())
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn collection_exists(&self) -> Result<bool> {
        self.collection
            .collection_exists(Commitment::schema())
            .await
            .map_err(DatabaseError::StorageError)
    }
}
