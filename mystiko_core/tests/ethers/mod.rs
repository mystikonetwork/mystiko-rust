mod asset;
mod deposit;
mod transaction;

use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, TxHash};
use ethers_providers::ProviderError;
use mockall::mock;
use mystiko_ethers::{JsonRpcClientWrapper, JsonRpcParams, Provider};
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
struct TimeoutProvider {
    timeout_ms: u64,
}

#[async_trait]
impl JsonRpcClientWrapper for TimeoutProvider {
    async fn request(&self, _method: &str, _params: JsonRpcParams) -> Result<serde_json::Value, ProviderError> {
        tokio::time::sleep(tokio::time::Duration::from_millis(self.timeout_ms)).await;
        Ok(serde_json::Value::Null)
    }
}

fn parse_call_args(params: &JsonRpcParams) -> TypedTransaction {
    if let JsonRpcParams::Value(value) = params {
        let mut values: Vec<serde_json::Value> = serde_json::from_value(value.clone()).unwrap();
        let tx = values.remove(0);
        serde_json::from_value(tx).unwrap()
    } else {
        panic!("Invalid params");
    }
}

mock! {
    #[derive(Debug)]
    Provider {}

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
    Providers {}

    #[async_trait]
    impl mystiko_ethers::Providers for Providers {
        async fn get_provider(&self, chain_id: u64) -> anyhow::Result<Arc<Provider>>;
        async fn has_provider(&self, chain_id: u64) -> bool;
        async fn set_provider(&self, chain_id: u64, provider: Arc<Provider>) -> Option<Arc<Provider>>;
        async fn delete_provider(&self, chain_id: u64) -> Option<Arc<Provider>>;
    }
}

mock! {
    TransactionSigner {}

    #[async_trait]
    impl mystiko_core::TransactionSigner for TransactionSigner {
        async fn address(&self) -> anyhow::Result<Address>;
        async fn send_transaction(&self, chain_id: u64, tx: TypedTransaction) -> anyhow::Result<TxHash>;
    }
}
