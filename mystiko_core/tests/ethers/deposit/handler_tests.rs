use crate::common::{MockProvider, MockProviders, MockTransactionSigner};
use crate::ethers::{parse_call_args, TimeoutProvider};
use ethers_core::abi::AbiDecode;
use ethers_core::abi::AbiEncode;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Bytes, TxHash, U256};
use ethers_providers::ProviderError;
use mystiko_abi::commitment_pool::CommitmentPoolCalls;
use mystiko_abi::mystiko_v2_bridge::MystikoV2BridgeCalls;
use mystiko_abi::mystiko_v2_loop::MystikoV2LoopCalls;
use mystiko_config::MystikoConfig;
use mystiko_core::{
    CrossChainDepositOptions, DepositContractHandler, DepositContracts, DepositOptions, DepositQuoteOptions,
};
use mystiko_ethers::{JsonRpcClientWrapper, Provider, ProviderWrapper};
use mystiko_types::ContractType;
use mystiko_utils::address::{ethers_address_from_string, ethers_address_to_string};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

#[tokio::test]
async fn test_loop_deposit_quote() {
    let min_amount = U256::from_dec_str("1000000000000000000").unwrap();
    let max_amount = U256::from_dec_str("5000000000000000000").unwrap();
    let min_rollup_fee_amount = U256::from_dec_str("100000000000000000").unwrap();
    let config = create_config().await;
    let provider = mock_quote_provider(
        config.clone(),
        5_u64,
        min_amount,
        max_amount,
        min_rollup_fee_amount,
        None,
        None,
    );
    let deposits = setup(config, HashMap::from([(5_u64, provider)]));
    let quote_options = DepositQuoteOptions::builder()
        .chain_id(5_u64)
        .contract_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
        .build();
    let quote = deposits.quote(quote_options).await.unwrap();
    assert_eq!(quote.min_amount, min_amount);
    assert_eq!(quote.max_amount, max_amount);
    assert_eq!(quote.min_rollup_fee_amount, min_rollup_fee_amount);
}

#[tokio::test]
async fn test_loop_deposit_quote_remote_error() {
    let mut provider = MockProvider::new();
    provider
        .expect_request()
        .returning(|_, _| Err(ProviderError::CustomError("Error".to_string())));
    let config = create_config().await;
    let deposits = setup(config, HashMap::from([(5_u64, provider)]));
    let quote_options = DepositQuoteOptions::builder()
        .chain_id(5_u64)
        .contract_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
        .build();
    let quote = deposits.quote(quote_options).await.unwrap();
    assert_eq!(quote.min_amount, U256::from_dec_str("10000000000000000").unwrap());
    assert_eq!(quote.max_amount, U256::from_dec_str("10000000000000000000").unwrap());
    assert_eq!(
        quote.min_rollup_fee_amount,
        U256::from_dec_str("40000000000000000").unwrap()
    );
}

#[tokio::test]
async fn test_loop_deposit_quote_remote_timeout() {
    let provider = TimeoutProvider::builder().timeout_ms(2000_u64).build();
    let config = create_config().await;
    let deposits = setup(config, HashMap::from([(5_u64, provider)]));
    let quote_options = DepositQuoteOptions::builder()
        .chain_id(5_u64)
        .contract_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
        .timeout_ms(10_u64)
        .build();
    let quote = deposits.quote(quote_options).await.unwrap();
    assert_eq!(quote.min_amount, U256::from_dec_str("10000000000000000").unwrap());
    assert_eq!(quote.max_amount, U256::from_dec_str("10000000000000000000").unwrap());
    assert_eq!(
        quote.min_rollup_fee_amount,
        U256::from_dec_str("40000000000000000").unwrap()
    );
}

