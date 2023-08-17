use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::U64;
use ethers_providers::MockResponse;
use ethers_providers::{MockError, MockProvider, RetryClientBuilder, RetryPolicy};
use log::LevelFilter;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::data::{FullData, LiteData};
use mystiko_dataloader::fetcher::provider::ProviderFetcher;
use mystiko_dataloader::fetcher::{ContractFetchOptions, DataFetcher, FetchOptions};
use mystiko_ethers::provider::factory::Provider;
use mystiko_ethers::provider::failover::FailoverProvider;
use mystiko_ethers::provider::pool::Providers;
use mystiko_ethers::provider::wrapper::ProviderWrapper;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
pub struct TestProvders {}

#[async_trait]
impl Providers for TestProvders {
    fn get_provider(&self, chain_id: u64) -> Option<Arc<Provider>> {
        if chain_id == 137 {
            None
        } else {
            let test_provider = MockProvider::default();
            let mock_cross_chain_data = serde_json::json!([]);
            let mock_queued_data = serde_json::json!([
              {
                "address": "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411",
                "blockHash": "0x224ac34e68f04a2d134affb0bf9181bae2cc4e7376a60687c072119247fb0e78",
                "blockNumber": "0x2b741b8",
                "data": "0x00000000000000000000000000000000000000000000000000000000000186a000000000000000000000000000000000000000000000000000000000000003a6000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000d1d6cb2d1e247af8403e556506eaa594d9040928fb8abd9515bb60274f3be6a348411d1abcf50aac3d7f305d7b2839f5afd02c2808145dcfd680f8fe8e0cb016b865ebe3c9d8ae96151d231cd6651ebf4dd2eb1480256e6a8a5f866120b833527eedd324006c33071f2bc3888f61c2c7b400d13d40739eab565365f7c2b8063bf2f45474008c1ab2cea99b82fd62efadba6a6d2128b5934fe89240f2a521153394a8577912130dae64642e7090a345afc5a8d43b6b864db2845d00d906a9b2d69f4a0bd43e6744cf1a3c1432bc607316b09a000000000000000000000000000000",
                "logIndex": "0xff",
                "removed": false,
                "topics": [
                  "0xf533f9705aac5020e21695ea3553ac7b6881070d2b6900ab2b1e3050304b5bf9",
                  "0x2020219d4d0c0f45583ca9b2db6f58e5b4e3f66a7a42b871bdfafc16f1416341"
                  ],
                "transactionHash": "0xa5832c0a90837280d29de8498144c40c295fbf4adae7efc97046c322cb81c1c2",
                "transactionIndex": "0x33"
              },
            ]);
            let mock_included_data = serde_json::json!([
              {
                "address": "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411",
                "blockHash": "0xb78ce396b7308d07c79b31d1a9b26b82f474c3e9619af4b35881fadfa5b0178a",
                "blockNumber": "0x2b741e8",
                "data": "0x",
                "logIndex": "0xff",
                "removed": false,
                "topics": [
                  "0xfe6b097b46a78e08506a3143b6337c2505ba77df76fe05c3663a987395d63413",
                  "0x23ebf431a5c6a79e85b8ce20e707761139298ae9a991e1a5ac52441f4ad4596c"
                  ],
                "transactionHash": "0xb595eaad5454ca2b761667424959fc77abdd79b8d10292c3b9d83560def1da24",
                "transactionIndex": "0x25"
              },
            ]);
            let mock_spent_data = serde_json::json!([
              {
                "address": "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411",
                "blockHash": "0xadb133a483e8898cc1164200c857089826bddfafddc789370eddd80195eb4f47",
                "blockNumber": "0x2b74499",
                "data": "0x",
                "logIndex": "0xe9",
                "removed": false,
                "topics": [
                  "0x3c2372ab6130817bd6b8fc6dbaecae947e84201b49535d358debaa6c34c23ecf",
                  "0x08ed20ef822fd552e5b9615f3f63cb367295173dfdb91eed7c8f323ef119a2b5",
                  "0x025f4e23a4774e4858a7f272c9a2b96ab4d901aa9bcbdc947c441c4fc9299ee4"
                  ],
                "transactionHash": "0x7e49489ef2cdeed41f538dae9468f5b8926586ae1803398452fac50d235823ea",
                "transactionIndex": "0x25"
              },
            ]);
            test_provider.push_response(MockResponse::Value(mock_spent_data.clone()));
            test_provider.push_response(MockResponse::Value(mock_included_data.clone()));
            test_provider.push_response(MockResponse::Value(mock_queued_data.clone()));
            test_provider.push_response(MockResponse::Value(mock_cross_chain_data.clone()));
            test_provider.push(U64::from(45565267)).unwrap();
            let provider = create_mock_provider(&test_provider);
            Some(Arc::new(provider))
        }
    }

