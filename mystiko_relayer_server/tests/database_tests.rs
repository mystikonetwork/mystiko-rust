use mystiko_relayer_server::database::init_sqlite_database;

const SQLITE_DATABASE_PATH: &str = "./tests/files/sqlite_database_test.db";

#[actix_rt::test]
async fn test_init_sqlite_database() {
    let sqlite = init_sqlite_database(SQLITE_DATABASE_PATH).await;
    assert!(sqlite.is_ok());
    // delete database file
    std::fs::remove_file(SQLITE_DATABASE_PATH).unwrap();
}
