#![forbid(unsafe_code)]
use crate::document::deposit::Deposit;
use futures::lock::Mutex;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::{Document, DocumentData, DocumentRawData};
use mystiko_storage::filter::QueryFilter;
use mystiko_storage::formatter::StatementFormatter;
use mystiko_storage::migration::Migration;
use mystiko_storage::storage::Storage;
use std::io::Error;
use std::sync::Arc;

pub struct DepositCollection<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    collection: Arc<Mutex<Collection<F, R, S>>>,
}

impl<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> DepositCollection<F, R, S> {
    pub fn new(collection: Arc<Mutex<Collection<F, R, S>>>) -> Self {
        DepositCollection { collection }
    }

    pub async fn insert(&self, deposit: &Deposit) -> Result<Document<Deposit>, Error> {
        let mut collection = self.collection.lock().await;
        collection.insert(deposit).await
    }

    pub async fn insert_batch(
        &self,
        deposits: &Vec<Deposit>,
    ) -> Result<Vec<Document<Deposit>>, Error> {
        let mut collection = self.collection.lock().await;
        collection.insert_batch(deposits).await
    }

    pub async fn find(&self, filter: QueryFilter) -> Result<Vec<Document<Deposit>>, Error> {
        let mut collection = self.collection.lock().await;
        collection.find::<Deposit>(Some(filter)).await
    }

    pub async fn find_all(&self) -> Result<Vec<Document<Deposit>>, Error> {
        let mut collection = self.collection.lock().await;
        collection.find::<Deposit>(None).await
    }

    pub async fn find_one(&self, filter: QueryFilter) -> Result<Option<Document<Deposit>>, Error> {
        let mut collection = self.collection.lock().await;
        collection.find_one(Some(filter)).await
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Document<Deposit>>, Error> {
        let mut collection = self.collection.lock().await;
        collection.find_by_id(id).await
    }

    pub async fn count(&self, filter: QueryFilter) -> Result<u64, Error> {
        let mut collection = self.collection.lock().await;
        collection.count::<Deposit>(Some(filter)).await
    }

    pub async fn count_all(&self) -> Result<u64, Error> {
        let mut collection = self.collection.lock().await;
        collection.count::<Deposit>(None).await
    }

    pub async fn update(&self, deposit: &Document<Deposit>) -> Result<Document<Deposit>, Error> {
        let mut collection = self.collection.lock().await;
        collection.update(deposit).await
    }

    pub async fn update_batch(
        &self,
        deposits: &Vec<Document<Deposit>>,
    ) -> Result<Vec<Document<Deposit>>, Error> {
        let mut collection = self.collection.lock().await;
        collection.update_batch(deposits).await
    }

    pub async fn delete(&self, deposit: &Document<Deposit>) -> Result<(), Error> {
        let mut collection = self.collection.lock().await;
        collection.delete(deposit).await
    }

    pub async fn delete_batch(&self, deposits: &Vec<Document<Deposit>>) -> Result<(), Error> {
        let mut collection = self.collection.lock().await;
        collection.delete_batch(deposits).await
    }

    pub async fn delete_all(&self) -> Result<(), Error> {
        let mut collection = self.collection.lock().await;
        collection.delete_by_filter::<Deposit>(None).await
    }

    pub async fn delete_by_filter(&self, filter: QueryFilter) -> Result<(), Error> {
        let mut collection = self.collection.lock().await;
        collection.delete_by_filter::<Deposit>(Some(filter)).await
    }

    pub async fn migrate(&self) -> Result<Document<Migration>, Error> {
        let mut collection = self.collection.lock().await;
        collection.migrate(Deposit::schema()).await
    }

    pub async fn collection_exists(&self) -> Result<bool, Error> {
        let mut collection = self.collection.lock().await;
        collection.collection_exists(Deposit::schema()).await
    }
}
