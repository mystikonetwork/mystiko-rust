use crate::synchronizer::mock::{create_synchronizer, MockSyncDataLoader};
use mystiko_core::SynchronizerError;
use mystiko_core::SynchronizerHandler;
use mystiko_dataloader::fetcher::{PACKER_FETCHER_NAME, PROVIDER_FETCHER_NAME, SEQUENCER_FETCHER_NAME};
use mystiko_dataloader::loader::{
    LoadFetcherOption, LoadFetcherSkipOption, LoadOption, LoadStatus, LoadValidatorOption, LoadValidatorSkipOption,
};
use mystiko_dataloader::validator::rule::{
    RULE_COUNTER_CHECKER_NAME, RULE_INTEGRITY_CHECKER_NAME, RULE_MERKLE_TREE_CHECKER_NAME, RULE_SEQUENCE_CHECKER_NAME,
    RULE_VALIDATOR_NAME,
};
use mystiko_dataloader::DataLoaderError;
use mystiko_protos::core::synchronizer::v1::{ChainStatus, SyncOptions};
use std::collections::HashMap;

#[tokio::test]
async fn test_chain_synced_with_default_options() {
    let chain_id = 1_u64;
    let mut loader = MockSyncDataLoader::new();
    let expected_load_options = build_default_expect_load_option();
    let load_status = LoadStatus::builder()
        .chain_id(chain_id)
        .loaded_block(100_u64)
        .target_block(200_u64)
        .build();
    loader
        .expect_load::<LoadOption>()
        .withf(move |options| load_options_compare(options, &expected_load_options))
        .returning(move |_| Ok(load_status));
    let synchronizer = create_synchronizer(chain_id, vec![loader]).await;
    let sync_options = SyncOptions::builder().build();
    let result = synchronizer.sync(sync_options).await.unwrap();
    assert_eq!(result.chains.len(), 1);
    assert_eq!(
        result.chains[0],
        ChainStatus::builder()
            .chain_id(chain_id)
            .synced_block(100_u64)
            .target_block(200_u64)
            .build()
    );
}

#[tokio::test]
async fn test_chain_synced_with_changed_options() {
    let chain_id = 1_u64;
    let load_status = LoadStatus::builder()
        .chain_id(chain_id)
        .loaded_block(100_u64)
        .target_block(200_u64)
        .build();
    let mut loader = MockSyncDataLoader::new();
    let mut expected_load_options = build_default_expect_load_option();
    expected_load_options.fetcher.query_loaded_block_timeout_ms = 1000;
    expected_load_options.fetcher.fetch_timeout_ms = 2000;
    expected_load_options.validator.concurrency = 300;
    loader
        .expect_load::<LoadOption>()
        .withf(move |options| load_options_compare(options, &expected_load_options))
        .returning(move |_| Ok(load_status));
    let synchronizer = create_synchronizer(chain_id, vec![loader]).await;
    let mut sync_options = SyncOptions::builder().build();
    sync_options.fetcher_query_loaded_block_timeout_ms = Some(1000);
    sync_options.fetcher_fetch_timeout_ms = Some(2000);
    sync_options.validator_validate_concurrency = Some(300);
    let result = synchronizer.sync(sync_options).await.unwrap();
    assert_eq!(result.chains.len(), 1);
    assert_eq!(
        result.chains[0],
        ChainStatus::builder()
            .chain_id(chain_id)
            .synced_block(100_u64)
            .target_block(200_u64)
            .build()
    );
}

