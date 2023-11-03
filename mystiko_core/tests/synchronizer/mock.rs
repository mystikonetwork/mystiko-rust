use async_trait::async_trait;
use mockall::mock;
use mystiko_config::MystikoConfig;
use mystiko_core::Synchronizer;
use mystiko_dataloader::loader::LoadOption;
use mystiko_dataloader::loader::{DataLoader, DataLoaderResult};
use std::collections::HashMap;
use std::sync::Arc;

mock! {
    #[derive(Debug)]
    pub SyncDataLoader
    {}

    #[async_trait]
    impl DataLoader for SyncDataLoader {
        async fn chain_loaded_block(&self, chain_id: u64) -> DataLoaderResult<Option<u64>>;
        async fn contract_loaded_block(&self, chain_id: u64, contract_address: &str) -> DataLoaderResult<Option<u64>>;
        async fn load<O>(&self, options: O) -> DataLoaderResult<()>
        where
            O: Into<LoadOption> + Send + Sync + 'static;
    }
}

pub async fn create_synchronizer(chain_id: u64, loaders: Vec<MockSyncDataLoader>) -> Synchronizer<MockSyncDataLoader> {
    let config = Arc::new(
        MystikoConfig::from_json_file("tests/files/mystiko/config.json")
            .await
            .unwrap(),
    );
    let loaders: HashMap<u64, MockSyncDataLoader> = loaders
        .into_iter()
        .enumerate()
        .map(|(index, loader)| ((index as u64) + chain_id, loader))
        .collect();
    Synchronizer::builder().mystiko_config(config).loaders(loaders).build()
}
