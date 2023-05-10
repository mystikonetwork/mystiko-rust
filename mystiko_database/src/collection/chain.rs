#![forbid(unsafe_code)]
use crate::document::chain::Chain;
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
pub struct ChainCollection<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    collection: Arc<Collection<F, R, S>>,
}

impl<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> ChainCollection<F, R, S> {
    pub fn new(collection: Arc<Collection<F, R, S>>) -> Self {
        ChainCollection { collection }
    }

    pub async fn insert(&self, chain: &Chain) -> Result<Document<Chain>> {
        self.collection.insert(chain).await.map_err(DatabaseError::StorageError)
    }

    pub async fn insert_batch(&self, chains: &Vec<Chain>) -> Result<Vec<Document<Chain>>> {
        self.collection
            .insert_batch(chains)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn find<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<Vec<Document<Chain>>> {
        self.collection
            .find::<Chain, Q>(Some(filter))
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn find_all(&self) -> Result<Vec<Document<Chain>>> {
        self.collection
            .find::<Chain, QueryFilter>(None)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn find_one<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<Option<Document<Chain>>> {
        self.collection
            .find_one(Some(filter))
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Document<Chain>>> {
        self.collection
            .find_by_id(id)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn count<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<u64> {
        self.collection
            .count::<Chain, Q>(Some(filter))
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn count_all(&self) -> Result<u64> {
        self.collection
            .count::<Chain, QueryFilter>(None)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn update(&self, chain: &Document<Chain>) -> Result<Document<Chain>> {
        self.collection.update(chain).await.map_err(DatabaseError::StorageError)
    }

    pub async fn update_batch(&self, chains: &Vec<Document<Chain>>) -> Result<Vec<Document<Chain>>> {
        self.collection
            .update_batch(chains)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn delete(&self, chain: &Document<Chain>) -> Result<()> {
        self.collection.delete(chain).await.map_err(DatabaseError::StorageError)
    }

    pub async fn delete_batch(&self, chains: &Vec<Document<Chain>>) -> Result<()> {
        self.collection
            .delete_batch(chains)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn delete_all(&self) -> Result<()> {
        self.collection
            .delete_by_filter::<Chain, QueryFilter>(None)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn delete_by_filter<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<()> {
        self.collection
            .delete_by_filter::<Chain, Q>(Some(filter))
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn migrate(&self) -> Result<Document<Migration>> {
        self.collection
            .migrate(Chain::schema())
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn collection_exists(&self) -> Result<bool> {
        self.collection
            .collection_exists(Chain::schema())
            .await
            .map_err(DatabaseError::StorageError)
    }
}
