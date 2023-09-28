use mystiko_config::MystikoConfig;
use mystiko_database::document::{Nullifier, NullifierCollection, NullifierColumn};
use mystiko_dataloader::handler::document::DatabaseNullifier;
use mystiko_protos::storage::v1::{ConditionOperator, QueryFilter, SubFilter};
use mystiko_storage::{Collection, Document, SqlStatementFormatter};
use mystiko_storage_sqlite::SqliteStorage;
use mystiko_utils::convert::biguint_to_bytes;
use mystiko_utils::hex::decode_hex;
use num_bigint::BigUint;
use std::sync::Arc;

async fn create_nullifiers() -> NullifierCollection<SqlStatementFormatter, SqliteStorage> {
    let storage = SqliteStorage::from_memory().await.unwrap();
    let nullifiers = NullifierCollection::new(Arc::new(Collection::new(SqlStatementFormatter::sqlite(), storage)));
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
                block_number: 10000000u64,
                nullifier: BigUint::from(1u32),
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
                    block_number: 10000000u64,
                    nullifier: BigUint::from(2u32),
                    transaction_hash: String::from("transaction_hash 2"),
                },
                Nullifier {
                    chain_id: 3,
                    contract_address: String::from("contract_address 3"),
                    block_number: 10000001u64,
                    nullifier: BigUint::from(3u32),
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
            .count(SubFilter::equal(NullifierColumn::ChainId, 2u64))
            .await
            .unwrap(),
        1
    );

    // testing find/find_all/find_one/find_by_id
    let mut found_nullifiers = nullifiers.find_all().await.unwrap();
    assert_eq!(found_nullifiers, inserted_nullifiers);
    found_nullifiers = nullifiers
        .find(
            QueryFilter::builder()
                .conditions_operator(ConditionOperator::And)
                .limit(2)
                .offset(1)
                .build(),
        )
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
        .delete_by_filter(SubFilter::equal(
            NullifierColumn::Nullifier,
            num_bigint::BigUint::from(1u32),
        ))
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
            block_number: 10000000u64,
            nullifier: BigUint::from(1u32),
            transaction_hash: String::from("transaction_hash 1"),
        })
        .await
        .unwrap();
    assert_eq!(
        nullifier,
        serde_json::from_str(&serde_json::to_string(&nullifier).unwrap()).unwrap()
    )
}

#[tokio::test]
async fn test_loader_database_nullifier() {
    assert_eq!(Nullifier::column_chain_id(), "chain_id");
    assert_eq!(Nullifier::column_contract_address(), "contract_address");
    assert_eq!(Nullifier::column_nullifier(), "nullifier");
    assert_eq!(Nullifier::column_block_number(), "block_number");
    assert_eq!(Nullifier::column_transaction_hash(), "transaction_hash");
    let mut nullifier = Nullifier {
        chain_id: 5,
        contract_address: String::from("0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"),
        block_number: 10000000u64,
        nullifier: BigUint::from(123456789u32),
        transaction_hash: String::from("0x81d3510c46dfe7a1fc282eb54034b848a3d83f440c551c19e4d513801be00130"),
    };
    assert_eq!(nullifier.get_chain_id(), 5);
    assert_eq!(
        nullifier.get_contract_address(),
        "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"
    );
    assert_eq!(nullifier.get_block_number(), 10000000u64);
    assert_eq!(nullifier.get_nullifier(), &BigUint::from(123456789u32));
    assert_eq!(
        nullifier.get_transaction_hash(),
        "0x81d3510c46dfe7a1fc282eb54034b848a3d83f440c551c19e4d513801be00130"
    );
    let proto = nullifier.clone().into_proto().unwrap();
    assert_eq!(proto.nullifier, biguint_to_bytes(&BigUint::from(123456789u32)));
    assert_eq!(proto.block_number, 10000000u64);
    assert_eq!(
        proto.transaction_hash,
        decode_hex("0x81d3510c46dfe7a1fc282eb54034b848a3d83f440c551c19e4d513801be00130").unwrap()
    );

    let config = Arc::new(
        MystikoConfig::from_json_file("tests/files/config/mystiko.json")
            .await
            .unwrap(),
    );
    let converted_nullifier = Nullifier::from_proto(
        config.clone(),
        5,
        "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d",
        proto.clone(),
    )
    .unwrap();
    assert_eq!(converted_nullifier, nullifier);

    nullifier.block_number = 10000001u64;
    nullifier.nullifier = BigUint::from(123456799u32);
    nullifier.transaction_hash = "0x81d3510c46dfe7a1fc282eb54034b848a3d83f440c551c19e4d513801be00131".to_string();
    nullifier.update_by_proto(config.clone(), &proto).unwrap();
    assert_eq!(converted_nullifier, nullifier);
}
