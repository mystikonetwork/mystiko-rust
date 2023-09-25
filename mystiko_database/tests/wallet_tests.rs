use mystiko_database::document::{Wallet, WalletCollection, WalletColumn};
use mystiko_protos::core::document::v1::Wallet as ProtoWallet;
use mystiko_protos::storage::v1::{ConditionOperator, QueryFilter, SubFilter};
use mystiko_storage::{Collection, Document, SqlStatementFormatter};
use mystiko_storage_sqlite::SqliteStorage;
use std::sync::Arc;

async fn create_wallets() -> WalletCollection<SqlStatementFormatter, SqliteStorage> {
    let storage = SqliteStorage::from_memory().await.unwrap();
    let wallets = WalletCollection::new(Arc::new(Collection::new(SqlStatementFormatter::sqlite(), storage)));
    wallets.migrate().await.unwrap();
    assert!(wallets.collection_exists().await.unwrap());
    wallets
}

#[tokio::test]
async fn test_wallets_crud() {
    let wallets = create_wallets().await;

    // testing insert/insert_batch
    let mut inserted_wallets: Vec<Document<Wallet>> = Vec::new();
    inserted_wallets.push(
        wallets
            .insert(&Wallet {
                encrypted_entropy: String::from("encrypted entropy 01"),
                hashed_password: String::from("hashed password 01"),
                account_nonce: 1,
            })
            .await
            .unwrap(),
    );
    inserted_wallets.extend(
        wallets
            .insert_batch(&[
                Wallet {
                    encrypted_entropy: String::from("encrypted entropy 02"),
                    hashed_password: String::from("hashed password 02"),
                    account_nonce: 2,
                },
                Wallet {
                    encrypted_entropy: String::from("encrypted entropy 03"),
                    hashed_password: String::from("hashed password 03"),
                    account_nonce: 3,
                },
            ])
            .await
            .unwrap(),
    );

    // testing count/count_all
    assert_eq!(wallets.count_all().await.unwrap(), 3);
    assert_eq!(
        wallets
            .count(SubFilter::equal(WalletColumn::AccountNonce, 2u32))
            .await
            .unwrap(),
        1
    );

    // testing find/find_all/find_one/find_by_id
    let mut found_wallets = wallets.find_all().await.unwrap();
    assert_eq!(found_wallets, inserted_wallets);
    found_wallets = wallets
        .find(
            QueryFilter::builder()
                .conditions_operator(ConditionOperator::And)
                .limit(2)
                .offset(1)
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(found_wallets, inserted_wallets[1..]);
    let mut found_wallet = wallets
        .find_one(SubFilter::equal(WalletColumn::HashedPassword, "hashed password 02"))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found_wallet, inserted_wallets[1]);
    found_wallet = wallets.find_by_id(&inserted_wallets[2].id).await.unwrap().unwrap();
    assert_eq!(found_wallet, inserted_wallets[2]);

    // testing update/update_batch
    found_wallet.data.account_nonce = 30;
    let updated_wallet = wallets.update(&found_wallet).await.unwrap();
    assert_eq!(updated_wallet.data, found_wallet.data);
    inserted_wallets[0].data.account_nonce = 10;
    inserted_wallets[1].data.account_nonce = 20;
    inserted_wallets[2].data.account_nonce = 30;
    found_wallets = wallets.update_batch(&inserted_wallets).await.unwrap();
    assert_eq!(found_wallets[0].data, inserted_wallets[0].data);
    assert_eq!(found_wallets[1].data, inserted_wallets[1].data);
    assert_eq!(found_wallets[2].data, inserted_wallets[2].data);

    // testing delete/delete_batch/delete_by_filter/delete_all
    wallets.delete(&inserted_wallets[0]).await.unwrap();
    assert_eq!(wallets.count_all().await.unwrap(), 2);
    wallets.delete_batch(&[inserted_wallets[1].clone()]).await.unwrap();
    assert_eq!(wallets.count_all().await.unwrap(), 1);
    wallets
        .insert(&Wallet {
            encrypted_entropy: String::from("encrypted entropy 01"),
            hashed_password: String::from("hashed password 01"),
            account_nonce: 1,
        })
        .await
        .unwrap();
    assert_eq!(wallets.count_all().await.unwrap(), 2);
    wallets
        .delete_by_filter(SubFilter::equal(WalletColumn::HashedPassword, "hashed password 01"))
        .await
        .unwrap();
    assert_eq!(wallets.count_all().await.unwrap(), 1);
    wallets.delete_all().await.unwrap();
    assert_eq!(wallets.count_all().await.unwrap(), 0);
}

#[tokio::test]
async fn test_wallet_serde() {
    let wallets = create_wallets().await;
    let wallet = wallets
        .insert(&Wallet {
            encrypted_entropy: String::from("encrypted entropy 01"),
            hashed_password: String::from("hashed password 01"),
            account_nonce: 1,
        })
        .await
        .unwrap();
    assert_eq!(
        wallet,
        serde_json::from_str(&serde_json::to_string(&wallet).unwrap()).unwrap()
    )
}

#[test]
fn test_from_proto() {
    let proto = ProtoWallet::builder()
        .id(String::from("123456"))
        .created_at(1234567890u64)
        .updated_at(1234567891u64)
        .encrypted_entropy(String::from("encrypted entropy 01"))
        .hashed_password(String::from("hashed password 01"))
        .account_nonce(1u32)
        .build();
    let wallet = Wallet::document_from_proto(proto);
    assert_eq!(wallet.id, String::from("123456"));
    assert_eq!(wallet.created_at, 1234567890u64);
    assert_eq!(wallet.updated_at, 1234567891u64);
    assert_eq!(wallet.data.encrypted_entropy, String::from("encrypted entropy 01"));
    assert_eq!(wallet.data.hashed_password, String::from("hashed password 01"));
    assert_eq!(wallet.data.account_nonce, 1u32);
}

#[test]
fn test_into_proto() {
    let wallet = Document::new(
        String::from("123456"),
        1234567890u64,
        1234567891u64,
        Wallet {
            encrypted_entropy: String::from("encrypted entropy 01"),
            hashed_password: String::from("hashed password 01"),
            account_nonce: 1u32,
        },
    );
    let proto = Wallet::document_into_proto(wallet);
    assert_eq!(proto.id, String::from("123456"));
    assert_eq!(proto.created_at, 1234567890u64);
    assert_eq!(proto.updated_at, 1234567891u64);
    assert_eq!(proto.encrypted_entropy, String::from("encrypted entropy 01"));
    assert_eq!(proto.hashed_password, String::from("hashed password 01"));
    assert_eq!(proto.account_nonce, 1u32);
}
