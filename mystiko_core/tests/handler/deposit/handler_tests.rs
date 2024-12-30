use crate::common::{create_database, MockProvider, MockProviders};
use crate::handler::{
    generate_private_key, MockCommitmentPoolContracts, MockDepositContracts, MockPublicAssets, MockScreeningClient,
    MockTransactions,
};
use ethers_core::abi::{AbiDecode, AbiEncode};
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Eip1559TransactionRequest, TransactionReceipt, TransactionRequest, TxHash, U256, U64};
use mystiko_config::MystikoConfig;
use mystiko_core::{
    AccountHandler, Accounts, CommitmentColumn, Database, DepositColumn, DepositHandler, DepositQuote, Deposits,
    DepositsOptions, PrivateKeySigner, WalletHandler, Wallets,
};
use mystiko_ethers::{Provider, ProviderWrapper};
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::document::v1::{Account, Deposit, Wallet};
use mystiko_protos::core::handler::v1::{
    CreateAccountOptions, CreateDepositOptions, CreateWalletOptions, FixDepositStatusOptions, QuoteDepositOptions,
    SendDepositOptions,
};
use mystiko_protos::core::v1::DepositStatus;
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_protos::storage::v1::SubFilter;
use mystiko_screening_client::{ScreeningClient, ScreeningResponse};
use mystiko_storage::{ColumnValues, DocumentColumn, SqlStatementFormatter};
use mystiko_storage_sqlite::SqliteStorage;
use mystiko_utils::address::{ethers_address_from_string, ethers_address_to_string};
use mystiko_utils::convert::number_to_u256_decimal;
use num_bigint::BigUint;
use std::collections::HashMap;
use std::ops::Add;
use std::str::FromStr;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[tokio::test]
async fn test_loop_deposit_quote() {
    let address = ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap();
    let mut deposit_contracts = MockDepositContracts::new();
    deposit_contracts
        .expect_quote()
        .withf(move |options| options.chain_id == 5_u64 && options.contract_address == address)
        .returning(|_| {
            Ok(DepositQuote::builder()
                .min_amount(U256::from_dec_str("10000000000000000").unwrap())
                .max_amount(U256::from_dec_str("100000000000000000000").unwrap())
                .min_rollup_fee_amount(U256::from_dec_str("100000000000000").unwrap())
                .build())
        });
    let options = MockOptions::builder().deposit_contracts(deposit_contracts).build();
    let (_, handler) = setup(options).await;
    let quote_options = QuoteDepositOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .build();
    let quote = handler.quote(quote_options).await.unwrap();
    assert_eq!(quote.asset_symbol, "MTT");
    assert_eq!(quote.asset_decimals, 16_u32);
    assert_eq!(quote.min_amount, 1_f64);
    assert_eq!(quote.min_decimal_amount, "10000000000000000");
    assert_eq!(quote.max_amount, 10000_f64);
    assert_eq!(quote.max_decimal_amount, "100000000000000000000");
    assert_eq!(quote.min_rollup_fee_amount, 0.01_f64);
    assert_eq!(quote.min_rollup_fee_decimal_amount, "100000000000000");
    assert_eq!(quote.rollup_fee_asset_symbol, "MTT");
    assert_eq!(quote.rollup_fee_asset_decimals, 16_u32);
    assert!(quote.min_bridge_fee_amount.is_none());
    assert!(quote.min_bridge_fee_decimal_amount.is_none());
    assert!(quote.bridge_fee_asset_symbol.is_none());
    assert!(quote.bridge_fee_asset_decimals.is_none());
    assert!(quote.min_executor_fee_amount.is_none());
    assert!(quote.min_executor_fee_decimal_amount.is_none());
    assert!(quote.executor_fee_asset_symbol.is_none());
    assert!(quote.executor_fee_asset_decimals.is_none());
    assert_eq!(quote.recommended_amounts, [1_f64, 10_f64]);
    assert_eq!(
        quote.recommended_decimal_amounts,
        ["10000000000000000", "100000000000000000"]
    );
}

#[tokio::test]
async fn test_cross_chain_deposit_quote() {
    let address = ethers_address_from_string("0x961F315A836542e603A3df2E0dd9d4ECd06ebC67").unwrap();
    let mut deposit_contracts = MockDepositContracts::new();
    deposit_contracts
        .expect_quote()
        .withf(move |options| options.chain_id == 5_u64 && options.contract_address == address)
        .returning(|_| {
            Ok(DepositQuote::builder()
                .min_amount(U256::from_dec_str("10000000000000000").unwrap())
                .max_amount(U256::from_dec_str("100000000000000000000").unwrap())
                .min_rollup_fee_amount(U256::from_dec_str("100000000000000").unwrap())
                .min_bridge_fee_amount(U256::from_dec_str("10000000000000").unwrap())
                .min_executor_fee_amount(U256::from_dec_str("1000000000000").unwrap())
                .build())
        });
    let options = MockOptions::builder().deposit_contracts(deposit_contracts).build();
    let (_, handler) = setup(options).await;
    let quote_options = QuoteDepositOptions::builder()
        .chain_id(5_u64)
        .dst_chain_id(97_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Tbridge as i32)
        .build();
    let quote = handler.quote(quote_options).await.unwrap();
    assert_eq!(quote.asset_symbol, "MTT");
    assert_eq!(quote.asset_decimals, 16_u32);
    assert_eq!(quote.min_amount, 1_f64);
    assert_eq!(quote.min_decimal_amount, "10000000000000000");
    assert_eq!(quote.max_amount, 10000_f64);
    assert_eq!(quote.max_decimal_amount, "100000000000000000000");
    assert_eq!(quote.min_rollup_fee_amount, 0.01_f64);
    assert_eq!(quote.min_rollup_fee_decimal_amount, "100000000000000");
    assert_eq!(quote.rollup_fee_asset_symbol, "MTT");
    assert_eq!(quote.rollup_fee_asset_decimals, 16_u32);
    assert_eq!(quote.min_bridge_fee_amount.unwrap(), 0.00001_f64);
    assert_eq!(quote.min_bridge_fee_decimal_amount.unwrap(), "10000000000000");
    assert_eq!(quote.bridge_fee_asset_symbol.unwrap(), "ETH");
    assert_eq!(quote.bridge_fee_asset_decimals.unwrap(), 18_u32);
    assert_eq!(quote.min_executor_fee_amount.unwrap(), 0.0001_f64);
    assert_eq!(quote.min_executor_fee_decimal_amount.unwrap(), "1000000000000");
    assert_eq!(quote.executor_fee_asset_symbol.unwrap(), "MTT");
    assert_eq!(quote.executor_fee_asset_decimals.unwrap(), 16_u32);
    assert_eq!(quote.recommended_amounts, [1_f64, 10_f64]);
    assert_eq!(
        quote.recommended_decimal_amounts,
        ["10000000000000000", "100000000000000000"]
    );
}

