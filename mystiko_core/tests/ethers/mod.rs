mod asset;
mod deposit;
mod pool;
mod provider_tests;
mod transaction;

use crate::common::MockProviders;
use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, TxHash};
use ethers_providers::ProviderError;
use mystiko_core::TransactionSigner;
use mystiko_ethers::{JsonRpcClientWrapper, JsonRpcParams, Provider, ProviderWrapper};
use std::collections::HashMap;
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

#[async_trait]
impl TransactionSigner for TimeoutProvider {
    async fn address(&self) -> anyhow::Result<Address> {
        tokio::time::sleep(tokio::time::Duration::from_millis(self.timeout_ms)).await;
        Ok(Address::zero())
    }

    async fn send_transaction(&self, _chain_id: u64, _tx: TypedTransaction) -> anyhow::Result<TxHash> {
        tokio::time::sleep(tokio::time::Duration::from_millis(self.timeout_ms)).await;
        Ok(TxHash::zero())
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

fn mock_providers<P>(providers: HashMap<u64, P>) -> MockProviders
where
    P: JsonRpcClientWrapper + 'static,
{
    let raw_providers = providers
        .into_iter()
        .map(|(chain_id, provider)| {
            let provider = Arc::new(Provider::new(ProviderWrapper::new(Box::new(provider))));
            (chain_id, provider)
        })
        .collect::<HashMap<_, _>>();
    let mut providers = MockProviders::new();
    providers.expect_get_provider().returning(move |chain_id| {
        raw_providers
            .get(&chain_id)
            .cloned()
            .ok_or(anyhow::anyhow!("No provider for chain_id {}", chain_id))
    });
    providers
}
