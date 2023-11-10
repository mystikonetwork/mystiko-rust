use crate::common::{create_database, MockProvider, MockProviders};
use crate::handler::{MockDepositContracts, MockPublicAssets, MockTransactions};
use ethers_core::abi::{AbiDecode, AbiEncode};
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, Eip1559TransactionRequest, TransactionReceipt, TransactionRequest, TxHash, U256};
use ethers_signers::{LocalWallet, Signer};
use mystiko_config::MystikoConfig;
use mystiko_core::{
    AccountHandler, Accounts, Database, DepositHandler, DepositQuote, Deposits, DepositsOptions, PrivateKeySigner,
    WalletHandler, Wallets,
};
use mystiko_ethers::{Provider, ProviderWrapper};
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::document::v1::{Account, Wallet};
use mystiko_protos::core::handler::v1::{
    CreateAccountOptions, CreateDepositOptions, CreateWalletOptions, QuoteDepositOptions, SendDepositOptions,
};
use mystiko_protos::core::v1::DepositStatus;
use mystiko_storage::SqlStatementFormatter;
use mystiko_storage_sqlite::SqliteStorage;
use mystiko_utils::address::ethers_address_from_string;
use mystiko_utils::convert::number_to_u256_decimal;
use mystiko_utils::hex::encode_hex;
use std::collections::HashMap;
use std::ops::Add;
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
    assert_eq!(quote.min_amount, 1_f64);
    assert_eq!(quote.max_amount, 10000_f64);
    assert_eq!(quote.min_rollup_fee_amount, 0.01_f64);
    assert_eq!(quote.rollup_fee_asset_symbol, "MTT");
    assert!(quote.min_bridge_fee_amount.is_none());
    assert!(quote.bridge_fee_asset_symbol.is_none());
    assert!(quote.min_executor_fee_amount.is_none());
    assert!(quote.executor_fee_asset_symbol.is_none());
    assert_eq!(quote.recommended_amounts, [1_f64, 10_f64]);
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
    assert_eq!(quote.min_amount, 1_f64);
    assert_eq!(quote.max_amount, 10000_f64);
    assert_eq!(quote.min_rollup_fee_amount, 0.01_f64);
    assert_eq!(quote.rollup_fee_asset_symbol, "MTT");
    assert_eq!(quote.min_bridge_fee_amount.unwrap(), 0.00001_f64);
    assert_eq!(quote.bridge_fee_asset_symbol.unwrap(), "ETH");
    assert_eq!(quote.min_executor_fee_amount.unwrap(), 0.0001_f64);
    assert_eq!(quote.executor_fee_asset_symbol.unwrap(), "MTT");
    assert_eq!(quote.recommended_amounts, [1_f64, 10_f64]);
}

#[tokio::test]
async fn test_loop_deposit_summary() {
    let (_, handler) = setup(Default::default()).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .min_amount(1_f64)
        .max_amount(10000_f64)
        .min_rollup_fee_amount(0.01_f64)
        .rollup_fee_asset_symbol("MTT".to_string())
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
    assert_eq!(summary.shielded_address, "secret address");
    assert_eq!(summary.amount, 10_f64);
    assert_eq!(summary.rollup_fee_amount, 0.01_f64);
    assert_eq!(summary.rollup_fee_asset_symbol, "MTT");
    assert_eq!(summary.bridge_type.unwrap(), BridgeType::Loop as i32);
    assert_eq!(summary.total_amounts, HashMap::from([("MTT".to_string(), 10.01_f64)]));
    assert!(summary.dst_chain_id.is_none());
    assert!(summary.bridge_fee_amount.is_none());
    assert!(summary.bridge_fee_asset_symbol.is_none());
    assert!(summary.executor_fee_amount.is_none());
    assert!(summary.executor_fee_asset_symbol.is_none());
}

