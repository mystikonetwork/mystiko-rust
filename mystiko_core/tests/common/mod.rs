use mystiko_database::Database;
use mystiko_storage::SqlStatementFormatter;
use mystiko_storage_sqlite::{SqliteStorage, SqliteStorageBuilder};

pub async fn create_database() -> Database<SqlStatementFormatter, SqliteStorage> {
    let _ = env_logger::builder()
        .filter_module("mystiko_core", log::LevelFilter::Info)
        .is_test(true)
        .try_init();
    let formatter = SqlStatementFormatter::sqlite();
    let storage = SqliteStorageBuilder::new().in_memory().build().await.unwrap();
    Database::new(formatter, storage)
}
