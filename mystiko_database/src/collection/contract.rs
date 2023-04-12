#![forbid(unsafe_code)]
use crate::document::contract::Contract;
use anyhow::Result;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::{Document, DocumentData, DocumentRawData};
use mystiko_storage::filter::QueryFilter;
use mystiko_storage::formatter::StatementFormatter;
use mystiko_storage::migration::Migration;
use mystiko_storage::storage::Storage;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ContractCollection<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    collection: Arc<Mutex<Collection<F, R, S>>>,
}

impl<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> ContractCollection<F, R, S> {
    pub fn new(collection: Arc<Mutex<Collection<F, R, S>>>) -> Self {
        ContractCollection { collection }
    }

    pub async fn insert(&self, contract: &Contract) -> Result<Document<Contract>> {
        let mut collection = self.collection.lock().await;
        collection.insert(contract).await
    }

    pub async fn insert_batch(&self, contracts: &Vec<Contract>) -> Result<Vec<Document<Contract>>> {
        let mut collection = self.collection.lock().await;
        collection.insert_batch(contracts).await
    }

    pub async fn find(&self, filter: QueryFilter) -> Result<Vec<Document<Contract>>> {
        let mut collection = self.collection.lock().await;
        collection.find::<Contract>(Some(filter)).await
    }

    pub async fn find_all(&self) -> Result<Vec<Document<Contract>>> {
        let mut collection = self.collection.lock().await;
        collection.find::<Contract>(None).await
    }

    pub async fn find_one(&self, filter: QueryFilter) -> Result<Option<Document<Contract>>> {
        let mut collection = self.collection.lock().await;
        collection.find_one(Some(filter)).await
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Document<Contract>>> {
        let mut collection = self.collection.lock().await;
        collection.find_by_id(id).await
    }

    pub async fn count(&self, filter: QueryFilter) -> Result<u64> {
        let mut collection = self.collection.lock().await;
        collection.count::<Contract>(Some(filter)).await
    }

    pub async fn count_all(&self) -> Result<u64> {
        let mut collection = self.collection.lock().await;
        collection.count::<Contract>(None).await
    }

    pub async fn update(&self, contract: &Document<Contract>) -> Result<Document<Contract>> {
        let mut collection = self.collection.lock().await;
        collection.update(contract).await
    }

    pub async fn update_batch(
        &self,
        contracts: &Vec<Document<Contract>>,
    ) -> Result<Vec<Document<Contract>>> {
        let mut collection = self.collection.lock().await;
        collection.update_batch(contracts).await
    }

    pub async fn delete(&self, contract: &Document<Contract>) -> Result<()> {
        let mut collection = self.collection.lock().await;
        collection.delete(contract).await
    }

    pub async fn delete_batch(&self, contracts: &Vec<Document<Contract>>) -> Result<()> {
        let mut collection = self.collection.lock().await;
        collection.delete_batch(contracts).await
    }

    pub async fn delete_all(&self) -> Result<()> {
        let mut collection = self.collection.lock().await;
        collection.delete_by_filter::<Contract>(None).await
    }

    pub async fn delete_by_filter(&self, filter: QueryFilter) -> Result<()> {
        let mut collection = self.collection.lock().await;
        collection.delete_by_filter::<Contract>(Some(filter)).await
    }

    pub async fn migrate(&self) -> Result<Document<Migration>> {
        let mut collection = self.collection.lock().await;
        collection.migrate(Contract::schema()).await
    }

    pub async fn collection_exists(&self) -> Result<bool> {
        let mut collection = self.collection.lock().await;
        collection.collection_exists(Contract::schema()).await
    }
}
