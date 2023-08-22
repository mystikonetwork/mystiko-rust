use crate::common::create_database;
use mystiko_core::handler::account::{
    AccountHandler, CreateAccountOptions, UpdateAccountOptions, DEFAULT_ACCOUNT_SCAN_SIZE,
};
use mystiko_core::handler::wallet::{CreateWalletOptions, WalletHandler};
use mystiko_crypto::crypto::decrypt_symmetric;
use mystiko_database::database::Database;
use mystiko_database::document::account::AccountColumn;
use mystiko_protocol::address::ShieldedAddress;
use mystiko_protocol::key::full_public_key;
use mystiko_protocol::types::{FullPk, FullSk};
use mystiko_protos::storage::v1::SubFilter;
use mystiko_storage::formatter::sql::SqlStatementFormatter;
use mystiko_storage_sqlite::SqliteStorage;
use mystiko_types::AccountStatus;
use mystiko_utils::hex::{decode_hex_with_length, encode_hex};
use std::sync::Arc;

type TypedDatabase = Database<SqlStatementFormatter, SqliteStorage>;
type TypedWalletHandler = WalletHandler<SqlStatementFormatter, SqliteStorage>;
type TypedAccountHandler = AccountHandler<SqlStatementFormatter, SqliteStorage>;

const DEFAULT_WALLET_PASSWORD: &str = "P@ssw0rd";

async fn setup() -> (TypedAccountHandler, TypedWalletHandler, Arc<TypedDatabase>) {
    let database = Arc::new(create_database().await);
    database.migrate().await.unwrap();
    let wallet_handler = WalletHandler::new(database.clone());
    wallet_handler
        .create(
            &CreateWalletOptions::builder()
                .password(DEFAULT_WALLET_PASSWORD.to_string())
                .build(),
        )
        .await
        .unwrap();
    (AccountHandler::new(database.clone()), wallet_handler, database)
}

#[tokio::test]
async fn test_create_default() {
    let (account_handler, _, _) = setup().await;
    let options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let account = account_handler.create(&options).await.unwrap();
    assert_eq!(account.data.name, "Account 1");
    assert_eq!(account.data.scan_size, DEFAULT_ACCOUNT_SCAN_SIZE);
    assert_eq!(account.data.status, AccountStatus::Created);
    let full_pk: FullPk = decode_hex_with_length(&account.data.public_key).unwrap();
    assert_eq!(
        account.data.shielded_address,
        ShieldedAddress::from_full_public_key(&full_pk).address()
    );
    let full_sk_str = decrypt_symmetric(DEFAULT_WALLET_PASSWORD, &account.data.encrypted_secret_key).unwrap();
    let full_sk: FullSk = decode_hex_with_length(&full_sk_str).unwrap();
    assert_eq!(full_pk, full_public_key(&full_sk));
}

#[tokio::test]
async fn test_create_with_name() {
    let (account_handler, _, _) = setup().await;
    let mut options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .name(String::from(""))
        .build();
    let account1 = account_handler.create(&options).await.unwrap();
    assert_eq!(account1.data.name, "Account 1");
    options.name = Some(String::from("Awesome Account 2"));
    let account2 = account_handler.create(&options).await.unwrap();
    assert_eq!(account2.data.name, "Awesome Account 2");
    assert_ne!(account1.data.shielded_address, account2.data.shielded_address);
}

#[tokio::test]
async fn test_create_with_scan_size() {
    let (account_handler, _, _) = setup().await;
    let mut options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .scan_size(0)
        .build();
    let mut account = account_handler.create(&options).await.unwrap();
    assert_eq!(account.data.scan_size, DEFAULT_ACCOUNT_SCAN_SIZE);
    options.scan_size = Some(100);
    account = account_handler.create(&options).await.unwrap();
    assert_eq!(account.data.scan_size, 100);
}