#[tokio::test]
async fn test_cross_chain_deposit_quote() {
    let min_amount = U256::from_dec_str("1000000000000000000").unwrap();
    let max_amount = U256::from_dec_str("5000000000000000000").unwrap();
    let min_rollup_fee_amount1 = U256::from_dec_str("100000000000000000").unwrap();
    let min_rollup_fee_amount2 = U256::from_dec_str("150000000000000000").unwrap();
    let min_bridge_fee_amount = U256::from_dec_str("200000000000000000").unwrap();
    let min_executor_fee_amount = U256::from_dec_str("300000000000000000").unwrap();
    let config = create_config().await;
    let provider1 = mock_quote_provider(
        config.clone(),
        5_u64,
        min_amount,
        max_amount,
        min_rollup_fee_amount1,
        Some(min_bridge_fee_amount),
        Some(min_executor_fee_amount),
    );
    let provider2 = mock_quote_provider(
        config.clone(),
        97_u64,
        min_amount,
        max_amount,
        min_rollup_fee_amount2,
        Some(min_bridge_fee_amount),
        Some(min_executor_fee_amount),
    );
    let deposits = setup(config, HashMap::from([(5_u64, provider1), (97_u64, provider2)]));
    let quote_options = DepositQuoteOptions::builder()
        .chain_id(5_u64)
        .contract_address(ethers_address_from_string("0x961F315A836542e603A3df2E0dd9d4ECd06ebC67").unwrap())
        .build();
    let quote = deposits.quote(quote_options).await.unwrap();
    assert_eq!(quote.min_amount, min_amount);
    assert_eq!(quote.max_amount, max_amount);
    assert_eq!(quote.min_rollup_fee_amount, min_rollup_fee_amount2);
    assert_eq!(quote.min_bridge_fee_amount.unwrap(), min_bridge_fee_amount);
    assert_eq!(quote.min_executor_fee_amount.unwrap(), min_executor_fee_amount);
}

#[tokio::test]
async fn test_cross_chain_deposit_quote_remote_error() {
    let mut provider1 = MockProvider::new();
    let mut provider2 = MockProvider::new();
    provider1
        .expect_request()
        .returning(|_, _| Err(ProviderError::CustomError("Error".to_string())));
    provider2
        .expect_request()
        .returning(|_, _| Err(ProviderError::CustomError("Error".to_string())));
    let config = create_config().await;
    let deposits = setup(config, HashMap::from([(5_u64, provider1), (97_u64, provider2)]));
    let quote_options = DepositQuoteOptions::builder()
        .chain_id(5_u64)
        .contract_address(ethers_address_from_string("0x961F315A836542e603A3df2E0dd9d4ECd06ebC67").unwrap())
        .build();
    let quote = deposits.quote(quote_options).await.unwrap();
    assert_eq!(quote.min_amount, U256::from_dec_str("10000000000000000").unwrap());
    assert_eq!(quote.max_amount, U256::from_dec_str("100000000000000000").unwrap());
    assert_eq!(
        quote.min_rollup_fee_amount,
        U256::from_dec_str("60000000000000000").unwrap()
    );
    assert_eq!(
        quote.min_bridge_fee_amount.unwrap(),
        U256::from_dec_str("20000000000000000").unwrap()
    );
    assert_eq!(
        quote.min_executor_fee_amount.unwrap(),
        U256::from_dec_str("30000000000000000").unwrap()
    );
}

#[tokio::test]
async fn test_cross_chain_deposit_quote_remote_timeout() {
    let provider1 = TimeoutProvider::builder().timeout_ms(2000_u64).build();
    let provider2 = TimeoutProvider::builder().timeout_ms(2000_u64).build();
    let config = create_config().await;
    let deposits = setup(config, HashMap::from([(5_u64, provider1), (97_u64, provider2)]));
    let quote_options = DepositQuoteOptions::builder()
        .chain_id(5_u64)
        .contract_address(ethers_address_from_string("0x961F315A836542e603A3df2E0dd9d4ECd06ebC67").unwrap())
        .timeout_ms(10_u64)
        .build();
    let quote = deposits.quote(quote_options).await.unwrap();
    assert_eq!(quote.min_amount, U256::from_dec_str("10000000000000000").unwrap());
    assert_eq!(quote.max_amount, U256::from_dec_str("100000000000000000").unwrap());
    assert_eq!(
        quote.min_rollup_fee_amount,
        U256::from_dec_str("60000000000000000").unwrap()
    );
    assert_eq!(
        quote.min_bridge_fee_amount.unwrap(),
        U256::from_dec_str("20000000000000000").unwrap()
    );
    assert_eq!(
        quote.min_executor_fee_amount.unwrap(),
        U256::from_dec_str("30000000000000000").unwrap()
    );
}

