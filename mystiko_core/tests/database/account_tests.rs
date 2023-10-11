use mystiko_core::{Account, AccountCollection, AccountColumn};
use mystiko_protos::core::document::v1::Account as ProtoAccount;
use mystiko_protos::storage::v1::{ConditionOperator, QueryFilter, SubFilter};
use mystiko_storage::{Collection, Document, SqlStatementFormatter};
use mystiko_storage_sqlite::SqliteStorage;
use mystiko_types::AccountStatus;
use std::sync::Arc;

async fn create_accounts() -> AccountCollection<SqlStatementFormatter, SqliteStorage> {
    let storage = SqliteStorage::from_memory().await.unwrap();
    let accounts = AccountCollection::new(Arc::new(Collection::new(SqlStatementFormatter::sqlite(), storage)));
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
            .insert_batch(&[
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
            .count(SubFilter::equal(AccountColumn::ScanSize, 2u32))
            .await
            .unwrap(),
        1
    );

    // testing find/find_all/find_one/find_by_id
    let mut found_accounts = accounts.find_all().await.unwrap();
    assert_eq!(found_accounts, inserted_accounts);

    found_accounts = accounts
        .find(
            QueryFilter::builder()
                .conditions_operator(ConditionOperator::And)
                .limit(2)
                .offset(1)
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(found_accounts, inserted_accounts[1..]);
    let mut found_account = accounts
        .find_one(SubFilter::equal(AccountColumn::ShieldedAddress, "shielded address 2"))
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
    accounts.delete_batch(&[inserted_accounts[1].clone()]).await.unwrap();
    assert_eq!(accounts.count_all().await.unwrap(), 1);
    accounts.insert(&inserted_accounts[0].data).await.unwrap();
    assert_eq!(accounts.count_all().await.unwrap(), 2);
    accounts
        .delete_by_filter(SubFilter::equal(AccountColumn::PublicKey, "public key 1"))
        .await
        .unwrap();
    assert_eq!(accounts.count_all().await.unwrap(), 1);
    accounts.delete_all().await.unwrap();
    assert_eq!(accounts.count_all().await.unwrap(), 0);
}

#[tokio::test]
async fn test_account_serde() {
    let accounts = create_accounts().await;
    let account = accounts
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
        .unwrap();
    assert_eq!(
        account,
        serde_json::from_str(&serde_json::to_string(&account).unwrap()).unwrap()
    );
}

#[test]
fn test_from_proto() {
    let proto = ProtoAccount::builder()
        .id(String::from("123456"))
        .created_at(1234567890u64)
        .updated_at(1234567891u64)
        .name(String::from("account 1"))
        .shielded_address(String::from("shielded address 1"))
        .public_key(String::from("public key 1"))
        .encrypted_secret_key(String::from("encrypted secret key 1"))
        .scan_size(1u32)
        .wallet_id(String::from("1"))
        .status(1)
        .build();
    let account = Account::document_from_proto(proto);
    assert_eq!(account.id, String::from("123456"));
    assert_eq!(account.created_at, 1234567890u64);
    assert_eq!(account.updated_at, 1234567891u64);
    assert_eq!(account.data.name, String::from("account 1"));
    assert_eq!(account.data.shielded_address, String::from("shielded address 1"));
    assert_eq!(account.data.public_key, String::from("public key 1"));
    assert_eq!(
        account.data.encrypted_secret_key,
        String::from("encrypted secret key 1")
    );
    assert_eq!(account.data.scan_size, 1u32);
    assert_eq!(account.data.wallet_id, String::from("1"));
    assert_eq!(account.data.status, AccountStatus::Created);
}

#[test]
fn test_into_proto() {
    let account = Document::new(
        String::from("123456"),
        1234567890u64,
        1234567891u64,
        Account {
            name: String::from("account 1"),
            shielded_address: String::from("shielded address 1"),
            public_key: String::from("public key 1"),
            encrypted_secret_key: String::from("encrypted secret key 1"),
            status: AccountStatus::Created,
            scan_size: 1,
            wallet_id: String::from("1"),
        },
    );
    let proto = Account::document_into_proto(account);
    assert_eq!(proto.id, String::from("123456"));
    assert_eq!(proto.created_at, 1234567890u64);
    assert_eq!(proto.updated_at, 1234567891u64);
    assert_eq!(proto.name, String::from("account 1"));
    assert_eq!(proto.shielded_address, String::from("shielded address 1"));
    assert_eq!(proto.public_key, String::from("public key 1"));
    assert_eq!(proto.encrypted_secret_key, String::from("encrypted secret key 1"));
    assert_eq!(proto.scan_size, 1u32);
    assert_eq!(proto.wallet_id, String::from("1"));
    assert_eq!(proto.status, 1);
}
