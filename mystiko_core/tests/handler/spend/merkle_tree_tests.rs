use crate::common::{MockProvider, MockProviders};
use crate::handler::spend::send_tests::{
    generate_merkle_tree, setup_commitment_pool_contracts, setup_config, setup_prover, setup_static_cache,
    setup_transactions, CommitmentPoolTestOptions, SendTestContext, SendTestOptions, TransactTestOptions,
    TransactionsTestOptions,
};
use crate::handler::{generate_private_key, MockDataPackerClient, MockRelayerClient};
use ethers_core::abi::AbiEncode;
use ethers_core::types::{TxHash, U256};
use ethers_signers::LocalWallet;
use mystiko_core::{PrivateKeySigner, SpendHandler};
use mystiko_crypto::merkle_tree::MerkleTree;
use mystiko_ethers::JsonRpcParams;
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::handler::v1::{CreateSpendOptions, SendSpendOptions};
use mystiko_protos::core::v1::{SpendStatus, SpendType};
use mystiko_protos::data::v1::MerkleTree as ProtoMerkleTree;
use mystiko_utils::address::ethers_address_from_string;
use mystiko_utils::convert::{biguint_to_bytes, biguint_to_u256, u256_to_bytes};
use prost::Message;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::Arc;

#[tokio::test]
async fn test_build_from_packer() {
    let chain_id = 5_u64;
    let (config, contract_config) = setup_config(chain_id, "0x223903804Ee95e264F74C88B4F8583429524593c").await;
    let (_, private_key) = generate_private_key();
    let merkle_tree = generate_merkle_tree(4_usize);
    let new_merkle_tree = generate_merkle_tree(6_usize);
    let tree_root = biguint_to_u256(&merkle_tree.root());
    let new_tree_root = biguint_to_u256(&new_merkle_tree.root());
    let tx_hash = TxHash::from_str("0xa35c998eaf5df995dba638efc114a8f58353784d08a60467fba6ed1e8f0e64a8").unwrap();
    let transact_options = TransactTestOptions::builder()
        .num_inputs(1_usize)
        .num_outputs(0_usize)
        .root_hash(HashSet::from([tree_root, new_tree_root]))
        .public_amount(HashSet::from([
            U256::from_dec_str("30000000000000000").unwrap(),
            U256::from_dec_str("40000000000000000").unwrap(),
            U256::from_dec_str("100000000000000000").unwrap(),
        ]))
        .public_recipient(ethers_address_from_string("0x87813A8E81729C0100ce2568b6283772cb31bdb8").unwrap())
        .tx_hash(tx_hash)
        .build();
    let commitment_pool_contracts_options = CommitmentPoolTestOptions::builder()
        .chain_id(chain_id)
        .spent_nullifiers(HashMap::from([
            (U256::from(0_u64), false),
            (U256::from(2_u64), false),
            (U256::from(4_u64), false),
        ]))
        .known_root(HashMap::from([(tree_root, true), (new_tree_root, true)]))
        .transact_options(transact_options)
        .query_timeout_ms(100_u64)
        .tx_send_timeout_ms(200_u64)
        .build();
    let commitment_pool_contracts =
        setup_commitment_pool_contracts(commitment_pool_contracts_options, &contract_config);
    let static_cache = setup_static_cache(&contract_config, &mystiko_types::CircuitType::Transaction1x0);
    let prover = setup_prover(true);
    let transactions_options = TransactionsTestOptions::builder()
        .chain_id(chain_id)
        .config(&config)
        .tx_hash(tx_hash)
        .block_number(200010000_u64)
        .tx_confirmations(10_u64)
        .tx_wait_interval_ms(10_u64)
        .tx_wait_timeout_ms(300_u64)
        .build();
    let transactions = setup_transactions(transactions_options);
    let mut mock_data_packer_client = MockDataPackerClient::new();
    let cms = merkle_tree.elements().iter().map(biguint_to_bytes).collect::<Vec<_>>();
    mock_data_packer_client
        .expect_query_merkle_tree()
        .times(2)
        .returning(move |_, _| {
            let proto_merkle_tree = ProtoMerkleTree::builder()
                .loaded_block_number(200010000_u64)
                .root_hash(u256_to_bytes(&tree_root))
                .commitment_hashes(cms.clone())
                .build();
            Ok(Some(proto_merkle_tree))
        });
    let mock_provider = build_mock_provider();
    let providers = HashMap::from([(chain_id, mock_provider)]);
    let test_options: SendTestOptions = SendTestOptions::builder()
        .chain_id(chain_id)
        .config(config)
        .contract_config(contract_config.clone())
        .relayer_client(MockRelayerClient::new())
        .data_packer_client(mock_data_packer_client)
        .commitment_pool_contracts(commitment_pool_contracts)
        .prover(prover)
        .providers(providers)
        .static_cache(static_cache)
        .transactions(transactions)
        .build();
    let context = SendTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;
    let create_options = CreateSpendOptions::builder()
        .chain_id(chain_id)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Withdraw as i32)
        .amount(3.0)
        .recipient("0x87813A8E81729C0100ce2568b6283772cb31bdb8".to_string())
        .wallet_password("P@ssw0rd".to_string())
        .query_timeout_ms(100_u64)
        .build();
    let spend = context.handler.create(create_options.clone()).await.unwrap();
    let signer = Arc::new(
        PrivateKeySigner::builder()
            .providers(MockProviders::new())
            .wallet(LocalWallet::from_str(&private_key).unwrap())
            .build(),
    );
    let send_options = SendSpendOptions::builder()
        .spend_id(spend.id)
        .wallet_password("P@ssw0rd".to_string())
        .private_key(private_key.clone())
        .query_timeout_ms(100_u64)
        .spend_confirmations(10_u64)
        .tx_send_timeout_ms(200_u64)
        .tx_wait_interval_ms(10_u64)
        .tx_wait_timeout_ms(300_u64)
        .build();
    // test build from packer
    let spend = context
        .handler
        .send_with_signer(send_options.clone(), signer.clone())
        .await
        .unwrap();
    assert_eq!(spend.chain_id, chain_id);
    assert_eq!(spend.contract_address, contract_config.address());
    assert_eq!(spend.asset_symbol, contract_config.asset_symbol());
    assert_eq!(spend.asset_decimals, contract_config.asset_decimals());
    assert_eq!(spend.amount, 3.0);
    assert_eq!(spend.decimal_amount, "30000000000000000");
    assert_eq!(spend.recipient, "0x87813A8E81729C0100ce2568b6283772cb31bdb8");
    assert_eq!(spend.wallet_id, context.account.wallet_id);
    assert_eq!(spend.input_commitments, vec!["0".to_string()]);
    assert!(spend.output_commitments.is_empty());
    assert_eq!(spend.nullifiers.len(), spend.input_commitments.len());
    assert_eq!(spend.signature_public_key_hashes.len(), spend.input_commitments.len());
    assert_eq!(spend.encrypted_auditor_notes.len(), spend.input_commitments.len() * 5);
    assert!(spend.rollup_fee_amount.is_none());
    assert!(spend.rollup_fee_decimal_amount.is_none());
    assert!(spend.rollup_fee_total_amount.is_none());
    assert!(spend.rollup_fee_total_decimal_amount.is_none());
    assert!(spend.gas_relayer_fee_amount.is_none());
    assert!(spend.gas_relayer_fee_decimal_amount.is_none());
    assert!(spend.gas_relayer_address.is_none());
    assert!(spend.gas_relayer_url.is_none());
    assert!(spend.signature_public_key.is_some());
    assert_eq!(spend.asset_address(), contract_config.asset_address().unwrap());
    assert!(spend.proof.is_some());
    assert_eq!(spend.root_hash(), tree_root.to_string());
    assert!(spend.signature.is_some());
    assert!(spend.random_auditing_public_key.is_some());
    assert!(spend.error_message.is_none());
    assert_eq!(spend.transaction_hash(), tx_hash.encode_hex());
    assert_eq!(spend.bridge_type, BridgeType::Loop as i32);
    assert_eq!(spend.spend_type, SpendType::Withdraw as i32);
    assert_eq!(spend.status, SpendStatus::Succeeded as i32);
    context.check_commitments(&spend, 200010000_u64, &[]).await;

    // test build from cache
    let create_options = CreateSpendOptions::builder()
        .chain_id(chain_id)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Withdraw as i32)
        .amount(4.0)
        .recipient("0x87813A8E81729C0100ce2568b6283772cb31bdb8".to_string())
        .wallet_password("P@ssw0rd".to_string())
        .query_timeout_ms(100_u64)
        .build();
    let spend = context.handler.create(create_options).await.unwrap();
    let send_options = SendSpendOptions::builder()
        .spend_id(spend.id)
        .wallet_password("P@ssw0rd".to_string())
        .private_key(private_key.clone())
        .query_timeout_ms(100_u64)
        .spend_confirmations(10_u64)
        .tx_send_timeout_ms(200_u64)
        .tx_wait_interval_ms(10_u64)
        .tx_wait_timeout_ms(300_u64)
        .build();
    let spend = context
        .handler
        .send_with_signer(send_options, signer.clone())
        .await
        .unwrap();
    assert_eq!(spend.chain_id, chain_id);
    assert_eq!(spend.contract_address, contract_config.address());
    assert_eq!(spend.asset_symbol, contract_config.asset_symbol());
    assert_eq!(spend.asset_decimals, contract_config.asset_decimals());
    assert_eq!(spend.amount, 4.0);
    assert_eq!(spend.decimal_amount, "40000000000000000");
    assert_eq!(spend.recipient, "0x87813A8E81729C0100ce2568b6283772cb31bdb8");
    assert!(spend.proof.is_some());
    assert_eq!(spend.root_hash(), tree_root.to_string());
    assert!(spend.error_message.is_none());

    // test build from provider
    let create_options = CreateSpendOptions::builder()
        .chain_id(chain_id)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Withdraw as i32)
        .amount(10.0)
        .recipient("0x87813A8E81729C0100ce2568b6283772cb31bdb8".to_string())
        .wallet_password("P@ssw0rd".to_string())
        .query_timeout_ms(100_u64)
        .build();
    let spend = context.handler.create(create_options).await.unwrap();
    let send_options = SendSpendOptions::builder()
        .spend_id(spend.id)
        .wallet_password("P@ssw0rd".to_string())
        .private_key(private_key)
        .query_timeout_ms(100_u64)
        .spend_confirmations(10_u64)
        .tx_send_timeout_ms(200_u64)
        .tx_wait_interval_ms(10_u64)
        .tx_wait_timeout_ms(300_u64)
        .build();
    let spend = context.handler.send_with_signer(send_options, signer).await.unwrap();
    assert_eq!(spend.chain_id, chain_id);
    assert_eq!(spend.contract_address, contract_config.address());
    assert_eq!(spend.asset_symbol, contract_config.asset_symbol());
    assert_eq!(spend.asset_decimals, contract_config.asset_decimals());
    assert_eq!(spend.amount, 10.0);
    assert_eq!(spend.decimal_amount, "100000000000000000");
    assert_eq!(spend.recipient, "0x87813A8E81729C0100ce2568b6283772cb31bdb8");
    assert!(spend.proof.is_some());
    assert_eq!(spend.root_hash(), new_tree_root.to_string());
    assert!(spend.error_message.is_none());
}

