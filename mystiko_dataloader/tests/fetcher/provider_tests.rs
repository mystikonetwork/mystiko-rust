use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::U64;
use ethers_providers::MockResponse;
use ethers_providers::{MockError, MockProvider, RetryClientBuilder, RetryPolicy};
use log::LevelFilter;
use mystiko_config::MystikoConfig;
use mystiko_dataloader::data::{FullData, LiteData};
use mystiko_dataloader::fetcher::{
    ContractFetchOptions, DataFetcher, FetchOptions, ProviderFetcher, ProviderFetcherError,
};
use mystiko_ethers::{FailoverProvider, Provider, ProviderWrapper, Providers};
use std::sync::Arc;
use tokio::time::Duration;

#[derive(Debug, Default)]
pub struct TestProvders;

#[async_trait]
impl Providers for TestProvders {
    async fn get_provider(&self, chain_id: u64) -> Result<Arc<Provider>> {
        let test_provider = MockProvider::default();
        if chain_id == 137 {
            Ok(Arc::new(mock_chain_137(test_provider)))
        } else {
            Ok(Arc::new(mock_chain_56(test_provider)))
        }
    }
    async fn has_provider(&self, _chain_id: u64) -> bool {
        true
    }

    async fn set_provider(&self, _chain_id: u64, _provider: Arc<Provider>) -> Option<Arc<Provider>> {
        None
    }

    async fn delete_provider(&self, _chain_id: u64) -> Option<Arc<Provider>> {
        None
    }
}

#[tokio::test]
async fn test_fulldata_fetch() {
    let _ = env_logger::builder()
        .filter_module("mystiko_dataloader", LevelFilter::Debug)
        .try_init();
    let provider_fetcher: ProviderFetcher<FullData, TestProvders> = Arc::new(TestProvders).into();
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
    assert_eq!(data.commitments.len(), 3);
    assert_eq!(data.commitments[0].block_number, 7969470);
    assert_eq!(data.nullifiers.len(), 1);
    assert_eq!(data.nullifiers[0].block_number, 45565081);
}

#[tokio::test]
async fn test_litedata_fetch() {
    let _ = env_logger::builder()
        .filter_module("mystiko_dataloader", LevelFilter::Debug)
        .try_init();
    let provider_fetcher: ProviderFetcher<LiteData, TestProvders> = Arc::new(TestProvders).into();
    let mystiko_config = Arc::new(
        MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
            .await
            .unwrap(),
    );
    let test_chain_id = 56;
    let test_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let test_start_block: u64 = 45564268;
    let test_end_block: u64 = 45565260;
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
    assert_eq!(result.end_block, 45565260);
    assert_eq!(result.address, test_address);
    assert!(result.data.is_some());
    let data = result.data.as_ref().unwrap();
    assert_eq!(data.commitments.len(), 3);
    assert_eq!(data.commitments[0].block_number, 7969470);
    // test no contract_options
    let mystiko_config = Arc::new(
        MystikoConfig::from_json_file("./tests/files/fetcher/config.json")
            .await
            .unwrap(),
    );
    let fetch_options = FetchOptions::builder()
        .chain_id(111_u64)
        .start_block(test_start_block)
        .target_block(test_end_block)
        .config(Arc::clone(&mystiko_config))
        .build();
    let result = provider_fetcher.fetch(&fetch_options).await;
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap().to_string(),
        ProviderFetcherError::UnsupportedChainError(111_u64).to_string()
    );
    let fetch_options = FetchOptions::builder()
        .chain_id(test_chain_id)
        .start_block(test_start_block)
        .target_block(test_end_block)
        .config(Arc::clone(&mystiko_config))
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
    assert_eq!(result.end_block, 45565260);
    assert_eq!(result.address, test_address);
    assert!(result.data.is_some());
    let data = result.data.as_ref().unwrap();
    assert_eq!(data.commitments.len(), 3);
    assert_eq!(data.commitments[0].block_number, 7969470);
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

fn mock_chain_137(test_provider: MockProvider) -> Provider {
    let mock_cross_chain_data = serde_json::json!([
      {
        "address": "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411",
        "blockHash": "0x608ed8a9b485fa71027503bf79a52c15417659caed2432d57f64b5615cc26d6e",
        "blockNumber": "0x799abe",
        "data": "0x",
        "logIndex": "0xff",
        "removed": false,
        "topics": [
          "0xd106eb38b3368b7c294e36fae5513fdefe880be5abfad529b37b044f2fdd2dbe",
          "0x21c43ba17de66454ef89a3aea71a046d39d3837696780502d6f017f0c16e206a"
          ],
        "transactionHash": "0xacebb4356bcb0d1c2763909586c2a396c79b6dd7951d7b9fc81144353043e6d8",
        "transactionIndex": "0x71"
      }
    ]);
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
    create_mock_provider(&test_provider)
}

fn mock_chain_56(test_provider: MockProvider) -> Provider {
    let mock_cross_chain_data = serde_json::json!([
      {
        "address": "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411",
        "blockHash": "0x608ed8a9b485fa71027503bf79a52c15417659caed2432d57f64b5615cc26d6e",
        "blockNumber": "0x799abe",
        "data": "0x",
        "logIndex": "0xff",
        "removed": false,
        "topics": [
          "0xd106eb38b3368b7c294e36fae5513fdefe880be5abfad529b37b044f2fdd2dbe",
          "0x21c43ba17de66454ef89a3aea71a046d39d3837696780502d6f017f0c16e206a"
          ],
        "transactionHash": "0xacebb4356bcb0d1c2763909586c2a396c79b6dd7951d7b9fc81144353043e6d8",
        "transactionIndex": "0x71"
      }
    ]);
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
    create_mock_provider(&test_provider)
}

fn create_mock_provider(provider: &MockProvider) -> Provider {
    let retry_provider_builder = RetryClientBuilder::default();
    let inner_provider = retry_provider_builder.build(provider.clone(), Box::<MockProviderRetryPolicy>::default());
    let mut provider_builder = FailoverProvider::dyn_rpc();
    provider_builder = provider_builder.add_provider(Box::new(inner_provider));
    Provider::new(ProviderWrapper::new(Box::new(provider_builder.build())))
}
