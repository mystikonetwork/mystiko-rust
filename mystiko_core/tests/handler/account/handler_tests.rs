use crate::common::create_database;
use mystiko_core::WalletHandler;
use mystiko_core::{AccountColumn, Database};
use mystiko_core::{AccountHandler, Accounts, Wallets};
use mystiko_crypto::crypto::decrypt_symmetric;
use mystiko_protocol::address::ShieldedAddress;
use mystiko_protocol::key::full_public_key;
use mystiko_protocol::types::{FullPk, FullSk};
use mystiko_protos::core::handler::v1::{
    CreateAccountOptions, CreateWalletOptions, MnemonicOptions, UpdateAccountOptions,
};
use mystiko_protos::core::v1::MnemonicType;
use mystiko_protos::storage::v1::SubFilter;
use mystiko_storage::SqlStatementFormatter;
use mystiko_storage_sqlite::SqliteStorage;
use mystiko_utils::hex::{decode_hex_with_length, encode_hex};
use std::sync::Arc;

type TypedDatabase = Database<SqlStatementFormatter, SqliteStorage>;
type TypedWalletHandler = Wallets<SqlStatementFormatter, SqliteStorage>;
type TypedAccountHandler = Accounts<SqlStatementFormatter, SqliteStorage>;

const DEFAULT_WALLET_PASSWORD: &str = "P@ssw0rd";

async fn setup() -> (TypedAccountHandler, TypedWalletHandler, Arc<TypedDatabase>) {
    let database = Arc::new(create_database().await);
    database.migrate().await.unwrap();
    let wallet_handler = Wallets::new(database.clone());
    wallet_handler
        .create(
            &CreateWalletOptions::builder()
                .password(DEFAULT_WALLET_PASSWORD.to_string())
                .build(),
        )
        .await
        .unwrap();
    (Accounts::new(database.clone()), wallet_handler, database)
}

#[tokio::test]
async fn test_create_default() {
    let (account_handler, _, _) = setup().await;
    let options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let account = account_handler.create(&options).await.unwrap();
    assert_eq!(account.name, "Account 1");
    let full_pk: FullPk = decode_hex_with_length(&account.public_key).unwrap();
    assert_eq!(
        account.shielded_address,
        ShieldedAddress::from_full_public_key(&full_pk).address()
    );
    let full_sk_str = decrypt_symmetric(DEFAULT_WALLET_PASSWORD, &account.encrypted_secret_key).unwrap();
    let full_sk: FullSk = decode_hex_with_length(full_sk_str).unwrap();
    assert_eq!(full_pk, full_public_key(&full_sk).unwrap());
}

#[tokio::test]
async fn test_create_with_name() {
    let (account_handler, _, _) = setup().await;
    let mut options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .name(String::from(""))
        .build();
    let account1 = account_handler.create(&options).await.unwrap();
    assert_eq!(account1.name, "Account 1");
    options.name = Some(String::from("Awesome Account 2"));
    let account2 = account_handler.create(&options).await.unwrap();
    assert_eq!(account2.name, "Awesome Account 2");
    assert_ne!(account1.shielded_address, account2.shielded_address);
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
    assert_eq!(account1.shielded_address, account2.shielded_address);
    assert_eq!(account2.shielded_address, account3.shielded_address);
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
        .mnemonic(
            MnemonicOptions::builder()
                .mnemonic_type(MnemonicType::Rust as i32)
                .mnemonic_phrase(mnemonic_phrase)
                .build(),
        )
        .build();
    let wallet2 = wallet_handler.create(&wallet_options).await.unwrap();
    assert_ne!(wallet1.id, wallet2.id);
    options.wallet_password = String::from("newP@ssw0rd");
    let account3 = account_handler.create(&options).await.unwrap();
    let account4 = account_handler.create(&options).await.unwrap();
    assert_ne!(account1.id, account3.id);
    assert_ne!(account2.id, account4.id);
    assert_eq!(account1.shielded_address, account3.shielded_address);
    assert_eq!(account2.shielded_address, account4.shielded_address);
}

