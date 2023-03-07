use futures::lock::Mutex;
use mystiko_database::collection::nullifier::NullifierCollection;
use mystiko_database::document::nullifier::Nullifier;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::Document;
use mystiko_storage::filter::{Condition, QueryFilterBuilder, SubFilter};
use mystiko_storage::formatter::SqlFormatter;
use mystiko_storage_sqlite::{SqliteRawData, SqliteStorage, SqliteStorageBuilder};
use std::sync::Arc;
use tokio_test::block_on;

async fn create_nullifiers() -> NullifierCollection<SqlFormatter, SqliteRawData, SqliteStorage> {
    let storage = SqliteStorageBuilder::new().build().await.unwrap();
    let nullifiers = NullifierCollection::new(Arc::new(Mutex::new(Collection::new(
        SqlFormatter {},
        storage,
    ))));
    nullifiers.migrate().await.unwrap();
    assert!(nullifiers.collection_exists().await.unwrap());
    nullifiers
}

#[test]
fn test_nullifiers_crud() {
    let nullifiers = block_on(create_nullifiers());

    // testing insert/insert_batch
    let mut inserted_nullifiers: Vec<Document<Nullifier>> = Vec::new();
    inserted_nullifiers.push(
        block_on(nullifiers.insert(&Nullifier {
            chain_id: 1,
            contract_address: String::from("contract_address 1"),
            serial_number: String::from("serial_number 1"),
            transaction_hash: String::from("transaction_hash 1"),
        }))
        .unwrap(),
    );
    inserted_nullifiers.extend(
        block_on(nullifiers.insert_batch(&vec![
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
        ]))
        .unwrap(),
    );

    // testing count/count_all
    assert_eq!(block_on(nullifiers.count_all()).unwrap(), 3);
    assert_eq!(
        block_on(
            nullifiers.count(
                QueryFilterBuilder::new()
                    .filter(Condition::FILTER(SubFilter::Equal(
                        String::from("chain_id"),
                        2.to_string()
                    )))
                    .build()
            )
        )
        .unwrap(),
        1
    );

    // testing find/find_all/find_one/find_by_id
    let mut found_nullifiers = block_on(nullifiers.find_all()).unwrap();
    assert_eq!(found_nullifiers, inserted_nullifiers);
    found_nullifiers =
        block_on(nullifiers.find(QueryFilterBuilder::new().limit(2).offset(1).build())).unwrap();
    assert_eq!(found_nullifiers, inserted_nullifiers[1..]);
    let mut found_nullifier = block_on(
        nullifiers.find_one(
            QueryFilterBuilder::new()
                .filter(Condition::FILTER(SubFilter::Equal(
                    String::from("contract_address"),
                    String::from("contract_address 2"),
                )))
                .build(),
        ),
    )
    .unwrap()
    .unwrap();
    assert_eq!(found_nullifier, inserted_nullifiers[1]);
    found_nullifier = block_on(nullifiers.find_by_id(&inserted_nullifiers[2].id))
        .unwrap()
        .unwrap();
    assert_eq!(found_nullifier, inserted_nullifiers[2]);

    // testing update/update_batch
    found_nullifier.data.chain_id = 10;
    let updated_nullifier = block_on(nullifiers.update(&found_nullifier)).unwrap();
    assert_eq!(updated_nullifier.data, found_nullifier.data);
    inserted_nullifiers[0].data.chain_id = 10;
    inserted_nullifiers[1].data.chain_id = 20;
    inserted_nullifiers[2].data.chain_id = 30;
    found_nullifiers = block_on(nullifiers.update_batch(&inserted_nullifiers)).unwrap();
    assert_eq!(found_nullifiers[0].data, inserted_nullifiers[0].data);
    assert_eq!(found_nullifiers[1].data, inserted_nullifiers[1].data);
    assert_eq!(found_nullifiers[2].data, inserted_nullifiers[2].data);

    // testing delete/delete_batch/delete_by_filter/delete_all
    block_on(nullifiers.delete(&inserted_nullifiers[0])).unwrap();
    assert_eq!(block_on(nullifiers.count_all()).unwrap(), 2);
    block_on(nullifiers.delete_batch(&vec![inserted_nullifiers[1].clone()])).unwrap();
    assert_eq!(block_on(nullifiers.count_all()).unwrap(), 1);
    block_on(nullifiers.insert(&inserted_nullifiers[0].data)).unwrap();
    assert_eq!(block_on(nullifiers.count_all()).unwrap(), 2);
    block_on(
        nullifiers.delete_by_filter(
            QueryFilterBuilder::new()
                .filter(Condition::FILTER(SubFilter::Equal(
                    String::from("serial_number"),
                    String::from("serial_number 1"),
                )))
                .build(),
        ),
    )
    .unwrap();
    assert_eq!(block_on(nullifiers.count_all()).unwrap(), 1);
    block_on(nullifiers.delete_all()).unwrap();
    assert_eq!(block_on(nullifiers.count_all()).unwrap(), 0);
}