    async fn get_or_create_provider(&mut self, chain_id: u64) -> Result<Arc<Provider>> {
        if chain_id != 137 {
            return Err(anyhow::anyhow!("test error"));
        }
        let test_provider = MockProvider::default();
        sleep(Duration::from_secs(1)).await;
        let mock_cross_chain_data = serde_json::json!([]);
        let mock_queued_data = serde_json::json!([
          {
            "address": "0xCB255075f38C75EAf2DE8A72897649dba9B90299",
            "blockHash": "0x224ac34e68f04a2d134affb0bf9181bae2cc4e7376a60687c072119247fb0e78",
            "blockNumber": "0x2b741b8",
            "data": "0x00000000000000000000000000000000000000000000000000000000000186a000000000000000000000000000000000000000000000000000000000000003a6000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000d1d6cb2d1e247af8403e556506eaa594d9040928fb8abd9515bb60274f3be6a348411d1abcf50aac3d7f305d7b2839f5afd02c2808145dcfd680f8fe8e0cb016b865ebe3c9d8ae96151d231cd6651ebf4dd2eb1480256e6a8a5f866120b833527eedd324006c33071f2bc3888f61c2c7b400d13d40739eab565365f7c2b8063bf2f45474008c1ab2cea99b82fd62efadba6a6d2128b5934fe89240f2a521153394a8577912130dae64642e7090a345afc5a8d43b6b864db2845d00d906a9b2d69f4a0bd43e6744cf1a3c1432bc607316b09a000000000000000000000000000000",
            "logIndex": "0xff",
            "removed": false,
            "topics": [
              "0xf533f9705aac5020e21695ea3553ac7b6881070d2b6900ab2b1e3050304b5bf9",
              "0x2020219d4d0c0f45583ca9b2db6f58e5b4e3f66a7a42b871bdfafc16f1416341"
              ],
            "transactionHash": "0xa5832c0a90837280d29de8498144c40c295fbf4adae7efc97046c322cb81c1c2",
            "transactionIndex": "0x33"
          },
        ]);
        let mock_included_data = serde_json::json!([
          {
            "address": "0xCB255075f38C75EAf2DE8A72897649dba9B90299",
            "blockHash": "0xb78ce396b7308d07c79b31d1a9b26b82f474c3e9619af4b35881fadfa5b0178a",
            "blockNumber": "0x2b741e8",
            "data": "0x",
            "logIndex": "0xff",
            "removed": false,
            "topics": [
              "0xfe6b097b46a78e08506a3143b6337c2505ba77df76fe05c3663a987395d63413",
              "0x23ebf431a5c6a79e85b8ce20e707761139298ae9a991e1a5ac52441f4ad4596c"
              ],
            "transactionHash": "0xb595eaad5454ca2b761667424959fc77abdd79b8d10292c3b9d83560def1da24",
            "transactionIndex": "0x25"
          },
        ]);
        let mock_spent_data = serde_json::json!([
          {
            "address": "0xCB255075f38C75EAf2DE8A72897649dba9B90299",
            "blockHash": "0xadb133a483e8898cc1164200c857089826bddfafddc789370eddd80195eb4f47",
            "blockNumber": "0x2b74499",
            "data": "0x",
            "logIndex": "0xe9",
            "removed": false,
            "topics": [
              "0x3c2372ab6130817bd6b8fc6dbaecae947e84201b49535d358debaa6c34c23ecf",
              "0x08ed20ef822fd552e5b9615f3f63cb367295173dfdb91eed7c8f323ef119a2b5",
              "0x025f4e23a4774e4858a7f272c9a2b96ab4d901aa9bcbdc947c441c4fc9299ee4"
              ],
            "transactionHash": "0x7e49489ef2cdeed41f538dae9468f5b8926586ae1803398452fac50d235823ea",
            "transactionIndex": "0x25"
          },
        ]);
        test_provider.push_response(MockResponse::Value(mock_spent_data.clone()));
        test_provider.push_response(MockResponse::Value(mock_included_data.clone()));
        test_provider.push_response(MockResponse::Value(mock_queued_data.clone()));
        test_provider.push_response(MockResponse::Value(mock_cross_chain_data.clone()));
        test_provider.push(U64::from(45565267)).unwrap();
        let provider = create_mock_provider(&test_provider);
        Ok(Arc::new(provider))
    }
}

