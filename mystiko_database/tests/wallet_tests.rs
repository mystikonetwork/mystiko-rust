use futures::lock::Mutex;
use mystiko_database::collection::wallet::WalletCollection;
use mystiko_database::document::wallet::Wallet;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::Document;
use mystiko_storage::filter::{Condition, QueryFilterBuilder, SubFilter};
use mystiko_storage::formatter::SqlFormatter;
use mystiko_storage_sqlite::{SqliteStorage, SqliteStorageBuilder};
use std::sync::Arc;
use tokio_test::block_on;

async fn create_wallets() -> WalletCollection<SqlFormatter, SqliteStorage> {
    let storage = SqliteStorageBuilder::new().build().await.unwrap();
    let wallets = WalletCollection::new(Arc::new(Mutex::new(Collection::new(
        SqlFormatter {},
        storage,
    ))));
    wallets.migrate().await.unwrap();
    assert!(wallets.collection_exists().await.unwrap());
    wallets
}

#[test]
fn test_wallets_crud() {
    let wallets = block_on(create_wallets());

    // testing insert/insert_batch
    let mut inserted_wallets: Vec<Document<Wallet>> = Vec::new();
    inserted_wallets.push(
        block_on(wallets.insert(&Wallet {
            encrypted_master_seed: String::from("encrypted master seed 01"),
            hashed_password: String::from("hashed password 01"),
            account_nonce: 1,
        }))
        .unwrap(),
    );
    inserted_wallets.extend(
        block_on(wallets.insert_batch(&vec![
            Wallet {
                encrypted_master_seed: String::from("encrypted master seed 02"),
                hashed_password: String::from("hashed password 02"),
                account_nonce: 2,
            },
            Wallet {
                encrypted_master_seed: String::from("encrypted master seed 03"),
                hashed_password: String::from("hashed password 03"),
                account_nonce: 3,
            },
        ]))
        .unwrap(),
    );

    // testing find/find_all/find_one/find_by_id
    let mut found_wallets = block_on(wallets.find_all()).unwrap();
    assert_eq!(found_wallets, inserted_wallets);
    found_wallets =
        block_on(wallets.find(QueryFilterBuilder::new().limit(2).offset(1).build())).unwrap();
    assert_eq!(found_wallets, inserted_wallets[1..]);
    let mut found_wallet = block_on(
        wallets.find_one(
            QueryFilterBuilder::new()
                .filter(Condition::FILTER(SubFilter::Equal(
                    String::from("hashed_password"),
                    String::from("hashed password 02"),
                )))
                .build(),
        ),
    )
    .unwrap().unwrap();
    assert_eq!(found_wallet, inserted_wallets[1]);
    found_wallet = block_on(wallets.find_by_id(&inserted_wallets[2].id)).unwrap().unwrap();
    assert_eq!(found_wallet, inserted_wallets[2]);

    // testing update/update_batch
    found_wallet.data.account_nonce = 30;
    let updated_wallet = block_on(wallets.update(&found_wallet)).unwrap();
    assert_eq!(updated_wallet.data, found_wallet.data);
    inserted_wallets[0].data.account_nonce = 10;
    inserted_wallets[1].data.account_nonce = 20;
    inserted_wallets[2].data.account_nonce = 30;
    found_wallets = block_on(wallets.update_batch(&inserted_wallets)).unwrap();
    assert_eq!(found_wallets[0].data, inserted_wallets[0].data);
    assert_eq!(found_wallets[1].data, inserted_wallets[1].data);
    assert_eq!(found_wallets[2].data, inserted_wallets[2].data);

    // testing delete/delete_batch
    block_on(wallets.delete(&inserted_wallets[0])).unwrap();
    found_wallets = block_on(wallets.find_all()).unwrap();
    assert_eq!(found_wallets.len(), 2);
    inserted_wallets.remove(0);
    block_on(wallets.delete_batch(&inserted_wallets)).unwrap();
    found_wallets = block_on(wallets.find_all()).unwrap();
    assert!(found_wallets.is_empty());
}