#[tokio::test]
async fn test_loop_deposit_summary() {
    let (_, handler) = setup(Default::default()).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(16_u32)
        .min_amount(1_f64)
        .min_decimal_amount("10000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("100000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("100000000000000".to_string())
        .rollup_fee_asset_symbol("MTT".to_string())
        .rollup_fee_asset_decimals(16_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .shielded_address("secret address".to_string())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .deposit_quote(quote)
        .build();
    let summary = handler.summary(options).await.unwrap();
    assert_eq!(summary.chain_id, 5_u64);
    assert_eq!(summary.asset_symbol, "MTT");
    assert_eq!(summary.asset_decimals, 16_u32);
    assert_eq!(summary.shielded_address, "secret address");
    assert_eq!(summary.amount, 10_f64);
    assert_eq!(summary.decimal_amount, "100000000000000000");
    assert_eq!(summary.rollup_fee_amount, 0.01_f64);
    assert_eq!(summary.rollup_fee_decimal_amount, "100000000000000");
    assert_eq!(summary.rollup_fee_asset_symbol, "MTT");
    assert_eq!(summary.rollup_fee_asset_decimals, 16_u32);
    assert_eq!(summary.bridge_type.unwrap(), BridgeType::Loop as i32);
    assert_eq!(summary.total_amounts, HashMap::from([("MTT".to_string(), 10.01_f64)]));
    assert_eq!(
        summary.total_decimal_amounts,
        HashMap::from([("MTT".to_string(), "100100000000000000".to_string())])
    );
    assert!(summary.dst_chain_id.is_none());
    assert!(summary.bridge_fee_amount.is_none());
    assert!(summary.bridge_fee_decimal_amount.is_none());
    assert!(summary.bridge_fee_asset_symbol.is_none());
    assert!(summary.bridge_fee_asset_decimals.is_none());
    assert!(summary.executor_fee_amount.is_none());
    assert!(summary.executor_fee_decimal_amount.is_none());
    assert!(summary.executor_fee_asset_symbol.is_none());
    assert!(summary.executor_fee_asset_decimals.is_none());
}

#[tokio::test]
async fn test_cross_chain_deposit_summary() {
    let (_, handler) = setup(Default::default()).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(16_u32)
        .min_amount(1_f64)
        .min_decimal_amount("10000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("100000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("100000000000000".to_string())
        .rollup_fee_asset_symbol("MTT".to_string())
        .rollup_fee_asset_decimals(16_u32)
        .min_bridge_fee_amount(0.00001_f64)
        .min_bridge_fee_decimal_amount("10000000000000".to_string())
        .bridge_fee_asset_symbol("ETH".to_string())
        .bridge_fee_asset_decimals(18_u32)
        .min_executor_fee_amount(0.0001_f64)
        .min_executor_fee_decimal_amount("1000000000000".to_string())
        .executor_fee_asset_symbol("MTT".to_string())
        .executor_fee_asset_decimals(16_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(5_u64)
        .dst_chain_id(97_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Tbridge as i32)
        .shielded_address("secret address".to_string())
        .amount(10_f64)
        .bridge_fee_amount(0.00001_f64)
        .executor_fee_amount(0.0001_f64)
        .deposit_quote(quote)
        .build();
    let summary = handler.summary(options).await.unwrap();
    assert_eq!(summary.chain_id, 5_u64);
    assert_eq!(summary.asset_symbol, "MTT");
    assert_eq!(summary.asset_decimals, 16_u32);
    assert_eq!(summary.shielded_address, "secret address");
    assert_eq!(summary.amount, 10_f64);
    assert_eq!(summary.decimal_amount, "100000000000000000");
    assert_eq!(summary.rollup_fee_amount, 0.01_f64);
    assert_eq!(summary.rollup_fee_decimal_amount, "100000000000000");
    assert_eq!(summary.rollup_fee_asset_symbol, "MTT");
    assert_eq!(summary.rollup_fee_asset_decimals, 16_u32);
    assert_eq!(summary.dst_chain_id.unwrap(), 97_u64);
    assert_eq!(summary.bridge_fee_amount.unwrap(), 0.00001_f64);
    assert_eq!(summary.bridge_fee_decimal_amount.unwrap(), "10000000000000");
    assert_eq!(summary.bridge_fee_asset_symbol.unwrap(), "ETH");
    assert_eq!(summary.bridge_fee_asset_decimals.unwrap(), 18_u32);
    assert_eq!(summary.executor_fee_amount.unwrap(), 0.0001_f64);
    assert_eq!(summary.executor_fee_decimal_amount.unwrap(), "1000000000000");
    assert_eq!(summary.executor_fee_asset_symbol.unwrap(), "MTT");
    assert_eq!(summary.executor_fee_asset_decimals.unwrap(), 16_u32);
    assert_eq!(summary.bridge_type.unwrap(), BridgeType::Tbridge as i32);
    assert_eq!(
        summary.total_amounts,
        HashMap::from([("MTT".to_string(), 10.0101_f64), ("ETH".to_string(), 0.00001_f64),])
    );
    assert_eq!(
        summary.total_decimal_amounts,
        HashMap::from([
            ("MTT".to_string(), "100101000000000000".to_string()),
            ("ETH".to_string(), "10000000000000".to_string()),
        ])
    );
}

#[tokio::test]
async fn test_deposit_summary_without_quote() {
    let address = ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap();
    let mut deposit_contracts = MockDepositContracts::new();
    deposit_contracts
        .expect_quote()
        .withf(move |options| options.chain_id == 5_u64 && options.contract_address == address)
        .returning(|_| {
            Ok(DepositQuote::builder()
                .min_amount(U256::from_dec_str("10000000000000000").unwrap())
                .max_amount(U256::from_dec_str("100000000000000000000").unwrap())
                .min_rollup_fee_amount(U256::from_dec_str("100000000000000").unwrap())
                .build())
        });
    let options = MockOptions::builder().deposit_contracts(deposit_contracts).build();
    let (_, handler) = setup(options).await;
    let options = CreateDepositOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .shielded_address("secret address".to_string())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .build();
    let summary = handler.summary(options).await.unwrap();
    assert_eq!(summary.chain_id, 5_u64);
    assert_eq!(summary.asset_symbol, "MTT");
    assert_eq!(summary.asset_decimals, 16_u32);
    assert_eq!(summary.shielded_address, "secret address");
    assert_eq!(summary.amount, 10_f64);
    assert_eq!(summary.decimal_amount, "100000000000000000");
    assert_eq!(summary.rollup_fee_amount, 0.01_f64);
    assert_eq!(summary.rollup_fee_decimal_amount, "100000000000000");
    assert_eq!(summary.rollup_fee_asset_symbol, "MTT");
    assert_eq!(summary.rollup_fee_asset_decimals, 16_u32);
    assert_eq!(summary.bridge_type.unwrap(), BridgeType::Loop as i32);
    assert_eq!(summary.total_amounts, HashMap::from([("MTT".to_string(), 10.01_f64)]));
    assert_eq!(
        summary.total_decimal_amounts,
        HashMap::from([("MTT".to_string(), "100100000000000000".to_string())])
    );
    assert!(summary.dst_chain_id.is_none());
    assert!(summary.bridge_fee_amount.is_none());
    assert!(summary.bridge_fee_decimal_amount.is_none());
    assert!(summary.bridge_fee_asset_symbol.is_none());
    assert!(summary.bridge_fee_asset_decimals.is_none());
    assert!(summary.executor_fee_amount.is_none());
    assert!(summary.executor_fee_decimal_amount.is_none());
    assert!(summary.executor_fee_asset_symbol.is_none());
    assert!(summary.executor_fee_asset_decimals.is_none());
}

#[tokio::test]
async fn test_deposit_summary_with_errors() {
    let (_, handler) = setup(Default::default()).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(16_u32)
        .min_amount(1_f64)
        .min_decimal_amount("10000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("100000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("100000000000000".to_string())
        .rollup_fee_asset_symbol("MTT".to_string())
        .rollup_fee_asset_decimals(16_u32)
        .min_bridge_fee_amount(0.00001_f64)
        .min_bridge_fee_decimal_amount("10000000000000".to_string())
        .bridge_fee_asset_symbol("ETH".to_string())
        .bridge_fee_asset_decimals(18_u32)
        .min_executor_fee_amount(0.0001_f64)
        .min_executor_fee_decimal_amount("1000000000000".to_string())
        .executor_fee_asset_symbol("MTT".to_string())
        .executor_fee_asset_decimals(16_u32)
        .build();
    let mut options = CreateDepositOptions::builder()
        .chain_id(5_u64)
        .dst_chain_id(97_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Tbridge as i32)
        .shielded_address("secret address".to_string())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .bridge_fee_amount(0.00001_f64)
        .executor_fee_amount(0.0001_f64)
        .deposit_quote(quote)
        .build();

    // min amount
    options.amount = 0.1_f64;
    let err = handler.summary(options.clone()).await.unwrap_err();
    assert_eq!(
        err.to_string(),
        "deposit amount 0.1 is less than min_amount 1 or greater than max_amount 10000"
    );

    //max amount
    options.amount = 1000000_f64;
    let err = handler.summary(options.clone()).await.unwrap_err();
    assert_eq!(
        err.to_string(),
        "deposit amount 1000000 is less than min_amount 1 or greater than max_amount 10000"
    );

    //min rollup fee
    options.amount = 10_f64;
    options.rollup_fee_amount = Some(0.001_f64);
    let err = handler.summary(options.clone()).await.unwrap_err();
    assert_eq!(
        err.to_string(),
        "rollup fee amount 0.001 is less than min_rollup_fee_amount 0.01"
    );

    //min bridge fee
    options.rollup_fee_amount = Some(0.01_f64);
    options.bridge_fee_amount = Some(0.000001_f64);
    let err = handler.summary(options.clone()).await.unwrap_err();
    assert_eq!(
        err.to_string(),
        "bridge fee amount 0.000001 is less than min_bridge_fee_amount 0.00001"
    );

    //min executor fee
    options.bridge_fee_amount = Some(0.00001_f64);
    options.executor_fee_amount = Some(0.0000001_f64);
    let err = handler.summary(options.clone()).await.unwrap_err();
    assert_eq!(
        err.to_string(),
        "executor fee amount 0.0000001 is less than min_executor_fee_amount 0.0001"
    );
}

#[tokio::test]
async fn test_loop_deposit_main_token_create() {
    let (db, handler) = setup(Default::default()).await;
    let (wallet, account) = create_wallet(db).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(18_u32)
        .min_amount(1_f64)
        .min_decimal_amount("1000000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("10000000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("10000000000000000".to_string())
        .rollup_fee_asset_symbol("BNB".to_string())
        .rollup_fee_asset_decimals(18_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(97_u64)
        .asset_symbol("BNB".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .shielded_address(account.shielded_address.clone())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .deposit_quote(quote)
        .build();
    let deposit = handler.create(options).await.unwrap();
    assert_eq!(deposit.chain_id, 97_u64);
    assert_eq!(deposit.contract_address, "0x390d485F4D43212D4ae8Cdd967a711514ed5a54f");
    assert_eq!(deposit.pool_address, "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
    assert_eq!(deposit.dst_chain_id, 97_u64);
    assert_eq!(
        deposit.dst_chain_contract_address,
        "0x390d485F4D43212D4ae8Cdd967a711514ed5a54f"
    );
    assert_eq!(deposit.dst_pool_address, "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
    assert_eq!(deposit.asset_symbol, "BNB");
    assert_eq!(deposit.asset_decimals, 18_u32);
    assert_eq!(deposit.amount, 10_f64);
    assert_eq!(deposit.decimal_amount, "10000000000000000000");
    assert_eq!(deposit.rollup_fee_amount, 0.01_f64);
    assert_eq!(deposit.rollup_fee_decimal_amount, "10000000000000000");
    assert_eq!(deposit.shielded_address, account.shielded_address);
    assert_eq!(deposit.wallet_id, wallet.id);
    assert_eq!(deposit.status, DepositStatus::Unspecified as i32);
    assert_eq!(deposit.bridge_type, BridgeType::Loop as i32);
    assert!(deposit.asset_address.is_none());
    assert!(deposit.bridge_fee_amount.is_none());
    assert!(deposit.bridge_fee_decimal_amount.is_none());
    assert!(deposit.bridge_fee_asset_symbol.is_none());
    assert!(deposit.bridge_fee_asset_address.is_none());
    assert!(deposit.bridge_fee_asset_decimals.is_none());
    assert!(deposit.executor_fee_amount.is_none());
    assert!(deposit.executor_fee_decimal_amount.is_none());
    assert!(deposit.executor_fee_asset_symbol.is_none());
    assert!(deposit.executor_fee_asset_address.is_none());
    assert!(deposit.executor_fee_asset_decimals.is_none());
    assert!(deposit.src_chain_transaction_hash.is_none());
    assert!(deposit.queued_transaction_hash.is_none());
    assert!(deposit.included_transaction_hash.is_none());
    assert!(deposit.error_message.is_none());
}

#[tokio::test]
async fn test_loop_deposit_erc20_token_create() {
    let (db, handler) = setup(Default::default()).await;
    let (wallet, account) = create_wallet(db).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(16_u32)
        .min_amount(1_f64)
        .min_decimal_amount("10000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("100000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("100000000000000".to_string())
        .rollup_fee_asset_symbol("MTT".to_string())
        .rollup_fee_asset_decimals(16_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .shielded_address(account.shielded_address.clone())
        .amount(10_f64)
        .deposit_quote(quote)
        .build();
    let deposit = handler.create(options).await.unwrap();
    assert_eq!(deposit.chain_id, 5_u64);
    assert_eq!(deposit.contract_address, "0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5");
    assert_eq!(deposit.pool_address, "0x223903804Ee95e264F74C88B4F8583429524593c");
    assert_eq!(deposit.dst_chain_id, 5_u64);
    assert_eq!(
        deposit.dst_chain_contract_address,
        "0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5"
    );
    assert_eq!(deposit.dst_pool_address, "0x223903804Ee95e264F74C88B4F8583429524593c");
    assert_eq!(deposit.asset_symbol, "MTT");
    assert_eq!(deposit.asset_decimals, 16_u32);
    assert_eq!(deposit.amount, 10_f64);
    assert_eq!(deposit.decimal_amount, "100000000000000000");
    assert_eq!(deposit.rollup_fee_amount, 0.01_f64);
    assert_eq!(deposit.rollup_fee_decimal_amount, "100000000000000");
    assert_eq!(deposit.shielded_address, account.shielded_address);
    assert_eq!(deposit.wallet_id, wallet.id);
    assert_eq!(deposit.status, DepositStatus::Unspecified as i32);
    assert_eq!(deposit.bridge_type, BridgeType::Loop as i32);
    assert_eq!(
        deposit.asset_address.unwrap(),
        "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a"
    );
    assert!(deposit.bridge_fee_amount.is_none());
    assert!(deposit.bridge_fee_decimal_amount.is_none());
    assert!(deposit.bridge_fee_asset_symbol.is_none());
    assert!(deposit.bridge_fee_asset_address.is_none());
    assert!(deposit.bridge_fee_asset_decimals.is_none());
    assert!(deposit.executor_fee_amount.is_none());
    assert!(deposit.executor_fee_decimal_amount.is_none());
    assert!(deposit.executor_fee_asset_symbol.is_none());
    assert!(deposit.executor_fee_asset_address.is_none());
    assert!(deposit.executor_fee_asset_decimals.is_none());
    assert!(deposit.src_chain_transaction_hash.is_none());
    assert!(deposit.queued_transaction_hash.is_none());
    assert!(deposit.included_transaction_hash.is_none());
    assert!(deposit.error_message.is_none());
}

#[tokio::test]
async fn test_cross_chain_deposit_main_token_create() {
    let (db, handler) = setup(Default::default()).await;
    let (wallet, account) = create_wallet(db).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(18_u32)
        .min_amount(1_f64)
        .min_decimal_amount("1000000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("10000000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("10000000000000000".to_string())
        .rollup_fee_asset_symbol("BNB".to_string())
        .rollup_fee_asset_decimals(18_u32)
        .min_bridge_fee_amount(0.00001_f64)
        .min_bridge_fee_decimal_amount("10000000000000".to_string())
        .bridge_fee_asset_symbol("BNB".to_string())
        .bridge_fee_asset_decimals(18_u32)
        .min_executor_fee_amount(0.0001_f64)
        .min_executor_fee_decimal_amount("100000000000000".to_string())
        .executor_fee_asset_symbol("BNB".to_string())
        .executor_fee_asset_decimals(18_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(97_u64)
        .dst_chain_id(5_u64)
        .asset_symbol("BNB".to_string())
        .bridge_type(BridgeType::Tbridge as i32)
        .shielded_address(account.shielded_address.clone())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .deposit_quote(quote)
        .build();
    let deposit = handler.create(options).await.unwrap();
    assert_eq!(deposit.chain_id, 97_u64);
    assert_eq!(deposit.contract_address, "0xd99F0C90BFDeDd5Bde0193b887c271C5458355Cf");
    assert_eq!(deposit.pool_address, "0xF85679316f1C3998C6387F6F707b31AeEB3a9aBe");
    assert_eq!(deposit.dst_chain_id, 5_u64);
    assert_eq!(
        deposit.dst_chain_contract_address,
        "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
    );
    assert_eq!(deposit.dst_pool_address, "0x5050F69a9786F081509234F1a7F4684b5E5b76C9");
    assert_eq!(deposit.asset_symbol, "BNB");
    assert_eq!(deposit.asset_decimals, 18_u32);
    assert_eq!(deposit.amount, 10_f64);
    assert_eq!(deposit.decimal_amount, "10000000000000000000");
    assert_eq!(deposit.rollup_fee_amount, 0.01_f64);
    assert_eq!(deposit.rollup_fee_decimal_amount, "10000000000000000");
    assert_eq!(deposit.shielded_address, account.shielded_address);
    assert_eq!(deposit.wallet_id, wallet.id);
    assert_eq!(deposit.status, DepositStatus::Unspecified as i32);
    assert_eq!(deposit.bridge_type, BridgeType::Tbridge as i32);
    assert!(deposit.asset_address.is_none());
    assert_eq!(deposit.bridge_fee_amount.unwrap(), 0.00001_f64);
    assert_eq!(deposit.bridge_fee_decimal_amount.unwrap(), "10000000000000");
    assert_eq!(deposit.bridge_fee_asset_symbol.unwrap(), "BNB");
    assert_eq!(deposit.bridge_fee_asset_decimals.unwrap(), 18_u32);
    assert!(deposit.bridge_fee_asset_address.is_none());
    assert_eq!(deposit.executor_fee_amount, Some(0.0001_f64));
    assert_eq!(deposit.executor_fee_decimal_amount.unwrap(), "100000000000000");
    assert_eq!(deposit.executor_fee_asset_symbol.unwrap(), "BNB");
    assert_eq!(deposit.executor_fee_asset_decimals.unwrap(), 18_u32);
    assert!(deposit.executor_fee_asset_address.is_none());
    assert!(deposit.src_chain_transaction_hash.is_none());
    assert!(deposit.queued_transaction_hash.is_none());
    assert!(deposit.included_transaction_hash.is_none());
    assert!(deposit.error_message.is_none());
}

#[tokio::test]
async fn test_cross_chain_deposit_erc20_token_create() {
    let (db, handler) = setup(Default::default()).await;
    let (wallet, account) = create_wallet(db).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_symbol("mBNB".to_string())
        .asset_decimals(18_u32)
        .min_amount(1_f64)
        .min_decimal_amount("1000000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("10000000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("10000000000000000".to_string())
        .rollup_fee_asset_symbol("mBNB".to_string())
        .rollup_fee_asset_decimals(18_u32)
        .min_bridge_fee_amount(0.00001_f64)
        .min_bridge_fee_decimal_amount("10000000000000".to_string())
        .bridge_fee_asset_symbol("mBNB".to_string())
        .bridge_fee_asset_decimals(18_u32)
        .min_executor_fee_amount(0.0001_f64)
        .min_executor_fee_decimal_amount("100000000000000".to_string())
        .executor_fee_asset_symbol("mBNB".to_string())
        .executor_fee_asset_decimals(18_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(5_u64)
        .dst_chain_id(97_u64)
        .asset_symbol("mBNB".to_string())
        .bridge_type(BridgeType::Tbridge as i32)
        .shielded_address(account.shielded_address.clone())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .bridge_fee_amount(0.00001_f64)
        .executor_fee_amount(0.0001_f64)
        .deposit_quote(quote)
        .build();
    let deposit = handler.create(options).await.unwrap();
    assert_eq!(deposit.chain_id, 5_u64);
    assert_eq!(deposit.contract_address, "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
    assert_eq!(deposit.pool_address, "0x5050F69a9786F081509234F1a7F4684b5E5b76C9");
    assert_eq!(deposit.dst_chain_id, 97_u64);
    assert_eq!(
        deposit.dst_chain_contract_address,
        "0xd99F0C90BFDeDd5Bde0193b887c271C5458355Cf"
    );
    assert_eq!(deposit.dst_pool_address, "0xF85679316f1C3998C6387F6F707b31AeEB3a9aBe");
    assert_eq!(deposit.asset_symbol, "mBNB");
    assert_eq!(deposit.asset_decimals, 18_u32);
    assert_eq!(deposit.amount, 10_f64);
    assert_eq!(deposit.decimal_amount, "10000000000000000000");
    assert_eq!(deposit.rollup_fee_amount, 0.01_f64);
    assert_eq!(deposit.rollup_fee_decimal_amount, "10000000000000000");
    assert_eq!(deposit.shielded_address, account.shielded_address);
    assert_eq!(deposit.wallet_id, wallet.id);
    assert_eq!(deposit.status, DepositStatus::Unspecified as i32);
    assert_eq!(deposit.bridge_type, BridgeType::Tbridge as i32);
    assert_eq!(
        deposit.asset_address.unwrap(),
        "0x388C818CA8B9251b393131C08a736A67ccB19297"
    );
    assert_eq!(deposit.bridge_fee_amount.unwrap(), 0.00001_f64);
    assert_eq!(deposit.bridge_fee_decimal_amount.unwrap(), "10000000000000");
    assert_eq!(deposit.bridge_fee_asset_symbol.unwrap(), "mBNB");
    assert_eq!(deposit.bridge_fee_asset_decimals.unwrap(), 18_u32);
    assert_eq!(
        deposit.bridge_fee_asset_address.unwrap(),
        "0x388C818CA8B9251b393131C08a736A67ccB19297"
    );
    assert_eq!(deposit.executor_fee_amount, Some(0.0001_f64));
    assert_eq!(deposit.executor_fee_decimal_amount.unwrap(), "100000000000000");
    assert_eq!(deposit.executor_fee_asset_symbol.unwrap(), "mBNB");
    assert_eq!(deposit.executor_fee_asset_decimals.unwrap(), 18_u32);
    assert_eq!(
        deposit.executor_fee_asset_address.unwrap(),
        "0x388C818CA8B9251b393131C08a736A67ccB19297"
    );
    assert!(deposit.src_chain_transaction_hash.is_none());
    assert!(deposit.queued_transaction_hash.is_none());
    assert!(deposit.included_transaction_hash.is_none());
    assert!(deposit.error_message.is_none());
}

#[tokio::test]
async fn test_loop_deposit_main_token_send_with_screening() {
    let (owner, private_key) = generate_private_key();
    let contract_address = ethers_address_from_string("0x390d485F4D43212D4ae8Cdd967a711514ed5a54f").unwrap();
    let deposit_tx_hash =
        TxHash::decode_hex("0xb56298dea53128b60ad2df8bf978c1a82d41798fa8272002f08e98fefdbc558f").unwrap();
    let amount = number_to_u256_decimal(10_f64, Some(18)).unwrap();
    let rollup_fee_amount = number_to_u256_decimal(0.01_f64, Some(18)).unwrap();
    let mut assets = MockPublicAssets::new();
    assets
        .expect_balance_of()
        .withf(move |options| {
            options.chain_id == 97_u64 && options.owner == owner && options.timeout_ms == Some(100_u64)
        })
        .returning(|_| Ok(U256::from_dec_str("100000000000000000000").unwrap()));
    let mut deposit_contracts = MockDepositContracts::new();
    deposit_contracts
        .expect_deposit::<TypedTransaction, PrivateKeySigner<MockProviders>>()
        .withf(move |options| {
            options.chain_id == 97_u64
                && options.contract_address == contract_address
                && options.timeout_ms == Some(1000_u64)
                && options.request.amount == amount
                && options.request.rollup_fee == rollup_fee_amount
        })
        .returning(move |_| Ok(deposit_tx_hash));
    let mut transactions = MockTransactions::new();
    transactions
        .expect_create()
        .withf(|_, tx_type| tx_type == &mystiko_types::TransactionType::Eip1559)
        .returning(|_, _| Ok(TypedTransaction::Eip1559(Eip1559TransactionRequest::new())));
    transactions
        .expect_wait()
        .withf(move |options| {
            options.chain_id == 97_u64
                && options.tx_hash == deposit_tx_hash
                && options.confirmations == Some(10_u64)
                && options.interval_ms == Some(10_u64)
                && options.timeout_ms == Some(2000_u64)
        })
        .returning(move |_| {
            Ok(TransactionReceipt {
                transaction_hash: deposit_tx_hash,
                block_number: Some(U64::from(200010000_u64)),
                ..Default::default()
            })
        });
    let screening_message = "test";
    let mut screening = MockScreeningClient::new();
    screening
        .expect_address_screening()
        .withf(move |options|
            options.chain_id == 97_u64 &&
                options.message==screening_message &&
                options.asset== Some("0x0000000000000000000000000000000000000000".to_string()) &&
                options.account== ethers_address_to_string(&owner))
        .returning(|_| {
            Ok(ScreeningResponse::builder()
                .deadline(200010000_u64)
                .signature("0x0f95f7effb9f3c8c20a6c78b2278a7ed2cee87ee5cf29031729124711623dd3b14e7e6fb419a61d9c262110c4812d2a37f2c137d2559192eee3f477cc08d92f51c".to_string())
                .build())
        });
    let mut commitment_pool_contracts = MockCommitmentPoolContracts::new();
    commitment_pool_contracts
        .expect_is_historic_commitment()
        .returning(move |options| {
            if options.timeout_ms == Some(2_u64)
                || options.timeout_ms == Some(3_u64)
                || options.timeout_ms == Some(5_u64)
                || options.timeout_ms == Some(6_u64)
            {
                Ok(true)
            } else {
                Ok(false)
            }
        });
    let options = MockOptions::builder()
        .assets(assets)
        .deposit_contracts(deposit_contracts)
        .commitment_pool_contracts(commitment_pool_contracts)
        .transactions(transactions)
        .screening(screening)
        .build();
    let (db, handler) = setup(options).await;
    let (_, account) = create_wallet(db.clone()).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(18_u32)
        .min_amount(1_f64)
        .min_decimal_amount("1000000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("10000000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("10000000000000000".to_string())
        .rollup_fee_asset_symbol("BNB".to_string())
        .rollup_fee_asset_decimals(18_u32)
        .build();
    let create_options = CreateDepositOptions::builder()
        .chain_id(97_u64)
        .asset_symbol("BNB".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .shielded_address(account.shielded_address.clone())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .deposit_quote(quote)
        .build();
    let deposit = handler.create(create_options.clone()).await.unwrap();
    let mut options = SendDepositOptions::builder()
        .deposit_id(deposit.id)
        .query_timeout_ms(100_u64)
        .tx_send_timeout_ms(1000_u64)
        .tx_wait_timeout_ms(2000_u64)
        .tx_wait_interval_ms(10_u64)
        .private_key(private_key)
        .deposit_confirmations(10_u64)
        .screening_message(Some(screening_message.to_string()))
        .build();
    let deposit = handler.send(options.clone()).await.unwrap();
    assert_eq!(deposit.status, DepositStatus::Queued as i32);
    assert_eq!(deposit.queued_transaction_hash(), deposit_tx_hash.encode_hex());
    assert!(deposit.error_message.is_none());
    check_commitment(&deposit, db.clone(), 200010000_u64, true, false).await;

    let fix_options = FixDepositStatusOptions::builder()
        .query_timeout_ms(1)
        .deposit_id(deposit.id.clone())
        .build();
    let deposit = handler.fix_status(fix_options).await.unwrap();
    assert_eq!(deposit.status, DepositStatus::Failed as i32);

    let fix_options = FixDepositStatusOptions::builder()
        .query_timeout_ms(2)
        .deposit_id(deposit.id.clone())
        .build();
    let deposit = handler.fix_status(fix_options).await.unwrap();
    assert_eq!(deposit.status, DepositStatus::Queued as i32);
    assert!(deposit.error_message.is_none());
    let mut cms = db.commitments.find_all().await.unwrap();
    assert_eq!(cms.len(), 1);
    assert_eq!(cms[0].data.status, CommitmentStatus::Queued as i32);

    cms[0].data.included_transaction_hash = Some("0x123".to_string());
    db.commitments.update(&cms[0]).await.unwrap();
    let mut deposits = db.deposits.find_all().await.unwrap();
    assert_eq!(deposits.len(), 1);
    deposits[0].data.status = DepositStatus::Failed as i32;
    deposits[0].data.error_message = Some("error".to_string());
    db.deposits.update(&deposits[0]).await.unwrap();

    let fix_options = FixDepositStatusOptions::builder()
        .query_timeout_ms(3)
        .deposit_id(deposit.id.clone())
        .build();
    let deposit = handler.fix_status(fix_options).await.unwrap();
    assert_eq!(deposit.status, DepositStatus::Included as i32);
    assert!(deposit.error_message.is_none());
    let cms = db.commitments.find_all().await.unwrap();
    assert_eq!(cms.len(), 1);
    assert_eq!(cms[0].data.status, CommitmentStatus::Included as i32);

    db.accounts.delete_all().await.unwrap();
    let deposit = handler.create(create_options).await.unwrap();
    options.deposit_id = deposit.id.clone();
    let deposit = handler.send(options).await.unwrap();
    check_commitment(&deposit, db.clone(), 200010000_u64, false, false).await;

    let fix_options = FixDepositStatusOptions::builder()
        .query_timeout_ms(4)
        .deposit_id(deposit.id.clone())
        .build();
    let deposit = handler.fix_status(fix_options).await.unwrap();
    assert_eq!(deposit.status, DepositStatus::Failed as i32);

    let fix_options = FixDepositStatusOptions::builder()
        .query_timeout_ms(5)
        .deposit_id(deposit.id.clone())
        .build();
    let deposit = handler.fix_status(fix_options).await.unwrap();
    assert_eq!(deposit.status, DepositStatus::Queued as i32);
    assert!(deposit.error_message.is_none());

    let fix_options = FixDepositStatusOptions::builder()
        .query_timeout_ms(6)
        .deposit_id(deposit.id)
        .build();
    let deposit = handler.fix_status(fix_options).await.unwrap();
    assert_eq!(deposit.status, DepositStatus::Queued as i32);
    assert!(deposit.error_message.is_none());
}

#[tokio::test]
async fn test_loop_deposit_erc20_token_send_without_screening() {
    let (owner, private_key) = generate_private_key();
    let contract_address = ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap();
    let asset_address = ethers_address_from_string("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a").unwrap();
    let asset_approve_tx_hash =
        TxHash::decode_hex("0xf5079a68aa75c4b4f1cfd2ef50e23d60ef9211fb2f33481164ebc7e2cf536493").unwrap();
    let deposit_tx_hash =
        TxHash::decode_hex("0xb56298dea53128b60ad2df8bf978c1a82d41798fa8272002f08e98fefdbc558f").unwrap();
    let amount = number_to_u256_decimal(10_f64, Some(16)).unwrap();
    let rollup_fee_amount = number_to_u256_decimal(0.01_f64, Some(16)).unwrap();
    let mut assets = MockPublicAssets::new();
    assets
        .expect_erc20_balance_of()
        .withf(move |options| {
            options.chain_id == 5_u64
                && options.owner == owner
                && options.timeout_ms == Some(100_u64)
                && options.asset_address == asset_address
        })
        .returning(|_| Ok(U256::from_dec_str("100000000000000000000").unwrap()));
    assets
        .expect_erc20_approve::<TypedTransaction, PrivateKeySigner<MockProviders>>()
        .withf(move |options| {
            options.chain_id == 5_u64
                && options.owner == owner
                && options.timeout_ms == Some(1000_u64)
                && options.asset_address == asset_address
                && options.recipient == contract_address
                && options.amount == amount.add(rollup_fee_amount)
        })
        .returning(move |_| Ok(Some(asset_approve_tx_hash)));
    let mut deposit_contracts = MockDepositContracts::new();
    deposit_contracts
        .expect_deposit::<TypedTransaction, PrivateKeySigner<MockProviders>>()
        .withf(move |options| {
            options.chain_id == 5_u64
                && options.contract_address == contract_address
                && options.timeout_ms == Some(1000_u64)
                && options.request.amount == amount
                && options.request.rollup_fee == rollup_fee_amount
        })
        .returning(move |_| Ok(deposit_tx_hash));
    let mut transactions = MockTransactions::new();
    transactions
        .expect_create()
        .withf(|_, tx_type| tx_type == &mystiko_types::TransactionType::Legacy)
        .returning(|_, _| Ok(TypedTransaction::Legacy(TransactionRequest::new())));
    transactions
        .expect_wait()
        .withf(move |options| {
            options.chain_id == 5_u64
                && options.confirmations == Some(10_u64)
                && options.interval_ms == Some(10_u64)
                && options.timeout_ms == Some(2000_u64)
        })
        .returning(move |options| {
            Ok(TransactionReceipt {
                transaction_hash: options.tx_hash,
                block_number: Some(U64::from(200010000_u64)),
                ..Default::default()
            })
        });
    let options = MockOptions::builder()
        .assets(assets)
        .deposit_contracts(deposit_contracts)
        .transactions(transactions)
        .build();
    let (db, handler) = setup(options).await;
    let (_, account) = create_wallet(db.clone()).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(16_u32)
        .min_amount(1_f64)
        .min_decimal_amount("10000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("100000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("100000000000000".to_string())
        .rollup_fee_asset_symbol("MTT".to_string())
        .rollup_fee_asset_decimals(16_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .shielded_address(account.shielded_address.clone())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .deposit_quote(quote)
        .build();
    let deposit = handler.create(options).await.unwrap();
    let options = SendDepositOptions::builder()
        .deposit_id(deposit.id)
        .query_timeout_ms(100_u64)
        .tx_send_timeout_ms(1000_u64)
        .tx_wait_timeout_ms(2000_u64)
        .tx_wait_interval_ms(10_u64)
        .private_key(private_key)
        .asset_approve_confirmations(10_u64)
        .deposit_confirmations(10_u64)
        .build();
    let deposit = handler.send(options).await.unwrap();
    assert_eq!(deposit.status, DepositStatus::Queued as i32);
    assert_eq!(
        deposit.asset_approve_transaction_hash[0],
        asset_approve_tx_hash.encode_hex()
    );
    assert_eq!(deposit.queued_transaction_hash(), deposit_tx_hash.encode_hex());
    assert!(deposit.error_message.is_none());
    check_commitment(&deposit, db, 200010000_u64, true, false).await;
}

#[tokio::test]
async fn test_cross_chain_deposit_main_token_send_with_screening() {
    let (owner, private_key) = generate_private_key();
    let contract_address = ethers_address_from_string("0xd99F0C90BFDeDd5Bde0193b887c271C5458355Cf").unwrap();
    let deposit_tx_hash =
        TxHash::decode_hex("0xb56298dea53128b60ad2df8bf978c1a82d41798fa8272002f08e98fefdbc558f").unwrap();
    let amount = number_to_u256_decimal(10_f64, Some(18)).unwrap();
    let rollup_fee_amount = number_to_u256_decimal(0.01_f64, Some(18)).unwrap();
    let bridge_fee_amount = number_to_u256_decimal(0.00001_f64, Some(18)).unwrap();
    let executor_fee_amount = number_to_u256_decimal(0.0001_f64, Some(18)).unwrap();
    let mut assets = MockPublicAssets::new();
    assets
        .expect_balance_of()
        .withf(move |options| {
            options.chain_id == 97_u64 && options.owner == owner && options.timeout_ms == Some(100_u64)
        })
        .returning(|_| Ok(U256::from_dec_str("100000000000000000000").unwrap()));
    let mut deposit_contracts = MockDepositContracts::new();
    deposit_contracts
        .expect_cross_chain_deposit::<TypedTransaction, PrivateKeySigner<MockProviders>>()
        .withf(move |options| {
            options.chain_id == 97_u64
                && options.contract_address == contract_address
                && options.timeout_ms == Some(1000_u64)
                && options.request.amount == amount
                && options.request.rollup_fee == rollup_fee_amount
                && options.request.bridge_fee == bridge_fee_amount
                && options.request.executor_fee == executor_fee_amount
        })
        .returning(move |_| Ok(deposit_tx_hash));
    let mut transactions = MockTransactions::new();
    transactions
        .expect_create()
        .withf(|_, tx_type| tx_type == &mystiko_types::TransactionType::Eip1559)
        .returning(|_, _| Ok(TypedTransaction::Eip1559(Eip1559TransactionRequest::new())));
    transactions
        .expect_wait()
        .withf(move |options| {
            options.chain_id == 97_u64
                && options.tx_hash == deposit_tx_hash
                && options.confirmations == Some(10_u64)
                && options.interval_ms == Some(10_u64)
                && options.timeout_ms == Some(2000_u64)
        })
        .returning(move |_| {
            Ok(TransactionReceipt {
                transaction_hash: deposit_tx_hash,
                block_number: Some(U64::from(200010000_u64)),
                ..Default::default()
            })
        });
    let mut screening = MockScreeningClient::new();
    screening
        .expect_address_screening()
        .withf(move |options|{
            options.chain_id == 97_u64 &&
            options.asset == Some("0x0000000000000000000000000000000000000000".to_string() ) &&
            options.account == ethers_address_to_string(&owner)
        })
        .returning(|_| Ok(ScreeningResponse::builder()
            .deadline(0_u64)
            .signature("0x0f95f7effb9f3c8c20a6c78b2278a7ed2cee87ee5cf29031729124711623dd3b14e7e6fb419a61d9c262110c4812d2a37f2c137d2559192eee3f477cc08d92f51c".to_string())
            .build()));
    let options = MockOptions::builder()
        .assets(assets)
        .deposit_contracts(deposit_contracts)
        .transactions(transactions)
        .screening(screening)
        .build();
    let (db, handler) = setup(options).await;
    let (_, account) = create_wallet(db.clone()).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(18_u32)
        .min_amount(1_f64)
        .min_decimal_amount("1000000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("10000000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("10000000000000000".to_string())
        .rollup_fee_asset_symbol("BNB".to_string())
        .rollup_fee_asset_decimals(18_u32)
        .min_bridge_fee_amount(0.00001_f64)
        .min_bridge_fee_decimal_amount("10000000000000".to_string())
        .bridge_fee_asset_symbol("BNB".to_string())
        .bridge_fee_asset_decimals(18_u32)
        .min_executor_fee_amount(0.0001_f64)
        .min_executor_fee_decimal_amount("100000000000000".to_string())
        .executor_fee_asset_symbol("BNB".to_string())
        .executor_fee_asset_decimals(18_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(97_u64)
        .dst_chain_id(5_u64)
        .asset_symbol("BNB".to_string())
        .bridge_type(BridgeType::Tbridge as i32)
        .shielded_address(account.shielded_address.clone())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .bridge_fee_amount(0.00001_f64)
        .executor_fee_amount(0.0001_f64)
        .deposit_quote(quote)
        .build();
    let deposit = handler.create(options).await.unwrap();
    let options = SendDepositOptions::builder()
        .deposit_id(deposit.id)
        .query_timeout_ms(100_u64)
        .tx_send_timeout_ms(1000_u64)
        .tx_wait_timeout_ms(2000_u64)
        .tx_wait_interval_ms(10_u64)
        .private_key(private_key)
        .deposit_confirmations(10_u64)
        .build();
    let deposit = handler.send(options).await.unwrap();
    assert_eq!(deposit.status, DepositStatus::SrcSucceeded as i32);
    assert_eq!(deposit.src_chain_transaction_hash(), deposit_tx_hash.encode_hex());
    assert!(deposit.error_message.is_none());
    check_commitment(&deposit, db, 200010000_u64, true, true).await;
}

#[tokio::test]
async fn test_cross_chain_deposit_erc20_token_send_without_screening() {
    let (owner, private_key) = generate_private_key();
    let contract_address = ethers_address_from_string("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap();
    let asset_address = ethers_address_from_string("0x388C818CA8B9251b393131C08a736A67ccB19297").unwrap();
    let asset_approve_tx_hash =
        TxHash::decode_hex("0xf5079a68aa75c4b4f1cfd2ef50e23d60ef9211fb2f33481164ebc7e2cf536493").unwrap();
    let deposit_tx_hash =
        TxHash::decode_hex("0xb56298dea53128b60ad2df8bf978c1a82d41798fa8272002f08e98fefdbc558f").unwrap();
    let amount = number_to_u256_decimal(10_f64, Some(18)).unwrap();
    let rollup_fee_amount = number_to_u256_decimal(0.01_f64, Some(18)).unwrap();
    let bridge_fee_amount = number_to_u256_decimal(0.00001_f64, Some(18)).unwrap();
    let executor_fee_amount = number_to_u256_decimal(0.0001_f64, Some(18)).unwrap();
    let mut assets = MockPublicAssets::new();
    assets
        .expect_erc20_balance_of()
        .withf(move |options| {
            options.chain_id == 5_u64
                && options.owner == owner
                && options.timeout_ms == Some(100_u64)
                && options.asset_address == asset_address
        })
        .returning(|_| Ok(U256::from_dec_str("100000000000000000000").unwrap()));
    assets
        .expect_erc20_approve::<TypedTransaction, PrivateKeySigner<MockProviders>>()
        .withf(move |options| {
            let total_amount = amount
                .add(rollup_fee_amount)
                .add(bridge_fee_amount)
                .add(executor_fee_amount);
            options.chain_id == 5_u64
                && options.owner == owner
                && options.timeout_ms == Some(1000_u64)
                && options.asset_address == asset_address
                && options.recipient == contract_address
                && options.amount == total_amount
        })
        .returning(move |_| Ok(Some(asset_approve_tx_hash)));
    let mut deposit_contracts = MockDepositContracts::new();
    deposit_contracts
        .expect_cross_chain_deposit::<TypedTransaction, PrivateKeySigner<MockProviders>>()
        .withf(move |options| {
            options.chain_id == 5_u64
                && options.contract_address == contract_address
                && options.timeout_ms == Some(1000_u64)
                && options.request.amount == amount
                && options.request.rollup_fee == rollup_fee_amount
                && options.request.bridge_fee == bridge_fee_amount
                && options.request.executor_fee == executor_fee_amount
        })
        .returning(move |_| Ok(deposit_tx_hash));
    let mut transactions = MockTransactions::new();
    transactions
        .expect_create()
        .withf(|_, tx_type| tx_type == &mystiko_types::TransactionType::Legacy)
        .returning(|_, _| Ok(TypedTransaction::Legacy(TransactionRequest::new())));
    transactions
        .expect_wait()
        .withf(move |options| {
            options.chain_id == 5_u64
                && options.confirmations == Some(10_u64)
                && options.interval_ms == Some(10_u64)
                && options.timeout_ms == Some(2000_u64)
        })
        .returning(move |options| {
            Ok(TransactionReceipt {
                transaction_hash: options.tx_hash,
                block_number: Some(U64::from(200010000_u64)),
                ..Default::default()
            })
        });
    let mut commitment_pool_contracts = MockCommitmentPoolContracts::new();
    commitment_pool_contracts
        .expect_is_historic_commitment()
        .returning(move |options| {
            if options.timeout_ms == Some(2_u64) || options.timeout_ms == Some(3_u64) {
                Ok(true)
            } else {
                Ok(false)
            }
        });
    let options = MockOptions::builder()
        .assets(assets)
        .deposit_contracts(deposit_contracts)
        .commitment_pool_contracts(commitment_pool_contracts)
        .transactions(transactions)
        .build();
    let (db, handler) = setup(options).await;
    let (_, account) = create_wallet(db.clone()).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(18_u32)
        .min_amount(1_f64)
        .min_decimal_amount("1000000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("10000000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("10000000000000000".to_string())
        .rollup_fee_asset_symbol("mBNB".to_string())
        .rollup_fee_asset_decimals(18_u32)
        .min_bridge_fee_amount(0.00001_f64)
        .min_bridge_fee_decimal_amount("10000000000000".to_string())
        .bridge_fee_asset_symbol("mBNB".to_string())
        .bridge_fee_asset_decimals(18_u32)
        .min_executor_fee_amount(0.0001_f64)
        .min_executor_fee_decimal_amount("100000000000000".to_string())
        .executor_fee_asset_symbol("mBNB".to_string())
        .executor_fee_asset_decimals(18_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(5_u64)
        .dst_chain_id(97_u64)
        .asset_symbol("mBNB".to_string())
        .bridge_type(BridgeType::Tbridge as i32)
        .shielded_address(account.shielded_address.clone())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .bridge_fee_amount(0.00001_f64)
        .executor_fee_amount(0.0001_f64)
        .deposit_quote(quote)
        .build();
    let deposit = handler.create(options).await.unwrap();
    let options = SendDepositOptions::builder()
        .deposit_id(deposit.id)
        .query_timeout_ms(100_u64)
        .tx_send_timeout_ms(1000_u64)
        .tx_wait_timeout_ms(2000_u64)
        .tx_wait_interval_ms(10_u64)
        .private_key(private_key)
        .asset_approve_confirmations(10_u64)
        .deposit_confirmations(10_u64)
        .build();
    let deposit = handler.send(options).await.unwrap();
    assert_eq!(deposit.status, DepositStatus::SrcSucceeded as i32);
    assert_eq!(
        deposit.asset_approve_transaction_hash[0],
        asset_approve_tx_hash.encode_hex()
    );
    assert_eq!(deposit.src_chain_transaction_hash(), deposit_tx_hash.encode_hex());
    assert!(deposit.error_message.is_none());
    check_commitment(&deposit, db.clone(), 200010000_u64, true, true).await;

    let fix_options = FixDepositStatusOptions::builder()
        .query_timeout_ms(1)
        .deposit_id(deposit.id.clone())
        .build();
    let deposit = handler.fix_status(fix_options.clone()).await.unwrap();
    assert_eq!(deposit.status, DepositStatus::SrcSucceeded as i32);

    let fix_options = FixDepositStatusOptions::builder()
        .query_timeout_ms(2)
        .deposit_id(deposit.id.clone())
        .build();
    let deposit = handler.fix_status(fix_options.clone()).await.unwrap();
    assert_eq!(deposit.status, DepositStatus::Queued as i32);
    assert!(deposit.error_message.is_none());
    let mut cms = db.commitments.find_all().await.unwrap();
    assert_eq!(cms.len(), 1);
    assert_eq!(cms[0].data.status, CommitmentStatus::Queued as i32);
    cms[0].data.included_transaction_hash = Some("0x123".to_string());
    db.commitments.update(&cms[0]).await.unwrap();
    let mut deposits = db.deposits.find_all().await.unwrap();
    assert_eq!(deposits.len(), 1);
    deposits[0].data.status = DepositStatus::Failed as i32;
    db.deposits.update(&deposits[0]).await.unwrap();

    let fix_options = FixDepositStatusOptions::builder()
        .query_timeout_ms(3)
        .deposit_id(deposit.id.clone())
        .build();
    let deposit = handler.fix_status(fix_options.clone()).await.unwrap();
    assert_eq!(deposit.status, DepositStatus::Included as i32);
    assert!(deposit.error_message.is_none());
    let cms = db.commitments.find_all().await.unwrap();
    assert_eq!(cms.len(), 1);
    assert_eq!(cms[0].data.status, CommitmentStatus::Included as i32);
}

#[tokio::test]
async fn test_loop_deposit_insufficient_main_token() {
    let (owner, private_key) = generate_private_key();
    let mut assets = MockPublicAssets::new();
    assets
        .expect_balance_of()
        .withf(move |options| {
            options.chain_id == 97_u64 && options.owner == owner && options.timeout_ms == Some(100_u64)
        })
        .returning(|_| Ok(U256::from_dec_str("100000000000000").unwrap()));
    let options = MockOptions::builder().assets(assets).build();
    let (db, handler) = setup(options).await;
    let (_, account) = create_wallet(db).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(18_u32)
        .min_amount(1_f64)
        .min_decimal_amount("1000000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("10000000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("10000000000000000".to_string())
        .rollup_fee_asset_symbol("BNB".to_string())
        .rollup_fee_asset_decimals(18_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(97_u64)
        .asset_symbol("BNB".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .shielded_address(account.shielded_address.clone())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .deposit_quote(quote)
        .build();
    let deposit = handler.create(options).await.unwrap();
    let options = SendDepositOptions::builder()
        .deposit_id(deposit.id)
        .query_timeout_ms(100_u64)
        .tx_send_timeout_ms(1000_u64)
        .tx_wait_timeout_ms(2000_u64)
        .tx_wait_interval_ms(10_u64)
        .private_key(private_key)
        .deposit_confirmations(10_u64)
        .build();
    let result = handler.send(options).await;
    assert_eq!(
        result.unwrap_err().to_string(),
        "insufficient balance for asset BNB amount 10.01"
    );
}

#[tokio::test]
async fn test_loop_deposit_insufficient_erc20_token() {
    let (owner, private_key) = generate_private_key();
    let asset_address = ethers_address_from_string("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a").unwrap();
    let mut assets = MockPublicAssets::new();
    assets
        .expect_erc20_balance_of()
        .withf(move |options| {
            options.chain_id == 5_u64
                && options.owner == owner
                && options.timeout_ms == Some(100_u64)
                && options.asset_address == asset_address
        })
        .returning(|_| Ok(U256::from_dec_str("10000000000000").unwrap()));
    let options = MockOptions::builder().assets(assets).build();
    let (db, handler) = setup(options).await;
    let (_, account) = create_wallet(db).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(16_u32)
        .min_amount(1_f64)
        .min_decimal_amount("10000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("100000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("100000000000000".to_string())
        .rollup_fee_asset_symbol("MTT".to_string())
        .rollup_fee_asset_decimals(16_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .shielded_address(account.shielded_address.clone())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .deposit_quote(quote)
        .build();
    let deposit = handler.create(options).await.unwrap();
    let options = SendDepositOptions::builder()
        .deposit_id(deposit.id)
        .query_timeout_ms(100_u64)
        .tx_send_timeout_ms(1000_u64)
        .tx_wait_timeout_ms(2000_u64)
        .tx_wait_interval_ms(10_u64)
        .private_key(private_key)
        .asset_approve_confirmations(10_u64)
        .deposit_confirmations(10_u64)
        .build();
    let result = handler.send(options).await;
    assert_eq!(
        result.unwrap_err().to_string(),
        "insufficient balance for asset MTT amount 10.01"
    );
}

#[tokio::test]
async fn test_loop_deposit_send_with_wrong_status() {
    let (_, private_key) = generate_private_key();
    let (db, handler) = setup(Default::default()).await;
    let (_, account) = create_wallet(db.clone()).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(18_u32)
        .min_amount(1_f64)
        .min_decimal_amount("1000000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("10000000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("10000000000000000".to_string())
        .rollup_fee_asset_symbol("BNB".to_string())
        .rollup_fee_asset_decimals(18_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(97_u64)
        .asset_symbol("BNB".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .shielded_address(account.shielded_address.clone())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .deposit_quote(quote)
        .build();
    let mut deposit = mystiko_core::Deposit::document_from_proto(handler.create(options).await.unwrap()).unwrap();
    deposit.data.status = DepositStatus::SrcSucceeded as i32;
    db.deposits.update(&deposit).await.unwrap();
    let options = SendDepositOptions::builder()
        .deposit_id(deposit.id)
        .private_key(private_key)
        .build();
    let result = handler.send(options).await;
    assert_eq!(
        result.unwrap_err().to_string(),
        "cannot send deposit with status=SrcSucceeded"
    );
}

#[tokio::test]
async fn test_loop_deposit_send_with_duplicate_commitment() {
    let pool_address = ethers_address_from_string("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap();
    let mut commitment_pool_contracts = MockCommitmentPoolContracts::new();
    commitment_pool_contracts
        .expect_is_historic_commitment()
        .withf(move |options| options.chain_id == 97_u64 && options.contract_address == pool_address)
        .returning(|_| Ok(true));
    let (_, private_key) = generate_private_key();
    let (db, handler) = setup(
        MockOptions::builder()
            .commitment_pool_contracts(commitment_pool_contracts)
            .build(),
    )
    .await;
    let (_, account) = create_wallet(db).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(18_u32)
        .min_amount(1_f64)
        .min_decimal_amount("1000000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("10000000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("10000000000000000".to_string())
        .rollup_fee_asset_symbol("BNB".to_string())
        .rollup_fee_asset_decimals(18_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(97_u64)
        .asset_symbol("BNB".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .shielded_address(account.shielded_address.clone())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .deposit_quote(quote)
        .build();
    let deposit = handler.create(options).await.unwrap();
    let options = SendDepositOptions::builder()
        .deposit_id(deposit.id)
        .private_key(private_key)
        .build();
    let result = handler.send(options).await;
    assert_eq!(
        result.unwrap_err().to_string(),
        format!(
            "duplicate commitment={} in chain_id=97 contract_address={}",
            deposit.commitment_hash,
            ethers_address_to_string(&pool_address)
        )
    );
}

#[tokio::test]
async fn test_cross_chain_deposit_send_with_duplicate_commitment() {
    let pool_address = ethers_address_from_string("0x5050F69a9786F081509234F1a7F4684b5E5b76C9").unwrap();
    let mut commitment_pool_contracts = MockCommitmentPoolContracts::new();
    commitment_pool_contracts
        .expect_is_historic_commitment()
        .withf(move |options| options.chain_id == 5_u64 && options.contract_address == pool_address)
        .returning(|_| Ok(true));
    let (_, private_key) = generate_private_key();
    let (db, handler) = setup(
        MockOptions::builder()
            .commitment_pool_contracts(commitment_pool_contracts)
            .build(),
    )
    .await;
    let (_, account) = create_wallet(db).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(18_u32)
        .min_amount(1_f64)
        .min_decimal_amount("1000000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("10000000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("10000000000000000".to_string())
        .rollup_fee_asset_symbol("BNB".to_string())
        .rollup_fee_asset_decimals(18_u32)
        .min_bridge_fee_amount(0.00001_f64)
        .min_bridge_fee_decimal_amount("10000000000000".to_string())
        .bridge_fee_asset_symbol("BNB".to_string())
        .bridge_fee_asset_decimals(18_u32)
        .min_executor_fee_amount(0.0001_f64)
        .min_executor_fee_decimal_amount("100000000000000".to_string())
        .executor_fee_asset_symbol("BNB".to_string())
        .executor_fee_asset_decimals(18_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(97_u64)
        .dst_chain_id(5_u64)
        .asset_symbol("BNB".to_string())
        .bridge_type(BridgeType::Tbridge as i32)
        .shielded_address(account.shielded_address.clone())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .bridge_fee_amount(0.00001_f64)
        .executor_fee_amount(0.0001_f64)
        .deposit_quote(quote)
        .build();
    let deposit = handler.create(options).await.unwrap();
    let options = SendDepositOptions::builder()
        .deposit_id(deposit.id)
        .private_key(private_key)
        .build();
    let result = handler.send(options).await;
    assert_eq!(
        result.unwrap_err().to_string(),
        format!(
            "duplicate commitment={} in chain_id=5 contract_address={}",
            deposit.commitment_hash,
            ethers_address_to_string(&pool_address)
        )
    );
}

#[tokio::test]
async fn test_crud() {
    let (db, handler) = setup(Default::default()).await;
    let (_, account) = create_wallet(db.clone()).await;
    let discarded_deposits = generate_deposits(account.shielded_address, &handler).await;
    let (_, account) = create_wallet(db).await;
    let mut deposits = generate_deposits(account.shielded_address, &handler).await;
    deposits.sort_by_key(|deposit| deposit.id.clone());

    let mut found_deposits = handler.find_all().await.unwrap();
    found_deposits.sort_by_key(|deposit| deposit.id.clone());
    assert_eq!(found_deposits, deposits);

    found_deposits = handler
        .find(SubFilter::equal(DocumentColumn::Id, deposits[0].id.clone()))
        .await
        .unwrap();
    assert_eq!(found_deposits.len(), 1);
    assert_eq!(found_deposits[0], deposits[0]);

    found_deposits = handler
        .find(SubFilter::equal(DocumentColumn::Id, discarded_deposits[0].id.clone()))
        .await
        .unwrap();
    assert_eq!(found_deposits.len(), 0);

    let mut found_deposit = handler
        .find_one(SubFilter::equal(DocumentColumn::Id, deposits[0].id.clone()))
        .await
        .unwrap();
    assert_eq!(found_deposit.unwrap(), deposits[0]);

    found_deposit = handler
        .find_one(SubFilter::equal(DocumentColumn::Id, discarded_deposits[0].id.clone()))
        .await
        .unwrap();
    assert!(found_deposit.is_none());

    found_deposit = handler.find_by_id(deposits[0].id.clone()).await.unwrap();
    assert_eq!(found_deposit.unwrap(), deposits[0]);

    assert_eq!(handler.count_all().await.unwrap(), 4);
    assert_eq!(
        handler
            .count(SubFilter::in_list(
                DocumentColumn::Id,
                vec![deposits[0].id.clone(), deposits[1].id.clone()]
            ))
            .await
            .unwrap(),
        2
    );
    assert_eq!(
        handler
            .count(SubFilter::in_list(
                DocumentColumn::Id,
                vec![discarded_deposits[0].id.clone(), discarded_deposits[1].id.clone()]
            ))
            .await
            .unwrap(),
        0
    );

    deposits[0].status = DepositStatus::Queued as i32;
    let updated_deposit = handler.update(deposits[0].clone()).await.unwrap();
    assert_eq!(updated_deposit.status, DepositStatus::Queued as i32);

    deposits[1].status = DepositStatus::SrcPending as i32;
    deposits[2].status = DepositStatus::SrcSucceeded as i32;
    let mut updated_deposits = handler
        .update_batch(vec![deposits[1].clone(), deposits[2].clone()])
        .await
        .unwrap();
    updated_deposits.sort_by_key(|deposit| deposit.id.clone());
    assert_eq!(updated_deposits[0].status, DepositStatus::SrcPending as i32);
    assert_eq!(updated_deposits[1].status, DepositStatus::SrcSucceeded as i32);

    let column_values = ColumnValues::new().set_value(DepositColumn::Status, DepositStatus::Failed as i32);
    handler
        .update_by_filter(
            column_values.clone(),
            SubFilter::equal(DocumentColumn::Id, deposits[3].id.clone()),
        )
        .await
        .unwrap();
    found_deposit = handler.find_by_id(deposits[3].id.clone()).await.unwrap();
    assert_eq!(found_deposit.unwrap().status, DepositStatus::Failed as i32);

    handler
        .update_by_filter(
            column_values.clone(),
            SubFilter::equal(DocumentColumn::Id, discarded_deposits[3].id.clone()),
        )
        .await
        .unwrap();
    found_deposit = handler.find_by_id(discarded_deposits[3].id.clone()).await.unwrap();
    assert_eq!(found_deposit.unwrap().status, DepositStatus::Unspecified as i32);

    handler.update_all(column_values).await.unwrap();
    found_deposits = handler.find_all().await.unwrap();
    found_deposits.iter().for_each(|deposit| {
        assert_eq!(deposit.status, DepositStatus::Failed as i32);
    });
    discarded_deposits.iter().for_each(|deposit| {
        assert_eq!(deposit.status, DepositStatus::Unspecified as i32);
    });

    handler.delete(deposits[0].clone()).await.unwrap();
    assert_eq!(handler.count_all().await.unwrap(), 3);

    handler.delete_batch(vec![deposits[1].clone()]).await.unwrap();
    assert_eq!(handler.count_all().await.unwrap(), 2);

    handler
        .delete_by_filter(SubFilter::equal(DocumentColumn::Id, deposits[2].id.clone()))
        .await
        .unwrap();
    assert_eq!(handler.count_all().await.unwrap(), 1);

    handler.delete_all().await.unwrap();
    assert_eq!(handler.count_all().await.unwrap(), 0);
}

#[derive(Debug, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
struct MockOptions {
    deposit_contracts: MockDepositContracts,
    commitment_pool_contracts: Option<MockCommitmentPoolContracts>,
    assets: MockPublicAssets,
    transactions: MockTransactions,
    providers: HashMap<u64, MockProvider>,
    screening: MockScreeningClient,
}

type DepositsOptionsType = DepositsOptions<
    SqlStatementFormatter,
    SqliteStorage,
    MockPublicAssets,
    MockDepositContracts,
    MockCommitmentPoolContracts,
    MockTransactions,
    MockProviders,
>;
type DepositsType = Deposits<
    SqlStatementFormatter,
    SqliteStorage,
    MockPublicAssets,
    MockDepositContracts,
    MockCommitmentPoolContracts,
    MockTransactions,
    MockProviders,
>;
type DatabaseType = Database<SqlStatementFormatter, SqliteStorage>;

async fn setup(options: MockOptions) -> (Arc<DatabaseType>, DepositsType) {
    let _ = env_logger::builder()
        .filter_module("mystiko_core", log::LevelFilter::Info)
        .try_init();
    let config = Arc::new(
        MystikoConfig::from_json_file("tests/files/mystiko/config.json")
            .await
            .unwrap(),
    );
    let database = Arc::new(create_database().await);
    database.migrate().await.unwrap();
    let mut raw_providers = options
        .providers
        .into_iter()
        .map(|(chain_id, provider)| {
            let provider = Arc::new(Provider::new(ProviderWrapper::new(Box::new(provider))));
            (chain_id, provider)
        })
        .collect::<HashMap<_, _>>();
    let mut providers = MockProviders::new();
    providers.expect_get_provider().returning(move |chain_id| {
        raw_providers
            .remove(&chain_id)
            .ok_or(anyhow::anyhow!("No provider for chain_id {}", chain_id))
    });
    let commitment_pool_contracts = if let Some(commitment_pool_contracts) = options.commitment_pool_contracts {
        commitment_pool_contracts
    } else {
        let mut commitment_pool_contracts = MockCommitmentPoolContracts::new();
        commitment_pool_contracts
            .expect_is_historic_commitment()
            .returning(|_| Ok(false));
        commitment_pool_contracts
    };
    let screening = Arc::new(Box::new(options.screening) as Box<dyn ScreeningClient>);
    let handler = DepositsType::new(
        DepositsOptionsType::builder()
            .config(config)
            .db(database.clone())
            .signer_providers(providers)
            .assets(options.assets)
            .deposit_contracts(options.deposit_contracts)
            .commitment_pool_contracts(commitment_pool_contracts)
            .transactions(options.transactions)
            .screening(screening)
            .build(),
    );
    (database, handler)
}

async fn create_wallet(db: Arc<DatabaseType>) -> (Wallet, Account) {
    let wallets = Wallets::new(db.clone());
    let accounts = Accounts::new(db);
    let wallet = wallets
        .create(&CreateWalletOptions::builder().password("P@ssw0rd").build())
        .await
        .unwrap();
    let account = accounts
        .create(&CreateAccountOptions::builder().wallet_password("P@ssw0rd").build())
        .await
        .unwrap();
    (wallet, account)
}

async fn generate_deposits(shielded_address: String, handler: &DepositsType) -> Vec<Deposit> {
    let mut deposits = vec![];
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(18_u32)
        .min_amount(1_f64)
        .min_decimal_amount("1000000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("10000000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("10000000000000000".to_string())
        .rollup_fee_asset_symbol("BNB".to_string())
        .rollup_fee_asset_decimals(18_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(97_u64)
        .asset_symbol("BNB".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .shielded_address(shielded_address.clone())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .deposit_quote(quote)
        .build();
    deposits.push(handler.create(options).await.unwrap());

    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(16_u32)
        .min_amount(1_f64)
        .min_decimal_amount("10000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("100000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("100000000000000".to_string())
        .rollup_fee_asset_symbol("MTT".to_string())
        .rollup_fee_asset_decimals(16_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(5_u64)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .shielded_address(shielded_address.clone())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .deposit_quote(quote)
        .build();
    deposits.push(handler.create(options).await.unwrap());

    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(18_u32)
        .min_amount(1_f64)
        .min_decimal_amount("1000000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("10000000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("10000000000000000".to_string())
        .rollup_fee_asset_symbol("BNB".to_string())
        .rollup_fee_asset_decimals(18_u32)
        .min_bridge_fee_amount(0.00001_f64)
        .min_bridge_fee_decimal_amount("10000000000000".to_string())
        .bridge_fee_asset_symbol("BNB".to_string())
        .bridge_fee_asset_decimals(18_u32)
        .min_executor_fee_amount(0.0001_f64)
        .min_executor_fee_decimal_amount("100000000000000".to_string())
        .executor_fee_asset_symbol("BNB".to_string())
        .executor_fee_asset_decimals(18_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(97_u64)
        .dst_chain_id(5_u64)
        .asset_symbol("BNB".to_string())
        .bridge_type(BridgeType::Tbridge as i32)
        .shielded_address(shielded_address.clone())
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .bridge_fee_amount(0.00001_f64)
        .executor_fee_amount(0.0001_f64)
        .deposit_quote(quote)
        .build();
    deposits.push(handler.create(options).await.unwrap());

    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .asset_decimals(18_u32)
        .min_amount(1_f64)
        .min_decimal_amount("1000000000000000000".to_string())
        .max_amount(10000_f64)
        .max_decimal_amount("10000000000000000000000".to_string())
        .min_rollup_fee_amount(0.01_f64)
        .min_rollup_fee_decimal_amount("10000000000000000".to_string())
        .rollup_fee_asset_symbol("mBNB".to_string())
        .rollup_fee_asset_decimals(18_u32)
        .min_bridge_fee_amount(0.00001_f64)
        .min_bridge_fee_decimal_amount("10000000000000".to_string())
        .bridge_fee_asset_symbol("mBNB".to_string())
        .bridge_fee_asset_decimals(18_u32)
        .min_executor_fee_amount(0.0001_f64)
        .min_executor_fee_decimal_amount("100000000000000".to_string())
        .executor_fee_asset_symbol("mBNB".to_string())
        .executor_fee_asset_decimals(18_u32)
        .build();
    let options = CreateDepositOptions::builder()
        .chain_id(5_u64)
        .dst_chain_id(97_u64)
        .asset_symbol("mBNB".to_string())
        .bridge_type(BridgeType::Tbridge as i32)
        .shielded_address(shielded_address)
        .amount(10_f64)
        .rollup_fee_amount(0.01_f64)
        .bridge_fee_amount(0.00001_f64)
        .executor_fee_amount(0.0001_f64)
        .deposit_quote(quote)
        .build();
    deposits.push(handler.create(options).await.unwrap());

    deposits
}

async fn check_commitment(
    deposit: &Deposit,
    db: Arc<DatabaseType>,
    block_number: u64,
    should_exist: bool,
    cross_chain: bool,
) {
    let commitment_hash = BigUint::from_str(&deposit.commitment_hash).unwrap();
    let commitment = db
        .commitments
        .find_one(SubFilter::equal(
            CommitmentColumn::CommitmentHash,
            commitment_hash.clone(),
        ))
        .await
        .unwrap();
    if should_exist {
        let commitment = commitment.unwrap();
        assert_eq!(commitment.data.chain_id, deposit.dst_chain_id);
        assert_eq!(commitment.data.bridge_type, deposit.bridge_type);
        assert_eq!(commitment.data.block_number, block_number);
        assert_eq!(commitment.data.asset_symbol, deposit.asset_symbol);
        assert_eq!(commitment.data.asset_decimals, deposit.asset_decimals);
        assert_eq!(commitment.data.asset_address, deposit.asset_address);
        assert_eq!(commitment.data.commitment_hash, commitment_hash);
        assert_eq!(
            commitment.data.amount,
            Some(deposit.decimal_amount_as_biguint().unwrap())
        );
        assert_eq!(
            commitment.data.rollup_fee_amount,
            Some(deposit.rollup_fee_decimal_amount_as_biguint().unwrap())
        );
        assert_eq!(commitment.data.encrypted_note.unwrap(), deposit.encrypted_note);
        assert_eq!(commitment.data.shielded_address.unwrap(), deposit.shielded_address);
        assert_eq!(commitment.data.queued_transaction_hash, deposit.queued_transaction_hash);
        assert_eq!(
            commitment.data.src_chain_transaction_hash,
            deposit.src_chain_transaction_hash
        );
        assert_eq!(
            commitment.data.included_transaction_hash,
            deposit.included_transaction_hash
        );
        assert!(!commitment.data.spent);
        assert!(commitment.data.leaf_index.is_none());
        assert!(commitment.data.nullifier.is_none());
        if cross_chain {
            assert_eq!(commitment.data.contract_address, deposit.dst_pool_address);
            assert_eq!(commitment.data.status, CommitmentStatus::SrcSucceeded as i32);
            assert_eq!(
                commitment.data.src_chain_block_number.unwrap(),
                commitment.data.block_number
            );
        } else {
            assert_eq!(commitment.data.contract_address, deposit.pool_address);
            assert_eq!(commitment.data.status, CommitmentStatus::Queued as i32);
            assert!(commitment.data.src_chain_block_number.is_none());
        }
    } else {
        assert!(commitment.is_none());
    }
}