#[tokio::test]
async fn test_cross_chain_deposit_summary() {
    let (_, handler) = setup(Default::default()).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .min_amount(1_f64)
        .max_amount(10000_f64)
        .min_rollup_fee_amount(0.01_f64)
        .rollup_fee_asset_symbol("MTT".to_string())
        .min_bridge_fee_amount(0.00001_f64)
        .bridge_fee_asset_symbol("ETH".to_string())
        .min_executor_fee_amount(0.0001_f64)
        .executor_fee_asset_symbol("MTT".to_string())
        .build();
    let options = CreateDepositOptions::builder()
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
    let summary = handler.summary(options).await.unwrap();
    assert_eq!(summary.chain_id, 5_u64);
    assert_eq!(summary.asset_symbol, "MTT");
    assert_eq!(summary.shielded_address, "secret address");
    assert_eq!(summary.amount, 10_f64);
    assert_eq!(summary.rollup_fee_amount, 0.01_f64);
    assert_eq!(summary.rollup_fee_asset_symbol, "MTT");
    assert_eq!(summary.dst_chain_id.unwrap(), 97_u64);
    assert_eq!(summary.bridge_fee_amount.unwrap(), 0.00001_f64);
    assert_eq!(summary.bridge_fee_asset_symbol.unwrap(), "ETH");
    assert_eq!(summary.executor_fee_amount.unwrap(), 0.0001_f64);
    assert_eq!(summary.executor_fee_asset_symbol.unwrap(), "MTT");
    assert_eq!(summary.bridge_type.unwrap(), BridgeType::Tbridge as i32);
    assert_eq!(
        summary.total_amounts,
        HashMap::from([("MTT".to_string(), 10.0101_f64), ("ETH".to_string(), 0.00001_f64),])
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
    assert_eq!(summary.shielded_address, "secret address");
    assert_eq!(summary.amount, 10_f64);
    assert_eq!(summary.rollup_fee_amount, 0.01_f64);
    assert_eq!(summary.rollup_fee_asset_symbol, "MTT");
    assert_eq!(summary.bridge_type.unwrap(), BridgeType::Loop as i32);
    assert_eq!(summary.total_amounts, HashMap::from([("MTT".to_string(), 10.01_f64)]));
    assert!(summary.dst_chain_id.is_none());
    assert!(summary.bridge_fee_amount.is_none());
    assert!(summary.bridge_fee_asset_symbol.is_none());
    assert!(summary.executor_fee_amount.is_none());
    assert!(summary.executor_fee_asset_symbol.is_none());
}

#[tokio::test]
async fn test_deposit_summary_with_errors() {
    let (_, handler) = setup(Default::default()).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .min_amount(1_f64)
        .max_amount(10000_f64)
        .min_rollup_fee_amount(0.01_f64)
        .rollup_fee_asset_symbol("MTT".to_string())
        .min_bridge_fee_amount(0.00001_f64)
        .bridge_fee_asset_symbol("ETH".to_string())
        .min_executor_fee_amount(0.0001_f64)
        .executor_fee_asset_symbol("MTT".to_string())
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
    options.rollup_fee_amount = 0.001_f64;
    let err = handler.summary(options.clone()).await.unwrap_err();
    assert_eq!(
        err.to_string(),
        "rollup fee amount 0.001 is less than min_rollup_fee_amount 0.01"
    );

    //min bridge fee
    options.rollup_fee_amount = 0.01_f64;
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
        .min_amount(1_f64)
        .max_amount(10000_f64)
        .min_rollup_fee_amount(0.01_f64)
        .rollup_fee_asset_symbol("BNB".to_string())
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
    assert_eq!(deposit.amount, 10_f64);
    assert_eq!(deposit.rollup_fee_amount, 0.01_f64);
    assert_eq!(deposit.shielded_recipient_address, account.shielded_address);
    assert_eq!(deposit.wallet_id, wallet.id);
    assert_eq!(deposit.status, DepositStatus::Unspecified as i32);
    assert_eq!(deposit.bridge_type, BridgeType::Loop as i32);
    assert!(deposit.asset_address.is_none());
    assert!(deposit.bridge_fee_amount.is_none());
    assert!(deposit.bridge_fee_asset_symbol.is_none());
    assert!(deposit.bridge_fee_asset_address.is_none());
    assert!(deposit.executor_fee_amount.is_none());
    assert!(deposit.executor_fee_asset_symbol.is_none());
    assert!(deposit.executor_fee_asset_address.is_none());
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
        .min_amount(1_f64)
        .max_amount(10000_f64)
        .min_rollup_fee_amount(0.01_f64)
        .rollup_fee_asset_symbol("MTT".to_string())
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
    assert_eq!(deposit.amount, 10_f64);
    assert_eq!(deposit.rollup_fee_amount, 0.01_f64);
    assert_eq!(deposit.shielded_recipient_address, account.shielded_address);
    assert_eq!(deposit.wallet_id, wallet.id);
    assert_eq!(deposit.status, DepositStatus::Unspecified as i32);
    assert_eq!(deposit.bridge_type, BridgeType::Loop as i32);
    assert_eq!(
        deposit.asset_address.unwrap(),
        "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a"
    );
    assert!(deposit.bridge_fee_amount.is_none());
    assert!(deposit.bridge_fee_asset_symbol.is_none());
    assert!(deposit.bridge_fee_asset_address.is_none());
    assert!(deposit.executor_fee_amount.is_none());
    assert!(deposit.executor_fee_asset_symbol.is_none());
    assert!(deposit.executor_fee_asset_address.is_none());
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
        .min_amount(1_f64)
        .max_amount(10000_f64)
        .min_rollup_fee_amount(0.01_f64)
        .rollup_fee_asset_symbol("BNB".to_string())
        .min_bridge_fee_amount(0.00001_f64)
        .bridge_fee_asset_symbol("BNB".to_string())
        .min_executor_fee_amount(0.0001_f64)
        .executor_fee_asset_symbol("BNB".to_string())
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
    assert_eq!(deposit.amount, 10_f64);
    assert_eq!(deposit.rollup_fee_amount, 0.01_f64);
    assert_eq!(deposit.shielded_recipient_address, account.shielded_address);
    assert_eq!(deposit.wallet_id, wallet.id);
    assert_eq!(deposit.status, DepositStatus::Unspecified as i32);
    assert_eq!(deposit.bridge_type, BridgeType::Tbridge as i32);
    assert!(deposit.asset_address.is_none());
    assert_eq!(deposit.bridge_fee_amount.unwrap(), 0.00001_f64);
    assert_eq!(deposit.bridge_fee_asset_symbol.unwrap(), "BNB");
    assert!(deposit.bridge_fee_asset_address.is_none());
    assert_eq!(deposit.executor_fee_amount, Some(0.0001_f64));
    assert_eq!(deposit.executor_fee_asset_symbol.unwrap(), "BNB");
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
        .min_amount(1_f64)
        .max_amount(10000_f64)
        .min_rollup_fee_amount(0.01_f64)
        .rollup_fee_asset_symbol("mBNB".to_string())
        .min_bridge_fee_amount(0.00001_f64)
        .bridge_fee_asset_symbol("mBNB".to_string())
        .min_executor_fee_amount(0.0001_f64)
        .executor_fee_asset_symbol("mBNB".to_string())
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
    assert_eq!(deposit.amount, 10_f64);
    assert_eq!(deposit.rollup_fee_amount, 0.01_f64);
    assert_eq!(deposit.shielded_recipient_address, account.shielded_address);
    assert_eq!(deposit.wallet_id, wallet.id);
    assert_eq!(deposit.status, DepositStatus::Unspecified as i32);
    assert_eq!(deposit.bridge_type, BridgeType::Tbridge as i32);
    assert_eq!(
        deposit.asset_address.unwrap(),
        "0x388C818CA8B9251b393131C08a736A67ccB19297"
    );
    assert_eq!(deposit.bridge_fee_amount.unwrap(), 0.00001_f64);
    assert_eq!(deposit.bridge_fee_asset_symbol.unwrap(), "mBNB");
    assert_eq!(
        deposit.bridge_fee_asset_address.unwrap(),
        "0x388C818CA8B9251b393131C08a736A67ccB19297"
    );
    assert_eq!(deposit.executor_fee_amount, Some(0.0001_f64));
    assert_eq!(deposit.executor_fee_asset_symbol.unwrap(), "mBNB");
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
async fn test_loop_deposit_main_token_send() {
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
                && options.amount == amount
                && options.rollup_fee == rollup_fee_amount
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
            Ok(Some(TransactionReceipt {
                transaction_hash: deposit_tx_hash,
                ..Default::default()
            }))
        });
    let options = MockOptions::builder()
        .assets(assets)
        .deposit_contracts(deposit_contracts)
        .transactions(transactions)
        .build();
    let (db, handler) = setup(options).await;
    let (_, account) = create_wallet(db).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .min_amount(1_f64)
        .max_amount(10000_f64)
        .min_rollup_fee_amount(0.01_f64)
        .rollup_fee_asset_symbol("BNB".to_string())
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
    let deposit = handler.send(options).await.unwrap();
    assert_eq!(deposit.status, DepositStatus::Queued as i32);
    assert_eq!(deposit.queued_transaction_hash(), deposit_tx_hash.encode_hex());
    assert!(deposit.error_message.is_none());
}

