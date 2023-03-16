#![forbid(unsafe_code)]
use crate::document::wallet::Wallet;
use anyhow::Result;
use futures::lock::Mutex;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::{Document, DocumentData, DocumentRawData};
use mystiko_storage::filter::QueryFilter;
use mystiko_storage::formatter::StatementFormatter;
use mystiko_storage::migration::Migration;
use mystiko_storage::storage::Storage;
use std::sync::Arc;

pub struct WalletCollection<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    collection: Arc<Mutex<Collection<F, R, S>>>,
}

impl<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> WalletCollection<F, R, S> {
    pub fn new(collection: Arc<Mutex<Collection<F, R, S>>>) -> Self {
        WalletCollection { collection }
    }

    pub async fn insert(&self, wallet: &Wallet) -> Result<Document<Wallet>> {
        let mut collection = self.collection.lock().await;
        collection.insert(wallet).await
    }

    pub async fn insert_batch(&self, wallets: &Vec<Wallet>) -> Result<Vec<Document<Wallet>>> {
        let mut collection = self.collection.lock().await;
        collection.insert_batch(wallets).await
    }

    pub async fn find(&self, filter: QueryFilter) -> Result<Vec<Document<Wallet>>> {
        let mut collection = self.collection.lock().await;
        collection.find::<Wallet>(Some(filter)).await
    }

    pub async fn find_all(&self) -> Result<Vec<Document<Wallet>>> {
        let mut collection = self.collection.lock().await;
        collection.find::<Wallet>(None).await
    }

    pub async fn find_one(&self, filter: QueryFilter) -> Result<Option<Document<Wallet>>> {
        let mut collection = self.collection.lock().await;
        collection.find_one(Some(filter)).await
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Document<Wallet>>> {
        let mut collection = self.collection.lock().await;
        collection.find_by_id(id).await
    }

    pub async fn count(&self, filter: QueryFilter) -> Result<u64> {
        let mut collection = self.collection.lock().await;
        collection.count::<Wallet>(Some(filter)).await
    }

    pub async fn count_all(&self) -> Result<u64> {
        let mut collection = self.collection.lock().await;
        collection.count::<Wallet>(None).await
    }

    pub async fn update(&self, wallet: &Document<Wallet>) -> Result<Document<Wallet>> {
        let mut collection = self.collection.lock().await;
        collection.update(wallet).await
    }

    pub async fn update_batch(
        &self,
        wallets: &Vec<Document<Wallet>>,
    ) -> Result<Vec<Document<Wallet>>> {
        let mut collection = self.collection.lock().await;
        collection.update_batch(wallets).await
    }

    pub async fn delete(&self, wallet: &Document<Wallet>) -> Result<()> {
        let mut collection = self.collection.lock().await;
        collection.delete(wallet).await
    }

    pub async fn delete_batch(&self, wallets: &Vec<Document<Wallet>>) -> Result<()> {
        let mut collection = self.collection.lock().await;
        collection.delete_batch(wallets).await
    }

    pub async fn delete_all(&self) -> Result<()> {
        let mut collection = self.collection.lock().await;
        collection.delete_by_filter::<Wallet>(None).await
    }

    pub async fn delete_by_filter(&self, filter: QueryFilter) -> Result<()> {
        let mut collection = self.collection.lock().await;
        collection.delete_by_filter::<Wallet>(Some(filter)).await
    }

    pub async fn migrate(&self) -> Result<Document<Migration>> {
        let mut collection = self.collection.lock().await;
        collection.migrate(Wallet::schema()).await
    }

    pub async fn collection_exists(&self) -> Result<bool> {
        let mut collection = self.collection.lock().await;
        collection.collection_exists(Wallet::schema()).await
    }
}
