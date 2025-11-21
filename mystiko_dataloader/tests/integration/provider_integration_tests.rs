use log::LevelFilter;
use mystiko_config::MystikoConfig;
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::fetcher::create_provider_pool_from_config;
use mystiko_dataloader::fetcher::{ContractFetchOptions, DataFetcher, FetchOptions, ProviderFetcher};
use mystiko_protos::loader::v1::ProviderFetcherConfig;
use std::sync::Arc;

#[tokio::test]
#[ignore] // Igore by default as it requires real network access
async fn test_provider_fetch_from_real_provider() {
    let _ = env_logger::builder()
        .filter_module("mystiko_dataloader", LevelFilter::Debug)
        .try_init();

    // Load mystiko config
    let mystiko_config = Arc::new(
        MystikoConfig::from_json_file("./tests/integration/mainnet.json")
            .await
            .unwrap(),
    );

    // Create provider pool from config (uses real BSC providers from config)
    let provider_fetcher_config = ProviderFetcherConfig::builder().build();
    let provider_pool = create_provider_pool_from_config(provider_fetcher_config, mystiko_config.clone());

    // Create provider fetcher
    let provider_fetcher: ProviderFetcher<FullData, _> =
        ProviderFetcher::from_config(ProviderFetcherConfig::builder().build(), Arc::new(provider_pool));

    // BSC chain ID
    let bsc_chain_id = 56u64;

    // Use a real BSC contract address from config
    let test_address = "0x16C1Cb42D59A4fe1facbe6F0F42956Db103bb6A9";

    // Use a reasonable block range (recent blocks to ensure data exists)
    // These blocks should have some activity
    let test_start_block: u64 = 67774675;
    let test_end_block: u64 = 68841642;

    // Get contract config from mystiko config
    let contract_config = mystiko_config
        .find_contract_by_address(bsc_chain_id, test_address)
        .expect("Contract should exist in config");

    // Create fetch options
    let contract_fetch_option = vec![ContractFetchOptions::builder()
        .contract_config(contract_config.clone())
        .start_block(test_start_block)
        .target_block(test_end_block)
        .build()];

    let fetch_options = FetchOptions::builder()
        .chain_id(bsc_chain_id)
        .start_block(test_start_block)
        .target_block(test_end_block)
        .config(Arc::clone(&mystiko_config))
        .contract_options(Some(contract_fetch_option))
        .build();

    // Fetch data from real BSC provider
    let result = provider_fetcher.fetch(&fetch_options).await;

    // Assert that fetch was successful
    assert!(result.is_ok(), "Fetch should succeed: {:?}", result.err());

    let chain_result = result.unwrap();
    assert_eq!(chain_result.chain_id, bsc_chain_id);
    assert_eq!(chain_result.contract_results.len(), 1);
    assert_eq!(chain_result.contract_results[0].address, test_address);

    // Assert that contract result is ok
    assert!(
        chain_result.contract_results[0].result.is_ok(),
        "Contract result should be ok: {:?}",
        chain_result.contract_results[0].result.as_ref().err()
    );

    let contract_data = chain_result.contract_results[0].result.as_ref().unwrap();
    assert_eq!(contract_data.address, test_address);
    assert!(contract_data.data.is_some());

    let data = contract_data.data.as_ref().unwrap();
    println!(
        "Fetched {} commitments and {} nullifiers from BSC",
        data.commitments.len(),
        data.nullifiers.len()
    );
    assert!(data.commitments.len() > 100000);
}
