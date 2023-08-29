use crate::common::create_database;
use bip32::{Language, Mnemonic};
use mystiko_core::handler::wallet::WalletHandler;
use mystiko_protos::core::handler::v1::CreateWalletOptions;
use mystiko_storage::formatter::sql::SqlStatementFormatter;
use mystiko_storage_sqlite::SqliteStorage;
use rand_core::OsRng;
use std::sync::Arc;

async fn setup() -> WalletHandler<SqlStatementFormatter, SqliteStorage> {
    let database = create_database().await;
    database.migrate().await.unwrap();
    WalletHandler::new(Arc::new(database))
}

#[tokio::test]
async fn test_create() {
    let handler = setup().await;
    let options = CreateWalletOptions::builder()
        .password(String::from("P@ssw0rd"))
        .build();
    let wallet = handler.create(&options).await.unwrap();
    assert_eq!(handler.check_current().await.unwrap(), wallet);
}

#[tokio::test]
async fn test_create_with_mnemonic() {
    let handler = setup().await;
    let mnemonic = Mnemonic::random(OsRng, Language::English);
    let options = CreateWalletOptions::builder()
        .password(String::from("P@ssw0rd"))
        .mnemonic_phrase(mnemonic.phrase().to_string())
        .build();
    handler.create(&options).await.unwrap();
    assert_eq!(
        handler.export_mnemonic_phrase("P@ssw0rd").await.unwrap(),
        mnemonic.phrase().to_string()
    );
}

#[tokio::test]
async fn test_create_with_invalid_password() {
    let handler = setup().await;
    let mut options = CreateWalletOptions::builder()
        .password(String::from("AAAAAAAA"))
        .build();
    assert!(handler.create(&options).await.is_err());
    options.password = String::from("aaaaaaaa");
    assert!(handler.create(&options).await.is_err());
    options.password = String::from("12345678");
    assert!(handler.create(&options).await.is_err());
    options.password = String::from("AaBbCc12");
    assert!(handler.create(&options).await.is_err());
    options.password = String::from("AaBb@12");
    assert!(handler.create(&options).await.is_err());
}

#[tokio::test]
async fn test_current() {
    let handler = setup().await;
    assert!(handler.current().await.unwrap().is_none());
    let options = CreateWalletOptions::builder()
        .password(String::from("P@ssw0rd"))
        .build();
    let wallet = handler.create(&options).await.unwrap();
    assert_eq!(handler.current().await.unwrap(), Some(wallet));
}

#[tokio::test]
async fn test_check_current() {
    let handler = setup().await;
    assert!(handler.check_current().await.is_err());
    let options = CreateWalletOptions::builder()
        .password(String::from("P@ssw0rd"))
        .build();
    let wallet = handler.create(&options).await.unwrap();
    assert_eq!(handler.check_current().await.unwrap(), wallet);
}

#[tokio::test]
async fn test_check_password() {
    let handler = setup().await;
    assert!(handler.check_password("P@ssw0rd").await.is_err());
    let options = CreateWalletOptions::builder()
        .password(String::from("P@ssw0rd"))
        .build();
    handler.create(&options).await.unwrap();
    assert!(handler.check_password("P@ssw0rd").await.is_ok());
    assert!(handler.check_password("wrong_password").await.is_err());
}

#[tokio::test]
async fn test_export_mnemonic_words() {
    let handler = setup().await;
    assert!(handler.export_mnemonic_phrase("P@ssw0rd").await.is_err());
    let options = CreateWalletOptions::builder()
        .password(String::from("P@ssw0rd"))
        .build();
    handler.create(&options).await.unwrap();
    assert!(handler.export_mnemonic_phrase("wrong_password").await.is_err());
    let mnemonic_words = handler.export_mnemonic_phrase("P@ssw0rd").await.unwrap();
    assert!(Mnemonic::new(mnemonic_words, Language::English).is_ok());
}

#[tokio::test]
async fn test_update_password() {
    let handler = setup().await;
    assert!(handler.update_password("P@ssw0rd", "P@ssw0rd2").await.is_err());
    let options = CreateWalletOptions::builder()
        .password(String::from("P@ssw0rd"))
        .build();
    handler.create(&options).await.unwrap();
    let mnemonic_words = handler.export_mnemonic_phrase("P@ssw0rd").await.unwrap();
    assert!(handler.update_password("wrong_password", "P@ssw0rd2").await.is_err());
    assert!(handler.update_password("P@ssw0rd", "invalid_password").await.is_err());
    assert!(handler.update_password("P@ssw0rd", "newP@ssw0rd").await.is_ok());
    assert!(handler.check_password("newP@ssw0rd").await.is_ok());
    assert_eq!(
        handler.export_mnemonic_phrase("newP@ssw0rd").await.unwrap(),
        mnemonic_words
    );
}