#[tokio::test]
async fn test_build_from_raw() {
    let chain_id = 5_u64;
    let (config, contract_config) = setup_config(chain_id, "0x223903804Ee95e264F74C88B4F8583429524593c").await;
    let (_, private_key) = generate_private_key();
    let merkle_tree = generate_merkle_tree(4_usize);
    let raw_merkle_tree = build_raw_merkle_tree_bytes(&merkle_tree);
    let tree_root = biguint_to_u256(&merkle_tree.root());
    let tx_hash = TxHash::from_str("0xa35c998eaf5df995dba638efc114a8f58353784d08a60467fba6ed1e8f0e64a8").unwrap();
    let transact_options = TransactTestOptions::builder()
        .num_inputs(1_usize)
        .num_outputs(0_usize)
        .root_hash(HashSet::from([tree_root]))
        .public_amount(HashSet::from([
            U256::from_dec_str("30000000000000000").unwrap(),
            U256::from_dec_str("40000000000000000").unwrap(),
            U256::from_dec_str("100000000000000000").unwrap(),
        ]))
        .public_recipient(ethers_address_from_string("0x87813A8E81729C0100ce2568b6283772cb31bdb8").unwrap())
        .tx_hash(tx_hash)
        .build();
    let commitment_pool_contracts_options = CommitmentPoolTestOptions::builder()
        .chain_id(chain_id)
        .spent_nullifiers(HashMap::from([
            (U256::from(0_u64), false),
            (U256::from(2_u64), false),
            (U256::from(4_u64), false),
        ]))
        .known_root(HashMap::from([(tree_root, true)]))
        .transact_options(transact_options)
        .query_timeout_ms(100_u64)
        .tx_send_timeout_ms(200_u64)
        .build();
    let commitment_pool_contracts =
        setup_commitment_pool_contracts(commitment_pool_contracts_options, &contract_config);
    let static_cache = setup_static_cache(&contract_config, &mystiko_types::CircuitType::Transaction1x0);
    let prover = setup_prover(true);
    let transactions_options = TransactionsTestOptions::builder()
        .chain_id(chain_id)
        .config(&config)
        .tx_hash(tx_hash)
        .block_number(200010000_u64)
        .tx_confirmations(10_u64)
        .tx_wait_interval_ms(10_u64)
        .tx_wait_timeout_ms(300_u64)
        .build();
    let transactions = setup_transactions(transactions_options);
    let mock_data_packer_client = MockDataPackerClient::new();
    let mock_provider = build_mock_provider();
    let providers = HashMap::from([(chain_id, mock_provider)]);
    let test_options: SendTestOptions = SendTestOptions::builder()
        .chain_id(chain_id)
        .config(config)
        .contract_config(contract_config.clone())
        .relayer_client(MockRelayerClient::new())
        .data_packer_client(mock_data_packer_client)
        .commitment_pool_contracts(commitment_pool_contracts)
        .prover(prover)
        .providers(providers)
        .static_cache(static_cache)
        .transactions(transactions)
        .build();
    let context = SendTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;
    let create_options = CreateSpendOptions::builder()
        .chain_id(chain_id)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Withdraw as i32)
        .amount(3.0)
        .recipient("0x87813A8E81729C0100ce2568b6283772cb31bdb8".to_string())
        .wallet_password("P@ssw0rd".to_string())
        .query_timeout_ms(100_u64)
        .build();
    let spend = context.handler.create(create_options.clone()).await.unwrap();
    let signer = Arc::new(
        PrivateKeySigner::builder()
            .providers(MockProviders::new())
            .wallet(LocalWallet::from_str(&private_key).unwrap())
            .build(),
    );
    let send_options = SendSpendOptions::builder()
        .spend_id(spend.id)
        .wallet_password("P@ssw0rd".to_string())
        .private_key(private_key.clone())
        .query_timeout_ms(100_u64)
        .spend_confirmations(10_u64)
        .tx_send_timeout_ms(200_u64)
        .tx_wait_interval_ms(10_u64)
        .tx_wait_timeout_ms(300_u64)
        .raw_merkle_tree(raw_merkle_tree)
        .build();
    // test build from packer
    let spend = context
        .handler
        .send_with_signer(send_options.clone(), signer.clone())
        .await
        .unwrap();
    assert_eq!(spend.chain_id, chain_id);
    assert_eq!(spend.contract_address, contract_config.address());
    assert_eq!(spend.asset_symbol, contract_config.asset_symbol());
    assert_eq!(spend.asset_decimals, contract_config.asset_decimals());
    assert_eq!(spend.amount, 3.0);
    assert_eq!(spend.decimal_amount, "30000000000000000");
    assert_eq!(spend.recipient, "0x87813A8E81729C0100ce2568b6283772cb31bdb8");
    assert_eq!(spend.wallet_id, context.account.wallet_id);
    assert_eq!(spend.input_commitments, vec!["0".to_string()]);
    assert!(spend.output_commitments.is_empty());
    assert_eq!(spend.nullifiers.len(), spend.input_commitments.len());
    assert_eq!(spend.signature_public_key_hashes.len(), spend.input_commitments.len());
    assert_eq!(spend.encrypted_auditor_notes.len(), spend.input_commitments.len() * 5);
    assert!(spend.rollup_fee_amount.is_none());
    assert!(spend.rollup_fee_decimal_amount.is_none());
    assert!(spend.rollup_fee_total_amount.is_none());
    assert!(spend.rollup_fee_total_decimal_amount.is_none());
    assert!(spend.gas_relayer_fee_amount.is_none());
    assert!(spend.gas_relayer_fee_decimal_amount.is_none());
    assert!(spend.gas_relayer_address.is_none());
    assert!(spend.gas_relayer_url.is_none());
    assert!(spend.signature_public_key.is_some());
    assert_eq!(spend.asset_address(), contract_config.asset_address().unwrap());
    assert!(spend.proof.is_some());
    assert_eq!(spend.root_hash(), tree_root.to_string());
    assert!(spend.signature.is_some());
    assert!(spend.random_auditing_public_key.is_some());
    assert!(spend.error_message.is_none());
    assert_eq!(spend.transaction_hash(), tx_hash.encode_hex());
    assert_eq!(spend.bridge_type, BridgeType::Loop as i32);
    assert_eq!(spend.spend_type, SpendType::Withdraw as i32);
    assert_eq!(spend.status, SpendStatus::Succeeded as i32);
    context.check_commitments(&spend, 200010000_u64, &[]).await;
}

