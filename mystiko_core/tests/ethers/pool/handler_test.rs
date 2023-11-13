use crate::common::{MockProvider, MockProviders};
use crate::ethers::{mock_providers, parse_call_args, TimeoutProvider};
use ethers_core::abi::{AbiDecode, AbiEncode};
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::U256;
use mystiko_abi::commitment_pool::CommitmentPoolCalls;
use mystiko_core::{
    CommitmentPoolContractHandler, CommitmentPoolContracts, IsHistoricCommitmentOptions, IsKnownRootOptions,
    IsSpentNullifierOptions,
};
use mystiko_ethers::JsonRpcClientWrapper;
use mystiko_utils::address::ethers_address_from_string;
use std::collections::HashMap;

#[tokio::test]
async fn test_is_historic_commitment() {
    let contract_address = ethers_address_from_string("0x390d485F4D43212D4ae8Cdd967a711514ed5a54f").unwrap();
    let commitment = U256::from(0xdeadbeef_u64);
    let mut provider = MockProvider::new();
    provider
        .expect_request()
        .withf(move |method, params| {
            let tx = parse_call_args(params);
            let call = parse_pool_call(&tx);
            match call {
                CommitmentPoolCalls::IsHistoricCommitment(args) => {
                    method == "eth_call" && args.commitment == commitment
                }
                _ => false,
            }
        })
        .returning(|_, _| Ok(serde_json::json!(U256::one().encode_hex())));
    let commitment_pools = setup(HashMap::from([(1, provider)]));
    let options = IsHistoricCommitmentOptions::builder()
        .chain_id(1_u64)
        .contract_address(contract_address)
        .commitment_hash(commitment)
        .build();
    assert!(commitment_pools.is_historic_commitment(options).await.unwrap());
}

#[tokio::test]
async fn test_is_historic_commitment_timeout_error() {
    let contract_address = ethers_address_from_string("0x390d485F4D43212D4ae8Cdd967a711514ed5a54f").unwrap();
    let commitment = U256::from(0xdeadbeef_u64);
    let provider = TimeoutProvider::builder().timeout_ms(1000_u64).build();
    let commitment_pools = setup(HashMap::from([(1, provider)]));
    let options = IsHistoricCommitmentOptions::builder()
        .chain_id(1_u64)
        .contract_address(contract_address)
        .commitment_hash(commitment)
        .timeout_ms(1_u64)
        .build();
    let result = commitment_pools.is_historic_commitment(options).await;
    assert_eq!(
        result.unwrap_err().to_string(),
        "querying is_historic_commitment timed out after 1 ms"
    );
}

#[tokio::test]
async fn test_is_spent_nullifier() {
    let contract_address = ethers_address_from_string("0x390d485F4D43212D4ae8Cdd967a711514ed5a54f").unwrap();
    let nullifier = U256::from(0xdeadbeef_u64);
    let mut provider = MockProvider::new();
    provider
        .expect_request()
        .withf(move |method, params| {
            let tx = parse_call_args(params);
            let call = parse_pool_call(&tx);
            match call {
                CommitmentPoolCalls::IsSpentSerialNumber(args) => {
                    method == "eth_call" && args.serial_number == nullifier
                }
                _ => false,
            }
        })
        .returning(|_, _| Ok(serde_json::json!(U256::one().encode_hex())));
    let commitment_pools = setup(HashMap::from([(1, provider)]));
    let options = IsSpentNullifierOptions::builder()
        .chain_id(1_u64)
        .contract_address(contract_address)
        .nullifier(nullifier)
        .build();
    assert!(commitment_pools.is_spent_nullifier(options).await.unwrap());
}

#[tokio::test]
async fn test_is_spent_nullifier_timeout_error() {
    let contract_address = ethers_address_from_string("0x390d485F4D43212D4ae8Cdd967a711514ed5a54f").unwrap();
    let nullifier = U256::from(0xdeadbeef_u64);
    let provider = TimeoutProvider::builder().timeout_ms(1000_u64).build();
    let commitment_pools = setup(HashMap::from([(1, provider)]));
    let options = IsSpentNullifierOptions::builder()
        .chain_id(1_u64)
        .contract_address(contract_address)
        .nullifier(nullifier)
        .timeout_ms(1_u64)
        .build();
    let result = commitment_pools.is_spent_nullifier(options).await;
    assert_eq!(
        result.unwrap_err().to_string(),
        "querying is_spent_nullifier timed out after 1 ms"
    );
}

#[tokio::test]
async fn test_is_known_root() {
    let contract_address = ethers_address_from_string("0x390d485F4D43212D4ae8Cdd967a711514ed5a54f").unwrap();
    let root_hash = U256::from(0xdeadbeef_u64);
    let mut provider = MockProvider::new();
    provider
        .expect_request()
        .withf(move |method, params| {
            let tx = parse_call_args(params);
            let call = parse_pool_call(&tx);
            match call {
                CommitmentPoolCalls::IsKnownRoot(args) => method == "eth_call" && args.root == root_hash,
                _ => false,
            }
        })
        .returning(|_, _| Ok(serde_json::json!(U256::one().encode_hex())));
    let commitment_pools = setup(HashMap::from([(1, provider)]));
    let options = IsKnownRootOptions::builder()
        .chain_id(1_u64)
        .contract_address(contract_address)
        .root_hash(root_hash)
        .build();
    assert!(commitment_pools.is_known_root(options).await.unwrap());
}

#[tokio::test]
async fn test_is_known_root_timeout_error() {
    let contract_address = ethers_address_from_string("0x390d485F4D43212D4ae8Cdd967a711514ed5a54f").unwrap();
    let root_hash = U256::from(0xdeadbeef_u64);
    let provider = TimeoutProvider::builder().timeout_ms(1000_u64).build();
    let commitment_pools = setup(HashMap::from([(1, provider)]));
    let options = IsKnownRootOptions::builder()
        .chain_id(1_u64)
        .contract_address(contract_address)
        .root_hash(root_hash)
        .timeout_ms(1_u64)
        .build();
    let result = commitment_pools.is_known_root(options).await;
    assert_eq!(
        result.unwrap_err().to_string(),
        "querying is_known_root timed out after 1 ms"
    );
}

fn setup<P>(providers: HashMap<u64, P>) -> CommitmentPoolContracts<MockProviders>
where
    P: JsonRpcClientWrapper + 'static,
{
    let _ = env_logger::builder()
        .filter_module("mystiko_core", log::LevelFilter::Info)
        .try_init();
    let providers = mock_providers::<P>(providers);
    CommitmentPoolContracts::builder().providers(providers).build()
}

fn parse_pool_call(tx: &TypedTransaction) -> CommitmentPoolCalls {
    CommitmentPoolCalls::decode(tx.data().unwrap()).unwrap()
}
