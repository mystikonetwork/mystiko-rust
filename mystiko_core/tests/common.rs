use mystiko_database::database::Database;
use mystiko_storage::formatter::SqlFormatter;
use mystiko_storage_sqlite::{SqliteRawData, SqliteStorage, SqliteStorageBuilder};

pub async fn create_database() -> Database<SqlFormatter, SqliteRawData, SqliteStorage> {
    let formatter = SqlFormatter {};
    let storage = SqliteStorageBuilder::new()
        .in_memory()
        .build()
        .await
        .unwrap();
    Database::new(formatter, storage)
}