#[tokio::test]
async fn test_chain_synced_with_two_loader() {
    let loader1 = MockSyncDataLoader::new();
    let loader2 = MockSyncDataLoader::new();
    let synchronizer = create_synchronizer(1, vec![loader1, loader2]).await;
    let mut sync_options = SyncOptions::builder().build();
    sync_options.chain_ids = vec![1, 999999];
    let result = synchronizer.sync(sync_options).await;
    assert!(matches!(
        result.err().unwrap(),
        SynchronizerError::UnsupportedChainError(_)
    ));

    let mut loader1 = MockSyncDataLoader::new();
    let mut loader2 = MockSyncDataLoader::new();
    let load_status1 = LoadStatus::builder()
        .chain_id(1_u64)
        .loaded_block(100_u64)
        .target_block(200_u64)
        .build();
    let load_status2 = LoadStatus::builder()
        .chain_id(2_u64)
        .loaded_block(300_u64)
        .target_block(400_u64)
        .build();
    let expected_load_options1 = build_default_expect_load_option();
    let expected_load_options2 = build_default_expect_load_option();
    loader1
        .expect_load::<LoadOption>()
        .withf(move |options| load_options_compare(options, &expected_load_options1))
        .returning(move |_| Ok(load_status1));
    loader2
        .expect_load::<LoadOption>()
        .withf(move |options| load_options_compare(options, &expected_load_options2))
        .returning(move |_| Ok(load_status2));
    let synchronizer = create_synchronizer(1, vec![loader1, loader2]).await;
    let mut sync_options = SyncOptions::builder().build();
    sync_options.chain_ids = vec![1_u64, 2_u64];
    let result = synchronizer.sync(sync_options).await.unwrap();
    assert_eq!(result.chains.len(), 2);
    for chain in result.chains {
        match chain.chain_id {
            1 => {
                assert_eq!(
                    chain,
                    ChainStatus::builder()
                        .chain_id(1_u64)
                        .synced_block(100_u64)
                        .target_block(200_u64)
                        .build()
                );
            }
            2 => {
                assert_eq!(
                    chain,
                    ChainStatus::builder()
                        .chain_id(2_u64)
                        .synced_block(300_u64)
                        .target_block(400_u64)
                        .build()
                );
            }
            _ => {
                panic!("unexpected chain id");
            }
        }
    }

    let mut loader1 = MockSyncDataLoader::new();
    let mut loader2 = MockSyncDataLoader::new();
    let load_status1 = LoadStatus::builder()
        .chain_id(1_u64)
        .loaded_block(100_u64)
        .target_block(200_u64)
        .build();
    let expected_load_options1 = build_default_expect_load_option();
    loader1
        .expect_load::<LoadOption>()
        .withf(move |options| load_options_compare(options, &expected_load_options1))
        .returning(move |_| Ok(load_status1));
    loader2
        .expect_load::<LoadOption>()
        .returning(move |_| Err(DataLoaderError::LoaderNoContractsError));
    let synchronizer = create_synchronizer(1, vec![loader1, loader2]).await;
    let mut sync_options = SyncOptions::builder().build();
    sync_options.chain_ids = vec![1_u64, 2_u64];
    let result = synchronizer.sync(sync_options).await;
    assert!(matches!(result.err().unwrap(), SynchronizerError::DataLoaderError(_)));
}

#[tokio::test]
async fn test_chain_synced_with_packer_disabled_options() {
    let chain_id = 2_u64;
    let load_status = LoadStatus::builder()
        .chain_id(chain_id)
        .loaded_block(100_u64)
        .target_block(200_u64)
        .build();
    for status in [true, false] {
        let mut loader = MockSyncDataLoader::new();
        let mut expected_load_options = build_default_expect_load_option();
        expected_load_options
            .fetcher
            .skips
            .entry(PACKER_FETCHER_NAME.to_string())
            .and_modify(|skip_option| {
                skip_option.skip_fetch = Some(status);
                skip_option.skip_validation = Some(!status);
            });
        loader
            .expect_load::<LoadOption>()
            .withf(move |options| load_options_compare(options, &expected_load_options))
            .returning(move |_| Ok(load_status));
        let synchronizer = create_synchronizer(chain_id, vec![loader]).await;
        let mut sync_options = SyncOptions::builder().build();
        sync_options.disable_datapacker_fetcher = Some(status);
        sync_options.enable_datapacker_fetcher_validate = Some(status);
        let result = synchronizer.sync(sync_options).await.unwrap();
        assert_eq!(result.chains.len(), 1);
        assert_eq!(
            result.chains[0],
            ChainStatus::builder()
                .chain_id(chain_id)
                .synced_block(100_u64)
                .target_block(200_u64)
                .build()
        );
    }
}

#[tokio::test]
async fn test_chain_synced_with_sequencer_disabled_options() {
    let chain_id = 1_u64;
    let load_status = LoadStatus::builder()
        .chain_id(chain_id)
        .loaded_block(100_u64)
        .target_block(200_u64)
        .build();
    for status in [true, false] {
        let mut loader = MockSyncDataLoader::new();
        let mut expected_load_options = build_default_expect_load_option();
        expected_load_options
            .fetcher
            .skips
            .entry(SEQUENCER_FETCHER_NAME.to_string())
            .and_modify(|skip_option| {
                skip_option.skip_fetch = Some(status);
                skip_option.skip_validation = Some(!status);
            });
        loader
            .expect_load::<LoadOption>()
            .withf(move |options| load_options_compare(options, &expected_load_options))
            .returning(move |_| Ok(load_status));
        let synchronizer = create_synchronizer(chain_id, vec![loader]).await;
        let mut sync_options = SyncOptions::builder().build();
        sync_options.disable_sequencer_fetcher = Some(status);
        sync_options.enable_sequencer_fetcher_validate = Some(status);
        let result = synchronizer.sync(sync_options).await.unwrap();
        assert_eq!(result.chains.len(), 1);
        assert_eq!(
            result.chains[0],
            ChainStatus::builder()
                .chain_id(chain_id)
                .synced_block(100_u64)
                .target_block(200_u64)
                .build()
        );
    }
}

