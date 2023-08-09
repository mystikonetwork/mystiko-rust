use anyhow::Result;
use ethers_core::abi::Address;
use ethers_core::types::{H256, U256};
use ethers_providers::Quorum;
use mystiko_abi::commitment_pool::{CommitmentIncludedFilter, CommitmentQueuedFilter, CommitmentSpentFilter};
use mystiko_abi::mystiko_v2_bridge::CommitmentCrossChainFilter;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::data::types::FullData;
use mystiko_dataloader::fetcher::provider::Event;
use mystiko_dataloader::fetcher::provider::ProviderFetcher;
use mystiko_dataloader::fetcher::provider::ProviderFetcherOptions;
use mystiko_dataloader::fetcher::provider::{
    wrap_commitments_from_events, wrap_nullifiers_from_events, CommitmentDataEvent, NullifierDataEvent,
};
use mystiko_dataloader::fetcher::types::ChainFetchOption;
use mystiko_dataloader::fetcher::types::ContractFetchOption;
use mystiko_dataloader::fetcher::types::DataFetcher;
use mystiko_dataloader::fetcher::types::FetchOption;
use mystiko_ethers::provider::factory::{DefaultProviderFactory, Provider, ProviderFactory, ProvidersOptions};
use mystiko_ethers::provider::types::{ProviderOptions, QuorumProviderOptions};
use mystiko_etherscan_client::log::{Log, LogMeta};
use mystiko_types::ProviderType;
use mystiko_utils::convert::u256_to_bytes;
use std::str::FromStr;
use std::sync::Arc;

pub async fn create_chain_provider(config: &MystikoConfig, chain_id: u64) -> Result<Provider> {
    let chain_config = config.find_chain(chain_id).unwrap();
    let provder_factory = DefaultProviderFactory::new();
    let provider = match chain_config.provider_type() {
        ProviderType::Failover => {
            let provider_options = chain_config
                .providers()
                .iter()
                .map(|p| ProviderOptions::builder().url(p.url().to_string()).build())
                .collect::<Vec<ProviderOptions>>();
            let providers_options = ProvidersOptions::Failover(provider_options);
            provder_factory.create_provider(providers_options).await
        }
        ProviderType::Quorum => {
            let provider_options = chain_config
                .providers()
                .iter()
                .map(|p| {
                    ProviderOptions::builder()
                        .url(p.url().to_string())
                        .quorum_weight(p.quorum_weight().into())
                        .build()
                })
                .collect::<Vec<ProviderOptions>>();
            let quorum_options = QuorumProviderOptions::builder().quorum(Quorum::Majority).build();
            let providers_options = ProvidersOptions::Quorum(provider_options, quorum_options);
            provder_factory.create_provider(providers_options).await
        }
    };
    provider
}

#[tokio::test]
async fn test_fetch() {
    let test_chain_id: u64 = 137;
    let test_start_block: u64 = 45564268;
    let test_target_block: u64 = 45565268;
    let test_address = String::from("0xCB255075f38C75EAf2DE8A72897649dba9B90299");
    let mystiko_config = Arc::new(MystikoConfig::from_json_file("./tests/files/config/mystiko.json").await.unwrap());
    let provider_result = create_chain_provider(&mystiko_config, 137).await.unwrap();
    let provider = Arc::new(provider_result);
    let fulldata_fetcher = ProviderFetcher::<FullData>::new(
        ProviderFetcherOptions::builder()
            .chain_id(test_chain_id)
            .provider(provider)
            .build(),
    );
    // test error chainFetchOption
    let contract_config = mystiko_config
        .find_contract_by_address(test_chain_id, &test_address)
        .unwrap();
    let error_chain_id: u64 = 138;
    let chain_option1 = ChainFetchOption::builder()
        .chain_id(error_chain_id)
        .start_block(test_start_block)
        .target_block(test_target_block)
        .config(Arc::clone(&mystiko_config))
        .contracts(Some(vec![contract_config]))
        .build();
    let chain_fetch_option1 = FetchOption::Chain(&chain_option1);
    let full_error_result1 = fulldata_fetcher.fetch(&chain_fetch_option1).await;
    assert!(full_error_result1.is_err());
    assert_eq!(
        full_error_result1.err().unwrap().to_string(),
        "chain_id param in ChainFetchOption(138) is different from ProviderFetcher(137)"
    );
    // test error ChainFetcherOptions
    let contracts_options = vec![ContractFetchOption::builder()
        .address("0xCB255075f38C75EAf2DE8A72897649dba9B90299")
        .chain_id(error_chain_id)
        .config(Arc::clone(&mystiko_config))
        .start_block(test_start_block)
        .target_block(test_target_block)
        .build()];
    let contracts_fetch_option = FetchOption::Contracts(&contracts_options);
    let full_error_result2 = fulldata_fetcher.fetch(&contracts_fetch_option).await;
    assert!(full_error_result2.is_err());
    assert_eq!(
        full_error_result2.err().unwrap().to_string(),
        "param chain_id in options is inconsistent with that in ProviderFetcher"
    );
    let contracts_options2 = vec![];
    let contracts_fetch_option2 = FetchOption::Contracts(&contracts_options2);
    let full_error_result3 = fulldata_fetcher.fetch(&contracts_fetch_option2).await;
    assert!(full_error_result3.is_err());
    assert_eq!(
        full_error_result3.err().unwrap().to_string(),
        "providerfetcher found ContractFetcherOptions is empty, will do nothing"
    );
}

