use crate::common::create_database;
use bip32::{Language, Mnemonic as MnemonicBip32};
use bip39::Mnemonic as MnemonicBi39;
use itertools::Itertools;
use mystiko_core::WalletsError::MnemonicError;
use mystiko_core::{WalletHandler, Wallets};
use mystiko_protos::core::handler::v1::{CreateWalletOptions, MnemonicOptions};
use mystiko_protos::core::v1::MnemonicType;
use mystiko_storage::SqlStatementFormatter;
use mystiko_storage_sqlite::SqliteStorage;
use rand_core::OsRng;
use std::sync::Arc;

async fn setup() -> Wallets<SqlStatementFormatter, SqliteStorage> {
    let database = create_database().await;
    database.migrate().await.unwrap();
    Wallets::new(Arc::new(database))
}

#[tokio::test]
async fn test_create() {
    let handler = setup().await;
    let options = CreateWalletOptions::builder()
        .password(String::from("P@ssw0rd"))
        .build();
    let wallet = handler.create(&options).await.unwrap();
    assert_eq!(handler.check_current().await.unwrap(), wallet);
    assert_eq!(wallet.mnemonic_type, MnemonicType::Rust as i32);
}

#[tokio::test]
async fn test_create_with_web_mnemonic() {
    let handler = setup().await;
    let mnemonic = MnemonicBi39::generate(24).unwrap();
    let options = CreateWalletOptions::builder()
        .password(String::from("P@ssw0rd"))
        .mnemonic(
            MnemonicOptions::builder()
                .mnemonic_type(MnemonicType::Web as i32)
                .mnemonic_phrase(mnemonic.words().collect_vec().iter().join(" ").to_string())
                .build(),
        )
        .build();
    let result = handler.create(&options).await;
    assert_eq!(
        result.err().unwrap().to_string(),
        MnemonicError(bip39::Error::BadWordCount(24)).to_string()
    );

    let mnemonic = MnemonicBi39::generate(12).unwrap();
    let options = CreateWalletOptions::builder()
        .password(String::from("P@ssw0rd"))
        .mnemonic(
            MnemonicOptions::builder()
                .mnemonic_type(MnemonicType::Web as i32)
                .mnemonic_phrase(mnemonic.words().collect_vec().iter().join(" ").to_string())
                .build(),
        )
        .build();
    let wallet = handler.create(&options).await.unwrap();
    let phrase = mnemonic.words().collect_vec().iter().join(" ").to_string();
    assert_eq!(handler.export_mnemonic_phrase("P@ssw0rd").await.unwrap(), phrase);
    assert_eq!(wallet.mnemonic_type, MnemonicType::Web as i32);
}

#[tokio::test]
async fn test_create_with_rust_mnemonic() {
    let handler = setup().await;
    let mnemonic = MnemonicBip32::random(OsRng, Language::English);
    let options = CreateWalletOptions::builder()
        .password(String::from("P@ssw0rd"))
        .mnemonic(
            MnemonicOptions::builder()
                .mnemonic_type(MnemonicType::Rust as i32)
                .mnemonic_phrase(mnemonic.phrase().to_string())
                .build(),
        )
        .build();
    let wallet = handler.create(&options).await.unwrap();
    assert_eq!(
        handler.export_mnemonic_phrase("P@ssw0rd").await.unwrap(),
        mnemonic.phrase().to_string()
    );
    assert_eq!(wallet.mnemonic_type, MnemonicType::Rust as i32);

    let mnemonic = MnemonicBi39::generate(12).unwrap();
    let options = CreateWalletOptions::builder()
        .password(String::from("P@ssw0rd"))
        .mnemonic(
            MnemonicOptions::builder()
                .mnemonic_type(MnemonicType::Rust as i32)
                .mnemonic_phrase(mnemonic.words().collect_vec().iter().join(" ").to_string())
                .build(),
        )
        .build();
    let result = handler.create(&options).await;
    assert_eq!(
        result.err().unwrap().to_string(),
        MnemonicError(bip39::Error::BadWordCount(12)).to_string()
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
    assert!(MnemonicBip32::new(mnemonic_words, Language::English).is_ok());
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