#[tokio::test]
async fn test_main_token_loop_deposit() {
    let contract_address = ethers_address_from_string("0x390d485F4D43212D4ae8Cdd967a711514ed5a54f").unwrap();
    let amount = U256::from_dec_str("1000000000000000000").unwrap();
    let rollup_fee = U256::from_dec_str("100000000000000000").unwrap();
    let commitment = U256::from_dec_str("1234").unwrap();
    let random_s = 2345_u128;
    let hash_k = U256::from_dec_str("3456").unwrap();
    let encrypted_notes = Bytes::from_str("0x1234").unwrap();
    let encrypted_notes_clone = encrypted_notes.clone();
    let expected_tx_hash =
        TxHash::decode_hex("0x8cbbb491f260cb9a810f81ebf8b51ae1adf322466232181b2eb2bb105b45f0b9").unwrap();
    let mut signer = MockTransactionSigner::new();
    signer
        .expect_send_transaction()
        .withf(move |chain_id, tx| {
            let calls = parse_loop_deposit_call(tx);
            if let MystikoV2LoopCalls::Deposit(deposit) = calls {
                let request = deposit.request;
                request.commitment == commitment
                    && request.random_s == random_s
                    && request.hash_k == hash_k
                    && request.encrypted_note == encrypted_notes_clone
                    && request.rollup_fee == rollup_fee
                    && request.amount == amount
                    && *chain_id == 97_u64
                    && *tx.to().unwrap().as_address().unwrap() == contract_address
                    && *tx.value().unwrap() == amount + rollup_fee
            } else {
                false
            }
        })
        .returning(move |_, _| Ok(expected_tx_hash));
    let deposit_options = DepositOptions::<TypedTransaction, MockTransactionSigner>::builder()
        .chain_id(97_u64)
        .contract_address(contract_address)
        .amount(amount)
        .rollup_fee(rollup_fee)
        .commitment(commitment)
        .random_s(random_s)
        .hash_k(hash_k)
        .encrypted_notes(encrypted_notes)
        .signer(signer)
        .build();
    let config = create_config().await;
    let deposits = setup(config, HashMap::from([(97_u64, MockProvider::new())]));
    let tx_hash = deposits.deposit(deposit_options).await.unwrap();
    assert_eq!(tx_hash, expected_tx_hash);
}

#[tokio::test]
async fn test_erc20_token_loop_deposit() {
    let contract_address = ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap();
    let amount = U256::from_dec_str("1000000000000000000").unwrap();
    let rollup_fee = U256::from_dec_str("100000000000000000").unwrap();
    let commitment = U256::from_dec_str("1234").unwrap();
    let random_s = 2345_u128;
    let hash_k = U256::from_dec_str("3456").unwrap();
    let encrypted_notes = Bytes::from_str("0x1234").unwrap();
    let encrypted_notes_clone = encrypted_notes.clone();
    let expected_tx_hash =
        TxHash::decode_hex("0x8cbbb491f260cb9a810f81ebf8b51ae1adf322466232181b2eb2bb105b45f0b9").unwrap();
    let mut signer = MockTransactionSigner::new();
    signer
        .expect_send_transaction()
        .withf(move |chain_id, tx| {
            let calls = parse_loop_deposit_call(tx);
            if let MystikoV2LoopCalls::Deposit(deposit) = calls {
                let request = deposit.request;
                request.commitment == commitment
                    && request.random_s == random_s
                    && request.hash_k == hash_k
                    && request.encrypted_note == encrypted_notes_clone
                    && request.rollup_fee == rollup_fee
                    && request.amount == amount
                    && *chain_id == 5_u64
                    && *tx.to().unwrap().as_address().unwrap() == contract_address
                    && tx.value().is_none()
            } else {
                false
            }
        })
        .returning(move |_, _| Ok(expected_tx_hash));
    let deposit_options = DepositOptions::<TypedTransaction, MockTransactionSigner>::builder()
        .chain_id(5_u64)
        .contract_address(contract_address)
        .amount(amount)
        .rollup_fee(rollup_fee)
        .commitment(commitment)
        .random_s(random_s)
        .hash_k(hash_k)
        .encrypted_notes(encrypted_notes)
        .signer(signer)
        .build();
    let config = create_config().await;
    let deposits = setup(config, HashMap::from([(5_u64, MockProvider::new())]));
    let tx_hash = deposits.deposit(deposit_options).await.unwrap();
    assert_eq!(tx_hash, expected_tx_hash);
}

