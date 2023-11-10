use crate::common::MockProviders;
use crate::ethers::TimeoutProvider;
use async_trait::async_trait;
use ethers_core::abi::AbiDecode;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{TxHash, H256, U64};
use ethers_providers::{Middleware, ProviderError};
use mystiko_core::{TransactionHandler, Transactions, WaitOptions};
use mystiko_ethers::{JsonRpcClientWrapper, JsonRpcParams, Provider, ProviderWrapper};
use mystiko_protos::core::v1::transaction::Transaction as TransactionEnum;
use mystiko_protos::core::v1::{Eip1559Transaction, Eip2930Transaction, LegacyTransaction, Transaction};
use mystiko_types::TransactionType;
use mystiko_utils::address::ethers_address_to_string;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;
use typed_builder::TypedBuilder;

#[test]
fn test_create_default_transaction() {
    let transactions = Transactions::<MockProviders>::builder()
        .providers(MockProviders::new())
        .build();
    let legacy_tx = transactions.create(None, &TransactionType::Legacy).unwrap();
    let eip1559_tx = transactions.create(None, &TransactionType::Eip1559).unwrap();
    let eip2930_tx = transactions.create(None, &TransactionType::Eip2930).unwrap();
    match legacy_tx {
        TypedTransaction::Legacy(_) => {}
        _ => panic!("Expected legacy transaction"),
    }
    match eip1559_tx {
        TypedTransaction::Eip1559(_) => {}
        _ => panic!("Expected EIP-1559 transaction"),
    }
    match eip2930_tx {
        TypedTransaction::Eip2930(_) => {}
        _ => panic!("Expected EIP-2930 transaction"),
    }
}

#[test]
fn test_create_transaction_from_option() {
    let transactions = Transactions::<MockProviders>::builder()
        .providers(MockProviders::new())
        .build();
    let legacy_raw_tx = Transaction::builder()
        .transaction(TransactionEnum::LegacyTransaction(
            LegacyTransaction::builder()
                .from("0x125E577F580857D7AF995D20104C6c7B96a3274d".to_string())
                .build(),
        ))
        .build();
    let eip1559_raw_tx = Transaction::builder()
        .transaction(TransactionEnum::Eip1559Transaction(
            Eip1559Transaction::builder()
                .from("0x125E577F580857D7AF995D20104C6c7B96a3274d".to_string())
                .build(),
        ))
        .build();
    let eip2930_raw_tx = Transaction::builder()
        .transaction(TransactionEnum::Eip2930Transaction(
            Eip2930Transaction::builder()
                .tx(LegacyTransaction::builder()
                    .from("0x125E577F580857D7AF995D20104C6c7B96a3274d".to_string())
                    .build())
                .build(),
        ))
        .build();
    let legacy_tx = transactions
        .create(Some(legacy_raw_tx), &TransactionType::Legacy)
        .unwrap();
    let eip1559_tx = transactions
        .create(Some(eip1559_raw_tx), &TransactionType::Eip1559)
        .unwrap();
    let eip2930_tx = transactions
        .create(Some(eip2930_raw_tx), &TransactionType::Eip2930)
        .unwrap();
    match legacy_tx {
        TypedTransaction::Legacy(tx) => assert_eq!(
            ethers_address_to_string(&tx.from.unwrap()),
            "0x125E577F580857D7AF995D20104C6c7B96a3274d"
        ),
        _ => panic!("Expected legacy transaction"),
    }
    match eip1559_tx {
        TypedTransaction::Eip1559(tx) => assert_eq!(
            ethers_address_to_string(&tx.from.unwrap()),
            "0x125E577F580857D7AF995D20104C6c7B96a3274d"
        ),
        _ => panic!("Expected EIP-1559 transaction"),
    }
    match eip2930_tx {
        TypedTransaction::Eip2930(tx) => assert_eq!(
            ethers_address_to_string(&tx.tx.from.unwrap()),
            "0x125E577F580857D7AF995D20104C6c7B96a3274d"
        ),
        _ => panic!("Expected EIP-2930 transaction"),
    }
}

#[test]
fn test_create_transaction_with_wrong_type() {
    let transactions = Transactions::<MockProviders>::builder()
        .providers(MockProviders::new())
        .build();
    let legacy_raw_tx = Transaction::builder()
        .transaction(TransactionEnum::LegacyTransaction(
            LegacyTransaction::builder()
                .from("0x125E577F580857D7AF995D20104C6c7B96a3274d".to_string())
                .build(),
        ))
        .build();
    let eip1559_raw_tx = Transaction::builder()
        .transaction(TransactionEnum::Eip1559Transaction(
            Eip1559Transaction::builder()
                .from("0x125E577F580857D7AF995D20104C6c7B96a3274d".to_string())
                .build(),
        ))
        .build();
    let eip2930_raw_tx = Transaction::builder()
        .transaction(TransactionEnum::Eip2930Transaction(
            Eip2930Transaction::builder()
                .tx(LegacyTransaction::builder()
                    .from("0x125E577F580857D7AF995D20104C6c7B96a3274d".to_string())
                    .build())
                .build(),
        ))
        .build();
    let eip2930_tx = transactions
        .create(Some(legacy_raw_tx), &TransactionType::Eip2930)
        .unwrap();
    let legacy_tx = transactions
        .create(Some(eip1559_raw_tx), &TransactionType::Legacy)
        .unwrap();
    let eip1559_tx = transactions
        .create(Some(eip2930_raw_tx), &TransactionType::Eip1559)
        .unwrap();
    match eip2930_tx {
        TypedTransaction::Eip2930(tx) => assert!(tx.tx.from.is_none()),
        _ => panic!("Expected legacy transaction"),
    }
    match legacy_tx {
        TypedTransaction::Legacy(tx) => assert!(tx.from.is_none()),
        _ => panic!("Expected EIP-1559 transaction"),
    }
    match eip1559_tx {
        TypedTransaction::Eip1559(tx) => assert!(tx.from.is_none()),
        _ => panic!("Expected EIP-2930 transaction"),
    }
}

