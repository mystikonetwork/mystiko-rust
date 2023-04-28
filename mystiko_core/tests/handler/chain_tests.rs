use crate::common::create_database;
use ethers_providers::Quorum;
use mystiko_config::raw::create_raw_from_file;
use mystiko_config::raw::mystiko::RawMystikoConfig;
use mystiko_config::raw::provider::RawProviderConfig;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_core::handler::chain::{
    ChainHandler, UpdateChainOptions, UpdateProviderOptions, DEFAULT_PROVIDER_MAX_TRY_COUNT,
    DEFAULT_PROVIDER_QUORUM_WEIGHT, DEFAULT_PROVIDER_TIMEOUT_MS,
};
use mystiko_core::handler::contract::ContractHandler;
use mystiko_database::database::Database;
use mystiko_database::document::chain::Provider;
use mystiko_ethers::provider::factory::ProvidersOptions;
use mystiko_ethers::provider::pool::ChainProvidersOptions;
use mystiko_storage::document::DOCUMENT_ID_FIELD;
use mystiko_storage::filter::{Condition, QueryFilterBuilder, SubFilter};
use mystiko_storage::formatter::SqlFormatter;
use mystiko_storage_sqlite::{SqliteRawData, SqliteStorage};
use std::sync::Arc;
use std::time::Duration;

type TypedDatabase = Database<SqlFormatter, SqliteRawData, SqliteStorage>;
type TypedChainHandler = ChainHandler<SqlFormatter, SqliteRawData, SqliteStorage>;
type TypedContractHandler = ContractHandler<SqlFormatter, SqliteRawData, SqliteStorage>;

async fn setup() -> (TypedChainHandler, Arc<TypedDatabase>, Arc<MystikoConfig>) {
    let database = Arc::new(create_database().await);
    database.migrate().await.unwrap();
    let config = Arc::new(
        MystikoConfig::from_json_file("tests/files/handler/contract/config.json")
            .await
            .unwrap(),
    );
    let handler = TypedChainHandler::new(database.clone(), config.clone());
    (handler, database, config)
}

#[tokio::test]
async fn test_chain_initialize() {
    let (handler, _, config) = setup().await;
    let chains = handler.initialize().await.unwrap();
    for chain in chains.iter() {
        if let Some(chain_config) = config.find_chain(chain.data.chain_id) {
            let provider_configs = chain_config.providers();
            assert_eq!(&chain.data.name, chain_config.name());
            assert!(!chain.data.name_override);
            assert_eq!(chain.data.providers.len(), provider_configs.len());
            for (index, _) in chain.data.providers.iter().enumerate() {
                assert_eq!(&chain.data.providers[index].url, provider_configs[index].url());
                assert_eq!(
                    chain.data.providers[index].timeout_ms,
                    provider_configs[index].timeout_ms()
                );
                assert_eq!(
                    chain.data.providers[index].max_try_count,
                    provider_configs[index].max_try_count()
                );
                assert_eq!(
                    chain.data.providers[index].quorum_weight,
                    provider_configs[index].quorum_weight()
                );
            }
            assert!(!chain.data.provider_override);
            assert_eq!(chain.data.synced_block_number, 0);
        } else {
            panic!("Chain config not found");
        }
    }
}

#[tokio::test]
async fn test_chain_initialize_upsert() {
    let (handler, db, _) = setup().await;
    let mut chains = handler.initialize().await.unwrap();
    chains[0].data.name = String::from("Chain #1");
    chains[0].data.name_override = true;
    chains[1].data.providers = vec![Provider {
        url: String::from("http://localhost:8545"),
        timeout_ms: 40000,
        max_try_count: 5,
        quorum_weight: 4,
    }];
    chains[1].data.provider_override = true;
    db.chains.update_batch(&chains).await.unwrap();
    let mut raw_config = create_raw_from_file::<RawMystikoConfig>("tests/files/handler/contract/config.json")
        .await
        .unwrap();
    let mut raw_chain_config_0 = raw_config.chains.remove(0).as_ref().clone();
    raw_chain_config_0.name = String::from("Chain #1.1");
    let mut raw_chain_config_1 = raw_config.chains.remove(0).as_ref().clone();
    raw_chain_config_1.providers = vec![Arc::new(RawProviderConfig {
        url: String::from("http://localhost:8546"),
        timeout_ms: 50000,
        max_try_count: 4,
        quorum_weight: 3,
    })];
    let mut raw_chain_config_2 = raw_config.chains.remove(0).as_ref().clone();
    raw_chain_config_2.name = String::from("Chain #2");
    raw_chain_config_2.providers = vec![Arc::new(RawProviderConfig {
        url: String::from("http://localhost:8547"),
        timeout_ms: 60000,
        max_try_count: 2,
        quorum_weight: 2,
    })];
    raw_config.chains.insert(0, Arc::new(raw_chain_config_2));
    raw_config.chains.insert(0, Arc::new(raw_chain_config_1));
    raw_config.chains.insert(0, Arc::new(raw_chain_config_0));
    let config = MystikoConfig::from_raw(raw_config).unwrap();
    let handler = TypedChainHandler::new(db, Arc::new(config));
    let chains = handler.initialize().await.unwrap();
    assert_eq!(&chains[0].data.name, "Chain #1");
    assert_eq!(chains[1].data.providers.len(), 1);
    assert_eq!(&chains[1].data.providers[0].url, "http://localhost:8545");
    assert_eq!(&chains[2].data.name, "Chain #2");
    assert_eq!(chains[2].data.providers.len(), 1);
    assert_eq!(&chains[2].data.providers[0].url, "http://localhost:8547");
}