#[tokio::test]
async fn test_loop_deposit_erc20_token_send() {
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
                && options.amount == amount
                && options.rollup_fee == rollup_fee_amount
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
            Ok(Some(TransactionReceipt {
                transaction_hash: options.tx_hash,
                ..Default::default()
            }))
        });
    let options = MockOptions::builder()
        .assets(assets)
        .deposit_contracts(deposit_contracts)
        .transactions(transactions)
        .build();
    let (db, handler) = setup(options).await;
    let (_, account) = create_wallet(db).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .min_amount(1_f64)
        .max_amount(10000_f64)
        .min_rollup_fee_amount(0.01_f64)
        .rollup_fee_asset_symbol("MTT".to_string())
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
}

#[tokio::test]
async fn test_cross_chain_deposit_main_token_send() {
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
                && options.amount == amount
                && options.rollup_fee == rollup_fee_amount
                && options.bridge_fee == bridge_fee_amount
                && options.executor_fee == executor_fee_amount
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
            Ok(Some(TransactionReceipt {
                transaction_hash: deposit_tx_hash,
                ..Default::default()
            }))
        });
    let options = MockOptions::builder()
        .assets(assets)
        .deposit_contracts(deposit_contracts)
        .transactions(transactions)
        .build();
    let (db, handler) = setup(options).await;
    let (_, account) = create_wallet(db).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .min_amount(1_f64)
        .max_amount(10000_f64)
        .min_rollup_fee_amount(0.01_f64)
        .rollup_fee_asset_symbol("BNB".to_string())
        .min_bridge_fee_amount(0.00001_f64)
        .bridge_fee_asset_symbol("BNB".to_string())
        .min_executor_fee_amount(0.0001_f64)
        .executor_fee_asset_symbol("BNB".to_string())
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
}