#[tokio::test]
async fn test_wait_transaction() {
    test_wait_transaction_inner(None).await
}

#[tokio::test]
async fn test_wait_transaction_with_confirmations() {
    test_wait_transaction_inner(Some(10)).await
}

#[tokio::test]
async fn test_wait_transaction_timeout_error() {
    let provider = TimeoutProvider::builder().timeout_ms(1000_u64).build();
    let provider = Arc::new(Provider::new(ProviderWrapper::new(Box::new(provider))));
    let mut providers = MockProviders::new();
    providers
        .expect_get_provider()
        .withf(|chain_id| *chain_id == 97_u64)
        .returning(move |_| Ok(provider.clone()));
    let transactions = Transactions::<MockProviders>::builder().providers(providers).build();
    let tx_hash = TxHash::decode_hex("0x0fe452459a23f379e4d40f154e1d8d58ab9b92d72b02939830dbfe3ecacc529d").unwrap();
    let wait_options = WaitOptions::builder()
        .chain_id(97_u64)
        .tx_hash(tx_hash)
        .timeout_ms(1_u64)
        .build();
    let result = transactions.wait(wait_options).await;
    assert_eq!(
        result.unwrap_err().to_string(),
        "wait transaction for confirmations timed out after 1 ms"
    );
}

async fn test_wait_transaction_inner(confirmations: Option<u64>) {
    let tx_hash = TxHash::decode_hex("0x0fe452459a23f379e4d40f154e1d8d58ab9b92d72b02939830dbfe3ecacc529d").unwrap();
    let block_hash = H256::decode_hex("0xa5b3c0f4869f2c725ec5f6a8facb225cd359fbcd02c77a7abec03565c14280f0").unwrap();
    let provider = MockProvider::builder()
        .expected_tx_hash(tx_hash)
        .expected_block_hash(block_hash)
        .confirmed_block(100010)
        .current_block(Mutex::new(100000))
        .build();
    let provider = Arc::new(Provider::new(ProviderWrapper::new(Box::new(provider))));
    let provider_clone = provider.clone();
    let mut providers = MockProviders::new();
    providers
        .expect_get_provider()
        .withf(|chain_id| *chain_id == 97_u64)
        .returning(move |_| Ok(provider.clone()));
    let transactions = Transactions::<MockProviders>::builder().providers(providers).build();
    let wait_options = WaitOptions::builder()
        .chain_id(97_u64)
        .tx_hash(tx_hash)
        .interval_ms(1)
        .confirmations(confirmations)
        .build();
    let receipt = transactions.wait(wait_options).await.unwrap().unwrap();
    assert_eq!(receipt.transaction_hash, tx_hash);
    assert_eq!(receipt.block_number.unwrap(), U64::from(100010));
    if let Some(confirmations) = confirmations {
        let current_block = provider_clone.get_block_number().await.unwrap().as_u64();
        assert!(current_block >= 100010 + confirmations);
    }
}

#[derive(Debug, TypedBuilder)]
struct MockProvider {
    expected_tx_hash: TxHash,
    expected_block_hash: H256,
    confirmed_block: u64,
    #[builder(default)]
    current_block: Mutex<u64>,
}

#[async_trait]
impl JsonRpcClientWrapper for MockProvider {
    async fn request(&self, method: &str, _params: JsonRpcParams) -> Result<Value, ProviderError> {
        let mut current_block = self.current_block.lock().await;
        *current_block += 1;
        let current_block = *current_block;
        if method == "eth_blockNumber" {
            Ok(serde_json::json!(U64::from(current_block)))
        } else if method == "eth_getTransactionByHash" {
            let tx = if current_block >= self.confirmed_block {
                ethers_core::types::Transaction {
                    hash: self.expected_tx_hash,
                    block_number: Some(U64::from(self.confirmed_block)),
                    block_hash: Some(self.expected_block_hash),
                    transaction_index: Some(U64::zero()),
                    ..Default::default()
                }
            } else {
                ethers_core::types::Transaction {
                    hash: self.expected_tx_hash,
                    ..Default::default()
                }
            };
            Ok(serde_json::json!(tx))
        } else if method == "eth_getTransactionReceipt" {
            if current_block >= self.confirmed_block {
                let receipt = ethers_core::types::TransactionReceipt {
                    transaction_hash: self.expected_tx_hash,
                    transaction_index: U64::zero(),
                    block_number: Some(U64::from(self.confirmed_block)),
                    block_hash: Some(self.expected_block_hash),
                    ..Default::default()
                };
                Ok(serde_json::json!(receipt))
            } else {
                Ok(Value::Null)
            }
        } else {
            panic!("Unexpected method: {}", method);
        }
    }
}