#[tokio::test]
async fn test_fulldata_fetch() {
    let _ = env_logger::builder().filter_module("", LevelFilter::Debug).try_init();
    let provider_fetcher: ProviderFetcher<FullData, TestProvders> =
        ProviderFetcher::<FullData, TestProvders>::builder()
            .providers(RwLock::new(TestProvders {}))
            .build();
    let mystiko_config = Arc::new(
        MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
            .await
            .unwrap(),
    );
    let test_chain_id = 137;
    let test_address = "0xCB255075f38C75EAf2DE8A72897649dba9B90299";
    let test_start_block: u64 = 45564268;
    let test_end_block: u64 = 45565268;
    let contract_config = mystiko_config
        .find_contract_by_address(test_chain_id, test_address)
        .unwrap();
    let contract_fetch_option = vec![ContractFetchOptions::builder()
        .contract_config(contract_config.clone())
        .start_block(test_start_block)
        .target_block(test_end_block)
        .build()];
    let fetch_options = FetchOptions::builder()
        .chain_id(test_chain_id)
        .start_block(test_start_block)
        .target_block(test_end_block)
        .config(Arc::clone(&mystiko_config))
        .contract_options(Some(contract_fetch_option))
        .build();
    let result = provider_fetcher.fetch(&fetch_options).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.chain_id, test_chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, test_address);
    assert!(result.contract_results[0].result.is_ok());
    let result = result.contract_results[0].result.as_ref().unwrap();
    assert_eq!(result.start_block, test_start_block);
    assert_eq!(result.end_block, 45565267);
    assert!(result.data.is_some());
    let data = result.data.as_ref().unwrap();
    assert_eq!(data.commitments.len(), 2);
    assert_eq!(data.commitments[0].block_number, 45564344);
    assert_eq!(data.nullifiers.len(), 1);
    assert_eq!(data.nullifiers[0].block_number, 45565081);
}

#[tokio::test]
async fn test_litedata_fetch() {
    let _ = env_logger::builder().filter_module("", LevelFilter::Debug).try_init();
    let provider_fetcher: ProviderFetcher<LiteData, TestProvders> =
        ProviderFetcher::<LiteData, TestProvders>::builder()
            .providers(RwLock::new(TestProvders {}))
            .build();
    let mystiko_config = Arc::new(
        MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
            .await
            .unwrap(),
    );
    let test_chain_id = 56;
    let test_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let test_start_block: u64 = 45564268;
    let test_end_block: u64 = 45565268;
    let contract_config = mystiko_config
        .find_contract_by_address(test_chain_id, test_address)
        .unwrap();
    let contract_fetch_option = vec![ContractFetchOptions::builder()
        .contract_config(contract_config.clone())
        .start_block(test_start_block)
        .target_block(test_end_block)
        .build()];
    let fetch_options = FetchOptions::builder()
        .chain_id(test_chain_id)
        .start_block(test_start_block)
        .target_block(test_end_block)
        .config(Arc::clone(&mystiko_config))
        .contract_options(Some(contract_fetch_option))
        .build();
    let result = provider_fetcher.fetch(&fetch_options).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.chain_id, test_chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, test_address);
    assert!(result.contract_results[0].result.is_ok());
    let result = result.contract_results[0].result.as_ref().unwrap();
    assert_eq!(result.start_block, test_start_block);
    assert_eq!(result.end_block, 45565267);
    assert_eq!(result.address, test_address);
    assert!(result.data.is_some());
    let data = result.data.as_ref().unwrap();
    assert_eq!(data.commitments.len(), 2);
    assert_eq!(data.commitments[0].block_number, 45564344);
}

#[derive(Debug, Default)]
struct MockProviderRetryPolicy;

impl RetryPolicy<MockError> for MockProviderRetryPolicy {
    fn should_retry(&self, _error: &MockError) -> bool {
        false
    }

    fn backoff_hint(&self, _error: &MockError) -> Option<Duration> {
        Duration::from_secs(1).into()
    }
}

fn create_mock_provider(provider: &MockProvider) -> Provider {
    let retry_provider_builder = RetryClientBuilder::default();
    let inner_provider = retry_provider_builder.build(provider.clone(), Box::<MockProviderRetryPolicy>::default());
    let mut provider_builder = FailoverProvider::dyn_rpc();
    provider_builder = provider_builder.add_provider(Box::new(inner_provider));
    Provider::new(ProviderWrapper::new(Box::new(provider_builder.build())))
}
