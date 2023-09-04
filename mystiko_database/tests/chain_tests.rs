use mystiko_database::document::chain::{Chain, ChainCollection, ChainColumn, Provider};
use mystiko_protos::core::document::v1::Chain as ProtoChain;
use mystiko_protos::core::document::v1::Provider as ProtoProvider;
use mystiko_protos::storage::v1::{ConditionOperator, QueryFilter, SubFilter};
use mystiko_storage::collection::Collection;
use mystiko_storage::document::Document;
use mystiko_storage::formatter::sql::SqlStatementFormatter;
use mystiko_storage_sqlite::{SqliteStorage, SqliteStorageBuilder};
use std::sync::Arc;

async fn create_chains() -> ChainCollection<SqlStatementFormatter, SqliteStorage> {
    let storage = SqliteStorageBuilder::new().build().await.unwrap();
    let chains = ChainCollection::new(Arc::new(Collection::new(SqlStatementFormatter::sqlite(), storage)));
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
            })
            .await
            .unwrap(),
    );
    assert_eq!(chains.count_all().await.unwrap(), 1);
    // testing insert_batch
    inserted_chains.extend(
        chains
            .insert_batch(&[
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
                },
            ])
            .await
            .unwrap(),
    );
    assert_eq!(chains.count_all().await.unwrap(), 3);

    // testing count
    assert_eq!(
        chains
            .count(SubFilter::equal(ChainColumn::NameOverride, true))
            .await
            .unwrap(),
        1
    );

    // testing find_all
    let mut found_chains = chains.find_all().await.unwrap();
    assert_eq!(found_chains, inserted_chains);
    // testing find
    found_chains = chains
        .find(
            QueryFilter::builder()
                .conditions_operator(ConditionOperator::And)
                .limit(2)
                .offset(1)
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(found_chains, inserted_chains[1..]);
    // testing find_one
    let mut found_chain = chains
        .find_one(SubFilter::equal(ChainColumn::Name, "Polygon"))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found_chain, inserted_chains[2]);
    // testing find_by_id
    found_chain = chains.find_by_id(&inserted_chains[1].id).await.unwrap().unwrap();
    assert_eq!(found_chain, inserted_chains[1]);

    // testing update
    found_chain.data.name = String::from("BSC");
    found_chain.data.name_override = true;
    let updated_chain = chains.update(&found_chain).await.unwrap();
    assert_eq!(updated_chain.data, found_chain.data);
    // testing update_batch
    inserted_chains[0].data.provider_override = true;
    inserted_chains[1].data.provider_override = true;
    inserted_chains[2].data.provider_override = true;
    found_chains = chains.update_batch(&inserted_chains).await.unwrap();
    assert_eq!(found_chains[0].data, inserted_chains[0].data);
    assert_eq!(found_chains[1].data, inserted_chains[1].data);
    assert_eq!(found_chains[2].data, inserted_chains[2].data);

    // testing delete
    chains.delete(&inserted_chains[0]).await.unwrap();
    assert_eq!(chains.count_all().await.unwrap(), 2);
    // testing delete_batch
    chains.delete_batch(&[inserted_chains[1].clone()]).await.unwrap();
    assert_eq!(chains.count_all().await.unwrap(), 1);
    // testing delete_by_filter
    chains.insert(&inserted_chains[0].data).await.unwrap();
    assert_eq!(chains.count_all().await.unwrap(), 2);
    chains
        .delete_by_filter(SubFilter::equal(ChainColumn::Name, "Ethereum Goerli"))
        .await
        .unwrap();
    assert_eq!(chains.count_all().await.unwrap(), 1);
    // testing delete_all
    chains.delete_all().await.unwrap();
    assert_eq!(chains.count_all().await.unwrap(), 0);
}

#[tokio::test]
async fn test_chain_serde() {
    let chains = create_chains().await;
    let chain = chains
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
        })
        .await
        .unwrap();
    assert_eq!(
        chain,
        serde_json::from_str(&serde_json::to_string(&chain).unwrap()).unwrap()
    );
}

#[test]
fn test_provider_from_proto() {
    let proto = ProtoProvider::builder()
        .url(String::from("http://localhost:8545"))
        .timeout_ms(10000u32)
        .max_try_count(3u32)
        .quorum_weight(2u32)
        .build();
    let provider: Provider = proto.into();
    assert_eq!(provider.url, String::from("http://localhost:8545"));
    assert_eq!(provider.timeout_ms, 10000u32);
    assert_eq!(provider.max_try_count, 3u32);
    assert_eq!(provider.quorum_weight, 2u32);
}