#[tokio::test]
async fn test_chains_find() {
    let (handler, _, _) = setup().await;
    let mut chains = handler.initialize().await.unwrap();
    assert_eq!(
        handler.find_all().await.unwrap().len() as u64,
        handler.count_all().await.unwrap()
    );
    chains.sort_by_key(|c| c.data.chain_id);
    let filter = QueryFilterBuilder::new()
        .filter(Condition::FILTER(SubFilter::IN(
            DOCUMENT_ID_FIELD.to_string(),
            vec![chains[0].id.clone(), chains[1].id.clone()],
        )))
        .build();
    let mut found_chains = handler.find(filter).await.unwrap();
    found_chains.sort_by_key(|c| c.data.chain_id);
    assert_eq!(found_chains[0], chains[0]);
    assert_eq!(found_chains[1], chains[1]);
    let found_chain = handler.find_by_id(&chains[0].id).await.unwrap().unwrap();
    assert_eq!(found_chain, chains[0]);
    let found_chain = handler
        .find_by_chain_id(chains[0].data.chain_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found_chain, chains[0]);
}

#[tokio::test]
async fn test_chains_count() {
    let (handler, _, _) = setup().await;
    let chains = handler.initialize().await.unwrap();
    let filter = QueryFilterBuilder::new()
        .filter(Condition::FILTER(SubFilter::IN(
            DOCUMENT_ID_FIELD.to_string(),
            vec![chains[0].id.clone(), chains[1].id.clone()],
        )))
        .build();
    assert_eq!(handler.count(filter).await.unwrap(), 2);
}

#[tokio::test]
async fn test_chains_reset_name_and_providers() {
    let (handler, db, config) = setup().await;
    let mut chains = handler.initialize().await.unwrap();
    chains[0].data.name = String::from("Chain #1");
    chains[0].data.name_override = true;
    chains[0].data.providers = vec![Provider {
        url: String::from("http://localhost:8545"),
        timeout_ms: 40000,
        max_try_count: 5,
        quorum_weight: 4,
    }];
    chains[0].data.provider_override = true;
    db.chains.update(&chains[0]).await.unwrap();
    handler.reset_name_and_providers(chains[0].data.chain_id).await.unwrap();
    let found_chain = handler
        .find_by_chain_id(chains[0].data.chain_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        &found_chain.data.name,
        config.find_chain(found_chain.data.chain_id).unwrap().name()
    );
    assert!(!found_chain.data.name_override);
    assert_eq!(
        found_chain.data.providers.len(),
        config.find_chain(found_chain.data.chain_id).unwrap().providers().len()
    );
    assert!(!found_chain.data.provider_override);
    assert!(handler.reset_name_and_providers(1242342345).await.unwrap().is_none());
    db.chains.delete(&chains[0]).await.unwrap();
    assert!(handler
        .reset_name_and_providers(chains[0].data.chain_id)
        .await
        .unwrap()
        .is_none());
}

