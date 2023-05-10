#![forbid(unsafe_code)]
use crate::document::wallet::Wallet;
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
pub struct WalletCollection<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    collection: Arc<Collection<F, R, S>>,
}

impl<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> WalletCollection<F, R, S> {
    pub fn new(collection: Arc<Collection<F, R, S>>) -> Self {
        WalletCollection { collection }
    }

    pub async fn insert(&self, wallet: &Wallet) -> Result<Document<Wallet>> {
        self.collection
            .insert(wallet)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn insert_batch(&self, wallets: &Vec<Wallet>) -> Result<Vec<Document<Wallet>>> {
        self.collection
            .insert_batch(wallets)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn find<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<Vec<Document<Wallet>>> {
        self.collection
            .find::<Wallet, Q>(Some(filter))
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn find_all(&self) -> Result<Vec<Document<Wallet>>> {
        self.collection
            .find::<Wallet, QueryFilter>(None)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn find_one<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<Option<Document<Wallet>>> {
        self.collection
            .find_one(Some(filter))
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Document<Wallet>>> {
        self.collection
            .find_by_id(id)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn count<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<u64> {
        self.collection
            .count::<Wallet, Q>(Some(filter))
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn count_all(&self) -> Result<u64> {
        self.collection
            .count::<Wallet, QueryFilter>(None)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn update(&self, wallet: &Document<Wallet>) -> Result<Document<Wallet>> {
        self.collection
            .update(wallet)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn update_batch(&self, wallets: &Vec<Document<Wallet>>) -> Result<Vec<Document<Wallet>>> {
        self.collection
            .update_batch(wallets)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn delete(&self, wallet: &Document<Wallet>) -> Result<()> {
        self.collection
            .delete(wallet)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn delete_batch(&self, wallets: &Vec<Document<Wallet>>) -> Result<()> {
        self.collection
            .delete_batch(wallets)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn delete_all(&self) -> Result<()> {
        self.collection
            .delete_by_filter::<Wallet, QueryFilter>(None)
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn delete_by_filter<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<()> {
        self.collection
            .delete_by_filter::<Wallet, Q>(Some(filter))
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn migrate(&self) -> Result<Document<Migration>> {
        self.collection
            .migrate(Wallet::schema())
            .await
            .map_err(DatabaseError::StorageError)
    }

    pub async fn collection_exists(&self) -> Result<bool> {
        self.collection
            .collection_exists(Wallet::schema())
            .await
            .map_err(DatabaseError::StorageError)
    }
}
