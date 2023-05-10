#![forbid(unsafe_code)]
use crate::document::deposit::Deposit;
use anyhow::Result;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::{Document, DocumentData, DocumentRawData};
use mystiko_storage::filter::QueryFilter;
use mystiko_storage::formatter::StatementFormatter;
use mystiko_storage::migration::Migration;
use mystiko_storage::storage::Storage;
use std::sync::Arc;

#[derive(Debug)]
pub struct DepositCollection<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    collection: Arc<Collection<F, R, S>>,
}

impl<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> DepositCollection<F, R, S> {
    pub fn new(collection: Arc<Collection<F, R, S>>) -> Self {
        DepositCollection { collection }
    }

    pub async fn insert(&self, deposit: &Deposit) -> Result<Document<Deposit>> {
        self.collection.insert(deposit).await
    }

    pub async fn insert_batch(&self, deposits: &Vec<Deposit>) -> Result<Vec<Document<Deposit>>> {
        self.collection.insert_batch(deposits).await
    }

    pub async fn find<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<Vec<Document<Deposit>>> {
        self.collection.find::<Deposit, Q>(Some(filter)).await
    }

    pub async fn find_all(&self) -> Result<Vec<Document<Deposit>>> {
        self.collection.find::<Deposit, QueryFilter>(None).await
    }

    pub async fn find_one<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<Option<Document<Deposit>>> {
        self.collection.find_one(Some(filter)).await
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Document<Deposit>>> {
        self.collection.find_by_id(id).await
    }

    pub async fn count<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<u64> {
        self.collection.count::<Deposit, Q>(Some(filter)).await
    }

    pub async fn count_all(&self) -> Result<u64> {
        self.collection.count::<Deposit, QueryFilter>(None).await
    }

    pub async fn update(&self, deposit: &Document<Deposit>) -> Result<Document<Deposit>> {
        self.collection.update(deposit).await
    }

    pub async fn update_batch(&self, deposits: &Vec<Document<Deposit>>) -> Result<Vec<Document<Deposit>>> {
        self.collection.update_batch(deposits).await
    }

    pub async fn delete(&self, deposit: &Document<Deposit>) -> Result<()> {
        self.collection.delete(deposit).await
    }

    pub async fn delete_batch(&self, deposits: &Vec<Document<Deposit>>) -> Result<()> {
        self.collection.delete_batch(deposits).await
    }

    pub async fn delete_all(&self) -> Result<()> {
        self.collection.delete_by_filter::<Deposit, QueryFilter>(None).await
    }

    pub async fn delete_by_filter<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<()> {
        self.collection.delete_by_filter::<Deposit, Q>(Some(filter)).await
    }

    pub async fn migrate(&self) -> Result<Document<Migration>> {
        self.collection.migrate(Deposit::schema()).await
    }

    pub async fn collection_exists(&self) -> Result<bool> {
        self.collection.collection_exists(Deposit::schema()).await
    }
}
