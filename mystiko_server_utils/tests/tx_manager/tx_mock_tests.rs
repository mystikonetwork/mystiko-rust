extern crate ethers_providers;
extern crate ethers_signers;
extern crate rand;
extern crate serde_json;
extern crate tokio;

use ethers_core::types::transaction::eip2930::AccessList;
use ethers_core::types::transaction::response::TransactionReceipt;
use ethers_core::types::{Address, Block, Bytes, FeeHistory, Transaction, H256, U256, U64};
use ethers_providers::Provider;
use ethers_signers::{LocalWallet, Signer};
use mystiko_server_utils::tx_manager::config::TxManagerConfig;
use mystiko_server_utils::tx_manager::error::TxManagerError;
use mystiko_server_utils::tx_manager::transaction::TxBuilder;
use serde_json::json;
use std::str::FromStr;

#[tokio::test]
async fn test_gas_price() {
    let (provider, mock) = Provider::mocked();
    let mut cfg = TxManagerConfig::new(None).unwrap();
    cfg.min_priority_fee_by_chain.insert("2000".to_string(), 4_000_000_123);
    let chain_id = 2000u64;
    let wallet = LocalWallet::new(&mut rand::thread_rng()).with_chain_id(chain_id);
    let builder = TxBuilder::builder()
        .config(cfg)
        .chain_id(chain_id)
        .wallet(wallet)
        .build();

    let block = get_1559_block();
    let history = get_fee_history();

    mock.push(history.clone()).unwrap();
    mock.push(block.clone()).unwrap();
    let tx = builder.build_tx(&provider).await;

    mock.push(history.clone()).unwrap();
    mock.push(block.clone()).unwrap();
    let gas_price = tx.gas_price(&provider).await.unwrap();
    assert_eq!(gas_price.to_string(), "7000000137");
}

#[tokio::test]
async fn test_send_1559_tx() {
    let (provider, mock) = Provider::mocked();

    let block = get_1559_block();
    let history = get_fee_history();
    let nonce = U256::from(100);
    let mut transaction = get_transaction();
    let transaction_receipt = get_transaction_receipt();
    let value = ethers_core::utils::parse_ether("1").unwrap();
    let max_gas_price = U256::from(100_000_000_000u64);
    let gas = U256::from(100_000_000_000u64);
    let tx_hash = H256::from_str("0x090b19818d9d087a49c3d2ecee4829ee4acea46089c1381ac5e588188627466d").unwrap();
    let chain_id = 2000u64;
    let wallet = LocalWallet::new(&mut rand::thread_rng()).with_chain_id(chain_id);
    let to_address = wallet.address();
    let mut cfg = TxManagerConfig::new(None).unwrap();
    cfg.confirm_interval_secs = 1;
    let builder = TxBuilder::builder()
        .config(cfg)
        .chain_id(chain_id)
        .wallet(wallet)
        .build();

    mock.push(history.clone()).unwrap();
    mock.push(block.clone()).unwrap();
    let tx = builder.build_tx(&provider).await;
    assert!(tx.is_1559_tx());

    mock.push(history.clone()).unwrap();
    mock.push(block.clone()).unwrap();
    let gas_price = tx.gas_price(&provider).await.unwrap();
    assert_eq!(gas_price.to_string(), "6000000014");

    mock.push(gas).unwrap();
    mock.push(nonce).unwrap();
    let gas = tx
        .estimate_gas(to_address, vec![].as_slice(), &value, &max_gas_price, &provider)
        .await
        .unwrap();
    assert_eq!(gas.to_string(), "100000000000");

    let transaction2 = transaction.clone();
    transaction.block_number = None;
    let block_number = U64::from(6203173);
    let block_number2 = U64::from(6203176);
    mock.push(transaction_receipt.clone()).unwrap();
    mock.push(block_number2).unwrap();
    mock.push(transaction2.clone()).unwrap();
    mock.push(block_number).unwrap();
    mock.push(transaction2.clone()).unwrap();
    mock.push(transaction.clone()).unwrap();
    mock.push(tx_hash).unwrap();
    mock.push(nonce).unwrap();
    mock.push(history.clone()).unwrap();
    mock.push(block.clone()).unwrap();
    let hash = tx
        .send(to_address, vec![].as_slice(), &value, &gas, &max_gas_price, &provider)
        .await
        .unwrap();
    assert_eq!(hash, tx_hash);
    let receipt = tx.confirm(&provider, hash).await.unwrap();
    assert_eq!(receipt, transaction_receipt);
}

