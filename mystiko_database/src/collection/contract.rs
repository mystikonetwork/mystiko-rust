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

pub struct ContractCollection<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    collection: Arc<Collection<F, R, S>>,
}

impl<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> ContractCollection<F, R, S> {
    pub fn new(collection: Arc<Collection<F, R, S>>) -> Self {
        ContractCollection { collection }
    }

    pub async fn insert(&self, contract: &Contract) -> Result<Document<Contract>> {
        self.collection.insert(contract).await
    }

    pub async fn insert_batch(&self, contracts: &Vec<Contract>) -> Result<Vec<Document<Contract>>> {
        self.collection.insert_batch(contracts).await
    }

    pub async fn find(&self, filter: QueryFilter) -> Result<Vec<Document<Contract>>> {
        self.collection.find::<Contract>(Some(filter)).await
    }

    pub async fn find_all(&self) -> Result<Vec<Document<Contract>>> {
        self.collection.find::<Contract>(None).await
    }

    pub async fn find_one(&self, filter: QueryFilter) -> Result<Option<Document<Contract>>> {
        self.collection.find_one(Some(filter)).await
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Document<Contract>>> {
        self.collection.find_by_id(id).await
    }

    pub async fn count(&self, filter: QueryFilter) -> Result<u64> {
        self.collection.count::<Contract>(Some(filter)).await
    }

    pub async fn count_all(&self) -> Result<u64> {
        self.collection.count::<Contract>(None).await
    }

    pub async fn update(&self, contract: &Document<Contract>) -> Result<Document<Contract>> {
        self.collection.update(contract).await
    }

    pub async fn update_batch(
        &self,
        contracts: &Vec<Document<Contract>>,
    ) -> Result<Vec<Document<Contract>>> {
        self.collection.update_batch(contracts).await
    }

    pub async fn delete(&self, contract: &Document<Contract>) -> Result<()> {
        self.collection.delete(contract).await
    }

    pub async fn delete_batch(&self, contracts: &Vec<Document<Contract>>) -> Result<()> {
        self.collection.delete_batch(contracts).await
    }

    pub async fn delete_all(&self) -> Result<()> {
        self.collection.delete_by_filter::<Contract>(None).await
    }

    pub async fn delete_by_filter(&self, filter: QueryFilter) -> Result<()> {
        self.collection
            .delete_by_filter::<Contract>(Some(filter))
            .await
    }

    pub async fn migrate(&self) -> Result<Document<Migration>> {
        self.collection.migrate(Contract::schema()).await
    }

    pub async fn collection_exists(&self) -> Result<bool> {
        self.collection.collection_exists(Contract::schema()).await
    }
}
