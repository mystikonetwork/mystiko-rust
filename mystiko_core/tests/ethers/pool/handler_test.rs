use crate::common::{MockProvider, MockProviders, MockTransactionSigner};
use crate::ethers::{mock_providers, parse_call_args, TimeoutProvider};
use ethers_core::abi::{AbiDecode, AbiEncode};
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Bytes, TxHash, U256};
use mystiko_abi::commitment_pool::{CommitmentPoolCalls, G1Point, G2Point, Proof, TransactRequest};
use mystiko_core::{
    AuditorPublicKeysOptions, CommitmentPoolContractHandler, CommitmentPoolContracts, IsHistoricCommitmentOptions,
    IsKnownRootOptions, IsSpentNullifierOptions, MinRollupFeeOptions, TransactOptions,
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

#[tokio::test]
async fn test_min_rollup_fee() {
    let contract_address = ethers_address_from_string("0x390d485F4D43212D4ae8Cdd967a711514ed5a54f").unwrap();
    let min_rollup_fee = U256::from(0xdeadbeef_u64);
    let mut provider = MockProvider::new();
    provider
        .expect_request()
        .withf(move |method, params| {
            let tx = parse_call_args(params);
            let call = parse_pool_call(&tx);
            match call {
                CommitmentPoolCalls::GetMinRollupFee(_) => method == "eth_call",
                _ => false,
            }
        })
        .returning(move |_, _| Ok(serde_json::json!(min_rollup_fee.encode_hex())));
    let commitment_pools = setup(HashMap::from([(1, provider)]));
    let options = MinRollupFeeOptions::builder()
        .chain_id(1_u64)
        .contract_address(contract_address)
        .build();
    assert_eq!(commitment_pools.min_rollup_fee(options).await.unwrap(), min_rollup_fee);
}

#[tokio::test]
async fn test_min_rollup_fee_timeout_error() {
    let contract_address = ethers_address_from_string("0x390d485F4D43212D4ae8Cdd967a711514ed5a54f").unwrap();
    let provider = TimeoutProvider::builder().timeout_ms(1000_u64).build();
    let commitment_pools = setup(HashMap::from([(1, provider)]));
    let options = MinRollupFeeOptions::builder()
        .chain_id(1_u64)
        .contract_address(contract_address)
        .timeout_ms(1_u64)
        .build();
    let result = commitment_pools.min_rollup_fee(options).await;
    assert_eq!(
        result.unwrap_err().to_string(),
        "querying min_rollup_fee timed out after 1 ms"
    );
}

#[tokio::test]
async fn test_auditor_public_keys() {
    let contract_address = ethers_address_from_string("0x390d485F4D43212D4ae8Cdd967a711514ed5a54f").unwrap();
    let auditor_public_keys = vec![U256::from(0xdeadbeef_u64), U256::from(0xbadebabe_u64)];
    let expected = auditor_public_keys.clone();
    let mut provider = MockProvider::new();
    provider
        .expect_request()
        .withf(move |method, params| {
            let tx = parse_call_args(params);
            let call = parse_pool_call(&tx);
            match call {
                CommitmentPoolCalls::GetAllAuditorPublicKeys(_) => method == "eth_call",
                _ => false,
            }
        })
        .returning(move |_, _| Ok(serde_json::json!(auditor_public_keys.clone().encode_hex())));
    let commitment_pools = setup(HashMap::from([(1, provider)]));
    let options = AuditorPublicKeysOptions::builder()
        .chain_id(1_u64)
        .contract_address(contract_address)
        .build();
    assert_eq!(commitment_pools.auditor_public_keys(options).await.unwrap(), expected);
}

#[tokio::test]
async fn test_auditor_public_keys_timeout_error() {
    let contract_address = ethers_address_from_string("0x390d485F4D43212D4ae8Cdd967a711514ed5a54f").unwrap();
    let provider = TimeoutProvider::builder().timeout_ms(1000_u64).build();
    let commitment_pools = setup(HashMap::from([(1, provider)]));
    let options = AuditorPublicKeysOptions::builder()
        .chain_id(1_u64)
        .contract_address(contract_address)
        .timeout_ms(1_u64)
        .build();
    let result = commitment_pools.auditor_public_keys(options).await;
    assert_eq!(
        result.unwrap_err().to_string(),
        "querying auditor_public_keys timed out after 1 ms"
    );
}

#[tokio::test]
async fn test_transact() {
    let contract_address = ethers_address_from_string("0x390d485F4D43212D4ae8Cdd967a711514ed5a54f").unwrap();
    let proof = Proof {
        a: G1Point {
            x: U256::from(100_u64),
            y: U256::from(200_u64),
        },
        b: G2Point {
            x: [U256::from(300_u64), U256::from(400_u64)],
            y: [U256::from(500_u64), U256::from(600_u64)],
        },
        c: G1Point {
            x: U256::from(700_u64),
            y: U256::from(800_u64),
        },
    };
    let request = TransactRequest {
        proof: proof.clone(),
        root_hash: U256::from(0xdeadbeef_u64),
        serial_numbers: vec![U256::from(0xdeadbeef_u64)],
        sig_hashes: vec![U256::from(0xdeadbeef_u64)],
        sig_pk: [0xf; 32],
        public_amount: U256::from(10001_u64),
        relayer_fee_amount: U256::from(101_u64),
        out_commitments: vec![U256::from(0xbadebabe_u64)],
        out_rollup_fees: vec![U256::from(11_u64)],
        public_recipient: ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap(),
        relayer_address: ethers_address_from_string("0xC4E30d504FE3b18423D82A34e7Ebe529a56f89c2").unwrap(),
        out_encrypted_notes: vec![Bytes::from(vec![0x1, 0x2, 0x3, 0x4])],
        random_auditing_public_key: U256::from(0xdeadbeef_u64),
        encrypted_auditor_notes: vec![U256::from(0xdeadbeef_u64)],
    };
    let expected_request = request.clone();
    let signature = Bytes::from(vec![0x5, 0x6, 0x7, 0x8]);
    let expected_signature = signature.clone();
    let expected_tx_hash =
        TxHash::decode_hex("0x8cbbb491f260cb9a810f81ebf8b51ae1adf322466232181b2eb2bb105b45f0b9").unwrap();
    let mut signer = MockTransactionSigner::new();
    signer
        .expect_send_transaction()
        .withf(move |chain_id, tx| {
            let call = parse_pool_call(tx);
            match call {
                CommitmentPoolCalls::Transact(args) => {
                    args.request == expected_request
                        && args.signature == expected_signature
                        && *chain_id == 1_u64
                        && *tx.to().unwrap().as_address().unwrap() == contract_address
                        && tx.value().is_none()
                }
                _ => false,
            }
        })
        .returning(move |_, _| Ok(expected_tx_hash));
    let commitment_pools = setup(HashMap::from([(1, MockProvider::new())]));
    let options = TransactOptions::<TypedTransaction, MockTransactionSigner>::builder()
        .chain_id(1_u64)
        .contract_address(contract_address)
        .request(request)
        .signature(signature)
        .signer(signer)
        .build();
    let tx_hash = commitment_pools.transact(options).await.unwrap();
    assert_eq!(tx_hash, expected_tx_hash);
}

#[tokio::test]
async fn test_transact_timeout_error() {
    let contract_address = ethers_address_from_string("0x390d485F4D43212D4ae8Cdd967a711514ed5a54f").unwrap();
    let proof = Proof {
        a: G1Point {
            x: U256::from(100_u64),
            y: U256::from(200_u64),
        },
        b: G2Point {
            x: [U256::from(300_u64), U256::from(400_u64)],
            y: [U256::from(500_u64), U256::from(600_u64)],
        },
        c: G1Point {
            x: U256::from(700_u64),
            y: U256::from(800_u64),
        },
    };
    let request = TransactRequest {
        proof: proof.clone(),
        root_hash: U256::from(0xdeadbeef_u64),
        serial_numbers: vec![U256::from(0xdeadbeef_u64)],
        sig_hashes: vec![U256::from(0xdeadbeef_u64)],
        sig_pk: [0xf; 32],
        public_amount: U256::from(10001_u64),
        relayer_fee_amount: U256::from(101_u64),
        out_commitments: vec![U256::from(0xbadebabe_u64)],
        out_rollup_fees: vec![U256::from(11_u64)],
        public_recipient: ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap(),
        relayer_address: ethers_address_from_string("0xC4E30d504FE3b18423D82A34e7Ebe529a56f89c2").unwrap(),
        out_encrypted_notes: vec![Bytes::from(vec![0x1, 0x2, 0x3, 0x4])],
        random_auditing_public_key: U256::from(0xdeadbeef_u64),
        encrypted_auditor_notes: vec![U256::from(0xdeadbeef_u64)],
    };
    let signature = Bytes::from(vec![0x5, 0x6, 0x7, 0x8]);
    let signer = TimeoutProvider::builder().timeout_ms(1000_u64).build();
    let commitment_pools = setup(HashMap::from([(1, MockProvider::new())]));
    let options = TransactOptions::<TypedTransaction, TimeoutProvider>::builder()
        .chain_id(1_u64)
        .contract_address(contract_address)
        .request(request)
        .signature(signature)
        .signer(signer)
        .timeout_ms(1_u64)
        .build();
    assert_eq!(
        commitment_pools.transact(options).await.unwrap_err().to_string(),
        "sending transact transaction timed out after 1 ms"
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
