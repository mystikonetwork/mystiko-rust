use mystiko_database::collection::chain::ChainCollection;
use mystiko_database::document::chain::{Chain, Provider};
use mystiko_storage::collection::Collection;
use mystiko_storage::document::Document;
use mystiko_storage::filter::{Condition, QueryFilterBuilder, SubFilter};
use mystiko_storage::formatter::SqlFormatter;
use mystiko_storage_sqlite::{SqliteRawData, SqliteStorage, SqliteStorageBuilder};
use std::sync::Arc;

async fn create_chains() -> ChainCollection<SqlFormatter, SqliteRawData, SqliteStorage> {
    let storage = SqliteStorageBuilder::new().build().await.unwrap();
    let chains = ChainCollection::new(Arc::new(Collection::new(SqlFormatter {}, storage)));
    chains.migrate().await.unwrap();
    assert!(chains.collection_exists().await.unwrap());
    chains
}

#[tokio::test]
async fn test_chains_crud() {
    let chains = create_chains().await;

    // testing insert
    let mut inserted_chains: Vec<Document<Chain>> = Vec::new();
    inserted_chains.push(
        chains
            .insert(&Chain {
                chain_id: 5,
                name: String::from("Ethereum Goerli"),
                name_override: false,
                providers: vec![
                    Provider {
                        url: String::from("http://localhost:8545"),
                        timeout_ms: 10000,
                        max_try_count: 3,
                        quorum_weight: 2,
                    },
                    Provider {
                        url: String::from("http://localhost:8546"),
                        timeout_ms: 20000,
                        max_try_count: 4,
                        quorum_weight: 3,
                    },
                ],
                provider_override: true,
                synced_block_number: 8497095,
            })
            .await
            .unwrap(),
    );
    assert_eq!(chains.count_all().await.unwrap(), 1);
    // testing insert_batch
    inserted_chains.extend(
        chains
            .insert_batch(&vec![
                Chain {
                    chain_id: 97,
                    name: String::from("BSC Testnet"),
                    name_override: false,
                    providers: vec![
                        Provider {
                            url: String::from("http://localhost:8547"),
                            timeout_ms: 30000,
                            max_try_count: 5,
                            quorum_weight: 4,
                        },
                        Provider {
                            url: String::from("http://localhost:8548"),
                            timeout_ms: 40000,
                            max_try_count: 6,
                            quorum_weight: 5,
                        },
                    ],
                    provider_override: false,
                    synced_block_number: 27265360,
                },
                Chain {
                    chain_id: 80001,
                    name: String::from("Polygon"),
                    name_override: true,
                    providers: vec![Provider {
                        url: String::from("http://localhost:8549"),
                        timeout_ms: 50000,
                        max_try_count: 7,
                        quorum_weight: 6,
                    }],
                    provider_override: true,
                    synced_block_number: 32076637,
                },
            ])
            .await
            .unwrap(),
    );
    assert_eq!(chains.count_all().await.unwrap(), 3);

    // testing count
    assert_eq!(
        chains
            .count(
                QueryFilterBuilder::new()
                    .filter(Condition::FILTER(SubFilter::Equal(
                        String::from("name_override"),
                        1.to_string(),
                    )))
                    .build()
            )
            .await
            .unwrap(),
        1
    );

    // testing find_all
    let mut found_chains = chains.find_all().await.unwrap();
    assert_eq!(found_chains, inserted_chains);
    // testing find
    found_chains = chains
        .find(QueryFilterBuilder::new().limit(2).offset(1).build())
        .await
        .unwrap();
    assert_eq!(found_chains, inserted_chains[1..]);
    // testing find_one
    let mut found_chain = chains
        .find_one(
            QueryFilterBuilder::new()
                .filter(Condition::FILTER(SubFilter::Equal(
                    String::from("name"),
                    String::from("Polygon"),
                )))
                .build(),
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found_chain, inserted_chains[2]);
    // testing find_by_id
    found_chain = chains
        .find_by_id(&inserted_chains[1].id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found_chain, inserted_chains[1]);

    // testing update
    found_chain.data.name = String::from("BSC");
    found_chain.data.name_override = true;
    let updated_chain = chains.update(&found_chain).await.unwrap();
    assert_eq!(updated_chain.data, found_chain.data);
    // testing update_batch
    inserted_chains[0].data.synced_block_number = 10000;
    inserted_chains[1].data.synced_block_number = 20000;
    inserted_chains[2].data.synced_block_number = 30000;
    found_chains = chains.update_batch(&inserted_chains).await.unwrap();
    assert_eq!(found_chains[0].data, inserted_chains[0].data);
    assert_eq!(found_chains[1].data, inserted_chains[1].data);
    assert_eq!(found_chains[2].data, inserted_chains[2].data);

    // testing delete
    chains.delete(&inserted_chains[0]).await.unwrap();
    assert_eq!(chains.count_all().await.unwrap(), 2);
    // testing delete_batch
    chains
        .delete_batch(&vec![inserted_chains[1].clone()])
        .await
        .unwrap();
    assert_eq!(chains.count_all().await.unwrap(), 1);
    // testing delete_by_filter
    chains.insert(&inserted_chains[0].data).await.unwrap();
    assert_eq!(chains.count_all().await.unwrap(), 2);
    chains
        .delete_by_filter(
            QueryFilterBuilder::new()
                .filter(Condition::FILTER(SubFilter::Equal(
                    String::from("name"),
                    String::from("Ethereum Goerli"),
                )))
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(chains.count_all().await.unwrap(), 1);
    // testing delete_all
    chains.delete_all().await.unwrap();
    assert_eq!(chains.count_all().await.unwrap(), 0);
}