#[tokio::test]
async fn test_send_legacy_tx() {
    let (provider, mock) = Provider::mocked();

    let nonce = U256::from(100);
    let price = U256::from(1000000);
    let transaction = get_transaction();
    let transaction_receipt = get_transaction_receipt();
    let value = ethers_core::utils::parse_ether("1").unwrap();
    let max_gas_price = U256::from(100_000_000_000u64);
    let gas = U256::from(100_000_000_000u64);
    let tx_hash = H256::from_str("0x090b19818d9d087a49c3d2ecee4829ee4acea46089c1381ac5e588188627466d").unwrap();
    let chain_id = 2000u64;
    let wallet = LocalWallet::new(&mut rand::thread_rng()).with_chain_id(chain_id);
    let to_address = wallet.address();
    let mut cfg = TxManagerConfig::new(None).unwrap();
    cfg.confirm_interval_secs = 1;
    let builder = TxBuilder::builder()
        .config(cfg)
        .chain_id(chain_id)
        .wallet(wallet)
        .build();

    let tx = builder.build_tx(&provider).await;
    assert!(!tx.is_1559_tx());

    mock.push(price).unwrap();
    let gas_price = tx.gas_price(&provider).await.unwrap();
    assert_eq!(gas_price.to_string(), "1000000");

    mock.push(gas).unwrap();
    mock.push(price).unwrap();
    let gas = tx
        .estimate_gas(to_address, vec![].as_slice(), &value, &max_gas_price, &provider)
        .await
        .unwrap();
    assert_eq!(gas.to_string(), "100000000000");

    let gas = U256::from(100_000_000_000u64);
    let block_number = U64::from(6203176);
    mock.push(transaction_receipt.clone()).unwrap();
    mock.push(block_number).unwrap();
    mock.push(transaction.clone()).unwrap();
    mock.push(tx_hash).unwrap();
    mock.push(nonce).unwrap();
    mock.push(price).unwrap();
    let hash = tx
        .send(to_address, vec![].as_slice(), &value, &gas, &max_gas_price, &provider)
        .await
        .unwrap();
    assert_eq!(hash, tx_hash);
    let receipt = tx.confirm(&provider, hash).await.unwrap();
    assert_eq!(receipt, transaction_receipt);
}

#[tokio::test]
async fn test_1559_tx_with_error() {
    let (provider, mock) = Provider::mocked();

    let block = get_1559_block();
    let history = get_fee_history();
    let nonce = U256::from(100);
    let chain_id = 2000u64;
    let value = ethers_core::utils::parse_ether("1").unwrap();

    let wallet = LocalWallet::new(&mut rand::thread_rng()).with_chain_id(chain_id);
    let to_address = wallet.address();
    let cfg = TxManagerConfig::new(None).unwrap();
    let builder = TxBuilder::builder()
        .config(cfg)
        .chain_id(chain_id)
        .wallet(wallet)
        .build();

    mock.push(history.clone()).unwrap();
    mock.push(block.clone()).unwrap();
    let tx = builder.build_tx(&provider).await;
    assert!(tx.is_1559_tx());

    let gas_price = tx.gas_price(&provider).await;
    assert!(matches!(gas_price.err().unwrap(), TxManagerError::GasPriceError(_)));

    let max_gas_price = U256::from(100_000_000_000u64);

    let gas = tx
        .estimate_gas(to_address, vec![].as_slice(), &value, &max_gas_price, &provider)
        .await;
    assert!(matches!(gas.err().unwrap(), TxManagerError::NonceError(_)));

    mock.push(nonce).unwrap();
    let gas = tx
        .estimate_gas(to_address, vec![].as_slice(), &value, &max_gas_price, &provider)
        .await;
    assert!(matches!(gas.err().unwrap(), TxManagerError::EstimateGasError(_)));

    let gas = U256::from(100_000_000_000u64);
    let tx_hash = tx
        .send(to_address, vec![].as_slice(), &value, &gas, &max_gas_price, &provider)
        .await;
    assert!(matches!(tx_hash.err().unwrap(), TxManagerError::GasPriceError(_)));

    let max_gas_price = U256::from(1u64);
    mock.push(history.clone()).unwrap();
    mock.push(block.clone()).unwrap();
    let tx_hash = tx
        .send(to_address, vec![].as_slice(), &value, &gas, &max_gas_price, &provider)
        .await;
    assert!(matches!(tx_hash.err().unwrap(), TxManagerError::GasPriceError(_)));

    let max_gas_price = U256::from(100_000_000_000u64);
    mock.push(history.clone()).unwrap();
    mock.push(block.clone()).unwrap();
    let tx_hash = tx
        .send(to_address, vec![].as_slice(), &value, &gas, &max_gas_price, &provider)
        .await;
    assert!(matches!(tx_hash.err().unwrap(), TxManagerError::NonceError(_)));

    let gas = U256::from(100_000_000_000u64);
    mock.push(nonce).unwrap();
    mock.push(gas).unwrap();
    mock.push(nonce).unwrap();
    mock.push(history.clone()).unwrap();
    mock.push(block.clone()).unwrap();
    let tx_hash = tx
        .send(to_address, vec![].as_slice(), &value, &gas, &max_gas_price, &provider)
        .await;
    assert!(matches!(tx_hash.err().unwrap(), TxManagerError::SendTxError(_)));
}