#[tokio::test]
async fn test_find() {
    let (account_handler, _, _) = setup().await;
    assert_eq!(account_handler.find_all().await.unwrap(), vec![]);
    let options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    let account1 = account_handler.create(&options).await.unwrap();
    let mut accounts = account_handler.find_all().await.unwrap();
    assert_eq!(accounts, vec![account1.clone()]);
    let account2 = account_handler.create(&options).await.unwrap();
    let filter = SubFilter::equal(AccountColumn::ShieldedAddress, account2.shielded_address.clone());
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
        .find_by_public_key(&account1.public_key)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(account1, account3);
    let account4 = account_handler
        .find_by_public_key(&account2.public_key)
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
        .find_by_shielded_address(&account1.shielded_address)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(account1, account3);
    let account4 = account_handler
        .find_by_shielded_address(&account2.shielded_address)
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
    let options = CreateAccountOptions::builder()
        .wallet_password(DEFAULT_WALLET_PASSWORD.to_string())
        .build();
    account_handler.create(&options).await.unwrap();
    assert_eq!(account_handler.count_all().await.unwrap(), 1);
    let account = account_handler.create(&options).await.unwrap();
    assert_eq!(account_handler.count_all().await.unwrap(), 2);
    let filter = SubFilter::equal(AccountColumn::ShieldedAddress, account.shielded_address);
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
        .update_by_public_key(&account.public_key, &update_options)
        .await
        .unwrap();
    assert_eq!(updated_account.name, account.name);
    assert_eq!(updated_account.updated_at, account.updated_at);
    update_options.name = Some(account.name.clone());
    updated_account = account_handler
        .update_by_shielded_address(&account.shielded_address, &update_options)
        .await
        .unwrap();
    assert_eq!(updated_account.name, account.name);
    assert_eq!(updated_account.updated_at, account.updated_at);
    update_options.name = Some(String::from("Awesome Account Name"));
    updated_account = account_handler
        .update_by_shielded_address(&account.shielded_address, &update_options)
        .await
        .unwrap();
    assert_eq!(updated_account.name, "Awesome Account Name");
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
        .update_by_public_key(&account.public_key, &update_options)
        .await;
    assert!(result.is_err());
    result = account_handler
        .update_by_shielded_address(&account.shielded_address, &update_options)
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
    let full_sk: FullSk = decode_hex_with_length(secret_key_str).unwrap();
    let full_pk = full_public_key(&full_sk).unwrap();
    assert_eq!(account.public_key, encode_hex(full_pk));
    assert_eq!(
        account.shielded_address,
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
        .export_secret_key_by_public_key("wrong password", &account.public_key)
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
    let result = account_handler
        .update_encryption(DEFAULT_WALLET_PASSWORD, new_wallet_password)
        .await;
    assert!(result.is_err());
    wallet_handler
        .update_password(DEFAULT_WALLET_PASSWORD, new_wallet_password)
        .await
        .unwrap();
    account_handler
        .update_encryption(DEFAULT_WALLET_PASSWORD, new_wallet_password)
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

#[tokio::test]
async fn test_create_wallet_with_web_mnemonic() {
    let wallet_password = "P@ssw0rd";
    let mnemonic_phrase = "blind race civil nerve pulse awkward cluster squirrel thumb stove cup client";
    let (account_handler, wallet_handler, db) = setup().await;
    let wallet_options = CreateWalletOptions::builder()
        .mnemonic(
            MnemonicOptions::builder()
                .mnemonic_type(MnemonicType::Web as i32)
                .mnemonic_phrase(mnemonic_phrase.to_string())
                .build(),
        )
        .password(String::from(wallet_password))
        .build();
    let wallet = wallet_handler.create(&wallet_options).await.unwrap();

    let private_key1 = "a9da6b0a337dc2073c8eae200293e5807296ea004e999a6c05628cf47a5a0cbe4a3d56192c1f7a732b72c20a2cd6aedd5c4974e51d3dad93b556f7277219c650";
    let public_key1 = "09d7f2d95639bbd3190077f6ce71721bf3ae4d7f482677260273f4fbf4014a1202f3c14d35551fe48f2e2e8ef36001d182fffc69aa675b65452ce0076853294987";
    let shield_address1 = "sP9XRNVcuF3fUUUgFXJPnATqPmz9pc7ea7p1YChCXfCu43AuTJRPWerMfWTpFDHUijS7ACGD8ZUBb96uarTFzd8S";
    let account1 = account_handler
        .create(&CreateAccountOptions::builder().wallet_password(wallet_password).build())
        .await
        .unwrap();
    let full_sk_str1 = decrypt_symmetric(wallet_password, &account1.encrypted_secret_key).unwrap();
    assert_eq!(full_sk_str1, private_key1);
    assert_eq!(account1.public_key, public_key1);
    assert_eq!(account1.shielded_address, shield_address1);

    let private_key2 = "466306666373289cb8691593694b14845368790be0a0609d060e0329ac027da78d08aa9ada9f96cbf6387e966b0d4908cb1f07d563294b2a618c485f4f31211f";
    let public_key2 = "4d98f9de3f7059e6e5937c7376bd63f92120db6f087e0464785e15e72776d008031bf0fd473f46988f231280e61278f7794ce66dade0633d5b93f936d4861abef3";
    let shield_address2 = "7rAYNWNCcmzNjYdHs7ra9LT1X3n6dNmwgve6Dfqd5xUr2EibtCJzMFzR9FVPmDhkuMwnxroFiLUCBPwDUrLSgfMbp";
    let account2 = account_handler
        .create(&CreateAccountOptions::builder().wallet_password(wallet_password).build())
        .await
        .unwrap();
    let full_sk_str2 = decrypt_symmetric(wallet_password, &account2.encrypted_secret_key).unwrap();
    assert_eq!(full_sk_str2, private_key2);
    assert_eq!(account2.public_key, public_key2);
    assert_eq!(account2.shielded_address, shield_address2);

    let mut wallet = db.wallets.find_by_id(&wallet.id).await.unwrap().unwrap();
    assert_eq!(wallet.data.account_nonce, 4);
    wallet.data.account_nonce = 204;
    db.wallets.update(&wallet).await.unwrap();

    let private_key102 = "ebc11118c5cb35aa77ed4bab4aeeef7afc52dd45386420285db05c6cb870b33d1a383a1a07033eacb7f9eee7fe298e6c7f603209f7c92c38b4553ffb9a39ab05";
    let public_key102 = "ffdc2a1aad284a26036fa36be56cf03bc343fe98bd35788871931b325624ec12036d58a9d92cb8e1f1348d053cc9919365ba0ca6b4bb5dc2d8e93246d862fb3b0d";
    let shield_address102 = "PaZNPVT8oDpwU7fdvGfp1Rgsqzd4Qd6VcdWeiyZY998AVnw6tHmdhhYNtN1bdiTHtRoK6UhvkGEp2Zys2fo1CumRW";
    let account102 = account_handler
        .create(&CreateAccountOptions::builder().wallet_password(wallet_password).build())
        .await
        .unwrap();
    let full_sk_str102 = decrypt_symmetric(wallet_password, &account102.encrypted_secret_key).unwrap();
    assert_eq!(full_sk_str102, private_key102);
    assert_eq!(account102.public_key, public_key102);
    assert_eq!(account102.shielded_address, shield_address102);
}
