#![forbid(unsafe_code)]

use crate::collection::account::AccountCollection;
use crate::collection::deposit::DepositCollection;
use crate::collection::transaction::TransactionCollection;
use crate::collection::wallet::WalletCollection;
use futures::lock::Mutex;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::{Document, DocumentRawData};
use mystiko_storage::formatter::StatementFormatter;
use mystiko_storage::migration::Migration;
use mystiko_storage::storage::Storage;
use std::io::Error;
use std::sync::Arc;

pub struct Database<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    pub accounts: AccountCollection<F, R, S>,
    pub wallets: WalletCollection<F, R, S>,
    pub deposits: DepositCollection<F, R, S>,
    pub transactions: TransactionCollection<F, R, S>,
}

impl<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> Database<F, R, S> {
    pub fn new(formatter: F, storage: S) -> Self {
        let collection = Arc::new(Mutex::new(Collection::new(formatter, storage)));
        Database {
            accounts: AccountCollection::new(collection.clone()),
            deposits: DepositCollection::new(collection.clone()),
            transactions: TransactionCollection::new(collection.clone()),
            wallets: WalletCollection::new(collection),
        }
    }

    pub async fn migrate(&self) -> Result<Vec<Document<Migration>>, Error> {
        let migrations: Vec<Document<Migration>> = vec![
            self.accounts.migrate().await?,
            self.wallets.migrate().await?,
            self.deposits.migrate().await?,
            self.transactions.migrate().await?,
        ];
        Ok(migrations)
    }
}
