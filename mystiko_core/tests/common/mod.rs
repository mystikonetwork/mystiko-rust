use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, TxHash};
use ethers_providers::ProviderError;
use mockall::mock;
use mystiko_core::Database;
use mystiko_ethers::{JsonRpcClientWrapper, JsonRpcParams, Provider};
use mystiko_storage::SqlStatementFormatter;
use mystiko_storage_sqlite::SqliteStorage;
use std::sync::Arc;

pub async fn create_database() -> Database<SqlStatementFormatter, SqliteStorage> {
    let _ = env_logger::builder()
        .filter_module("mystiko_core", log::LevelFilter::Info)
        .is_test(true)
        .try_init();
    let formatter = SqlStatementFormatter::sqlite();
    let storage = SqliteStorage::from_memory().await.unwrap();
    Database::new(formatter, storage)
}

mock! {
    #[derive(Debug)]
    pub Provider {}

    #[async_trait]
    impl JsonRpcClientWrapper for Provider {
         async fn request(
            &self,
            method: &str,
            params: JsonRpcParams,
        ) -> Result<serde_json::Value, ProviderError>;
    }
}

mock! {
    #[derive(Debug)]
    pub Providers {}

    #[async_trait]
    impl mystiko_ethers::Providers for Providers {
        async fn get_provider(&self, chain_id: u64) -> anyhow::Result<Arc<Provider>>;
        async fn has_provider(&self, chain_id: u64) -> bool;
        async fn set_provider(&self, chain_id: u64, provider: Arc<Provider>) -> Option<Arc<Provider>>;
        async fn delete_provider(&self, chain_id: u64) -> Option<Arc<Provider>>;
    }
}

mock! {
    pub TransactionSigner {}

    #[async_trait]
    impl mystiko_core::TransactionSigner for TransactionSigner {
        async fn address(&self) -> anyhow::Result<Address>;
        async fn send_transaction(&self, chain_id: u64, tx: TypedTransaction) -> anyhow::Result<TxHash>;
    }
}
