use crate::ethers::{parse_call_args, MockProvider, MockProviders};
use ethers_core::abi::AbiDecode;
use ethers_core::abi::AbiEncode;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::U256;
use ethers_providers::ProviderError;
use mystiko_abi::commitment_pool::CommitmentPoolCalls;
use mystiko_abi::mystiko_v2_bridge::MystikoV2BridgeCalls;
use mystiko_abi::mystiko_v2_loop::MystikoV2LoopCalls;
use mystiko_config::MystikoConfig;
use mystiko_core::{DepositContract, DepositQuoteOptions, Deposits};
use mystiko_ethers::{JsonRpcClientWrapper, Provider, ProviderWrapper};
use mystiko_types::ContractType;
use mystiko_utils::address::{ethers_address_from_string, ethers_address_to_string};
use std::collections::HashMap;
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
    let provider = crate::ethers::TimeoutProvider::builder().timeout_ms(2000_u64).build();
    let config = create_config().await;
    let deposits = setup(config, HashMap::from([(5_u64, provider)]));
    let quote_options = DepositQuoteOptions::builder()
        .chain_id(5_u64)
        .contract_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
        .query_timeout_ms(10_u64)
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
    let provider1 = crate::ethers::TimeoutProvider::builder().timeout_ms(2000_u64).build();
    let provider2 = crate::ethers::TimeoutProvider::builder().timeout_ms(2000_u64).build();
    let config = create_config().await;
    let deposits = setup(config, HashMap::from([(5_u64, provider1), (97_u64, provider2)]));
    let quote_options = DepositQuoteOptions::builder()
        .chain_id(5_u64)
        .contract_address(ethers_address_from_string("0x961F315A836542e603A3df2E0dd9d4ECd06ebC67").unwrap())
        .query_timeout_ms(10_u64)
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

fn setup<P>(config: Arc<MystikoConfig>, providers: HashMap<u64, P>) -> Deposits<MockProviders>
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
    Deposits::builder().providers(providers).config(config).build()
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