#[tokio::test]
async fn test_chain_synced_with_provider_disabled_options() {
    let chain_id = 1_u64;
    let load_status = LoadStatus::builder()
        .chain_id(chain_id)
        .loaded_block(100_u64)
        .target_block(200_u64)
        .build();
    for status in [true, false] {
        let mut loader = MockSyncDataLoader::new();
        let mut expected_load_options = build_default_expect_load_option();
        expected_load_options
            .fetcher
            .skips
            .entry(PROVIDER_FETCHER_NAME.to_string())
            .and_modify(|skip_option| {
                skip_option.skip_fetch = Some(status);
                skip_option.skip_validation = Some(status);
            });
        loader
            .expect_load::<LoadOption>()
            .withf(move |options| load_options_compare(options, &expected_load_options))
            .returning(move |_| Ok(load_status));
        let synchronizer = create_synchronizer(chain_id, vec![loader]).await;
        let mut sync_options = SyncOptions::builder().build();
        sync_options.disable_provider_fetcher = Some(status);
        sync_options.disable_provider_fetcher_validate = Some(status);
        let result = synchronizer.sync(sync_options).await.unwrap();
        assert_eq!(result.chains.len(), 1);
        assert_eq!(
            result.chains[0],
            ChainStatus::builder()
                .chain_id(chain_id)
                .synced_block(100_u64)
                .target_block(200_u64)
                .build()
        );
    }
}

#[tokio::test]
async fn test_chain_synced_with_rule_validator_disabled_options() {
    let chain_id = 1_u64;
    let load_status = LoadStatus::builder()
        .chain_id(chain_id)
        .loaded_block(100_u64)
        .target_block(200_u64)
        .build();
    for status in [true, false] {
        let mut loader = MockSyncDataLoader::new();
        let mut expected_load_options = build_default_expect_load_option();
        expected_load_options
            .validator
            .skips
            .entry(RULE_VALIDATOR_NAME.to_string())
            .and_modify(|skip_option| {
                skip_option.skip_validation = Some(status);
            });
        loader
            .expect_load::<LoadOption>()
            .withf(move |options| load_options_compare(options, &expected_load_options))
            .returning(move |_| Ok(load_status));
        let synchronizer = create_synchronizer(chain_id, vec![loader]).await;
        let mut sync_options = SyncOptions::builder().build();
        sync_options.disable_rule_validator = Some(status);
        let result = synchronizer.sync(sync_options).await.unwrap();
        assert_eq!(result.chains.len(), 1);
        assert_eq!(
            result.chains[0],
            ChainStatus::builder()
                .chain_id(chain_id)
                .synced_block(100_u64)
                .target_block(200_u64)
                .build()
        );
    }
}

#[tokio::test]
async fn test_chain_synced_with_checker_disabled_options() {
    let chain_id = 1_u64;
    let load_status = LoadStatus::builder()
        .chain_id(chain_id)
        .loaded_block(100_u64)
        .target_block(200_u64)
        .build();
    for status in [true, false] {
        let mut loader = MockSyncDataLoader::new();
        let mut expected_load_options = build_default_expect_load_option();
        expected_load_options
            .validator
            .skips
            .entry(RULE_VALIDATOR_NAME.to_string())
            .and_modify(|skip_option| {
                skip_option
                    .skip_checkers
                    .insert(RULE_COUNTER_CHECKER_NAME.to_string(), status);
                skip_option
                    .skip_checkers
                    .insert(RULE_SEQUENCE_CHECKER_NAME.to_string(), status);
                skip_option
                    .skip_checkers
                    .insert(RULE_INTEGRITY_CHECKER_NAME.to_string(), status);
                skip_option
                    .skip_checkers
                    .insert(RULE_MERKLE_TREE_CHECKER_NAME.to_string(), status);
            });
        loader
            .expect_load::<LoadOption>()
            .withf(move |options| load_options_compare(options, &expected_load_options))
            .returning(move |_| Ok(load_status));
        let synchronizer = create_synchronizer(chain_id, vec![loader]).await;
        let mut sync_options = SyncOptions::builder().build();
        sync_options.disable_rule_validator_counter_check = Some(status);
        sync_options.disable_rule_validator_sequence_check = Some(status);
        sync_options.disable_rule_validator_integrity_check = Some(status);
        sync_options.disable_rule_validator_tree_check = Some(status);
        let result = synchronizer.sync(sync_options).await.unwrap();
        assert_eq!(result.chains.len(), 1);
        assert_eq!(
            result.chains[0],
            ChainStatus::builder()
                .chain_id(chain_id)
                .synced_block(100_u64)
                .target_block(200_u64)
                .build()
        );
    }
}

fn load_options_compare(a: &LoadOption, b: &LoadOption) -> bool {
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
    skips.insert(
        PACKER_FETCHER_NAME.to_string(),
        LoadFetcherSkipOption::builder().build(),
    );
    skips.insert(
        SEQUENCER_FETCHER_NAME.to_string(),
        LoadFetcherSkipOption::builder().build(),
    );
    skips.insert(
        PROVIDER_FETCHER_NAME.to_string(),
        LoadFetcherSkipOption::builder().build(),
    );
    LoadFetcherOption::builder().skips(skips).build()
}

fn build_default_expect_load_validator_option() -> LoadValidatorOption {
    let skip_checkers = HashMap::new();
    let mut skips = HashMap::new();
    skips.insert(
        RULE_VALIDATOR_NAME.to_string(),
        LoadValidatorSkipOption::builder().skip_checkers(skip_checkers).build(),
    );
    LoadValidatorOption::builder().skips(skips).build()
}