#[tokio::test]
async fn test_main_token_cross_chain_deposit() {
    let contract_address = ethers_address_from_string("0xd99F0C90BFDeDd5Bde0193b887c271C5458355Cf").unwrap();
    let amount = U256::from_dec_str("1000000000000000000").unwrap();
    let rollup_fee = U256::from_dec_str("100000000000000000").unwrap();
    let bridge_fee = U256::from_dec_str("200000000000000000").unwrap();
    let executor_fee = U256::from_dec_str("300000000000000000").unwrap();
    let commitment = U256::from_dec_str("1234").unwrap();
    let random_s = 2345_u128;
    let hash_k = U256::from_dec_str("3456").unwrap();
    let encrypted_notes = Bytes::from_str("0x1234").unwrap();
    let encrypted_notes_clone = encrypted_notes.clone();
    let expected_tx_hash =
        TxHash::decode_hex("0x8cbbb491f260cb9a810f81ebf8b51ae1adf322466232181b2eb2bb105b45f0b9").unwrap();
    let mut signer = MockTransactionSigner::new();
    signer
        .expect_send_transaction()
        .withf(move |chain_id, tx| {
            let calls = parse_cross_chain_deposit_call(tx);
            if let MystikoV2BridgeCalls::Deposit(deposit) = calls {
                let request = deposit.request;
                request.commitment == commitment
                    && request.random_s == random_s
                    && request.hash_k == hash_k
                    && request.encrypted_note == encrypted_notes_clone
                    && request.amount == amount
                    && request.rollup_fee == rollup_fee
                    && request.bridge_fee == bridge_fee
                    && request.executor_fee == executor_fee
                    && *chain_id == 97_u64
                    && *tx.to().unwrap().as_address().unwrap() == contract_address
                    && *tx.value().unwrap() == amount + rollup_fee + bridge_fee + executor_fee
            } else {
                false
            }
        })
        .returning(move |_, _| Ok(expected_tx_hash));
    let deposit_options = CrossChainDepositOptions::<TypedTransaction, MockTransactionSigner>::builder()
        .chain_id(97_u64)
        .contract_address(contract_address)
        .amount(amount)
        .rollup_fee(rollup_fee)
        .bridge_fee(bridge_fee)
        .executor_fee(executor_fee)
        .commitment(commitment)
        .random_s(random_s)
        .hash_k(hash_k)
        .encrypted_notes(encrypted_notes)
        .signer(signer)
        .build();
    let config = create_config().await;
    let deposits = setup(config, HashMap::from([(97_u64, MockProvider::new())]));
    let tx_hash = deposits.cross_chain_deposit(deposit_options).await.unwrap();
    assert_eq!(tx_hash, expected_tx_hash);
}

