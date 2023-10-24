use mystiko_core::Database;
use mystiko_storage::SqlStatementFormatter;
use mystiko_storage_sqlite::SqliteStorage;

#[tokio::test]
async fn test_database_migration() {
    let storage = SqliteStorage::from_memory().await.unwrap();
    let database = Database::new(SqlStatementFormatter::sqlite(), storage);
    database.migrate().await.unwrap();
    assert!(database.accounts.collection_exists().await.unwrap());
    assert!(database.deposits.collection_exists().await.unwrap());
    assert!(database.nullifiers.collection_exists().await.unwrap());
    assert!(database.spends.collection_exists().await.unwrap());
    assert!(database.wallets.collection_exists().await.unwrap());
    assert!(database.contracts.collection_exists().await.unwrap());
    assert!(database.commitments.collection_exists().await.unwrap());
}
