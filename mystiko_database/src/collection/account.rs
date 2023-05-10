#![forbid(unsafe_code)]
use crate::document::account::Account;
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
pub struct AccountCollection<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    collection: Arc<Collection<F, R, S>>,
}

impl<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> AccountCollection<F, R, S> {
    pub fn new(collection: Arc<Collection<F, R, S>>) -> Self {
        AccountCollection { collection }
    }

    pub async fn insert(&self, account: &Account) -> Result<Document<Account>> {
        self.collection
            .insert(account)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn insert_batch(&self, accounts: &Vec<Account>) -> Result<Vec<Document<Account>>> {
        self.collection
            .insert_batch(accounts)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn find<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<Vec<Document<Account>>> {
        self.collection
            .find::<Account, Q>(Some(filter))
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn find_all(&self) -> Result<Vec<Document<Account>>> {
        self.collection
            .find::<Account, QueryFilter>(None)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn find_one<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<Option<Document<Account>>> {
        self.collection
            .find_one(Some(filter))
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Document<Account>>> {
        self.collection
            .find_by_id(id)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn count<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<u64> {
        self.collection
            .count::<Account, Q>(Some(filter))
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn count_all(&self) -> Result<u64> {
        self.collection
            .count::<Account, QueryFilter>(None)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn update(&self, account: &Document<Account>) -> Result<Document<Account>> {
        self.collection
            .update(account)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn update_batch(&self, accounts: &Vec<Document<Account>>) -> Result<Vec<Document<Account>>> {
        self.collection
            .update_batch(accounts)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn delete(&self, account: &Document<Account>) -> Result<()> {
        self.collection
            .delete(account)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn delete_batch(&self, accounts: &Vec<Document<Account>>) -> Result<()> {
        self.collection
            .delete_batch(accounts)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn delete_all(&self) -> Result<()> {
        self.collection
            .delete_by_filter::<Account, QueryFilter>(None)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn delete_by_filter<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<()> {
        self.collection
            .delete_by_filter::<Account, Q>(Some(filter))
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn migrate(&self) -> Result<Document<Migration>> {
        self.collection
            .migrate(Account::schema())
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn collection_exists(&self) -> Result<bool> {
        self.collection
            .collection_exists(Account::schema())
            .await
            .map_err(DatabaseError::StorageError)
    }
}