fn build_mock_provider() -> MockProvider {
    let mut mock_provider = MockProvider::new();
    mock_provider.expect_request().returning(|method, params| {
        match method {
            "eth_blockNumber" => Ok(serde_json::json!("0xbebe911")), // Example block number
            "eth_getLogs" => {
                let value_params = match params {
                    JsonRpcParams::Value(params) => params,
                    _ => return Err(ethers_providers::ProviderError::CustomError("mock provider not support".to_string())),
                };
                if let Some(topics) = value_params.get(0).and_then(|obj| {
                    obj.get("topics").and_then(|topics| topics.as_array())
                }) {
                    let target_topic = "0xf533f9705aac5020e21695ea3553ac7b6881070d2b6900ab2b1e3050304b5bf9";
                    if topics.iter().any(|topic| topic == target_topic) {
                        let result = serde_json::json!([{
                            "address": "0xCB255075f38C75EAf2DE8A72897649dba9B90299",
                            "blockHash": "0x224ac34e68f04a2d134affb0bf9181bae2cc4e7376a60687c072119247fb0e78",
                            "blockNumber": "0xbebe911",
                            "data": "0x00000000000000000000000000000000000000000000000000000000000186a000000000000000000000000000000000000000000000000000000000000003a6000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000d1d6cb2d1e247af8403e556506eaa594d9040928fb8abd9515bb60274f3be6a348411d1abcf50aac3d7f305d7b2839f5afd02c2808145dcfd680f8fe8e0cb016b865ebe3c9d8ae96151d231cd6651ebf4dd2eb1480256e6a8a5f866120b833527eedd324006c33071f2bc3888f61c2c7b400d13d40739eab565365f7c2b8063bf2f45474008c1ab2cea99b82fd62efadba6a6d2128b5934fe89240f2a521153394a8577912130dae64642e7090a345afc5a8d43b6b864db2845d00d906a9b2d69f4a0bd43e6744cf1a3c1432bc607316b09a000000000000000000000000000000",
                            "logIndex": "0xff",
                            "removed": false,
                            "topics": [
                              "0xf533f9705aac5020e21695ea3553ac7b6881070d2b6900ab2b1e3050304b5bf9",
                              "0x0000000000000000000000000000000000000000000000000000000000000004"
                              ],
                            "transactionHash": "0xa5832c0a90837280d29de8498144c40c295fbf4adae7efc97046c322cb81c1c2",
                            "transactionIndex": "0x33"
                        },
                        {
                            "address": "0xCB255075f38C75EAf2DE8A72897649dba9B90299",
                            "blockHash": "0x224ac34e68f04a2d134affb0bf9181bae2cc4e7376a60687c072119247fb0e78",
                            "blockNumber": "0xbebe911",
                            "data": "0x00000000000000000000000000000000000000000000000000000000000186a000000000000000000000000000000000000000000000000000000000000003a6000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000d1d6cb2d1e247af8403e556506eaa594d9040928fb8abd9515bb60274f3be6a348411d1abcf50aac3d7f305d7b2839f5afd02c2808145dcfd680f8fe8e0cb016b865ebe3c9d8ae96151d231cd6651ebf4dd2eb1480256e6a8a5f866120b833527eedd324006c33071f2bc3888f61c2c7b400d13d40739eab565365f7c2b8063bf2f45474008c1ab2cea99b82fd62efadba6a6d2128b5934fe89240f2a521153394a8577912130dae64642e7090a345afc5a8d43b6b864db2845d00d906a9b2d69f4a0bd43e6744cf1a3c1432bc607316b09a000000000000000000000000000000",
                            "logIndex": "0xff",
                            "removed": false,
                            "topics": [
                              "0xf533f9705aac5020e21695ea3553ac7b6881070d2b6900ab2b1e3050304b5bf9",
                              "0x0000000000000000000000000000000000000000000000000000000000000005"
                              ],
                            "transactionHash": "0xb595eaad5454ca2b761667424959fc77abdd79b8d10292c3b9d83560def1da24",
                            "transactionIndex": "0x34"
                        }
                        ]);
                        return Ok(result);
                    }
                }
                Ok(serde_json::json!([]))
            }
            _ => Err(ethers_providers::ProviderError::CustomError("mock provider not support".to_string())),
        }
    });

    mock_provider
}

fn build_raw_merkle_tree_bytes(tree: &MerkleTree) -> Vec<u8> {
    let proto_tree = ProtoMerkleTree::builder()
        .loaded_block_number(200010000_u64)
        .root_hash(biguint_to_bytes(&tree.root()))
        .commitment_hashes(tree.elements().iter().map(biguint_to_bytes).collect::<Vec<_>>())
        .build();
    let mut data = Vec::new();
    proto_tree.encode(&mut data).unwrap();
    zstd::encode_all(&*data, 0).unwrap()
}
