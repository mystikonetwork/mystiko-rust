use mystiko_database::document::nullifier::{Nullifier, NullifierCollection, NullifierColumn};
use mystiko_storage::collection::Collection;
use mystiko_storage::document::Document;
use mystiko_storage::filter::{QueryFilterBuilder, SubFilter};
use mystiko_storage::formatter::sql::SqlStatementFormatter;
use mystiko_storage_sqlite::{SqliteStorage, SqliteStorageBuilder};
use num_bigint::BigInt;
use std::sync::Arc;

async fn create_nullifiers() -> NullifierCollection<SqlStatementFormatter, SqliteStorage> {
    let storage = SqliteStorageBuilder::new().build().await.unwrap();
    let nullifiers = NullifierCollection::new(Arc::new(Collection::new(SqlStatementFormatter::default(), storage)));
    nullifiers.migrate().await.unwrap();
    assert!(nullifiers.collection_exists().await.unwrap());
    nullifiers
}

#[tokio::test]
async fn test_nullifiers_crud() {
    let nullifiers = create_nullifiers().await;

    // testing insert/insert_batch
    let mut inserted_nullifiers: Vec<Document<Nullifier>> = Vec::new();
    inserted_nullifiers.push(
        nullifiers
            .insert(&Nullifier {
                chain_id: 1,
                contract_address: String::from("contract_address 1"),
                nullifier: BigInt::from(1),
                transaction_hash: String::from("transaction_hash 1"),
            })
            .await
            .unwrap(),
    );
    inserted_nullifiers.extend(
        nullifiers
            .insert_batch(&[
                Nullifier {
                    chain_id: 2,
                    contract_address: String::from("contract_address 2"),
                    nullifier: BigInt::from(2),
                    transaction_hash: String::from("transaction_hash 2"),
                },
                Nullifier {
                    chain_id: 3,
                    contract_address: String::from("contract_address 3"),
                    nullifier: BigInt::from(3),
                    transaction_hash: String::from("transaction_hash 3"),
                },
            ])
            .await
            .unwrap(),
    );

    // testing count/count_all
    assert_eq!(nullifiers.count_all().await.unwrap(), 3);
    assert_eq!(
        nullifiers
            .count(SubFilter::equal(NullifierColumn::ChainId, 2))
            .await
            .unwrap(),
        1
    );

    // testing find/find_all/find_one/find_by_id
    let mut found_nullifiers = nullifiers.find_all().await.unwrap();
    assert_eq!(found_nullifiers, inserted_nullifiers);
    found_nullifiers = nullifiers
        .find(QueryFilterBuilder::new().limit(2).offset(1).build())
        .await
        .unwrap();
    assert_eq!(found_nullifiers, inserted_nullifiers[1..]);
    let mut found_nullifier = nullifiers
        .find_one(SubFilter::equal(NullifierColumn::ContractAddress, "contract_address 2"))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found_nullifier, inserted_nullifiers[1]);
    found_nullifier = nullifiers
        .find_by_id(&inserted_nullifiers[2].id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found_nullifier, inserted_nullifiers[2]);

    // testing update/update_batch
    found_nullifier.data.chain_id = 10;
    let updated_nullifier = nullifiers.update(&found_nullifier).await.unwrap();
    assert_eq!(updated_nullifier.data, found_nullifier.data);
    inserted_nullifiers[0].data.chain_id = 10;
    inserted_nullifiers[1].data.chain_id = 20;
    inserted_nullifiers[2].data.chain_id = 30;
    found_nullifiers = nullifiers.update_batch(&inserted_nullifiers).await.unwrap();
    assert_eq!(found_nullifiers[0].data, inserted_nullifiers[0].data);
    assert_eq!(found_nullifiers[1].data, inserted_nullifiers[1].data);
    assert_eq!(found_nullifiers[2].data, inserted_nullifiers[2].data);

    // testing delete/delete_batch/delete_by_filter/delete_all
    nullifiers.delete(&inserted_nullifiers[0]).await.unwrap();
    assert_eq!(nullifiers.count_all().await.unwrap(), 2);
    nullifiers
        .delete_batch(&[inserted_nullifiers[1].clone()])
        .await
        .unwrap();
    assert_eq!(nullifiers.count_all().await.unwrap(), 1);
    nullifiers.insert(&inserted_nullifiers[0].data).await.unwrap();
    assert_eq!(nullifiers.count_all().await.unwrap(), 2);
    nullifiers
        .delete_by_filter(SubFilter::equal(NullifierColumn::Nullifier, 1))
        .await
        .unwrap();
    assert_eq!(nullifiers.count_all().await.unwrap(), 1);
    nullifiers.delete_all().await.unwrap();
    assert_eq!(nullifiers.count_all().await.unwrap(), 0);
}

#[tokio::test]
async fn test_nullifier_serde() {
    let nullifiers = create_nullifiers().await;
    let nullifier = nullifiers
        .insert(&Nullifier {
            chain_id: 1,
            contract_address: String::from("contract_address 1"),
            nullifier: BigInt::from(1),
            transaction_hash: String::from("transaction_hash 1"),
        })
        .await
        .unwrap();
    assert_eq!(
        nullifier,
        serde_json::from_str(&serde_json::to_string(&nullifier).unwrap()).unwrap()
    )
}