#[test]
fn test_provider_into_proto() {
    let provider = Provider {
        url: String::from("http://localhost:8545"),
        timeout_ms: 10000u32,
        max_try_count: 3u32,
        quorum_weight: 2u32,
    };
    let proto: ProtoProvider = provider.into();
    assert_eq!(proto.url, String::from("http://localhost:8545"));
    assert_eq!(proto.timeout_ms, 10000u32);
    assert_eq!(proto.max_try_count, 3u32);
    assert_eq!(proto.quorum_weight, 2u32);
}

#[test]
fn test_chain_from_proto() {
    let proto = ProtoChain::builder()
        .id(String::from("123456"))
        .created_at(1234567890u64)
        .updated_at(1234567891u64)
        .chain_id(5u64)
        .name(String::from("Ethereum Goerli"))
        .name_override(false)
        .providers(vec![
            ProtoProvider::builder()
                .url(String::from("http://localhost:8545"))
                .timeout_ms(10000u32)
                .max_try_count(3u32)
                .quorum_weight(2u32)
                .build(),
            ProtoProvider::builder()
                .url(String::from("http://localhost:8546"))
                .timeout_ms(20000u32)
                .max_try_count(4u32)
                .quorum_weight(3u32)
                .build(),
        ])
        .provider_override(true)
        .build();
    let chain = Chain::from_proto(proto);
    assert_eq!(chain.id, String::from("123456"));
    assert_eq!(chain.created_at, 1234567890u64);
    assert_eq!(chain.updated_at, 1234567891u64);
    assert_eq!(chain.data.chain_id, 5u64);
    assert_eq!(chain.data.name, String::from("Ethereum Goerli"));
    assert!(!chain.data.name_override);
    assert_eq!(chain.data.providers.len(), 2);
    assert_eq!(chain.data.providers[0].url, String::from("http://localhost:8545"));
    assert_eq!(chain.data.providers[0].timeout_ms, 10000u32);
    assert_eq!(chain.data.providers[0].max_try_count, 3u32);
    assert_eq!(chain.data.providers[0].quorum_weight, 2u32);
    assert_eq!(chain.data.providers[1].url, String::from("http://localhost:8546"));
    assert_eq!(chain.data.providers[1].timeout_ms, 20000u32);
    assert_eq!(chain.data.providers[1].max_try_count, 4u32);
    assert_eq!(chain.data.providers[1].quorum_weight, 3u32);
    assert!(chain.data.provider_override);
}

#[test]
fn test_chain_into_proto() {
    let chain = Document::new(
        String::from("123456"),
        1234567890u64,
        1234567891u64,
        Chain {
            chain_id: 5u64,
            name: String::from("Ethereum Goerli"),
            name_override: false,
            providers: vec![
                Provider {
                    url: String::from("http://localhost:8545"),
                    timeout_ms: 10000u32,
                    max_try_count: 3u32,
                    quorum_weight: 2u32,
                },
                Provider {
                    url: String::from("http://localhost:8546"),
                    timeout_ms: 20000u32,
                    max_try_count: 4u32,
                    quorum_weight: 3u32,
                },
            ],
            provider_override: true,
        },
    );
    let proto = Chain::into_proto(chain);
    assert_eq!(proto.id, String::from("123456"));
    assert_eq!(proto.created_at, 1234567890u64);
    assert_eq!(proto.updated_at, 1234567891u64);
    assert_eq!(proto.chain_id, 5u64);
    assert_eq!(proto.name, String::from("Ethereum Goerli"));
    assert!(!proto.name_override);
    assert_eq!(proto.providers.len(), 2);
    assert_eq!(proto.providers[0].url, String::from("http://localhost:8545"));
    assert_eq!(proto.providers[0].timeout_ms, 10000u32);
    assert_eq!(proto.providers[0].max_try_count, 3u32);
    assert_eq!(proto.providers[0].quorum_weight, 2u32);
    assert_eq!(proto.providers[1].url, String::from("http://localhost:8546"));
    assert_eq!(proto.providers[1].timeout_ms, 20000u32);
    assert_eq!(proto.providers[1].max_try_count, 4u32);
    assert_eq!(proto.providers[1].quorum_weight, 3u32);
    assert!(proto.provider_override);
}