#[tokio::test]
async fn test_erc20_token_cross_chain_deposit() {
    let contract_address = ethers_address_from_string("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap();
    let amount = U256::from_dec_str("1000000000000000000").unwrap();
    let rollup_fee = U256::from_dec_str("100000000000000000").unwrap();
    let bridge_fee = U256::from_dec_str("200000000000000000").unwrap();
    let executor_fee = U256::from_dec_str("300000000000000000").unwrap();
    let commitment = U256::from_dec_str("1234").unwrap();
    let random_s = 2345_u128;
    let hash_k = U256::from_dec_str("3456").unwrap();
    let encrypted_notes = Bytes::from_str("0x1234").unwrap();
    let encrypted_notes_clone = encrypted_notes.clone();
    let expected_tx_hash =
        TxHash::decode_hex("0x8cbbb491f260cb9a810f81ebf8b51ae1adf322466232181b2eb2bb105b45f0b9").unwrap();
    let mut signer = MockTransactionSigner::new();
    signer
        .expect_send_transaction()
        .withf(move |chain_id, tx| {
            let calls = parse_cross_chain_deposit_call(tx);
            if let MystikoV2BridgeCalls::Deposit(deposit) = calls {
                let request = deposit.request;
                request.commitment == commitment
                    && request.random_s == random_s
                    && request.hash_k == hash_k
                    && request.encrypted_note == encrypted_notes_clone
                    && request.amount == amount
                    && request.rollup_fee == rollup_fee
                    && request.bridge_fee == bridge_fee
                    && request.executor_fee == executor_fee
                    && *chain_id == 5_u64
                    && *tx.to().unwrap().as_address().unwrap() == contract_address
                    && tx.value().is_none()
            } else {
                false
            }
        })
        .returning(move |_, _| Ok(expected_tx_hash));
    let deposit_options = CrossChainDepositOptions::<TypedTransaction, MockTransactionSigner>::builder()
        .chain_id(5_u64)
        .contract_address(contract_address)
        .amount(amount)
        .rollup_fee(rollup_fee)
        .bridge_fee(bridge_fee)
        .executor_fee(executor_fee)
        .commitment(commitment)
        .random_s(random_s)
        .hash_k(hash_k)
        .encrypted_notes(encrypted_notes)
        .signer(signer)
        .build();
    let config = create_config().await;
    let deposits = setup(config, HashMap::from([(5_u64, MockProvider::new())]));
    let tx_hash = deposits.cross_chain_deposit(deposit_options).await.unwrap();
    assert_eq!(tx_hash, expected_tx_hash);
}

#[tokio::test]
async fn test_deposit_unsupported_chain() {
    let config = create_config().await;
    let deposits = setup(config, HashMap::from([(5_u64, MockProvider::new())]));
    let deposit_options = DepositOptions::<TypedTransaction, MockTransactionSigner>::builder()
        .chain_id(10001_u64)
        .contract_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
        .amount(U256::from_dec_str("1000000000000000000").unwrap())
        .rollup_fee(U256::from_dec_str("100000000000000000").unwrap())
        .commitment(U256::from_dec_str("1234").unwrap())
        .random_s(2345_u128)
        .hash_k(U256::from_dec_str("3456").unwrap())
        .encrypted_notes(Bytes::from_str("0x1234").unwrap())
        .signer(MockTransactionSigner::new())
        .build();
    let tx_hash = deposits.deposit(deposit_options).await;
    assert!(tx_hash.is_err());
    assert_eq!(tx_hash.unwrap_err().to_string(), "unsupported chain_id=10001");
}

#[tokio::test]
async fn test_deposit_unsupported_contract() {
    let config = create_config().await;
    let deposits = setup(config, HashMap::from([(5_u64, MockProvider::new())]));
    let deposit_options = DepositOptions::<TypedTransaction, MockTransactionSigner>::builder()
        .chain_id(5_u64)
        .contract_address(ethers_address_from_string("0x125E577F580857D7AF995D20104C6c7B96a3274d").unwrap())
        .amount(U256::from_dec_str("1000000000000000000").unwrap())
        .rollup_fee(U256::from_dec_str("100000000000000000").unwrap())
        .commitment(U256::from_dec_str("1234").unwrap())
        .random_s(2345_u128)
        .hash_k(U256::from_dec_str("3456").unwrap())
        .encrypted_notes(Bytes::from_str("0x1234").unwrap())
        .signer(MockTransactionSigner::new())
        .build();
    let tx_hash = deposits.deposit(deposit_options).await;
    assert!(tx_hash.is_err());
    assert_eq!(
        tx_hash.unwrap_err().to_string(),
        "unsupported chain_id=5, \
        contract_address=0x125E577F580857D7AF995D20104C6c7B96a3274d"
    );
}

fn mock_quote_provider(
    config: Arc<MystikoConfig>,
    chain_id: u64,
    min_amount: U256,
    max_amount: U256,
    min_rollup_fee_amount: U256,
    min_bridge_fee_amount: Option<U256>,
    min_executor_fee_amount: Option<U256>,
) -> MockProvider {
    let mut provider = MockProvider::new();
    let returning_config = config.clone();
    provider
        .expect_request()
        .withf(move |method, params| {
            let tx = parse_call_args(params);
            let contract_type = parse_contract_type(chain_id, config.clone(), &tx);
            match contract_type {
                ContractType::Deposit => {
                    if min_bridge_fee_amount.is_some() && min_executor_fee_amount.is_some() {
                        let calls = parse_cross_chain_deposit_call(&tx);
                        match calls {
                            MystikoV2BridgeCalls::GetMinAmount(_) => method == "eth_call",
                            MystikoV2BridgeCalls::GetMaxAmount(_) => method == "eth_call",
                            MystikoV2BridgeCalls::GetMinBridgeFee(_) => method == "eth_call",
                            MystikoV2BridgeCalls::GetMinExecutorFee(_) => method == "eth_call",
                            _ => false,
                        }
                    } else {
                        let calls = parse_loop_deposit_call(&tx);
                        match calls {
                            MystikoV2LoopCalls::GetMinAmount(_) => method == "eth_call",
                            MystikoV2LoopCalls::GetMaxAmount(_) => method == "eth_call",
                            _ => false,
                        }
                    }
                }
                ContractType::Pool => {
                    let calls = parse_pool_call(&tx);
                    match calls {
                        CommitmentPoolCalls::GetMinRollupFee(_) => method == "eth_call",
                        _ => false,
                    }
                }
            }
        })
        .returning(move |_, params| {
            let tx = parse_call_args(&params);
            let contract_type = parse_contract_type(chain_id, returning_config.clone(), &tx);
            match contract_type {
                ContractType::Deposit => {
                    if let (Some(min_bridge_fee_amount), Some(min_executor_fee_amount)) =
                        (min_bridge_fee_amount, min_executor_fee_amount)
                    {
                        let calls = parse_cross_chain_deposit_call(&tx);
                        match calls {
                            MystikoV2BridgeCalls::GetMinAmount(_) => Ok(serde_json::json!(min_amount.encode_hex())),
                            MystikoV2BridgeCalls::GetMaxAmount(_) => Ok(serde_json::json!(max_amount.encode_hex())),
                            MystikoV2BridgeCalls::GetMinBridgeFee(_) => {
                                Ok(serde_json::json!(min_bridge_fee_amount.encode_hex()))
                            }
                            MystikoV2BridgeCalls::GetMinExecutorFee(_) => {
                                Ok(serde_json::json!(min_executor_fee_amount.encode_hex()))
                            }
                            _ => Err(ProviderError::CustomError("Invalid call".to_string())),
                        }
                    } else {
                        let calls = parse_loop_deposit_call(&tx);
                        match calls {
                            MystikoV2LoopCalls::GetMinAmount(_) => Ok(serde_json::json!(min_amount.encode_hex())),
                            MystikoV2LoopCalls::GetMaxAmount(_) => Ok(serde_json::json!(max_amount.encode_hex())),
                            _ => Err(ProviderError::CustomError("Invalid call".to_string())),
                        }
                    }
                }
                ContractType::Pool => {
                    let calls = parse_pool_call(&tx);
                    match calls {
                        CommitmentPoolCalls::GetMinRollupFee(_) => {
                            Ok(serde_json::json!(min_rollup_fee_amount.encode_hex()))
                        }
                        _ => Err(ProviderError::CustomError("Invalid call".to_string())),
                    }
                }
            }
        });
    provider
}

fn setup<P>(config: Arc<MystikoConfig>, providers: HashMap<u64, P>) -> DepositContracts<MockProviders>
where
    P: JsonRpcClientWrapper + 'static,
{
    let _ = env_logger::builder()
        .filter_module("mystiko_core", log::LevelFilter::Info)
        .try_init();
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
    DepositContracts::builder().providers(providers).config(config).build()
}

async fn create_config() -> Arc<MystikoConfig> {
    Arc::new(
        MystikoConfig::from_json_file("tests/files/mystiko/config.json")
            .await
            .unwrap(),
    )
}

fn parse_contract_type(chain_id: u64, config: Arc<MystikoConfig>, tx: &TypedTransaction) -> ContractType {
    let address = tx.to().unwrap().as_address().unwrap();
    let address = ethers_address_to_string(address);
    let contract_config = config.find_contract_by_address(chain_id, &address).unwrap();
    contract_config.contract_type().clone()
}

fn parse_loop_deposit_call(tx: &TypedTransaction) -> MystikoV2LoopCalls {
    MystikoV2LoopCalls::decode(tx.data().unwrap()).unwrap()
}

fn parse_cross_chain_deposit_call(tx: &TypedTransaction) -> MystikoV2BridgeCalls {
    MystikoV2BridgeCalls::decode(tx.data().unwrap()).unwrap()
}

fn parse_pool_call(tx: &TypedTransaction) -> CommitmentPoolCalls {
    CommitmentPoolCalls::decode(tx.data().unwrap()).unwrap()
}
