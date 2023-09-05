use mystiko_database::Database;
use mystiko_storage::SqlStatementFormatter;
use mystiko_storage_sqlite::SqliteStorageBuilder;

#[tokio::test]
async fn test_database_migration() {
    let storage = SqliteStorageBuilder::new().build().await.unwrap();
    let database = Database::new(SqlStatementFormatter::sqlite(), storage);
    database.migrate().await.unwrap();
    assert!(database.accounts.collection_exists().await.unwrap());
    assert!(database.deposits.collection_exists().await.unwrap());
    assert!(database.nullifiers.collection_exists().await.unwrap());
    assert!(database.transactions.collection_exists().await.unwrap());
    assert!(database.wallets.collection_exists().await.unwrap());
    assert!(database.chains.collection_exists().await.unwrap());
    assert!(database.contracts.collection_exists().await.unwrap());
    assert!(database.commitments.collection_exists().await.unwrap());
}