#[tokio::test]
async fn test_legacy_tx_with_error() {
    let (provider, mock) = Provider::mocked();

    let nonce = U256::from(100);
    let price = U256::from(1000000);

    let chain_id = 2000u64;
    let wallet = LocalWallet::new(&mut rand::thread_rng()).with_chain_id(chain_id);
    let to_address = wallet.address();
    let cfg = TxManagerConfig::new(None).unwrap();
    let builder = TxBuilder::builder()
        .config(cfg)
        .chain_id(chain_id)
        .wallet(wallet)
        .build();
    let tx = builder.build_tx(&provider).await;
    assert!(!tx.is_1559_tx());

    let gas_price = tx.gas_price(&provider).await;
    assert!(matches!(gas_price.err().unwrap(), TxManagerError::GasPriceError(_)));

    let max_gas_price = U256::from(100_000_000_000u64);
    mock.push(price).unwrap();
    let value = ethers_core::utils::parse_ether("1").unwrap();
    let gas = tx
        .estimate_gas(to_address, vec![].as_slice(), &value, &max_gas_price, &provider)
        .await;
    assert!(matches!(gas.err().unwrap(), TxManagerError::EstimateGasError(_)));

    let gas = U256::from(100_000_000_000u64);
    let tx_hash = tx
        .send(to_address, vec![].as_slice(), &value, &gas, &max_gas_price, &provider)
        .await;
    assert!(matches!(tx_hash.err().unwrap(), TxManagerError::GasPriceError(_)));

    mock.push(price).unwrap();
    let max_gas_price = U256::from(1u64);
    let tx_hash = tx
        .send(to_address, vec![].as_slice(), &value, &gas, &max_gas_price, &provider)
        .await;
    assert!(matches!(tx_hash.err().unwrap(), TxManagerError::GasPriceError(_)));

    mock.push(price).unwrap();
    let max_gas_price = U256::from(100_000_000_000u64);
    let tx_hash = tx
        .send(to_address, vec![].as_slice(), &value, &gas, &max_gas_price, &provider)
        .await;
    assert!(matches!(tx_hash.err().unwrap(), TxManagerError::NonceError(_)));

    let gas = U256::from(100_000_000_000u64);
    mock.push(nonce).unwrap();
    mock.push(gas).unwrap();
    mock.push(nonce).unwrap();
    mock.push(price).unwrap();
    let tx_hash = tx
        .send(to_address, vec![].as_slice(), &value, &gas, &max_gas_price, &provider)
        .await;
    assert!(matches!(tx_hash.err().unwrap(), TxManagerError::SendTxError(_)));
}

