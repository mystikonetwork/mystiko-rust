#![forbid(unsafe_code)]
use crate::document::account::Account;
use anyhow::Result;
use futures::lock::Mutex;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::{Document, DocumentData, DocumentRawData};
use mystiko_storage::filter::QueryFilter;
use mystiko_storage::formatter::StatementFormatter;
use mystiko_storage::migration::Migration;
use mystiko_storage::storage::Storage;
use std::sync::Arc;

pub struct AccountCollection<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    collection: Arc<Mutex<Collection<F, R, S>>>,
}

impl<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> AccountCollection<F, R, S> {
    pub fn new(collection: Arc<Mutex<Collection<F, R, S>>>) -> Self {
        AccountCollection { collection }
    }

    pub async fn insert(&self, account: &Account) -> Result<Document<Account>> {
        let mut collection = self.collection.lock().await;
        collection.insert(account).await
    }

    pub async fn insert_batch(&self, accounts: &Vec<Account>) -> Result<Vec<Document<Account>>> {
        let mut collection = self.collection.lock().await;
        collection.insert_batch(accounts).await
    }

    pub async fn find(&self, filter: QueryFilter) -> Result<Vec<Document<Account>>> {
        let mut collection = self.collection.lock().await;
        collection.find::<Account>(Some(filter)).await
    }

    pub async fn find_all(&self) -> Result<Vec<Document<Account>>> {
        let mut collection = self.collection.lock().await;
        collection.find::<Account>(None).await
    }

    pub async fn find_one(&self, filter: QueryFilter) -> Result<Option<Document<Account>>> {
        let mut collection = self.collection.lock().await;
        collection.find_one(Some(filter)).await
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Document<Account>>> {
        let mut collection = self.collection.lock().await;
        collection.find_by_id(id).await
    }

    pub async fn count(&self, filter: QueryFilter) -> Result<u64> {
        let mut collection = self.collection.lock().await;
        collection.count::<Account>(Some(filter)).await
    }

    pub async fn count_all(&self) -> Result<u64> {
        let mut collection = self.collection.lock().await;
        collection.count::<Account>(None).await
    }

    pub async fn update(&self, account: &Document<Account>) -> Result<Document<Account>> {
        let mut collection = self.collection.lock().await;
        collection.update(account).await
    }

    pub async fn update_batch(
        &self,
        accounts: &Vec<Document<Account>>,
    ) -> Result<Vec<Document<Account>>> {
        let mut collection = self.collection.lock().await;
        collection.update_batch(accounts).await
    }

    pub async fn delete(&self, account: &Document<Account>) -> Result<()> {
        let mut collection = self.collection.lock().await;
        collection.delete(account).await
    }

    pub async fn delete_batch(&self, accounts: &Vec<Document<Account>>) -> Result<()> {
        let mut collection = self.collection.lock().await;
        collection.delete_batch(accounts).await
    }

    pub async fn delete_all(&self) -> Result<()> {
        let mut collection = self.collection.lock().await;
        collection.delete_by_filter::<Account>(None).await
    }

    pub async fn delete_by_filter(&self, filter: QueryFilter) -> Result<()> {
        let mut collection = self.collection.lock().await;
        collection.delete_by_filter::<Account>(Some(filter)).await
    }

    pub async fn migrate(&self) -> Result<Document<Migration>> {
        let mut collection = self.collection.lock().await;
        collection.migrate(Account::schema()).await
    }

    pub async fn collection_exists(&self) -> Result<bool> {
        let mut collection = self.collection.lock().await;
        collection.collection_exists(Account::schema()).await
    }
}