#[tokio::test]
async fn test_chains_update_name() {
    let (handler, _, _) = setup().await;
    let chains = handler.initialize().await.unwrap();
    let previous_name = chains[0].data.name.clone();
    let chain = handler
        .update_by_id(&chains[0].id, &UpdateChainOptions::builder().build())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(&chain.data.name, &previous_name);
    assert!(!chain.data.name_override);
    let chain = handler
        .update_by_chain_id(
            chains[0].data.chain_id,
            &UpdateChainOptions::builder().name(String::from("")).build(),
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(&chain.data.name, &previous_name);
    assert!(!chain.data.name_override);
    let chain = handler
        .update_by_chain_id(
            chains[0].data.chain_id,
            &UpdateChainOptions::builder().name(previous_name.clone()).build(),
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(&chain.data.name, &previous_name);
    assert!(!chain.data.name_override);
    let chain = handler
        .update_by_chain_id(
            chains[0].data.chain_id,
            &UpdateChainOptions::builder().name(String::from("Chain #1")).build(),
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(&chain.data.name, "Chain #1");
    assert!(chain.data.name_override);
    assert!(handler
        .update_by_chain_id(23234234, &UpdateChainOptions::builder().build())
        .await
        .unwrap()
        .is_none());
}

#[tokio::test]
async fn test_chains_update_providers() {
    let (handler, db, _) = setup().await;
    let mut chains = handler.initialize().await.unwrap();
    let previous_providers = chains[0].data.providers.clone();
    let chain = handler
        .update_by_id(&chains[0].id, &UpdateChainOptions::builder().build())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(chain.data.providers, previous_providers);
    assert!(!chain.data.provider_override);
    let chain = handler
        .update_by_chain_id(
            chains[0].data.chain_id,
            &UpdateChainOptions::builder().providers(vec![]).build(),
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(chain.data.providers, previous_providers);
    assert!(!chain.data.provider_override);
    let update_providers_options: Vec<UpdateProviderOptions> = previous_providers
        .iter()
        .map(|p| UpdateProviderOptions::builder().url(p.url.clone()).build())
        .collect();
    let chain = handler
        .update_by_chain_id(
            chains[0].data.chain_id,
            &UpdateChainOptions::builder()
                .providers(update_providers_options)
                .build(),
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(chain.data.providers, previous_providers);
    assert!(!chain.data.provider_override);
    let update_providers_options: Vec<UpdateProviderOptions> = previous_providers
        .iter()
        .map(|p| {
            UpdateProviderOptions::builder()
                .url(p.url.clone())
                .timeout_ms(p.timeout_ms)
                .max_try_count(p.max_try_count)
                .quorum_weight(p.quorum_weight)
                .build()
        })
        .collect();
    let chain = handler
        .update_by_chain_id(
            chains[0].data.chain_id,
            &UpdateChainOptions::builder()
                .providers(update_providers_options.clone())
                .build(),
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(chain.data.providers, previous_providers);
    assert!(!chain.data.provider_override);
    chains[0].data.providers = vec![];
    db.chains.update(&chains[0]).await.unwrap();
    let chain = handler
        .update_by_chain_id(
            chains[0].data.chain_id,
            &UpdateChainOptions::builder()
                .providers(update_providers_options)
                .build(),
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(chain.data.providers, previous_providers);
    assert!(chain.data.provider_override);
    let chain = handler
        .update_by_chain_id(
            chains[0].data.chain_id,
            &UpdateChainOptions::builder()
                .providers(vec![
                    UpdateProviderOptions::builder()
                        .url(String::from("http://localhost:8545"))
                        .timeout_ms(100_000)
                        .max_try_count(10)
                        .quorum_weight(10)
                        .build(),
                    UpdateProviderOptions::builder()
                        .url(String::from("http://localhost:8546"))
                        .build(),
                ])
                .build(),
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        chain.data.providers,
        vec![
            Provider {
                url: String::from("http://localhost:8545"),
                timeout_ms: 100_000,
                max_try_count: 10,
                quorum_weight: 10
            },
            Provider {
                url: String::from("http://localhost:8546"),
                timeout_ms: DEFAULT_PROVIDER_TIMEOUT_MS,
                max_try_count: DEFAULT_PROVIDER_MAX_TRY_COUNT,
                quorum_weight: DEFAULT_PROVIDER_QUORUM_WEIGHT,
            }
        ]
    );
    assert!(chain.data.provider_override);
    assert!(handler
        .update_by_chain_id(
            chains[0].data.chain_id,
            &UpdateChainOptions::builder()
                .providers(vec![UpdateProviderOptions::builder()
                    .url(String::from("wrong_scheme://localhost:9565"))
                    .build(),])
                .build(),
        )
        .await
        .is_err())
}

#[tokio::test]
async fn test_chains_reset_synced_block() {
    let (handler, db, config) = setup().await;
    let contract_handler = TypedContractHandler::new(db.clone(), config);
    let mut contracts = contract_handler.initialize().await.unwrap();
    let mut chains = handler.initialize().await.unwrap();
    for contract in contracts.iter_mut() {
        if contract.data.chain_id == chains[0].data.chain_id {
            contract.data.synced_block_number = 10;
        }
    }
    db.contracts.update_batch(&contracts).await.unwrap();
    chains[0].data.synced_block_number = 10;
    db.chains.update(&chains[0]).await.unwrap();
    handler.reset_synced_block(chains[0].data.chain_id).await.unwrap();
    let chain = handler
        .find_by_chain_id(chains[0].data.chain_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(chain.data.synced_block_number, 0);
    let contracts = contract_handler
        .find_by_chain_id(chains[0].data.chain_id)
        .await
        .unwrap();
    for contract in contracts.iter() {
        assert_eq!(contract.data.synced_block_number, contract.data.sync_start);
    }
    handler
        .reset_synced_block_to(chains[0].data.chain_id, 20)
        .await
        .unwrap();
    let chain = handler
        .find_by_chain_id(chains[0].data.chain_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(chain.data.synced_block_number, 20);
    let contracts = contract_handler
        .find_by_chain_id(chains[0].data.chain_id)
        .await
        .unwrap();
    for contract in contracts.iter() {
        assert_eq!(contract.data.synced_block_number, 20);
    }
    assert!(handler.reset_synced_block(23423432).await.unwrap().is_none());
    db.chains.delete(&chains[0]).await.unwrap();
    assert!(handler
        .reset_synced_block(chains[0].data.chain_id)
        .await
        .unwrap()
        .is_none());
}

#[tokio::test]
async fn test_chains_providers_options() {
    let (handler, db, _) = setup().await;
    let chains = handler.initialize().await.unwrap();
    let providers_options = handler.providers_options(11155111).await.unwrap().unwrap();
    let chain = handler.find_by_chain_id(11155111).await.unwrap().unwrap();
    match providers_options {
        ProvidersOptions::Failover(options) => {
            assert_eq!(options.len(), chain.data.providers.len());
            for (index, _) in options.iter().enumerate() {
                assert_eq!(options[index].url, chain.data.providers[index].url);
                assert_eq!(
                    options[index].quorum_weight.unwrap(),
                    chain.data.providers[index].quorum_weight as u64
                );
                assert_eq!(
                    options[index].timeout_retries.unwrap(),
                    chain.data.providers[index].max_try_count - 1
                );
                assert_eq!(
                    options[index].rate_limit_retries.unwrap(),
                    chain.data.providers[index].max_try_count - 1
                );
                assert_eq!(
                    options[index].request_timeout.unwrap(),
                    Duration::from_millis(chain.data.providers[index].timeout_ms as u64)
                );
            }
        }
        _ => panic!("unexpected provider options type"),
    }
    let providers_options = handler.providers_options(97).await.unwrap().unwrap();
    let chain = handler.find_by_chain_id(97).await.unwrap().unwrap();
    match providers_options {
        ProvidersOptions::Quorum(options, quorum_options) => {
            assert_eq!(options.len(), chain.data.providers.len());
            for (index, _) in options.iter().enumerate() {
                assert_eq!(options[index].url, chain.data.providers[index].url);
                assert_eq!(
                    options[index].quorum_weight.unwrap(),
                    chain.data.providers[index].quorum_weight as u64
                );
                assert_eq!(
                    options[index].timeout_retries.unwrap(),
                    chain.data.providers[index].max_try_count - 1
                );
                assert_eq!(
                    options[index].rate_limit_retries.unwrap(),
                    chain.data.providers[index].max_try_count - 1
                );
                assert_eq!(
                    options[index].request_timeout.unwrap(),
                    Duration::from_millis(chain.data.providers[index].timeout_ms as u64)
                );
            }
            match quorum_options.quorum.unwrap() {
                Quorum::Percentage(percentage) => assert_eq!(percentage, 75),
                _ => panic!("unexpected quorum type"),
            }
        }
        _ => panic!("unexpected provider options type"),
    }
    assert!(handler.providers_options(123234234).await.unwrap().is_none());
    db.chains.delete(&chains[0]).await.unwrap();
    assert!(handler
        .providers_options(chains[0].data.chain_id)
        .await
        .unwrap()
        .is_none());
}
