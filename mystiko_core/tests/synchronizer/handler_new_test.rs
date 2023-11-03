use mystiko_config::MystikoConfig;
use mystiko_core::{Database, Synchronizer, SynchronizerOptions};
use mystiko_ethers::{ChainConfigProvidersOptions, ProviderPool, Providers};
use mystiko_storage::SqlStatementFormatter;
use mystiko_storage_sqlite::SqliteStorage;
use std::sync::Arc;

#[tokio::test]
async fn test_handler_new() {
    let mystiko_config = MystikoConfig::from_json_file("tests/files/mystiko/config.json")
        .await
        .unwrap();
    let mystiko_config = Arc::new(mystiko_config);
    let providers: ProviderPool<ChainConfigProvidersOptions> = mystiko_config.clone().into();
    let providers = Arc::new(Box::new(providers) as Box<dyn Providers>);

    let formatter = SqlStatementFormatter::sqlite();
    let storage = SqliteStorage::from_memory().await.unwrap();
    let mystiko_db = Database::new(formatter, storage);
    let _ = mystiko_db.migrate().await.unwrap();
    let mystiko_db = Arc::new(mystiko_db);

    let option: SynchronizerOptions<SqlStatementFormatter, SqliteStorage> = SynchronizerOptions::builder()
        .providers(providers)
        .mystiko_config(mystiko_config)
        .db(mystiko_db)
        .build();
    let sync = Synchronizer::new(option).await;
    assert!(sync.is_ok());
}