#[tokio::test]
async fn test_confirm_with_error() {
    let (provider, mock) = Provider::mocked();

    let mut transaction = get_transaction();
    let mut transaction_receipt = get_transaction_receipt();

    let chain_id = 2000u64;
    let wallet = LocalWallet::new(&mut rand::thread_rng()).with_chain_id(chain_id);
    let mut cfg = TxManagerConfig::new(None).unwrap();
    cfg.confirm_interval_secs = 0;
    cfg.max_confirm_count = 4;
    let builder = TxBuilder::builder()
        .config(cfg.clone())
        .chain_id(chain_id)
        .wallet(wallet)
        .build();
    let tx = builder.build_tx(&provider).await;
    assert!(!tx.is_1559_tx());

    let tx_hash = H256::from_str("0x090b19818d9d087a49c3d2ecee4829ee4acea46089c1381ac5e588188627466d").unwrap();
    let receipt = tx.confirm(&provider, tx_hash).await;
    assert!(matches!(receipt.err().unwrap(), TxManagerError::ConfirmTxError(_)));

    mock.push(json!(null)).unwrap();
    let receipt = tx.confirm(&provider, tx_hash).await;
    assert!(matches!(receipt.err().unwrap(), TxManagerError::ConfirmTxError(_)));

    mock.push(json!(null)).unwrap();
    mock.push(json!(null)).unwrap();
    let receipt = tx.confirm(&provider, tx_hash).await;
    assert!(matches!(receipt.err().unwrap(), TxManagerError::TxDropped));

    transaction.block_number = None;
    mock.push(transaction.clone()).unwrap();
    mock.push(json!(null)).unwrap();
    let receipt = tx.confirm(&provider, tx_hash).await;
    assert!(matches!(receipt.err().unwrap(), TxManagerError::ConfirmTxError(_)));

    transaction.block_number = Some(U64::from(100));
    let block_number1 = U64::from(10);
    let block_number2 = U64::from(110);
    mock.push(block_number2).unwrap();
    mock.push(transaction.clone()).unwrap();
    mock.push(block_number1).unwrap();
    mock.push(transaction.clone()).unwrap();
    mock.push(json!(null)).unwrap();
    let receipt = tx.confirm(&provider, tx_hash).await;
    assert!(matches!(receipt.err().unwrap(), TxManagerError::ConfirmTxError(_)));

    transaction_receipt.status = Some(U64::from(0));
    mock.push(transaction_receipt.clone()).unwrap();
    mock.push(block_number2).unwrap();
    mock.push(transaction.clone()).unwrap();
    let receipt = tx.confirm(&provider, tx_hash).await;
    assert!(matches!(receipt.err().unwrap(), TxManagerError::ConfirmTxError(_)));

    transaction_receipt.status = Some(U64::from(1));
    mock.push(transaction_receipt.clone()).unwrap();
    mock.push(block_number2).unwrap();
    mock.push(transaction.clone()).unwrap();
    let receipt = tx.confirm(&provider, tx_hash).await;
    assert_eq!(receipt.unwrap(), transaction_receipt);

    transaction.block_number = None;
    for _ in 0..cfg.max_confirm_count {
        mock.push(transaction.clone()).unwrap();
    }
    let receipt = tx.confirm(&provider, tx_hash).await;
    assert!(matches!(receipt.err().unwrap(), TxManagerError::ConfirmTxError(_)));
}

fn get_1559_block() -> Block<()> {
    let json = serde_json::json!(
    {
        "baseFeePerGas": "0x7",
        "miner": "0x0000000000000000000000000000000000000001",
        "number": "0x1b4",
        "hash": "0x0e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331",
        "parentHash": "0x9646252be9520f6e71339a8df9c55e4d7619deeb018d2a3f2d21fc165dde5eb5",
        "mixHash": "0x1010101010101010101010101010101010101010101010101010101010101010",
        "nonce": "0x0000000000000000",
        "sealFields": [
        "0xe04d296d2460cfb8472af2c5fd05b5a214109c25688d3704aed5484f9a7792f2",
        "0x0000000000000042"
        ],
        "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
        "logsBloom":  "0x0e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331",
        "transactionsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
        "receiptsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
        "stateRoot": "0xd5855eb08b3387c0af375e9cdb6acfc05eb8f519e419b874b6ff2ffda7ed1dff",
        "difficulty": "0x27f07",
        "totalDifficulty": "0x27f07",
        "extraData": "0x0000000000000000000000000000000000000000000000000000000000000000",
        "size": "0x27f07",
        "gasLimit": "0x9f759",
        "minGasPrice": "0x9f759",
        "gasUsed": "0x9f759",
        "timestamp": "0x54e34e8e",
        "transactions": [],
        "uncles": []
        }
    );

    let block: Block<()> = serde_json::from_value(json).unwrap();
    block
}

