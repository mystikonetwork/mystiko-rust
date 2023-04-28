use mystiko_database::collection::account::AccountCollection;
use mystiko_database::document::account::Account;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::Document;
use mystiko_storage::filter::{Condition, QueryFilterBuilder, SubFilter};
use mystiko_storage::formatter::SqlFormatter;
use mystiko_storage_sqlite::{SqliteRawData, SqliteStorage, SqliteStorageBuilder};
use mystiko_types::AccountStatus;
use std::sync::Arc;

async fn create_accounts() -> AccountCollection<SqlFormatter, SqliteRawData, SqliteStorage> {
    let storage = SqliteStorageBuilder::new().build().await.unwrap();
    let accounts = AccountCollection::new(Arc::new(Collection::new(SqlFormatter {}, storage)));
    accounts.migrate().await.unwrap();
    assert!(accounts.collection_exists().await.unwrap());
    accounts
}

#[tokio::test]
async fn test_accounts_crud() {
    let accounts = create_accounts().await;

    // testing insert/insert_batch
    let mut inserted_accounts: Vec<Document<Account>> = Vec::new();
    inserted_accounts.push(
        accounts
            .insert(&Account {
                name: String::from("account 1"),
                shielded_address: String::from("shielded address 1"),
                public_key: String::from("public key 1"),
                encrypted_secret_key: String::from("encrypted secret key 1"),
                status: AccountStatus::Created,
                scan_size: 1,
                wallet_id: String::from("1"),
            })
            .await
            .unwrap(),
    );
    inserted_accounts.extend(
        accounts
            .insert_batch(&vec![
                Account {
                    name: String::from("account 2"),
                    shielded_address: String::from("shielded address 2"),
                    public_key: String::from("public key 2"),
                    encrypted_secret_key: String::from("encrypted secret key 2"),
                    status: AccountStatus::Scanned,
                    scan_size: 2,
                    wallet_id: String::from("2"),
                },
                Account {
                    name: String::from("account 3"),
                    shielded_address: String::from("shielded address 3"),
                    public_key: String::from("public key 3"),
                    encrypted_secret_key: String::from("encrypted secret key 3"),
                    status: AccountStatus::Scanning,
                    scan_size: 3,
                    wallet_id: String::from("3"),
                },
            ])
            .await
            .unwrap(),
    );

    // testing count/count_all
    assert_eq!(accounts.count_all().await.unwrap(), 3);
    assert_eq!(
        accounts
            .count(
                QueryFilterBuilder::new()
                    .filter(Condition::FILTER(SubFilter::Equal(
                        String::from("scan_size"),
                        2.to_string()
                    )))
                    .build()
            )
            .await
            .unwrap(),
        1
    );

    // testing find/find_all/find_one/find_by_id
    let mut found_accounts = accounts.find_all().await.unwrap();
    assert_eq!(found_accounts, inserted_accounts);
    found_accounts = accounts
        .find(QueryFilterBuilder::new().limit(2).offset(1).build())
        .await
        .unwrap();
    assert_eq!(found_accounts, inserted_accounts[1..]);
    let mut found_account = accounts
        .find_one(
            QueryFilterBuilder::new()
                .filter(Condition::FILTER(SubFilter::Equal(
                    String::from("shielded_address"),
                    String::from("shielded address 2"),
                )))
                .build(),
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found_account, inserted_accounts[1]);
    found_account = accounts.find_by_id(&inserted_accounts[2].id).await.unwrap().unwrap();
    assert_eq!(found_account, inserted_accounts[2]);

    // testing update/update_batch
    found_account.data.scan_size = 30;
    let updated_account = accounts.update(&found_account).await.unwrap();
    assert_eq!(updated_account.data, found_account.data);
    inserted_accounts[0].data.scan_size = 10;
    inserted_accounts[1].data.scan_size = 20;
    inserted_accounts[2].data.scan_size = 30;
    found_accounts = accounts.update_batch(&inserted_accounts).await.unwrap();
    assert_eq!(found_accounts[0].data, inserted_accounts[0].data);
    assert_eq!(found_accounts[1].data, inserted_accounts[1].data);
    assert_eq!(found_accounts[2].data, inserted_accounts[2].data);

    // testing delete/delete_batch/delete_by_filter/delete_all
    accounts.delete(&inserted_accounts[0]).await.unwrap();
    assert_eq!(accounts.count_all().await.unwrap(), 2);
    accounts
        .delete_batch(&vec![inserted_accounts[1].clone()])
        .await
        .unwrap();
    assert_eq!(accounts.count_all().await.unwrap(), 1);
    accounts.insert(&inserted_accounts[0].data).await.unwrap();
    assert_eq!(accounts.count_all().await.unwrap(), 2);
    accounts
        .delete_by_filter(
            QueryFilterBuilder::new()
                .filter(Condition::FILTER(SubFilter::Equal(
                    String::from("public_key"),
                    String::from("public key 1"),
                )))
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(accounts.count_all().await.unwrap(), 1);
    accounts.delete_all().await.unwrap();
    assert_eq!(accounts.count_all().await.unwrap(), 0);
}
