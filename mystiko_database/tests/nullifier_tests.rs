use mystiko_database::collection::nullifier::NullifierCollection;
use mystiko_database::document::nullifier::Nullifier;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::Document;
use mystiko_storage::filter::{Condition, QueryFilterBuilder, SubFilter};
use mystiko_storage::formatter::SqlFormatter;
use mystiko_storage_sqlite::{SqliteRawData, SqliteStorage, SqliteStorageBuilder};
use std::sync::Arc;

async fn create_nullifiers() -> NullifierCollection<SqlFormatter, SqliteRawData, SqliteStorage> {
    let storage = SqliteStorageBuilder::new().build().await.unwrap();
    let nullifiers = NullifierCollection::new(Arc::new(Collection::new(SqlFormatter {}, storage)));
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
                serial_number: String::from("serial_number 1"),
                transaction_hash: String::from("transaction_hash 1"),
            })
            .await
            .unwrap(),
    );
    inserted_nullifiers.extend(
        nullifiers
            .insert_batch(&vec![
                Nullifier {
                    chain_id: 2,
                    contract_address: String::from("contract_address 2"),
                    serial_number: String::from("serial_number 2"),
                    transaction_hash: String::from("transaction_hash 2"),
                },
                Nullifier {
                    chain_id: 3,
                    contract_address: String::from("contract_address 3"),
                    serial_number: String::from("serial_number 3"),
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
            .count(
                QueryFilterBuilder::new()
                    .filter(Condition::FILTER(SubFilter::Equal(
                        String::from("chain_id"),
                        2.to_string()
                    )))
                    .build()
            )
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
        .find_one(
            QueryFilterBuilder::new()
                .filter(Condition::FILTER(SubFilter::Equal(
                    String::from("contract_address"),
                    String::from("contract_address 2"),
                )))
                .build(),
        )
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
        .delete_batch(&vec![inserted_nullifiers[1].clone()])
        .await
        .unwrap();
    assert_eq!(nullifiers.count_all().await.unwrap(), 1);
    nullifiers
        .insert(&inserted_nullifiers[0].data)
        .await
        .unwrap();
    assert_eq!(nullifiers.count_all().await.unwrap(), 2);
    nullifiers
        .delete_by_filter(
            QueryFilterBuilder::new()
                .filter(Condition::FILTER(SubFilter::Equal(
                    String::from("serial_number"),
                    String::from("serial_number 1"),
                )))
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(nullifiers.count_all().await.unwrap(), 1);
    nullifiers.delete_all().await.unwrap();
    assert_eq!(nullifiers.count_all().await.unwrap(), 0);
}
