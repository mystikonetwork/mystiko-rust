#![forbid(unsafe_code)]
use crate::document::contract::Contract;
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
pub struct ContractCollection<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    collection: Arc<Collection<F, R, S>>,
}

impl<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> ContractCollection<F, R, S> {
    pub fn new(collection: Arc<Collection<F, R, S>>) -> Self {
        ContractCollection { collection }
    }

    pub async fn insert(&self, contract: &Contract) -> Result<Document<Contract>> {
        self.collection
            .insert(contract)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn insert_batch(&self, contracts: &Vec<Contract>) -> Result<Vec<Document<Contract>>> {
        self.collection
            .insert_batch(contracts)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn find<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<Vec<Document<Contract>>> {
        self.collection
            .find::<Contract, Q>(Some(filter))
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn find_all(&self) -> Result<Vec<Document<Contract>>> {
        self.collection
            .find::<Contract, QueryFilter>(None)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn find_one<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<Option<Document<Contract>>> {
        self.collection
            .find_one(Some(filter))
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Document<Contract>>> {
        self.collection
            .find_by_id(id)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn count<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<u64> {
        self.collection
            .count::<Contract, Q>(Some(filter))
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn count_all(&self) -> Result<u64> {
        self.collection
            .count::<Contract, QueryFilter>(None)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn update(&self, contract: &Document<Contract>) -> Result<Document<Contract>> {
        self.collection
            .update(contract)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn update_batch(&self, contracts: &Vec<Document<Contract>>) -> Result<Vec<Document<Contract>>> {
        self.collection
            .update_batch(contracts)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn delete(&self, contract: &Document<Contract>) -> Result<()> {
        self.collection
            .delete(contract)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn delete_batch(&self, contracts: &Vec<Document<Contract>>) -> Result<()> {
        self.collection
            .delete_batch(contracts)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn delete_all(&self) -> Result<()> {
        self.collection
            .delete_by_filter::<Contract, QueryFilter>(None)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn delete_by_filter<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<()> {
        self.collection
            .delete_by_filter::<Contract, Q>(Some(filter))
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn migrate(&self) -> Result<Document<Migration>> {
        self.collection
            .migrate(Contract::schema())
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn collection_exists(&self) -> Result<bool> {
        self.collection
            .collection_exists(Contract::schema())
            .await
            .map_err(DatabaseError::StorageError)
    }
}
