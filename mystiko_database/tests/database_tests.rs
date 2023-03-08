use mystiko_database::database::Database;
use mystiko_storage::formatter::SqlFormatter;
use mystiko_storage_sqlite::SqliteStorageBuilder;
use tokio_test::block_on;

#[test]
fn test_database_migration() {
    let storage = block_on(SqliteStorageBuilder::new().build()).unwrap();
    let database = Database::new(SqlFormatter {}, storage);
    block_on(database.migrate()).unwrap();
    assert!(block_on(database.accounts.collection_exists()).unwrap());
    assert!(block_on(database.deposits.collection_exists()).unwrap());
    assert!(block_on(database.nullifiers.collection_exists()).unwrap());
    assert!(block_on(database.transactions.collection_exists()).unwrap());
    assert!(block_on(database.wallets.collection_exists()).unwrap());
    assert!(block_on(database.chains.collection_exists()).unwrap());
    assert!(block_on(database.contracts.collection_exists()).unwrap());
    assert!(block_on(database.commitments.collection_exists()).unwrap());
}
