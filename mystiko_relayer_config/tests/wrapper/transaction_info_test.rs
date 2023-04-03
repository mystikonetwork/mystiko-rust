use std::sync::Arc;
use mystiko_relayer_config::raw::create_raw_from_file;
use mystiko_relayer_config::raw::transaction_info::RawTransactionInfoConfig;
use mystiko_relayer_config::wrapper::transaction_info::TransactionInfoConfig;

const VALID_CONFIG_FILE: &str = "tests/files/transaction_info.valid.json";

#[tokio::test]
async fn test_create_main_gas_cost() {}

#[tokio::test]
async fn test_create_erc20_gas_cost() {}

#[tokio::test]
async fn test_create_transaction_info() {
    let raw_config = create_raw_from_file::<RawTransactionInfoConfig>(VALID_CONFIG_FILE)
        .await
        .unwrap();
    let config = TransactionInfoConfig::new(Arc::new(raw_config));
    config.validate().unwrap();
}
