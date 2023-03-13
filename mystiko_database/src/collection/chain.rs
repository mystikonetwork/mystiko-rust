#![forbid(unsafe_code)]
use crate::document::chain::Chain;
use futures::lock::Mutex;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::{Document, DocumentData, DocumentRawData};
use mystiko_storage::filter::QueryFilter;
use mystiko_storage::formatter::StatementFormatter;
use mystiko_storage::migration::Migration;
use mystiko_storage::storage::Storage;
use std::io::Error;
use std::sync::Arc;

pub struct ChainCollection<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    collection: Arc<Mutex<Collection<F, R, S>>>,
}

impl<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> ChainCollection<F, R, S> {
    pub fn new(collection: Arc<Mutex<Collection<F, R, S>>>) -> Self {
        ChainCollection { collection }
    }

    pub async fn insert(&self, chain: &Chain) -> Result<Document<Chain>, Error> {
        let mut collection = self.collection.lock().await;
        collection.insert(chain).await
    }

    pub async fn insert_batch(&self, chains: &Vec<Chain>) -> Result<Vec<Document<Chain>>, Error> {
        let mut collection = self.collection.lock().await;
        collection.insert_batch(chains).await
    }

    pub async fn find(&self, filter: QueryFilter) -> Result<Vec<Document<Chain>>, Error> {
        let mut collection = self.collection.lock().await;
        collection.find::<Chain>(Some(filter)).await
    }

    pub async fn find_all(&self) -> Result<Vec<Document<Chain>>, Error> {
        let mut collection = self.collection.lock().await;
        collection.find::<Chain>(None).await
    }

    pub async fn find_one(&self, filter: QueryFilter) -> Result<Option<Document<Chain>>, Error> {
        let mut collection = self.collection.lock().await;
        collection.find_one(Some(filter)).await
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Document<Chain>>, Error> {
        let mut collection = self.collection.lock().await;
        collection.find_by_id(id).await
    }

    pub async fn count(&self, filter: QueryFilter) -> Result<u64, Error> {
        let mut collection = self.collection.lock().await;
        collection.count::<Chain>(Some(filter)).await
    }

    pub async fn count_all(&self) -> Result<u64, Error> {
        let mut collection = self.collection.lock().await;
        collection.count::<Chain>(None).await
    }

    pub async fn update(&self, chain: &Document<Chain>) -> Result<Document<Chain>, Error> {
        let mut collection = self.collection.lock().await;
        collection.update(chain).await
    }

    pub async fn update_batch(
        &self,
        chains: &Vec<Document<Chain>>,
    ) -> Result<Vec<Document<Chain>>, Error> {
        let mut collection = self.collection.lock().await;
        collection.update_batch(chains).await
    }

    pub async fn delete(&self, chain: &Document<Chain>) -> Result<(), Error> {
        let mut collection = self.collection.lock().await;
        collection.delete(chain).await
    }

    pub async fn delete_batch(&self, chains: &Vec<Document<Chain>>) -> Result<(), Error> {
        let mut collection = self.collection.lock().await;
        collection.delete_batch(chains).await
    }

    pub async fn delete_all(&self) -> Result<(), Error> {
        let mut collection = self.collection.lock().await;
        collection.delete_by_filter::<Chain>(None).await
    }

    pub async fn delete_by_filter(&self, filter: QueryFilter) -> Result<(), Error> {
        let mut collection = self.collection.lock().await;
        collection.delete_by_filter::<Chain>(Some(filter)).await
    }

    pub async fn migrate(&self) -> Result<Document<Migration>, Error> {
        let mut collection = self.collection.lock().await;
        collection.migrate(Chain::schema()).await
    }

    pub async fn collection_exists(&self) -> Result<bool, Error> {
        let mut collection = self.collection.lock().await;
        collection.collection_exists(Chain::schema()).await
    }
}