fn get_transaction() -> Transaction {
    Transaction {
        hash: H256::from_str("5e2fc091e15119c97722e9b63d5d32b043d077d834f377b91f80d32872c78109").unwrap(),
        nonce: 65.into(),
        block_hash: Some(H256::from_str("f43869e67c02c57d1f9a07bb897b54bec1cfa1feb704d91a2ee087566de5df2c").unwrap()),
        block_number: Some(6203173.into()),
        transaction_index: Some(10.into()),
        from: Address::from_str("e66b278fa9fbb181522f6916ec2f6d66ab846e04").unwrap(),
        to: Some(Address::from_str("11d7c2ab0d4aa26b7d8502f6a7ef6844908495c2").unwrap()),
        value: 0.into(),
        gas_price: Some(1500000007.into()),
        gas: 106703.into(),
        input: Bytes::from_str("0xe5225381").unwrap(),
        v: 1.into(),
        r: U256::from_str_radix(
            "12010114865104992543118914714169554862963471200433926679648874237672573604889",
            10,
        )
        .unwrap(),
        s: U256::from_str_radix(
            "22830728216401371437656932733690354795366167672037272747970692473382669718804",
            10,
        )
        .unwrap(),
        transaction_type: Some(2.into()),
        access_list: Some(AccessList::default()),
        max_priority_fee_per_gas: Some(1500000000.into()),
        max_fee_per_gas: Some(1500000009.into()),
        chain_id: Some(5.into()),
        other: Default::default(),
    }
}

fn get_transaction_receipt() -> TransactionReceipt {
    let v: serde_json::Value = serde_json::from_str(
        r#"{
        "transactionHash": "0x611b173b0e0dfda94da7bfb6cb77c9f1c03e2f2149ba060e6bddfaa219942369",
        "blockHash": "0xa11871d61e0e703ae33b358a6a9653c43e4216f277d4a1c7377b76b4d5b4cbf1",
        "blockNumber": "0xe3c1d8",
        "contractAddress": "0x08f6db30039218894067023a3593baf27d3f4a2b",
        "cumulativeGasUsed": "0x1246047",
        "effectiveGasPrice": "0xa02ffee00",
        "from": "0x0968995a48162a23af60d3ca25cddfa143cd8891",
        "gasUsed": "0x1b9229",
        "logs": [
          {
            "address": "0x08f6db30039218894067023a3593baf27d3f4a2b",
            "topics": [
              "0x40c340f65e17194d14ddddb073d3c9f888e3cb52b5aae0c6c7706b4fbc905fac"
            ],
            "data": "0x0000000000000000000000000968995a48162a23af60d3ca25cddfa143cd88910000000000000000000000000000000000000000000000000000000000002616",
            "blockNumber": "0xe3c1d8",
            "transactionHash": "0x611b173b0e0dfda94da7bfb6cb77c9f1c03e2f2149ba060e6bddfaa219942369",
            "transactionIndex": "0xdf",
            "blockHash": "0xa11871d61e0e703ae33b358a6a9653c43e4216f277d4a1c7377b76b4d5b4cbf1",
            "logIndex": "0x196",
            "removed": false
          },
          {
            "address": "0x08f6db30039218894067023a3593baf27d3f4a2b",
            "topics": [
              "0x40c340f65e17194d14ddddb073d3c9f888e3cb52b5aae0c6c7706b4fbc905fac"
            ],
            "data": "0x00000000000000000000000059750ac0631f63bfdce0f0867618e468e11ee34700000000000000000000000000000000000000000000000000000000000000fa",
            "blockNumber": "0xe3c1d8",
            "transactionHash": "0x611b173b0e0dfda94da7bfb6cb77c9f1c03e2f2149ba060e6bddfaa219942369",
            "transactionIndex": "0xdf",
            "blockHash": "0xa11871d61e0e703ae33b358a6a9653c43e4216f277d4a1c7377b76b4d5b4cbf1",
            "logIndex": "0x197",
            "removed": false
          }
        ],
        "logsBloom": "0x00000000000000800000000040000000000000000000000000000000000000000000008000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "status": "0x1",
        "to": null,
        "transactionIndex": "0xdf",
        "type": "0x2"
        }
        "#,
    ).unwrap();

    let receipt: TransactionReceipt = serde_json::from_value(v).unwrap();
    receipt
}

fn get_fee_history() -> FeeHistory {
    FeeHistory {
        base_fee_per_gas: vec![U256::from(100), U256::from(200)],
        gas_used_ratio: vec![0.5, 0.6],
        oldest_block: U256::from(1),
        reward: vec![
            vec![U256::from(10), U256::from(15)],
            vec![U256::from(15), U256::from(30)],
        ],
    }
}