#[tokio::test]
async fn test_wrap_commitments_from_events() {
    let mut mock_crosschain_events: Vec<Event<CommitmentCrossChainFilter>> = vec![];
    let crosschain_filter1: CommitmentCrossChainFilter = CommitmentCrossChainFilter {
        commitment: U256::from_str("0x111").unwrap(),
    };
    let mock_log1 = Log {
        address: Address::from_str("0xCB255075f38C75EAf2DE8A72897649dba9B90299").expect("invalid contract address"),
        topics: vec![
            String::from("0xf533f9705aac5020e21695ea3553ac7b6881070d2b6900ab2b1e3050304b5bf9")
                .parse::<H256>()
                .expect("msg"),
        ],
        data: b"0x111".into(),
        block_hash: Some(
            String::from("0xadb133a483e8898cc1164200c857089826bddfafddc789370eddd80195eb4f47")
                .parse::<H256>()
                .expect("msg"),
        ),
        block_number: Some(10000.into()),
        transaction_hash: Some(
            String::from("0x7e49489ef2cdeed41f538dae9468f5b8926586ae1803398452fac50d235823ea")
                .parse::<H256>()
                .expect("msg"),
        ),
    };
    let metadata1: LogMeta = (&mock_log1).into();
    let mock_crosschain_event1: Event<CommitmentCrossChainFilter> =
        Event::builder().raw(crosschain_filter1).metadata(metadata1).build();
    mock_crosschain_events.push(mock_crosschain_event1.clone());

    let mut mock_queued_events: Vec<Event<CommitmentQueuedFilter>> = vec![];
    let queued_filter1: CommitmentQueuedFilter = CommitmentQueuedFilter {
        commitment: U256::from_str("0x111").unwrap(),
        rollup_fee: U256::from_str("0x111").unwrap(),
        leaf_index: U256::from_str("0x111").unwrap(),
        encrypted_note: b"0x111".into(),
    };
    let mock_log2 = Log {
        address: Address::from_str("0xCB255075f38C75EAf2DE8A72897649dba9B90299").expect("invalid contract address"),
        topics: vec![
            String::from("0xf533f9705aac5020e21695ea3553ac7b6881070d2b6900ab2b1e3050304b5bf9")
                .parse::<H256>()
                .expect("msg"),
        ],
        data: b"0x222".into(),
        block_hash: Some(
            String::from("0xadb133a483e8898cc1164200c857089826bddfafddc789370eddd80195eb4f47")
                .parse::<H256>()
                .expect("msg"),
        ),
        block_number: Some(20000.into()),
        transaction_hash: Some(
            String::from("0x7e49489ef2cdeed41f538dae9468f5b8926586ae1803398452fac50d235823ea")
                .parse::<H256>()
                .expect("msg"),
        ),
    };
    let metadata2: LogMeta = (&mock_log2).into();
    let mock_queued_event2: Event<CommitmentQueuedFilter> =
        Event::builder().raw(queued_filter1).metadata(metadata2).build();
    mock_queued_events.push(mock_queued_event2.clone());

    let mut mock_included_events: Vec<Event<CommitmentIncludedFilter>> = vec![];
    let included_filter1: CommitmentIncludedFilter = CommitmentIncludedFilter {
        commitment: U256::from_str("0x333").unwrap(),
    };
    let mock_log3 = Log {
        address: Address::from_str("0xCB255075f38C75EAf2DE8A72897649dba9B90299").expect("invalid contract address"),
        topics: vec![
            String::from("0xf533f9705aac5020e21695ea3553ac7b6881070d2b6900ab2b1e3050304b5bf9")
                .parse::<H256>()
                .expect("msg"),
        ],
        data: b"0x222".into(),
        block_hash: Some(
            String::from("0xadb133a483e8898cc1164200c857089826bddfafddc789370eddd80195eb4f47")
                .parse::<H256>()
                .expect("msg"),
        ),
        block_number: Some(30000.into()),
        transaction_hash: Some(
            String::from("0x7e49489ef2cdeed41f538dae9468f5b8926586ae1803398452fac50d235823ea")
                .parse::<H256>()
                .expect("msg"),
        ),
    };
    let metadata3: LogMeta = (&mock_log3).into();
    let mock_included_event3: Event<CommitmentIncludedFilter> =
        Event::builder().raw(included_filter1).metadata(metadata3).build();
    mock_included_events.push(mock_included_event3.clone());
    let commitments_result = wrap_commitments_from_events(
        CommitmentDataEvent::builder()
            .crosschain_events(mock_crosschain_events)
            .included_events(mock_included_events)
            .queued_events(mock_queued_events)
            .build(),
    )
    .await;
    assert!(commitments_result.is_ok());
    let commitments = commitments_result.unwrap();
    assert_eq!(commitments.len(), 3);
    assert_eq!(commitments[0].block_number, 10000);
    assert_eq!(commitments[1].block_number, 20000);
    assert_eq!(
        commitments[2].commitment_hash,
        u256_to_bytes(&U256::from_str("0x333").unwrap())
    );
}

