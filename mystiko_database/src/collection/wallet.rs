#![forbid(unsafe_code)]
use crate::document::wallet::Wallet;
use futures::lock::Mutex;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::{Document, DocumentData};
use mystiko_storage::filter::QueryFilter;
use mystiko_storage::formatter::StatementFormatter;
use mystiko_storage::migration::Migration;
use mystiko_storage::storage::Storage;
use std::io::Error;
use std::sync::Arc;

pub struct WalletCollection<F: StatementFormatter, S: Storage> {
    collection: Arc<Mutex<Collection<F, S>>>,
}

impl<F: StatementFormatter, S: Storage> WalletCollection<F, S> {
    pub fn new(collection: Arc<Mutex<Collection<F, S>>>) -> Self {
        WalletCollection { collection }
    }

    pub async fn insert(&self, wallet: &Wallet) -> Result<Document<Wallet>, Error> {
        let mut collection = self.collection.lock().await;
        collection.insert(wallet).await
    }

    pub async fn insert_batch(
        &self,
        wallets: &Vec<Wallet>,
    ) -> Result<Vec<Document<Wallet>>, Error> {
        let mut collection = self.collection.lock().await;
        collection.insert_batch(wallets).await
    }

    pub async fn find(&self, filter: QueryFilter) -> Result<Vec<Document<Wallet>>, Error> {
        let mut collection = self.collection.lock().await;
        collection.find::<Wallet>(Some(filter)).await
    }

    pub async fn find_all(&self) -> Result<Vec<Document<Wallet>>, Error> {
        let mut collection = self.collection.lock().await;
        collection.find::<Wallet>(None).await
    }

    pub async fn find_one(&self, filter: QueryFilter) -> Result<Option<Document<Wallet>>, Error> {
        let mut collection = self.collection.lock().await;
        collection.find_one(Some(filter)).await
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Document<Wallet>>, Error> {
        let mut collection = self.collection.lock().await;
        collection.find_by_id(id).await
    }

    pub async fn update(&self, wallet: &Document<Wallet>) -> Result<Document<Wallet>, Error> {
        let mut collection = self.collection.lock().await;
        collection.update(wallet).await
    }

    pub async fn update_batch(
        &self,
        wallets: &Vec<Document<Wallet>>,
    ) -> Result<Vec<Document<Wallet>>, Error> {
        let mut collection = self.collection.lock().await;
        collection.update_batch(wallets).await
    }

    pub async fn delete(&self, wallet: &Document<Wallet>) -> Result<(), Error> {
        let mut collection = self.collection.lock().await;
        collection.delete(wallet).await
    }

    pub async fn delete_batch(&self, wallets: &Vec<Document<Wallet>>) -> Result<(), Error> {
        let mut collection = self.collection.lock().await;
        collection.delete_batch(wallets).await
    }

    pub async fn migrate(&self) -> Result<Document<Migration>, Error> {
        let mut collection = self.collection.lock().await;
        collection.migrate(Wallet::schema()).await
    }

    pub async fn collection_exists(&self) -> Result<bool, Error> {
        let mut collection = self.collection.lock().await;
        collection.collection_exists(Wallet::schema()).await
    }
}
