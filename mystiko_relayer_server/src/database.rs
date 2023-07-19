use crate::document::account::AccountCollection;
use crate::document::transaction::TransactionCollection;
use anyhow::Result;
use log::info;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::Document;
use mystiko_storage::formatter::sql::SqlStatementFormatter;
use mystiko_storage::formatter::types::StatementFormatter;
use mystiko_storage::migration::history::MigrationHistory;
use mystiko_storage::storage::Storage;
use mystiko_storage_sqlite::{SqliteStorage, SqliteStorageBuilder};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

#[derive(Debug)]
pub struct Database<F: StatementFormatter, S: Storage> {
    pub accounts: AccountCollection<F, S>,
    pub transactions: TransactionCollection<F, S>,
}

impl<F: StatementFormatter, S: Storage> Database<F, S> {
    pub fn new(formatter: F, storage: S) -> Self {
        let collection = Arc::new(Collection::new(formatter, storage));
        Database {
            accounts: AccountCollection::new(collection.clone()),
            transactions: TransactionCollection::new(collection),
        }
    }

    pub async fn migrate(&self) -> Result<Vec<Document<MigrationHistory>>> {
        let migrations: Vec<Document<MigrationHistory>> =
            vec![self.accounts.migrate().await?, self.transactions.migrate().await?];
        Ok(migrations)
    }
}

pub async fn init_sqlite_database(path: &str) -> Result<Database<SqlStatementFormatter, SqliteStorage>> {
    let storage = init_sqlite_storage(path).await?;
    let database = Database::new(SqlStatementFormatter::default(), storage);
    database.migrate().await?;
    Ok(database)
}

async fn init_sqlite_storage(path: &str) -> Result<SqliteStorage> {
    if !Path::new(path).exists() {
        info!("path {} db file not exists, create sqlite db file", path);
        let mut file = File::create(path)?;
        file.write_all(b"")?;
    }
    let storage = SqliteStorageBuilder::new().path(path).build().await?;
    Ok(storage)
}