#[tokio::test]
async fn test_create_with_secret_key() {
    let (account_handler, _, db) = setup().await;
    let mut options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let account1 = account_handler.create(&options).await.unwrap();
    options.secret_key = Some(
        account_handler
            .export_secret_key_by_id(DEFAULT_WALLET_PASSWORD, &account1.id)
            .await
            .unwrap(),
    );
    let account2 = account_handler.create(&options).await.unwrap();
    db.accounts.delete_all().await.unwrap();
    let account3 = account_handler.create(&options).await.unwrap();
    assert_eq!(account1.id, account2.id);
    assert_ne!(account2.id, account3.id);
    assert_eq!(account1.data.shielded_address, account2.data.shielded_address);
    assert_eq!(account2.data.shielded_address, account3.data.shielded_address);
}

#[tokio::test]
async fn test_create_with_wrong_password() {
    let (account_handler, _, _) = setup().await;
    let options = CreateAccountOptions::builder()
        .wallet_password(String::from("wrong password"))
        .build();
    let result = account_handler.create(&options).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_with_no_wallet() {
    let (account_handler, _, db) = setup().await;
    let options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    db.wallets.delete_all().await.unwrap();
    let result = account_handler.create(&options).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_with_same_wallet_mnemonic_phrase() {
    let (account_handler, wallet_handler, db) = setup().await;
    let mut options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let wallet1 = wallet_handler.check_current().await.unwrap();
    let mnemonic_phrase = wallet_handler
        .export_mnemonic_phrase(DEFAULT_WALLET_PASSWORD)
        .await
        .unwrap();
    let account1 = account_handler.create(&options).await.unwrap();
    let account2 = account_handler.create(&options).await.unwrap();
    db.accounts.delete_all().await.unwrap();
    db.wallets.delete_all().await.unwrap();
    let wallet_options = CreateWalletOptions::builder()
        .password(String::from("newP@ssw0rd"))
        .mnemonic_phrase(mnemonic_phrase)
        .build();
    let wallet2 = wallet_handler.create(&wallet_options).await.unwrap();
    assert_ne!(wallet1.id, wallet2.id);
    options.wallet_password = String::from("newP@ssw0rd");
    let account3 = account_handler.create(&options).await.unwrap();
    let account4 = account_handler.create(&options).await.unwrap();
    assert_ne!(account1.id, account3.id);
    assert_ne!(account2.id, account4.id);
    assert_eq!(account1.data.shielded_address, account3.data.shielded_address);
    assert_eq!(account2.data.shielded_address, account4.data.shielded_address);
}

#[tokio::test]
async fn test_find() {
    let (account_handler, _, _) = setup().await;
    assert_eq!(account_handler.find_all().await.unwrap(), vec![]);
    let mut options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let account1 = account_handler.create(&options).await.unwrap();
    let mut accounts = account_handler.find_all().await.unwrap();
    assert_eq!(accounts, vec![account1.clone()]);
    options.scan_size = Some(200);
    let account2 = account_handler.create(&options).await.unwrap();
    let filter = SubFilter::less_equal(AccountColumn::ScanSize, 200);
    accounts = account_handler.find(filter).await.unwrap();
    assert_eq!(accounts, vec![account2.clone()]);
    accounts = account_handler.find_all().await.unwrap();
    accounts.sort_by_key(|a| a.id.to_string());
    assert_eq!(accounts, vec![account1, account2]);
}

#[tokio::test]
async fn test_find_by_id() {
    let (account_handler, _, _) = setup().await;
    let options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let account1 = account_handler.create(&options).await.unwrap();
    let account2 = account_handler.create(&options).await.unwrap();
    let account3 = account_handler.find_by_id(&account1.id).await.unwrap().unwrap();
    assert_eq!(account1, account3);
    let account4 = account_handler.find_by_id(&account2.id).await.unwrap().unwrap();
    assert_eq!(account2, account4);
    assert!(account_handler.find_by_id("non_existing_id").await.unwrap().is_none());
}

#[tokio::test]
async fn test_find_by_public_key() {
    let (account_handler, _, _) = setup().await;
    let options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let account1 = account_handler.create(&options).await.unwrap();
    let account2 = account_handler.create(&options).await.unwrap();
    let account3 = account_handler
        .find_by_public_key(&account1.data.public_key)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(account1, account3);
    let account4 = account_handler
        .find_by_public_key(&account2.data.public_key)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(account2, account4);
    assert!(account_handler
        .find_by_public_key("non_existing_public_key")
        .await
        .unwrap()
        .is_none());
}

#[tokio::test]
async fn test_find_by_shielded_address() {
    let (account_handler, _, _) = setup().await;
    let options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let account1 = account_handler.create(&options).await.unwrap();
    let account2 = account_handler.create(&options).await.unwrap();
    let account3 = account_handler
        .find_by_shielded_address(&account1.data.shielded_address)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(account1, account3);
    let account4 = account_handler
        .find_by_shielded_address(&account2.data.shielded_address)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(account2, account4);
    assert!(account_handler
        .find_by_shielded_address("non_existing_shielded_address")
        .await
        .unwrap()
        .is_none());
}

#[tokio::test]
async fn test_count() {
    let (account_handler, _, _) = setup().await;
    assert_eq!(account_handler.count_all().await.unwrap(), 0);
    let mut options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    account_handler.create(&options).await.unwrap();
    assert_eq!(account_handler.count_all().await.unwrap(), 1);
    options.scan_size = Some(200);
    account_handler.create(&options).await.unwrap();
    assert_eq!(account_handler.count_all().await.unwrap(), 2);
    let filter = SubFilter::less_equal(AccountColumn::ScanSize, 200);
    assert_eq!(account_handler.count(filter).await.unwrap(), 1);
}

#[tokio::test]
async fn test_update_name() {
    let (account_handler, _, _) = setup().await;
    let options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let account = account_handler.create(&options).await.unwrap();
    let mut update_options = UpdateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .name(String::new())
        .build();
    let mut updated_account = account_handler
        .update_by_public_key(&account.data.public_key, &update_options)
        .await
        .unwrap();
    assert_eq!(updated_account.data.name, account.data.name);
    assert_eq!(updated_account.updated_at, account.updated_at);
    update_options.name = Some(account.data.name.clone());
    updated_account = account_handler
        .update_by_shielded_address(&account.data.shielded_address, &update_options)
        .await
        .unwrap();
    assert_eq!(updated_account.data.name, account.data.name);
    assert_eq!(updated_account.updated_at, account.updated_at);
    update_options.name = Some(String::from("Awesome Account Name"));
    updated_account = account_handler
        .update_by_shielded_address(&account.data.shielded_address, &update_options)
        .await
        .unwrap();
    assert_eq!(updated_account.data.name, "Awesome Account Name");
}

#[tokio::test]
async fn test_update_scan_size() {
    let (account_handler, _, _) = setup().await;
    let options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let account = account_handler.create(&options).await.unwrap();
    let mut update_options = UpdateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .scan_size(0)
        .build();
    let mut updated_account = account_handler
        .update_by_public_key(&account.data.public_key, &update_options)
        .await
        .unwrap();
    assert_eq!(updated_account.data.scan_size, account.data.scan_size);
    assert_eq!(updated_account.updated_at, account.updated_at);
    update_options.scan_size = Some(account.data.scan_size);
    updated_account = account_handler
        .update_by_shielded_address(&account.data.shielded_address, &update_options)
        .await
        .unwrap();
    assert_eq!(updated_account.data.scan_size, account.data.scan_size);
    assert_eq!(updated_account.updated_at, account.updated_at);
    update_options.scan_size = Some(200);
    updated_account = account_handler
        .update_by_shielded_address(&account.data.shielded_address, &update_options)
        .await
        .unwrap();
    assert_eq!(updated_account.data.scan_size, 200);
}

#[tokio::test]
async fn test_update_status() {
    let (account_handler, _, _) = setup().await;
    let options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let account = account_handler.create(&options).await.unwrap();
    let mut update_options = UpdateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .status(account.data.status.clone())
        .build();
    let mut updated_account = account_handler
        .update_by_public_key(&account.data.public_key, &update_options)
        .await
        .unwrap();
    assert_eq!(updated_account.data.status, account.data.status);
    assert_eq!(updated_account.updated_at, account.updated_at);
    update_options.status = Some(AccountStatus::Scanning);
    updated_account = account_handler
        .update_by_shielded_address(&account.data.shielded_address, &update_options)
        .await
        .unwrap();
    assert_eq!(updated_account.data.status, AccountStatus::Scanning);
}

#[tokio::test]
async fn test_update_wrong_wallet_password() {
    let (account_handler, _, _) = setup().await;
    let options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let account = account_handler.create(&options).await.unwrap();
    let update_options = UpdateAccountOptions::builder()
        .wallet_password(String::from("wrong_password"))
        .build();
    let mut result = account_handler.update_by_id(&account.id, &update_options).await;
    assert!(result.is_err());
    result = account_handler
        .update_by_public_key(&account.data.public_key, &update_options)
        .await;
    assert!(result.is_err());
    result = account_handler
        .update_by_shielded_address(&account.data.shielded_address, &update_options)
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_non_existing_account() {
    let (account_handler, _, _) = setup().await;
    let update_options = UpdateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let mut result = account_handler.update_by_id("non_existing_id", &update_options).await;
    assert!(result.is_err());
    result = account_handler
        .update_by_public_key("non_existing_public_key", &update_options)
        .await;
    assert!(result.is_err());
    result = account_handler
        .update_by_shielded_address("non_existing_address", &update_options)
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_export_secret_key() {
    let (account_handler, _, _) = setup().await;
    let options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let account = account_handler.create(&options).await.unwrap();
    let secret_key_str = account_handler
        .export_secret_key_by_id(DEFAULT_WALLET_PASSWORD, &account.id)
        .await
        .unwrap();
    let full_sk: FullSk = decode_hex_with_length(&secret_key_str).unwrap();
    let full_pk = full_public_key(&full_sk);
    assert_eq!(account.data.public_key, encode_hex(&full_pk));
    assert_eq!(
        account.data.shielded_address,
        ShieldedAddress::from_full_public_key(&full_pk).address()
    );
}

#[tokio::test]
async fn test_export_secret_key_wrong_wallet_password() {
    let (account_handler, _, _) = setup().await;
    let options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let account = account_handler.create(&options).await.unwrap();
    let result = account_handler
        .export_secret_key_by_public_key("wrong password", &account.data.public_key)
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_export_secret_key_non_existing_account() {
    let (account_handler, _, _) = setup().await;
    let result = account_handler
        .export_secret_key_by_shielded_address(DEFAULT_WALLET_PASSWORD, "non_existing_shielded_address")
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_encryption() {
    let (account_handler, wallet_handler, _) = setup().await;
    let options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let new_wallet_password: &str = "newP@ssw0rd";
    let account1 = account_handler.create(&options).await.unwrap();
    let account2 = account_handler.create(&options).await.unwrap();
    let sk1 = account_handler
        .export_secret_key_by_id(DEFAULT_WALLET_PASSWORD, &account1.id)
        .await
        .unwrap();
    let sk2 = account_handler
        .export_secret_key_by_id(DEFAULT_WALLET_PASSWORD, &account2.id)
        .await
        .unwrap();
    account_handler
        .update_encryption(DEFAULT_WALLET_PASSWORD, new_wallet_password)
        .await
        .unwrap();
    wallet_handler
        .update_password(DEFAULT_WALLET_PASSWORD, new_wallet_password)
        .await
        .unwrap();
    let sk3 = account_handler
        .export_secret_key_by_id(new_wallet_password, &account1.id)
        .await
        .unwrap();
    let sk4 = account_handler
        .export_secret_key_by_id(new_wallet_password, &account2.id)
        .await
        .unwrap();
    assert_eq!(sk1, sk3);
    assert_eq!(sk2, sk4);
}

#[tokio::test]
async fn test_update_encryption_wrong_wallet_password() {
    let (account_handler, _, _) = setup().await;
    let result = account_handler.update_encryption("wrong password", "newP@ssw0rd").await;
    assert!(result.is_err());
}
