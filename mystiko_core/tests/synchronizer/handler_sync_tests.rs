use crate::synchronizer::mock::{create_synchronizer, MockSyncDataLoader};
use mystiko_core::SynchronizerHandler;
use mystiko_dataloader::fetcher::{PACKER_FETCHER_NAME, PROVIDER_FETCHER_NAME, SEQUENCER_FETCHER_NAME};
use mystiko_dataloader::loader::{
    LoadFetcherOption, LoadFetcherSkipOption, LoadOption, LoadValidatorOption, LoadValidatorSkipOption,
};
use mystiko_dataloader::validator::rule::{
    RULE_COUNTER_CHECKER_NAME, RULE_INTEGRITY_CHECKER_NAME, RULE_MERKLE_TREE_CHECKER_NAME, RULE_SEQUENCE_CHECKER_NAME,
    RULE_VALIDATOR_NAME,
};
use mystiko_protos::core::synchronizer::v1::SyncOptions;
use std::collections::HashMap;

#[tokio::test]
async fn test_chain_synced_with_default_options() {
    let mut loader = MockSyncDataLoader::new();
    let expected_load_options = build_default_expect_load_option();
    loader
        .expect_load::<LoadOption>()
        .withf(move |options| load_options_compare(options, &expected_load_options))
        .returning(|_| Ok(()));
    let synchronizer = create_synchronizer(1, vec![loader]).await;
    let sync_options = SyncOptions::builder().build();
    let result = synchronizer.sync(sync_options).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_chain_synced_with_changed_options() {
    let mut loader = MockSyncDataLoader::new();
    let mut expected_load_options = build_default_expect_load_option();
    expected_load_options.fetcher.query_loaded_block_timeout_ms = 1000;
    expected_load_options.fetcher.fetch_timeout_ms = 2000;
    expected_load_options.validator.concurrency = 300;
    loader
        .expect_load::<LoadOption>()
        .withf(move |options| load_options_compare(options, &expected_load_options))
        .returning(|_| Ok(()));
    let synchronizer = create_synchronizer(1, vec![loader]).await;
    let mut sync_options = SyncOptions::builder().build();
    sync_options.fetcher_query_loaded_block_timeout_ms = Some(1000);
    sync_options.fetcher_fetch_timeout_ms = Some(2000);
    sync_options.validator_validate_concurrency = Some(300);
    let result = synchronizer.sync(sync_options).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_chain_synced_with_chain_ids() {
    let loader = MockSyncDataLoader::new();
    let synchronizer = create_synchronizer(1, vec![loader]).await;
    let mut sync_options = SyncOptions::builder().build();
    sync_options.chain_ids = vec![2];
    let result = synchronizer.sync(sync_options).await;
    assert!(result.is_ok());

    let mut loader = MockSyncDataLoader::new();
    let expected_load_options = build_default_expect_load_option();
    loader
        .expect_load::<LoadOption>()
        .withf(move |options| load_options_compare(options, &expected_load_options))
        .returning(|_| Ok(()));
    let synchronizer = create_synchronizer(1, vec![loader]).await;
    let mut sync_options = SyncOptions::builder().build();
    sync_options.chain_ids = vec![1];
    let result = synchronizer.sync(sync_options).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_chain_synced_with_packer_disabled_options() {
    for status in [true, false] {
        let mut loader = MockSyncDataLoader::new();
        let mut expected_load_options = build_default_expect_load_option();
        expected_load_options
            .fetcher
            .skips
            .entry(PACKER_FETCHER_NAME)
            .and_modify(|skip_option| {
                skip_option.skip_fetch = Some(status);
                skip_option.skip_validation = Some(!status);
            });
        loader
            .expect_load::<LoadOption>()
            .withf(move |options| load_options_compare(options, &expected_load_options))
            .returning(|_| Ok(()));
        let synchronizer = create_synchronizer(1, vec![loader]).await;
        let mut sync_options = SyncOptions::builder().build();
        sync_options.disable_datapacker_fetcher = Some(status);
        sync_options.enable_datapacker_fetcher_validate = Some(status);
        let result = synchronizer.sync(sync_options).await;
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_chain_synced_with_sequencer_disabled_options() {
    for status in [true, false] {
        let mut loader = MockSyncDataLoader::new();
        let mut expected_load_options = build_default_expect_load_option();
        expected_load_options
            .fetcher
            .skips
            .entry(SEQUENCER_FETCHER_NAME)
            .and_modify(|skip_option| {
                skip_option.skip_fetch = Some(status);
                skip_option.skip_validation = Some(!status);
            });
        loader
            .expect_load::<LoadOption>()
            .withf(move |options| load_options_compare(options, &expected_load_options))
            .returning(|_| Ok(()));
        let synchronizer = create_synchronizer(1, vec![loader]).await;
        let mut sync_options = SyncOptions::builder().build();
        sync_options.disable_sequencer_fetcher = Some(status);
        sync_options.enable_sequencer_fetcher_validate = Some(status);
        let result = synchronizer.sync(sync_options).await;
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_chain_synced_with_provider_disabled_options() {
    for status in [true, false] {
        let mut loader = MockSyncDataLoader::new();
        let mut expected_load_options = build_default_expect_load_option();
        expected_load_options
            .fetcher
            .skips
            .entry(PROVIDER_FETCHER_NAME)
            .and_modify(|skip_option| {
                skip_option.skip_fetch = Some(status);
                skip_option.skip_validation = Some(status);
            });
        loader
            .expect_load::<LoadOption>()
            .withf(move |options| load_options_compare(options, &expected_load_options))
            .returning(|_| Ok(()));
        let synchronizer = create_synchronizer(1, vec![loader]).await;
        let mut sync_options = SyncOptions::builder().build();
        sync_options.disable_provider_fetcher = Some(status);
        sync_options.disable_provider_fetcher_validate = Some(status);
        let result = synchronizer.sync(sync_options).await;
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_chain_synced_with_rule_validator_disabled_options() {
    for status in [true, false] {
        let mut loader = MockSyncDataLoader::new();
        let mut expected_load_options = build_default_expect_load_option();
        expected_load_options
            .validator
            .skips
            .entry(RULE_VALIDATOR_NAME)
            .and_modify(|skip_option| {
                skip_option.skip_validation = Some(status);
            });
        loader
            .expect_load::<LoadOption>()
            .withf(move |options| load_options_compare(options, &expected_load_options))
            .returning(|_| Ok(()));
        let synchronizer = create_synchronizer(1, vec![loader]).await;
        let mut sync_options = SyncOptions::builder().build();
        sync_options.disable_rule_validator = Some(status);
        let result = synchronizer.sync(sync_options).await;
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_chain_synced_with_checker_disabled_options() {
    for status in [true, false] {
        let mut loader = MockSyncDataLoader::new();
        let mut expected_load_options = build_default_expect_load_option();
        expected_load_options
            .validator
            .skips
            .entry(RULE_VALIDATOR_NAME)
            .and_modify(|skip_option| {
                skip_option.skip_checkers.insert(RULE_COUNTER_CHECKER_NAME, status);
                skip_option.skip_checkers.insert(RULE_SEQUENCE_CHECKER_NAME, status);
                skip_option.skip_checkers.insert(RULE_INTEGRITY_CHECKER_NAME, status);
                skip_option.skip_checkers.insert(RULE_MERKLE_TREE_CHECKER_NAME, status);
            });
        loader
            .expect_load::<LoadOption>()
            .withf(move |options| load_options_compare(options, &expected_load_options))
            .returning(|_| Ok(()));
        let synchronizer = create_synchronizer(1, vec![loader]).await;
        let mut sync_options = SyncOptions::builder().build();
        sync_options.disable_rule_validator_counter_check = Some(status);
        sync_options.disable_rule_validator_sequence_check = Some(status);
        sync_options.disable_rule_validator_integrity_check = Some(status);
        sync_options.disable_rule_validator_tree_check = Some(status);
        let result = synchronizer.sync(sync_options).await;
        assert!(result.is_ok());
    }
}

fn load_options_compare(a: &LoadOption, b: &LoadOption) -> bool {
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    a == b
}

fn build_default_expect_load_option() -> LoadOption {
    let fetcher_option = build_default_expect_load_fetcher_option();
    let validator_option = build_default_expect_load_validator_option();

    LoadOption::builder()
        .fetcher(fetcher_option)
        .validator(validator_option)
        .build()
}

fn build_default_expect_load_fetcher_option() -> LoadFetcherOption {
    let mut skips = HashMap::new();
    skips.insert(PACKER_FETCHER_NAME, LoadFetcherSkipOption::builder().build());
    skips.insert(SEQUENCER_FETCHER_NAME, LoadFetcherSkipOption::builder().build());
    skips.insert(PROVIDER_FETCHER_NAME, LoadFetcherSkipOption::builder().build());
    LoadFetcherOption::builder().skips(skips).build()
}

fn build_default_expect_load_validator_option() -> LoadValidatorOption {
    let skip_checkers = HashMap::new();
    let mut skips = HashMap::new();
    skips.insert(
        RULE_VALIDATOR_NAME,
        LoadValidatorSkipOption::builder().skip_checkers(skip_checkers).build(),
    );
    LoadValidatorOption::builder().skips(skips).build()
}
