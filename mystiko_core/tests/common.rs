use mystiko_database::database::Database;
use mystiko_storage::formatter::SqlFormatter;
use mystiko_storage_sqlite::{SqliteRawData, SqliteStorage, SqliteStorageBuilder};

pub async fn create_database() -> Database<SqlFormatter, SqliteRawData, SqliteStorage> {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init();
    let formatter = SqlFormatter {};
    let storage = SqliteStorageBuilder::new()
        .in_memory()
        .build()
        .await
        .unwrap();
    Database::new(formatter, storage)
}
