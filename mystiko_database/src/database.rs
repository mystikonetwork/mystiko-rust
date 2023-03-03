#![forbid(unsafe_code)]

use crate::collection::wallet::WalletCollection;
use futures::lock::Mutex;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::Document;
use mystiko_storage::formatter::StatementFormatter;
use mystiko_storage::migration::Migration;
use mystiko_storage::storage::Storage;
use std::io::Error;
use std::sync::Arc;

pub struct Database<F: StatementFormatter, S: Storage> {
    pub wallets: WalletCollection<F, S>,
}

impl<F: StatementFormatter, S: Storage> Database<F, S> {
    pub fn new(formatter: F, storage: S) -> Self {
        let collection = Arc::new(Mutex::new(Collection::new(formatter, storage)));
        Database {
            wallets: WalletCollection::new(collection),
        }
    }

    pub async fn migrate(&self) -> Result<Vec<Document<Migration>>, Error> {
        let migrations: Vec<Document<Migration>> = vec![self.wallets.migrate().await?];
        Ok(migrations)
    }
}