#[tokio::test]
async fn test_cross_chain_deposit_erc20_token_send() {
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
                && options.amount == amount
                && options.rollup_fee == rollup_fee_amount
                && options.bridge_fee == bridge_fee_amount
                && options.executor_fee == executor_fee_amount
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
            Ok(Some(TransactionReceipt {
                transaction_hash: options.tx_hash,
                ..Default::default()
            }))
        });
    let options = MockOptions::builder()
        .assets(assets)
        .deposit_contracts(deposit_contracts)
        .transactions(transactions)
        .build();
    let (db, handler) = setup(options).await;
    let (_, account) = create_wallet(db).await;
    let quote = mystiko_protos::core::handler::v1::DepositQuote::builder()
        .min_amount(1_f64)
        .max_amount(10000_f64)
        .min_rollup_fee_amount(0.01_f64)
        .rollup_fee_asset_symbol("mBNB".to_string())
        .min_bridge_fee_amount(0.00001_f64)
        .bridge_fee_asset_symbol("mBNB".to_string())
        .min_executor_fee_amount(0.0001_f64)
        .executor_fee_asset_symbol("mBNB".to_string())
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
        .min_amount(1_f64)
        .max_amount(10000_f64)
        .min_rollup_fee_amount(0.01_f64)
        .rollup_fee_asset_symbol("BNB".to_string())
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
        .min_amount(1_f64)
        .max_amount(10000_f64)
        .min_rollup_fee_amount(0.01_f64)
        .rollup_fee_asset_symbol("MTT".to_string())
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

#[derive(Debug, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
struct MockOptions {
    deposit_contracts: MockDepositContracts,
    assets: MockPublicAssets,
    transactions: MockTransactions,
    providers: HashMap<u64, MockProvider>,
}

type DepositHandlerV1OptionsType = DepositsOptions<
    SqlStatementFormatter,
    SqliteStorage,
    MockPublicAssets,
    MockDepositContracts,
    MockTransactions,
    MockProviders,
>;
type DepositHandlerV1Type = Deposits<
    SqlStatementFormatter,
    SqliteStorage,
    MockPublicAssets,
    MockDepositContracts,
    MockTransactions,
    MockProviders,
>;
type DatabaseType = Database<SqlStatementFormatter, SqliteStorage>;

async fn setup(options: MockOptions) -> (Arc<DatabaseType>, DepositHandlerV1Type) {
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
    let handler = DepositHandlerV1Type::new(
        DepositHandlerV1OptionsType::builder()
            .config(config)
            .db(database.clone())
            .signer_providers(providers)
            .assets(options.assets)
            .deposit_contracts(options.deposit_contracts)
            .transactions(options.transactions)
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

fn generate_private_key() -> (Address, String) {
    let local_wallet = LocalWallet::new(&mut rand::thread_rng());
    let address = local_wallet.address();
    let key = local_wallet.signer().to_bytes();
    (address, encode_hex(key))
}
