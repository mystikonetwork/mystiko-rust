use tokio_test::block_on;
use mystiko_database::database::Database;
use mystiko_storage::formatter::SqlFormatter;
use mystiko_storage_sqlite::SqliteStorageBuilder;

#[test]
fn test_database_migration() {
    let storage = block_on(SqliteStorageBuilder::new().build()).unwrap();
    let database = Database::new(SqlFormatter{}, storage);
    block_on(database.migrate()).unwrap();
    assert!(block_on(database.wallets.collection_exists()).unwrap());
}