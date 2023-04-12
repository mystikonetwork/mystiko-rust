use mystiko_database::collection::chain::ChainCollection;
use mystiko_database::document::chain::Chain;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::Document;
use mystiko_storage::filter::{Condition, QueryFilterBuilder, SubFilter};
use mystiko_storage::formatter::SqlFormatter;
use mystiko_storage_sqlite::{SqliteRawData, SqliteStorage, SqliteStorageBuilder};
use std::sync::Arc;
use tokio::sync::Mutex;

async fn create_chains() -> ChainCollection<SqlFormatter, SqliteRawData, SqliteStorage> {
    let storage = SqliteStorageBuilder::new().build().await.unwrap();
    let chains = ChainCollection::new(Arc::new(Mutex::new(Collection::new(
        SqlFormatter {},
        storage,
    ))));
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
                name_override: 0,
                providers: vec![
                String::from(
                    "{\"url\": \"wss://goerli.infura.io/ws/v3/9aa4d95b3bc440fa88ea12eaa4456161\"}",
                ),
                String::from(
                    "{\"url\": \"https://goerli.infura.io/v3/9aa4d95b3bc440fa88ea12eaa4456161\"}",
                ),
            ],
                provider_override: 1,
                event_filter_size: 2000,
                synced_block_number: 8497095,
            })
            .await
            .unwrap(),
    );
    assert_eq!(chains.count_all().await.unwrap(), 1);
    // testing insert_batch
    inserted_chains.extend(
        chains.insert_batch(&vec![
            Chain {
                chain_id:97,
                name:String::from("BSC Testnet"),
                name_override:0,
                providers:vec![String::from("{\"url\": \"wss://ws-nd-302-890-317.p2pify.com/430d98aabb1fe49ec6517602e1e40f01\"}"),String::from("{\"url\": \"https://nd-302-890-317.p2pify.com/430d98aabb1fe49ec6517602e1e40f01\"}")],
                provider_override:0,
                event_filter_size:100000,
                synced_block_number:27265360,
            },
            Chain {
                chain_id:80001,
                name:String::from("Polygon"),
                name_override:1,
                providers:vec![String::from("{\"url\": \"https://matic-mumbai.chainstacklabs.com\"}")],
                provider_override:1,
                event_filter_size:10000,
                synced_block_number:32076637,
            },
        ]).await
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
                        1.to_string()
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
    found_chain.data.name_override = 1;
    let updated_chain = chains.update(&found_chain).await.unwrap();
    assert_eq!(updated_chain.data, found_chain.data);
    // testing update_batch
    inserted_chains[0].data.event_filter_size = 10000;
    inserted_chains[1].data.event_filter_size = 20000;
    inserted_chains[2].data.event_filter_size = 30000;
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