#[tokio::test]
async fn test_wrap_nullifiers_from_events() {
    let mut mock_spent_events: Vec<Event<CommitmentSpentFilter>> = vec![];
    let spent_filter1: CommitmentSpentFilter = CommitmentSpentFilter {
        root_hash: U256::from_str("0x111").unwrap(),
        serial_number: U256::from_str("0x1111").unwrap(),
    };
    let mock_log1 = Log {
        address: Address::from_str("0xCB255075f38C75EAf2DE8A72897649dba9B90299").expect("invalid contract address"),
        topics: vec![
            String::from("0xf533f9705aac5020e21695ea3553ac7b6881070d2b6900ab2b1e3050304b5bf9")
                .parse::<H256>()
                .expect("msg"),
        ],
        data: b"0x111".into(),
        block_hash: Some(
            String::from("0xadb133a483e8898cc1164200c857089826bddfafddc789370eddd80195eb4f47")
                .parse::<H256>()
                .expect("msg"),
        ),
        block_number: Some(10000.into()),
        transaction_hash: Some(
            String::from("0x7e49489ef2cdeed41f538dae9468f5b8926586ae1803398452fac50d235823ea")
                .parse::<H256>()
                .expect("msg"),
        ),
    };
    let metadata1: LogMeta = (&mock_log1).into();
    let mock_spent_event1: Event<CommitmentSpentFilter> =
        Event::builder().raw(spent_filter1).metadata(metadata1).build();
    mock_spent_events.push(mock_spent_event1.clone());

    let spent_filter2: CommitmentSpentFilter = CommitmentSpentFilter {
        root_hash: U256::from_str("0x222").unwrap(),
        serial_number: U256::from_str("0x2222").unwrap(),
    };
    let mock_log2 = Log {
        address: Address::from_str("0xCB255075f38C75EAf2DE8A72897649dba9B90299").expect("invalid contract address"),
        topics: vec![
            String::from("0xf533f9705aac5020e21695ea3553ac7b6881070d2b6900ab2b1e3050304b5bf9")
                .parse::<H256>()
                .expect("msg"),
        ],
        data: b"0x111".into(),
        block_hash: Some(
            String::from("0xadb133a483e8898cc1164200c857089826bddfafddc789370eddd80195eb4f47")
                .parse::<H256>()
                .expect("msg"),
        ),
        block_number: Some(20000.into()),
        transaction_hash: Some(
            String::from("0x7e49489ef2cdeed41f538dae9468f5b8926586ae1803398452fac50d235823ea")
                .parse::<H256>()
                .expect("msg"),
        ),
    };
    let metadata2: LogMeta = (&mock_log2).into();
    let mock_spent_event2: Event<CommitmentSpentFilter> =
        Event::builder().raw(spent_filter2).metadata(metadata2).build();
    mock_spent_events.push(mock_spent_event2.clone());
    let nullifiers_result =
        wrap_nullifiers_from_events(NullifierDataEvent::builder().spent_events(mock_spent_events).build()).await;
    assert!(nullifiers_result.is_ok());
    let nullifiers = nullifiers_result.unwrap();
    assert_eq!(nullifiers.len(), 2);
    assert_eq!(nullifiers[0].block_number, 10000);
    assert_eq!(
        nullifiers[1].nullifier,
        u256_to_bytes(&U256::from_str("0x2222").unwrap())
    );
}
