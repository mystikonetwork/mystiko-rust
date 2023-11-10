mod asset;
mod deposit;
mod provider_tests;
mod transaction;

use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, TxHash};
use ethers_providers::ProviderError;
use mystiko_core::TransactionSigner;
use mystiko_ethers::{JsonRpcClientWrapper, JsonRpcParams};
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
