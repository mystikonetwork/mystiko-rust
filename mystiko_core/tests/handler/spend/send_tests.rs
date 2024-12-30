use crate::common::{MockProvider, MockProviders};
use crate::handler::spend::setup::{
    create_wallet, generate_commitments, setup, CommitmentOptions, DatabaseType, MockOptions, SpendsType,
};
use crate::handler::{
    generate_private_key, MockCommitmentPoolContracts, MockDataPackerClient, MockPublicAssets, MockRelayerClient,
    MockStaticCache, MockTransactions, MockZKProver,
};
use ethers_core::abi::AbiEncode;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::transaction::eip2930::AccessList;
use ethers_core::types::{
    Address, Eip1559TransactionRequest, Eip2930TransactionRequest, TransactionReceipt, TransactionRequest, TxHash,
    U256, U64,
};
use ethers_signers::LocalWallet;
use mystiko_config::{MystikoConfig, PoolContractConfig};
use mystiko_core::{Commitment, CommitmentColumn, PrivateKeySigner, SpendColumn, SpendHandler};
use mystiko_crypto::merkle_tree::MerkleTree;
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::document::v1::{Account, Spend};
use mystiko_protos::core::handler::v1::{CreateSpendOptions, FixSpendStatusOptions, SendSpendOptions};
use mystiko_protos::core::v1::{SpendStatus, SpendType};
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_protos::data::v1::MerkleTree as ProtoMerkleTree;
use mystiko_protos::storage::v1::{Condition, SubFilter};
use mystiko_relayer_client::types::register::RegisterInfo;
use mystiko_relayer_types::{ContractInfo, RelayTransactResponse, RelayTransactStatusResponse, TransactStatus};
use mystiko_storage::{ColumnValues, Document, DocumentColumn};
use mystiko_types::TransactionType;
use mystiko_utils::address::ethers_address_from_string;
use mystiko_utils::convert::{biguint_to_bytes, biguint_to_u256, number_to_biguint_decimal, u256_to_bytes};
use num_bigint::BigUint;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use std::vec;
use typed_builder::TypedBuilder;

#[tokio::test]
async fn test_send_withdraw1x0() {
    let chain_id = 5_u64;
    let (config, contract_config) = setup_config(chain_id, "0x223903804Ee95e264F74C88B4F8583429524593c").await;
    let (_, private_key) = generate_private_key();
    let merkle_tree = generate_merkle_tree(6_usize);
    let tree_root = biguint_to_u256(&merkle_tree.root());
    let tx_hash = TxHash::from_str("0xa35c998eaf5df995dba638efc114a8f58353784d08a60467fba6ed1e8f0e64a8").unwrap();
    let transact_options = TransactTestOptions::builder()
        .num_inputs(1_usize)
        .num_outputs(0_usize)
        .root_hash(HashSet::from([tree_root]))
        .public_amount(HashSet::from([(U256::from_dec_str("30000000000000000").unwrap())]))
        .public_recipient(ethers_address_from_string("0x87813A8E81729C0100ce2568b6283772cb31bdb8").unwrap())
        .tx_hash(tx_hash)
        .build();
    let commitment_pool_contracts_options = CommitmentPoolTestOptions::builder()
        .chain_id(chain_id)
        .spent_nullifiers(HashMap::from([(U256::from(0_u64), false)]))
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
    let mut mock_data_packer_client = MockDataPackerClient::new();
    mock_data_packer_client
        .expect_query_merkle_tree()
        .times(1)
        .returning(move |_, _| {
            let proto_merkle_tree = ProtoMerkleTree::builder()
                .loaded_block_number(200010000_u64)
                .root_hash(u256_to_bytes(&tree_root))
                .commitment_hashes(merkle_tree.elements().iter().map(biguint_to_bytes).collect::<Vec<_>>())
                .build();
            Ok(Some(proto_merkle_tree))
        });
    let test_options: SendTestOptions = SendTestOptions::builder()
        .chain_id(chain_id)
        .config(config)
        .contract_config(contract_config.clone())
        .relayer_client(MockRelayerClient::new())
        .data_packer_client(mock_data_packer_client)
        .commitment_pool_contracts(commitment_pool_contracts)
        .prover(prover)
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
    let spend = context.handler.create(create_options).await.unwrap();
    let signer = PrivateKeySigner::builder()
        .providers(MockProviders::new())
        .wallet(LocalWallet::from_str(&private_key).unwrap())
        .build();
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
    let spend = context
        .handler
        .send_with_signer(send_options, Arc::new(signer))
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

    let fix_options = FixSpendStatusOptions::builder()
        .query_timeout_ms(101)
        .spend_id(spend.id)
        .build();
    let spend = context.handler.fix_status(fix_options).await.unwrap();
    assert_eq!(spend.status, SpendStatus::Failed as i32);
    let cms = context
        .db
        .commitments
        .find(SubFilter::equal(
            CommitmentColumn::CommitmentHash,
            BigUint::from_str(&spend.input_commitments[0]).unwrap(),
        ))
        .await
        .unwrap();
    assert_eq!(cms.len(), 1);
    assert!(!cms[0].data.spent);

    let fix_options = FixSpendStatusOptions::builder()
        .query_timeout_ms(102)
        .spend_id(spend.id)
        .build();
    let spend = context.handler.fix_status(fix_options).await.unwrap();
    assert_eq!(spend.status, SpendStatus::Succeeded as i32);
    let cms = context
        .db
        .commitments
        .find(SubFilter::equal(
            CommitmentColumn::CommitmentHash,
            BigUint::from_str(&spend.input_commitments[0]).unwrap(),
        ))
        .await
        .unwrap();
    assert_eq!(cms.len(), 1);
    assert!(cms[0].data.spent);
}

#[tokio::test]
async fn test_send_withdraw2x0() {
    let chain_id = 5_u64;
    let (config, contract_config) = setup_config(chain_id, "0x223903804Ee95e264F74C88B4F8583429524593c").await;
    let (_, private_key) = generate_private_key();
    let merkle_tree = generate_merkle_tree(6_usize);
    let tree_root = biguint_to_u256(&merkle_tree.root());
    let tx_hash = TxHash::from_str("0xa35c998eaf5df995dba638efc114a8f58353784d08a60467fba6ed1e8f0e64a8").unwrap();
    let transact_options = TransactTestOptions::builder()
        .num_inputs(2_usize)
        .num_outputs(0_usize)
        .root_hash(HashSet::from([tree_root]))
        .public_amount(HashSet::from([(U256::from_dec_str("70000000000000000").unwrap())]))
        .public_recipient(ethers_address_from_string("0x87813A8E81729C0100ce2568b6283772cb31bdb8").unwrap())
        .tx_hash(tx_hash)
        .build();
    let commitment_pool_contracts_options = CommitmentPoolTestOptions::builder()
        .chain_id(chain_id)
        .spent_nullifiers(HashMap::from([(U256::from(0_u64), false), (U256::from(2_u64), false)]))
        .known_root(HashMap::from([(tree_root, true)]))
        .transact_options(transact_options)
        .build();
    let commitment_pool_contracts =
        setup_commitment_pool_contracts(commitment_pool_contracts_options, &contract_config);
    let static_cache = setup_static_cache(&contract_config, &mystiko_types::CircuitType::Transaction2x0);
    let prover = setup_prover(true);
    let transactions_options = TransactionsTestOptions::builder()
        .chain_id(chain_id)
        .config(&config)
        .tx_hash(tx_hash)
        .block_number(200010000_u64)
        .build();
    let transactions = setup_transactions(transactions_options);
    let mut mock_data_packer_client = MockDataPackerClient::new();
    mock_data_packer_client
        .expect_query_merkle_tree()
        .times(1)
        .returning(move |_, _| {
            let proto_merkle_tree = ProtoMerkleTree::builder()
                .loaded_block_number(200010000_u64)
                .root_hash(u256_to_bytes(&tree_root))
                .commitment_hashes(merkle_tree.elements().iter().map(biguint_to_bytes).collect::<Vec<_>>())
                .build();
            Ok(Some(proto_merkle_tree))
        });
    let test_options: SendTestOptions = SendTestOptions::builder()
        .chain_id(chain_id)
        .config(config)
        .contract_config(contract_config.clone())
        .relayer_client(MockRelayerClient::new())
        .data_packer_client(mock_data_packer_client)
        .commitment_pool_contracts(commitment_pool_contracts)
        .prover(prover)
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
        .amount(7.0)
        .recipient("0x87813A8E81729C0100ce2568b6283772cb31bdb8".to_string())
        .wallet_password("P@ssw0rd".to_string())
        .build();
    let spend = context.handler.create(create_options).await.unwrap();
    let send_options = SendSpendOptions::builder()
        .spend_id(spend.id)
        .wallet_password("P@ssw0rd".to_string())
        .private_key(private_key)
        .build();
    let mut spend = context.handler.send(send_options).await.unwrap();
    assert_eq!(spend.chain_id, chain_id);
    assert_eq!(spend.contract_address, contract_config.address());
    assert_eq!(spend.asset_symbol, contract_config.asset_symbol());
    assert_eq!(spend.asset_decimals, contract_config.asset_decimals());
    assert_eq!(spend.amount, 7.0);
    assert_eq!(spend.decimal_amount, "70000000000000000");
    assert_eq!(spend.recipient, "0x87813A8E81729C0100ce2568b6283772cb31bdb8");
    assert_eq!(spend.wallet_id, context.account.wallet_id);
    spend.input_commitments.sort();
    assert_eq!(spend.input_commitments, vec!["0".to_string(), "2".to_string()]);
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

    let fix_options = FixSpendStatusOptions::builder()
        .query_timeout_ms(201)
        .spend_id(spend.id)
        .build();
    let spend = context.handler.fix_status(fix_options.clone()).await.unwrap();
    assert_eq!(spend.status, SpendStatus::Failed as i32);
    let cms = context
        .db
        .commitments
        .find(SubFilter::equal(
            CommitmentColumn::CommitmentHash,
            BigUint::from_str(&spend.input_commitments[0]).unwrap(),
        ))
        .await
        .unwrap();
    assert_eq!(cms.len(), 1);
    assert!(!cms[0].data.spent);
    let cms = context
        .db
        .commitments
        .find(SubFilter::equal(
            CommitmentColumn::CommitmentHash,
            BigUint::from_str(&spend.input_commitments[1]).unwrap(),
        ))
        .await
        .unwrap();
    assert_eq!(cms.len(), 1);
    assert!(!cms[0].data.spent);

    let fix_options = FixSpendStatusOptions::builder()
        .query_timeout_ms(202)
        .spend_id(spend.id)
        .build();
    let spend = context.handler.fix_status(fix_options.clone()).await.unwrap();
    assert_eq!(spend.status, SpendStatus::Succeeded as i32);
    let cms = context
        .db
        .commitments
        .find(SubFilter::equal(
            CommitmentColumn::CommitmentHash,
            BigUint::from_str(&spend.input_commitments[0]).unwrap(),
        ))
        .await
        .unwrap();
    assert_eq!(cms.len(), 1);
    assert!(cms[0].data.spent);
    let cms = context
        .db
        .commitments
        .find(SubFilter::equal(
            CommitmentColumn::CommitmentHash,
            BigUint::from_str(&spend.input_commitments[1]).unwrap(),
        ))
        .await
        .unwrap();
    assert_eq!(cms.len(), 1);
    assert!(!cms[0].data.spent);
}

#[tokio::test]
async fn test_send_withdraw1x1() {
    let chain_id = 5_u64;
    let (config, contract_config) = setup_config(chain_id, "0x223903804Ee95e264F74C88B4F8583429524593c").await;
    let (_, private_key) = generate_private_key();
    let merkle_tree = generate_merkle_tree(6_usize);
    let tree_root = biguint_to_u256(&merkle_tree.root());
    let tx_hash = TxHash::from_str("0xa35c998eaf5df995dba638efc114a8f58353784d08a60467fba6ed1e8f0e64a8").unwrap();
    let transact_options = TransactTestOptions::builder()
        .num_inputs(1_usize)
        .num_outputs(1_usize)
        .root_hash(HashSet::from([tree_root]))
        .public_amount(HashSet::from([(U256::from_dec_str("40000000000000000").unwrap())]))
        .public_recipient(ethers_address_from_string("0x87813A8E81729C0100ce2568b6283772cb31bdb8").unwrap())
        .tx_hash(tx_hash)
        .out_rollup_fees(vec![U256::from_dec_str("40000000000000000").unwrap()])
        .build();
    let commitment_pool_contracts_options = CommitmentPoolTestOptions::builder()
        .chain_id(chain_id)
        .spent_nullifiers(HashMap::from([(U256::from(4_u64), false)]))
        .known_root(HashMap::from([(tree_root, true)]))
        .transact_options(transact_options)
        .build();
    let commitment_pool_contracts =
        setup_commitment_pool_contracts(commitment_pool_contracts_options, &contract_config);
    let static_cache = setup_static_cache(&contract_config, &mystiko_types::CircuitType::Transaction1x1);
    let prover = setup_prover(true);
    let transactions_options = TransactionsTestOptions::builder()
        .chain_id(chain_id)
        .config(&config)
        .tx_hash(tx_hash)
        .block_number(200010000_u64)
        .build();
    let transactions = setup_transactions(transactions_options);
    let mut mock_data_packer_client = MockDataPackerClient::new();
    mock_data_packer_client
        .expect_query_merkle_tree()
        .times(1)
        .returning(move |_, _| {
            let proto_merkle_tree = ProtoMerkleTree::builder()
                .loaded_block_number(200010000_u64)
                .root_hash(u256_to_bytes(&tree_root))
                .commitment_hashes(merkle_tree.elements().iter().map(biguint_to_bytes).collect::<Vec<_>>())
                .build();
            Ok(Some(proto_merkle_tree))
        });
    let test_options: SendTestOptions = SendTestOptions::builder()
        .chain_id(chain_id)
        .config(config)
        .contract_config(contract_config.clone())
        .relayer_client(MockRelayerClient::new())
        .data_packer_client(mock_data_packer_client)
        .commitment_pool_contracts(commitment_pool_contracts)
        .prover(prover)
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
        .amount(8.0)
        .rollup_fee_amount(4.0)
        .recipient("0x87813A8E81729C0100ce2568b6283772cb31bdb8".to_string())
        .wallet_password("P@ssw0rd".to_string())
        .build();
    let spend = context.handler.create(create_options).await.unwrap();
    let send_options = SendSpendOptions::builder()
        .spend_id(spend.id)
        .wallet_password("P@ssw0rd".to_string())
        .private_key(private_key)
        .build();
    let spend = context.handler.send(send_options).await.unwrap();
    assert_eq!(spend.chain_id, chain_id);
    assert_eq!(spend.contract_address, contract_config.address());
    assert_eq!(spend.asset_symbol, contract_config.asset_symbol());
    assert_eq!(spend.asset_decimals, contract_config.asset_decimals());
    assert_eq!(spend.amount, 8.0);
    assert_eq!(spend.decimal_amount, "80000000000000000");
    assert_eq!(spend.recipient, "0x87813A8E81729C0100ce2568b6283772cb31bdb8");
    assert_eq!(spend.wallet_id, context.account.wallet_id);
    assert_eq!(spend.input_commitments, vec!["4".to_string()]);
    assert_eq!(spend.output_commitments.len(), 1);
    assert_eq!(spend.nullifiers.len(), spend.input_commitments.len());
    assert_eq!(spend.signature_public_key_hashes.len(), spend.input_commitments.len());
    assert_eq!(spend.encrypted_auditor_notes.len(), spend.input_commitments.len() * 5);
    assert_eq!(spend.rollup_fee_amount.unwrap(), 4.0);
    assert_eq!(spend.rollup_fee_decimal_amount.clone().unwrap(), "40000000000000000");
    assert_eq!(spend.rollup_fee_total_amount.unwrap(), 4.0);
    assert_eq!(
        spend.rollup_fee_total_decimal_amount.clone().unwrap(),
        "40000000000000000"
    );
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
    context.check_commitments(&spend, 200010000_u64, &[2.0]).await;

    let fix_options = FixSpendStatusOptions::builder()
        .query_timeout_ms(301)
        .spend_id(spend.id)
        .build();
    let spend = context.handler.fix_status(fix_options.clone()).await.unwrap();
    assert_eq!(spend.status, SpendStatus::Failed as i32);
    let input_cms = context
        .db
        .commitments
        .find(SubFilter::equal(
            CommitmentColumn::CommitmentHash,
            BigUint::from_str(&spend.input_commitments[0]).unwrap(),
        ))
        .await
        .unwrap();
    assert_eq!(input_cms.len(), 1);
    assert!(!input_cms[0].data.spent);
    let output_cms = context
        .db
        .commitments
        .find(SubFilter::equal(
            CommitmentColumn::CommitmentHash,
            BigUint::from_str(&spend.output_commitments[0]).unwrap(),
        ))
        .await
        .unwrap();
    assert_eq!(output_cms.len(), 1);
    assert!(!output_cms[0].data.spent);
    assert_eq!(output_cms[0].data.status, CommitmentStatus::Queued as i32);

    let fix_options = FixSpendStatusOptions::builder()
        .query_timeout_ms(302)
        .spend_id(spend.id)
        .build();
    let spend = context.handler.fix_status(fix_options.clone()).await.unwrap();
    assert_eq!(spend.status, SpendStatus::Succeeded as i32);
    let input_cms = context
        .db
        .commitments
        .find(SubFilter::equal(
            CommitmentColumn::CommitmentHash,
            BigUint::from_str(&spend.input_commitments[0]).unwrap(),
        ))
        .await
        .unwrap();
    assert_eq!(input_cms.len(), 1);
    assert!(input_cms[0].data.spent);
    let mut output_cms = context
        .db
        .commitments
        .find(SubFilter::equal(
            CommitmentColumn::CommitmentHash,
            BigUint::from_str(&spend.output_commitments[0]).unwrap(),
        ))
        .await
        .unwrap();
    assert_eq!(output_cms.len(), 1);
    assert!(!output_cms[0].data.spent);
    assert_eq!(output_cms[0].data.status, CommitmentStatus::Queued as i32);
    output_cms[0].data.included_transaction_hash = Some("123".to_string());
    context.db.commitments.update(&output_cms[0]).await.unwrap();

    let fix_options = FixSpendStatusOptions::builder()
        .query_timeout_ms(302)
        .spend_id(spend.id)
        .build();
    let spend = context.handler.fix_status(fix_options.clone()).await.unwrap();
    assert_eq!(spend.status, SpendStatus::Succeeded as i32);
    let input_cms = context
        .db
        .commitments
        .find(SubFilter::equal(
            CommitmentColumn::CommitmentHash,
            BigUint::from_str(&spend.input_commitments[0]).unwrap(),
        ))
        .await
        .unwrap();
    assert_eq!(input_cms.len(), 1);
    assert!(input_cms[0].data.spent);
    let output_cms = context
        .db
        .commitments
        .find(SubFilter::equal(
            CommitmentColumn::CommitmentHash,
            BigUint::from_str(&spend.output_commitments[0]).unwrap(),
        ))
        .await
        .unwrap();
    assert_eq!(output_cms.len(), 1);
    assert!(!output_cms[0].data.spent);
    assert_eq!(output_cms[0].data.status, CommitmentStatus::Included as i32);
}

#[tokio::test]
async fn test_send_withdraw2x1() {
    let chain_id = 5_u64;
    let (config, contract_config) = setup_config(chain_id, "0x223903804Ee95e264F74C88B4F8583429524593c").await;
    let (_, private_key) = generate_private_key();
    let merkle_tree = generate_merkle_tree(6_usize);
    let tree_root = biguint_to_u256(&merkle_tree.root());
    let tx_hash = TxHash::from_str("0xa35c998eaf5df995dba638efc114a8f58353784d08a60467fba6ed1e8f0e64a8").unwrap();
    let transact_options = TransactTestOptions::builder()
        .num_inputs(2_usize)
        .num_outputs(1_usize)
        .root_hash(HashSet::from([tree_root]))
        .public_amount(HashSet::from([(U256::from_dec_str("80000000000000000").unwrap())]))
        .public_recipient(ethers_address_from_string("0x87813A8E81729C0100ce2568b6283772cb31bdb8").unwrap())
        .tx_hash(tx_hash)
        .out_rollup_fees(vec![U256::from_dec_str("40000000000000000").unwrap()])
        .build();
    let commitment_pool_contracts_options = CommitmentPoolTestOptions::builder()
        .chain_id(chain_id)
        .spent_nullifiers(HashMap::from([(U256::from(2_u64), false), (U256::from(4_u64), false)]))
        .known_root(HashMap::from([(tree_root, true)]))
        .transact_options(transact_options)
        .build();
    let commitment_pool_contracts =
        setup_commitment_pool_contracts(commitment_pool_contracts_options, &contract_config);
    let static_cache = setup_static_cache(&contract_config, &mystiko_types::CircuitType::Transaction2x1);
    let prover = setup_prover(true);
    let transactions_options = TransactionsTestOptions::builder()
        .chain_id(chain_id)
        .config(&config)
        .tx_hash(tx_hash)
        .block_number(200010000_u64)
        .build();
    let transactions = setup_transactions(transactions_options);
    let mut mock_data_packer_client = MockDataPackerClient::new();
    mock_data_packer_client
        .expect_query_merkle_tree()
        .times(1)
        .returning(move |_, _| {
            let proto_merkle_tree = ProtoMerkleTree::builder()
                .loaded_block_number(200010000_u64)
                .root_hash(u256_to_bytes(&tree_root))
                .commitment_hashes(merkle_tree.elements().iter().map(biguint_to_bytes).collect::<Vec<_>>())
                .build();
            Ok(Some(proto_merkle_tree))
        });
    let test_options: SendTestOptions = SendTestOptions::builder()
        .chain_id(chain_id)
        .config(config)
        .contract_config(contract_config.clone())
        .relayer_client(MockRelayerClient::new())
        .data_packer_client(mock_data_packer_client)
        .commitment_pool_contracts(commitment_pool_contracts)
        .prover(prover)
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
        .amount(12.0)
        .rollup_fee_amount(4.0)
        .recipient("0x87813A8E81729C0100ce2568b6283772cb31bdb8".to_string())
        .wallet_password("P@ssw0rd".to_string())
        .build();
    let spend = context.handler.create(create_options).await.unwrap();
    let send_options = SendSpendOptions::builder()
        .spend_id(spend.id)
        .wallet_password("P@ssw0rd".to_string())
        .private_key(private_key)
        .build();
    let mut spend = context.handler.send(send_options).await.unwrap();
    assert_eq!(spend.chain_id, chain_id);
    assert_eq!(spend.contract_address, contract_config.address());
    assert_eq!(spend.asset_symbol, contract_config.asset_symbol());
    assert_eq!(spend.asset_decimals, contract_config.asset_decimals());
    assert_eq!(spend.amount, 12.0);
    assert_eq!(spend.decimal_amount, "120000000000000000");
    assert_eq!(spend.recipient, "0x87813A8E81729C0100ce2568b6283772cb31bdb8");
    assert_eq!(spend.wallet_id, context.account.wallet_id);
    spend.input_commitments.sort();
    assert_eq!(spend.input_commitments, vec!["2".to_string(), "4".to_string()]);
    assert_eq!(spend.output_commitments.len(), 1);
    assert_eq!(spend.nullifiers.len(), spend.input_commitments.len());
    assert_eq!(spend.signature_public_key_hashes.len(), spend.input_commitments.len());
    assert_eq!(spend.encrypted_auditor_notes.len(), spend.input_commitments.len() * 5);
    assert_eq!(spend.rollup_fee_amount.unwrap(), 4.0);
    assert_eq!(spend.rollup_fee_decimal_amount.clone().unwrap(), "40000000000000000");
    assert_eq!(spend.rollup_fee_total_amount.unwrap(), 4.0);
    assert_eq!(
        spend.rollup_fee_total_decimal_amount.clone().unwrap(),
        "40000000000000000"
    );
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
    context.check_commitments(&spend, 200010000_u64, &[2.0]).await;
}

#[tokio::test]
async fn test_send_transfer1x1() {
    let chain_id = 5_u64;
    let (config, contract_config) = setup_config(chain_id, "0x223903804Ee95e264F74C88B4F8583429524593c").await;
    let (_, private_key) = generate_private_key();
    let merkle_tree = generate_merkle_tree(6_usize);
    let tree_root = biguint_to_u256(&merkle_tree.root());
    let tx_hash = TxHash::from_str("0xa35c998eaf5df995dba638efc114a8f58353784d08a60467fba6ed1e8f0e64a8").unwrap();
    let transact_options = TransactTestOptions::builder()
        .num_inputs(1_usize)
        .num_outputs(1_usize)
        .root_hash(HashSet::from([tree_root]))
        .tx_hash(tx_hash)
        .public_amount(HashSet::from([(U256::from_dec_str("0").unwrap())]))
        .out_rollup_fees(vec![U256::from_dec_str("40000000000000000").unwrap()])
        .build();
    let commitment_pool_contracts_options = CommitmentPoolTestOptions::builder()
        .chain_id(chain_id)
        .spent_nullifiers(HashMap::from([(U256::from(4_u64), false)]))
        .known_root(HashMap::from([(tree_root, true)]))
        .transact_options(transact_options)
        .build();
    let commitment_pool_contracts =
        setup_commitment_pool_contracts(commitment_pool_contracts_options, &contract_config);
    let static_cache = setup_static_cache(&contract_config, &mystiko_types::CircuitType::Transaction1x1);
    let prover = setup_prover(true);
    let transactions_options = TransactionsTestOptions::builder()
        .chain_id(chain_id)
        .config(&config)
        .tx_hash(tx_hash)
        .block_number(200010000_u64)
        .build();
    let transactions = setup_transactions(transactions_options);
    let mut mock_data_packer_client = MockDataPackerClient::new();
    mock_data_packer_client
        .expect_query_merkle_tree()
        .times(1)
        .returning(move |_, _| {
            let proto_merkle_tree = ProtoMerkleTree::builder()
                .loaded_block_number(200010000_u64)
                .root_hash(u256_to_bytes(&tree_root))
                .commitment_hashes(merkle_tree.elements().iter().map(biguint_to_bytes).collect::<Vec<_>>())
                .build();
            Ok(Some(proto_merkle_tree))
        });
    let test_options: SendTestOptions = SendTestOptions::builder()
        .chain_id(chain_id)
        .config(config)
        .contract_config(contract_config.clone())
        .relayer_client(MockRelayerClient::new())
        .data_packer_client(mock_data_packer_client)
        .commitment_pool_contracts(commitment_pool_contracts)
        .prover(prover)
        .static_cache(static_cache)
        .transactions(transactions)
        .build();
    let context = SendTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;
    let create_options = CreateSpendOptions::builder()
        .chain_id(chain_id)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Transfer as i32)
        .amount(10.0)
        .rollup_fee_amount(4.0)
        .recipient(context.account.shielded_address.clone())
        .wallet_password("P@ssw0rd".to_string())
        .build();
    let spend = context.handler.create(create_options).await.unwrap();
    let send_options = SendSpendOptions::builder()
        .spend_id(spend.id)
        .wallet_password("P@ssw0rd".to_string())
        .private_key(private_key)
        .build();
    let spend = context.handler.send(send_options).await.unwrap();
    assert_eq!(spend.chain_id, chain_id);
    assert_eq!(spend.contract_address, contract_config.address());
    assert_eq!(spend.asset_symbol, contract_config.asset_symbol());
    assert_eq!(spend.asset_decimals, contract_config.asset_decimals());
    assert_eq!(spend.amount, 10.0);
    assert_eq!(spend.decimal_amount, "100000000000000000");
    assert_eq!(spend.recipient, context.account.shielded_address);
    assert_eq!(spend.wallet_id, context.account.wallet_id);
    assert_eq!(spend.input_commitments, vec!["4".to_string()]);
    assert_eq!(spend.output_commitments.len(), 1);
    assert_eq!(spend.nullifiers.len(), spend.input_commitments.len());
    assert_eq!(spend.signature_public_key_hashes.len(), spend.input_commitments.len());
    assert_eq!(spend.encrypted_auditor_notes.len(), spend.input_commitments.len() * 5);
    assert_eq!(spend.rollup_fee_amount.unwrap(), 4.0);
    assert_eq!(spend.rollup_fee_decimal_amount.clone().unwrap(), "40000000000000000");
    assert_eq!(spend.rollup_fee_total_amount.unwrap(), 4.0);
    assert_eq!(
        spend.rollup_fee_total_decimal_amount.clone().unwrap(),
        "40000000000000000"
    );
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
    assert_eq!(spend.spend_type, SpendType::Transfer as i32);
    assert_eq!(spend.status, SpendStatus::Succeeded as i32);
    context.check_commitments(&spend, 200010000_u64, &[6.0]).await;
}

#[tokio::test]
async fn test_send_transfer2x1() {
    let chain_id = 5_u64;
    let (config, contract_config) = setup_config(chain_id, "0x223903804Ee95e264F74C88B4F8583429524593c").await;
    let (_, private_key) = generate_private_key();
    let merkle_tree = generate_merkle_tree(6_usize);
    let tree_root = biguint_to_u256(&merkle_tree.root());
    let tx_hash = TxHash::from_str("0xa35c998eaf5df995dba638efc114a8f58353784d08a60467fba6ed1e8f0e64a8").unwrap();
    let transact_options = TransactTestOptions::builder()
        .num_inputs(2_usize)
        .num_outputs(1_usize)
        .root_hash(HashSet::from([tree_root]))
        .tx_hash(tx_hash)
        .public_amount(HashSet::from([(U256::from_dec_str("0").unwrap())]))
        .out_rollup_fees(vec![U256::from_dec_str("40000000000000000").unwrap()])
        .build();
    let commitment_pool_contracts_options = CommitmentPoolTestOptions::builder()
        .chain_id(chain_id)
        .spent_nullifiers(HashMap::from([(U256::from(2_u64), false), (U256::from(4_u64), false)]))
        .known_root(HashMap::from([(tree_root, true)]))
        .transact_options(transact_options)
        .build();
    let commitment_pool_contracts =
        setup_commitment_pool_contracts(commitment_pool_contracts_options, &contract_config);
    let static_cache = setup_static_cache(&contract_config, &mystiko_types::CircuitType::Transaction2x1);
    let prover = setup_prover(true);
    let transactions_options = TransactionsTestOptions::builder()
        .chain_id(chain_id)
        .config(&config)
        .tx_hash(tx_hash)
        .block_number(200010000_u64)
        .build();
    let transactions = setup_transactions(transactions_options);
    let mut mock_data_packer_client = MockDataPackerClient::new();
    mock_data_packer_client
        .expect_query_merkle_tree()
        .times(1)
        .returning(move |_, _| {
            let proto_merkle_tree = ProtoMerkleTree::builder()
                .loaded_block_number(200010000_u64)
                .root_hash(u256_to_bytes(&tree_root))
                .commitment_hashes(merkle_tree.elements().iter().map(biguint_to_bytes).collect::<Vec<_>>())
                .build();
            Ok(Some(proto_merkle_tree))
        });
    let test_options: SendTestOptions = SendTestOptions::builder()
        .chain_id(chain_id)
        .config(config)
        .contract_config(contract_config.clone())
        .relayer_client(MockRelayerClient::new())
        .data_packer_client(mock_data_packer_client)
        .commitment_pool_contracts(commitment_pool_contracts)
        .prover(prover)
        .static_cache(static_cache)
        .transactions(transactions)
        .build();
    let context = SendTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;
    let create_options = CreateSpendOptions::builder()
        .chain_id(chain_id)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Transfer as i32)
        .amount(14.0)
        .rollup_fee_amount(4.0)
        .recipient(context.account.shielded_address.clone())
        .wallet_password("P@ssw0rd".to_string())
        .build();
    let spend = context.handler.create(create_options).await.unwrap();
    let send_options = SendSpendOptions::builder()
        .spend_id(spend.id)
        .wallet_password("P@ssw0rd".to_string())
        .private_key(private_key)
        .build();
    let mut spend = context.handler.send(send_options).await.unwrap();
    assert_eq!(spend.chain_id, chain_id);
    assert_eq!(spend.contract_address, contract_config.address());
    assert_eq!(spend.asset_symbol, contract_config.asset_symbol());
    assert_eq!(spend.asset_decimals, contract_config.asset_decimals());
    assert_eq!(spend.amount, 14.0);
    assert_eq!(spend.decimal_amount, "140000000000000000");
    assert_eq!(spend.recipient, context.account.shielded_address);
    assert_eq!(spend.wallet_id, context.account.wallet_id);
    spend.input_commitments.sort();
    assert_eq!(spend.input_commitments, vec!["2".to_string(), "4".to_string()]);
    assert_eq!(spend.output_commitments.len(), 1);
    assert_eq!(spend.nullifiers.len(), spend.input_commitments.len());
    assert_eq!(spend.signature_public_key_hashes.len(), spend.input_commitments.len());
    assert_eq!(spend.encrypted_auditor_notes.len(), spend.input_commitments.len() * 5);
    assert_eq!(spend.rollup_fee_amount.unwrap(), 4.0);
    assert_eq!(spend.rollup_fee_decimal_amount.clone().unwrap(), "40000000000000000");
    assert_eq!(spend.rollup_fee_total_amount.unwrap(), 4.0);
    assert_eq!(
        spend.rollup_fee_total_decimal_amount.clone().unwrap(),
        "40000000000000000"
    );
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
    assert_eq!(spend.spend_type, SpendType::Transfer as i32);
    assert_eq!(spend.status, SpendStatus::Succeeded as i32);
    context.check_commitments(&spend, 200010000_u64, &[10.0]).await;
}

#[tokio::test]
async fn test_send_transfer1x2() {
    let chain_id = 5_u64;
    let (config, contract_config) = setup_config(chain_id, "0x223903804Ee95e264F74C88B4F8583429524593c").await;
    let (_, private_key) = generate_private_key();
    let merkle_tree = generate_merkle_tree(6_usize);
    let tree_root = biguint_to_u256(&merkle_tree.root());
    let tx_hash = TxHash::from_str("0xa35c998eaf5df995dba638efc114a8f58353784d08a60467fba6ed1e8f0e64a8").unwrap();
    let transact_options = TransactTestOptions::builder()
        .num_inputs(1_usize)
        .num_outputs(2_usize)
        .root_hash(HashSet::from([tree_root]))
        .tx_hash(tx_hash)
        .public_amount(HashSet::from([(U256::from_dec_str("0").unwrap())]))
        .out_rollup_fees(vec![
            U256::from_dec_str("40000000000000000").unwrap(),
            U256::from_dec_str("40000000000000000").unwrap(),
        ])
        .build();
    let commitment_pool_contracts_options = CommitmentPoolTestOptions::builder()
        .chain_id(chain_id)
        .spent_nullifiers(HashMap::from([(U256::from(4_u64), false)]))
        .known_root(HashMap::from([(tree_root, true)]))
        .transact_options(transact_options)
        .build();
    let commitment_pool_contracts =
        setup_commitment_pool_contracts(commitment_pool_contracts_options, &contract_config);
    let static_cache = setup_static_cache(&contract_config, &mystiko_types::CircuitType::Transaction1x2);
    let prover = setup_prover(true);
    let transactions_options = TransactionsTestOptions::builder()
        .chain_id(chain_id)
        .config(&config)
        .tx_hash(tx_hash)
        .block_number(200010000_u64)
        .build();
    let transactions = setup_transactions(transactions_options);
    let mut mock_data_packer_client = MockDataPackerClient::new();
    mock_data_packer_client
        .expect_query_merkle_tree()
        .times(1)
        .returning(move |_, _| {
            let proto_merkle_tree = ProtoMerkleTree::builder()
                .loaded_block_number(200010000_u64)
                .root_hash(u256_to_bytes(&tree_root))
                .commitment_hashes(merkle_tree.elements().iter().map(biguint_to_bytes).collect::<Vec<_>>())
                .build();
            Ok(Some(proto_merkle_tree))
        });
    let test_options: SendTestOptions = SendTestOptions::builder()
        .chain_id(chain_id)
        .config(config)
        .contract_config(contract_config.clone())
        .relayer_client(MockRelayerClient::new())
        .data_packer_client(mock_data_packer_client)
        .commitment_pool_contracts(commitment_pool_contracts)
        .prover(prover)
        .static_cache(static_cache)
        .transactions(transactions)
        .build();
    let context = SendTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;
    let create_options = CreateSpendOptions::builder()
        .chain_id(chain_id)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Transfer as i32)
        .amount(9.0)
        .rollup_fee_amount(4.0)
        .recipient(context.account.shielded_address.clone())
        .wallet_password("P@ssw0rd".to_string())
        .build();
    let spend = context.handler.create(create_options).await.unwrap();
    let send_options = SendSpendOptions::builder()
        .spend_id(spend.id)
        .wallet_password("P@ssw0rd".to_string())
        .private_key(private_key)
        .build();
    let spend = context.handler.send(send_options).await.unwrap();
    assert_eq!(spend.chain_id, chain_id);
    assert_eq!(spend.contract_address, contract_config.address());
    assert_eq!(spend.asset_symbol, contract_config.asset_symbol());
    assert_eq!(spend.asset_decimals, contract_config.asset_decimals());
    assert_eq!(spend.amount, 9.0);
    assert_eq!(spend.decimal_amount, "90000000000000000");
    assert_eq!(spend.recipient, context.account.shielded_address);
    assert_eq!(spend.wallet_id, context.account.wallet_id);
    assert_eq!(spend.input_commitments, vec!["4".to_string()]);
    assert_eq!(spend.output_commitments.len(), 2);
    assert_eq!(spend.nullifiers.len(), spend.input_commitments.len());
    assert_eq!(spend.signature_public_key_hashes.len(), spend.input_commitments.len());
    assert_eq!(spend.encrypted_auditor_notes.len(), spend.input_commitments.len() * 5);
    assert_eq!(spend.rollup_fee_amount.unwrap(), 4.0);
    assert_eq!(spend.rollup_fee_decimal_amount.clone().unwrap(), "40000000000000000");
    assert_eq!(spend.rollup_fee_total_amount.unwrap(), 8.0);
    assert_eq!(
        spend.rollup_fee_total_decimal_amount.clone().unwrap(),
        "80000000000000000"
    );
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
    assert_eq!(spend.spend_type, SpendType::Transfer as i32);
    assert_eq!(spend.status, SpendStatus::Succeeded as i32);
    context.check_commitments(&spend, 200010000_u64, &[1.0, 1.0]).await;
}

#[tokio::test]
async fn test_send_transfer2x2() {
    let chain_id = 5_u64;
    let (config, contract_config) = setup_config(chain_id, "0x223903804Ee95e264F74C88B4F8583429524593c").await;
    let (_, private_key) = generate_private_key();
    let merkle_tree = generate_merkle_tree(6_usize);
    let tree_root = biguint_to_u256(&merkle_tree.root());
    let tx_hash = TxHash::from_str("0xa35c998eaf5df995dba638efc114a8f58353784d08a60467fba6ed1e8f0e64a8").unwrap();
    let transact_options = TransactTestOptions::builder()
        .num_inputs(2_usize)
        .num_outputs(2_usize)
        .root_hash(HashSet::from([tree_root]))
        .tx_hash(tx_hash)
        .public_amount(HashSet::from([(U256::from_dec_str("0").unwrap())]))
        .out_rollup_fees(vec![
            U256::from_dec_str("40000000000000000").unwrap(),
            U256::from_dec_str("40000000000000000").unwrap(),
        ])
        .build();
    let commitment_pool_contracts_options = CommitmentPoolTestOptions::builder()
        .chain_id(chain_id)
        .spent_nullifiers(HashMap::from([(U256::from(2_u64), false), (U256::from(4_u64), false)]))
        .known_root(HashMap::from([(tree_root, true)]))
        .transact_options(transact_options)
        .build();
    let commitment_pool_contracts =
        setup_commitment_pool_contracts(commitment_pool_contracts_options, &contract_config);
    let static_cache = setup_static_cache(&contract_config, &mystiko_types::CircuitType::Transaction2x2);
    let prover = setup_prover(true);
    let transactions_options = TransactionsTestOptions::builder()
        .chain_id(chain_id)
        .config(&config)
        .tx_hash(tx_hash)
        .block_number(200010000_u64)
        .build();
    let transactions = setup_transactions(transactions_options);
    let mut mock_data_packer_client = MockDataPackerClient::new();
    mock_data_packer_client
        .expect_query_merkle_tree()
        .times(1)
        .returning(move |_, _| {
            let proto_merkle_tree = ProtoMerkleTree::builder()
                .loaded_block_number(200010000_u64)
                .root_hash(u256_to_bytes(&tree_root))
                .commitment_hashes(merkle_tree.elements().iter().map(biguint_to_bytes).collect::<Vec<_>>())
                .build();
            Ok(Some(proto_merkle_tree))
        });
    let test_options: SendTestOptions = SendTestOptions::builder()
        .chain_id(chain_id)
        .config(config)
        .contract_config(contract_config.clone())
        .relayer_client(MockRelayerClient::new())
        .data_packer_client(mock_data_packer_client)
        .commitment_pool_contracts(commitment_pool_contracts)
        .prover(prover)
        .static_cache(static_cache)
        .transactions(transactions)
        .build();
    let context = SendTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;
    let create_options = CreateSpendOptions::builder()
        .chain_id(chain_id)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Transfer as i32)
        .amount(12.0)
        .rollup_fee_amount(4.0)
        .recipient(context.account.shielded_address.clone())
        .wallet_password("P@ssw0rd".to_string())
        .build();
    let spend = context.handler.create(create_options).await.unwrap();
    let send_options = SendSpendOptions::builder()
        .spend_id(spend.id)
        .wallet_password("P@ssw0rd".to_string())
        .private_key(private_key)
        .build();
    let mut spend = context.handler.send(send_options).await.unwrap();
    assert_eq!(spend.chain_id, chain_id);
    assert_eq!(spend.contract_address, contract_config.address());
    assert_eq!(spend.asset_symbol, contract_config.asset_symbol());
    assert_eq!(spend.asset_decimals, contract_config.asset_decimals());
    assert_eq!(spend.amount, 12.0);
    assert_eq!(spend.decimal_amount, "120000000000000000");
    assert_eq!(spend.recipient, context.account.shielded_address);
    assert_eq!(spend.wallet_id, context.account.wallet_id);
    spend.input_commitments.sort();
    assert_eq!(spend.input_commitments, vec!["2".to_string(), "4".to_string()]);
    assert_eq!(spend.output_commitments.len(), 2);
    assert_eq!(spend.nullifiers.len(), spend.input_commitments.len());
    assert_eq!(spend.signature_public_key_hashes.len(), spend.input_commitments.len());
    assert_eq!(spend.encrypted_auditor_notes.len(), spend.input_commitments.len() * 5);
    assert_eq!(spend.rollup_fee_amount.unwrap(), 4.0);
    assert_eq!(spend.rollup_fee_decimal_amount.clone().unwrap(), "40000000000000000");
    assert_eq!(spend.rollup_fee_total_amount.unwrap(), 8.0);
    assert_eq!(
        spend.rollup_fee_total_decimal_amount.clone().unwrap(),
        "80000000000000000"
    );
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
    assert_eq!(spend.spend_type, SpendType::Transfer as i32);
    assert_eq!(spend.status, SpendStatus::Succeeded as i32);
    context.check_commitments(&spend, 200010000_u64, &[2.0, 4.0]).await;

    let fix_options = FixSpendStatusOptions::builder()
        .query_timeout_ms(601)
        .spend_id(spend.id)
        .build();
    let spend = context.handler.fix_status(fix_options.clone()).await.unwrap();
    assert_eq!(spend.status, SpendStatus::Failed as i32);
    let input_cms = context
        .db
        .commitments
        .find(SubFilter::in_list(
            CommitmentColumn::CommitmentHash,
            vec![
                BigUint::from_str(&spend.input_commitments[0]).unwrap(),
                BigUint::from_str(&spend.input_commitments[1]).unwrap(),
            ],
        ))
        .await
        .unwrap();
    assert_eq!(input_cms.len(), 2);
    assert!(!input_cms[0].data.spent);
    assert!(!input_cms[1].data.spent);
    let mut output_cms = context
        .db
        .commitments
        .find(SubFilter::in_list(
            CommitmentColumn::CommitmentHash,
            vec![
                BigUint::from_str(&spend.output_commitments[0]).unwrap(),
                BigUint::from_str(&spend.output_commitments[1]).unwrap(),
            ],
        ))
        .await
        .unwrap();
    assert_eq!(output_cms.len(), 2);
    assert!(!output_cms[0].data.spent);
    assert!(!output_cms[1].data.spent);
    assert_eq!(output_cms[0].data.status, CommitmentStatus::Queued as i32);
    assert_eq!(output_cms[1].data.status, CommitmentStatus::Queued as i32);
    output_cms[0].data.included_transaction_hash = Some("123".to_string());
    context.db.commitments.update(&output_cms[0]).await.unwrap();

    let fix_options = FixSpendStatusOptions::builder()
        .query_timeout_ms(602)
        .spend_id(spend.id)
        .build();
    let spend = context.handler.fix_status(fix_options.clone()).await.unwrap();
    assert_eq!(spend.status, SpendStatus::Succeeded as i32);
    let input_cms = context
        .db
        .commitments
        .find(SubFilter::in_list(
            CommitmentColumn::CommitmentHash,
            vec![
                BigUint::from_str(&spend.input_commitments[0]).unwrap(),
                BigUint::from_str(&spend.input_commitments[1]).unwrap(),
            ],
        ))
        .await
        .unwrap();
    assert_eq!(input_cms.len(), 2);
    assert!(input_cms[0].data.spent);
    assert!(input_cms[1].data.spent);
    let output_cms = context
        .db
        .commitments
        .find(SubFilter::in_list(
            CommitmentColumn::CommitmentHash,
            vec![
                BigUint::from_str(&spend.output_commitments[0]).unwrap(),
                BigUint::from_str(&spend.output_commitments[1]).unwrap(),
            ],
        ))
        .await
        .unwrap();
    assert_eq!(output_cms.len(), 2);
    assert!(!output_cms[0].data.spent);
    assert!(!output_cms[1].data.spent);
    assert_eq!(output_cms[0].data.status, CommitmentStatus::Included as i32);
    assert_eq!(output_cms[1].data.status, CommitmentStatus::Queued as i32);
}

#[tokio::test]
async fn test_send_withdraw_with_gas_relayer() {
    let chain_id = 5_u64;
    let (config, contract_config) = setup_config(chain_id, "0x223903804Ee95e264F74C88B4F8583429524593c").await;
    let (_, private_key) = generate_private_key();
    let merkle_tree = generate_merkle_tree(6_usize);
    let tree_root = biguint_to_u256(&merkle_tree.root());
    let tx_hash = TxHash::from_str("0xa35c998eaf5df995dba638efc114a8f58353784d08a60467fba6ed1e8f0e64a8").unwrap();
    let transact_options = TransactTestOptions::builder()
        .num_inputs(1_usize)
        .num_outputs(1_usize)
        .root_hash(HashSet::from([tree_root]))
        .public_amount(HashSet::from([(U256::from_dec_str("39820000000000000").unwrap())]))
        .public_recipient(ethers_address_from_string("0x87813A8E81729C0100ce2568b6283772cb31bdb8").unwrap())
        .tx_hash(tx_hash)
        .build();
    let commitment_pool_contracts_options = CommitmentPoolTestOptions::builder()
        .chain_id(chain_id)
        .spent_nullifiers(HashMap::from([(U256::from(4_u64), false)]))
        .known_root(HashMap::from([(tree_root, true)]))
        .transact_options(transact_options)
        .build();
    let commitment_pool_contracts =
        setup_commitment_pool_contracts(commitment_pool_contracts_options, &contract_config);
    let mut relayer_client = MockRelayerClient::new();
    let gas_relayers = vec![RegisterInfo::builder()
        .chain_id(5_u64)
        .name("test_relayer_1".to_string())
        .url("https://test_relayer1.mystiko.network".to_string())
        .relayer_address("0x357c6Fd2cEE77bA5de49e0bB9d49444781A8f0cc".to_string())
        .relayer_contract_address("0x3f4a3378852887b81dFE593Ee1A68Be4adcd888d".to_string())
        .available(true)
        .support(true)
        .contracts(vec![ContractInfo::builder()
            .asset_symbol("MTT".to_string())
            .relayer_fee_of_ten_thousandth(10_u32)
            .minimum_gas_fee("100000000000000".to_string())
            .build()])
        .build()];
    relayer_client
        .expect_register_info()
        .withf(move |options| {
            let more_options = options.options.as_ref().unwrap();
            options.chain_id == chain_id
                && options.name.is_none()
                && more_options.asset_symbol == "MTT"
                && more_options.circuit_type == mystiko_types::CircuitType::Transaction1x1
                && !more_options.show_unavailable
        })
        .returning(move |_| Ok(gas_relayers.clone()));
    relayer_client
        .expect_relay_transact()
        .withf(move |request| {
            request.relayer_url == "https://test_relayer1.mystiko.network"
                && request.data.chain_id == chain_id
                && request.data.pool_address == "0x223903804Ee95e264F74C88B4F8583429524593c"
                && request.data.asset_symbol == "MTT"
                && request.data.asset_decimals == 16
                && request.data.spend_type == SpendType::Withdraw
                && request.data.bridge_type == mystiko_types::BridgeType::Loop
                && request.data.circuit_type == mystiko_types::CircuitType::Transaction1x1
                && request.data.contract_param.root_hash == tree_root
                && request.data.contract_param.public_amount == U256::from_dec_str("39820000000000000").unwrap()
        })
        .returning(|_| Ok(RelayTransactResponse::builder().uuid("job_1".to_string()).build()));
    relayer_client
        .expect_wait_transaction()
        .withf(|request| {
            request.relayer_url == "https://test_relayer1.mystiko.network"
                && request.uuid == "job_1"
                && request.interval == Some(Duration::from_millis(10_u64))
                && request.timeout == Some(Duration::from_millis(100_u64))
                && request.waiting_status == TransactStatus::Succeeded
        })
        .returning(move |_| {
            Ok(RelayTransactStatusResponse::builder()
                .uuid("job_1".to_string())
                .chain_id(chain_id)
                .spend_type(SpendType::Withdraw)
                .transaction_hash(tx_hash.encode_hex())
                .status(TransactStatus::Succeeded)
                .error_msg(None)
                .build())
        });
    let mut mock_data_packer_client = MockDataPackerClient::new();
    mock_data_packer_client
        .expect_query_merkle_tree()
        .times(1)
        .returning(move |_, _| {
            let proto_merkle_tree = ProtoMerkleTree::builder()
                .loaded_block_number(200010000_u64)
                .root_hash(u256_to_bytes(&tree_root))
                .commitment_hashes(merkle_tree.elements().iter().map(biguint_to_bytes).collect::<Vec<_>>())
                .build();
            Ok(Some(proto_merkle_tree))
        });
    let transactions_options = TransactionsTestOptions::builder()
        .chain_id(chain_id)
        .config(&config)
        .tx_hash(tx_hash)
        .block_number(200010000_u64)
        .build();
    let transactions = setup_transactions(transactions_options);
    let static_cache = setup_static_cache(&contract_config, &mystiko_types::CircuitType::Transaction1x1);
    let prover = setup_prover(true);
    let test_options: SendTestOptions = SendTestOptions::builder()
        .chain_id(chain_id)
        .config(config)
        .contract_config(contract_config.clone())
        .relayer_client(relayer_client)
        .data_packer_client(mock_data_packer_client)
        .commitment_pool_contracts(commitment_pool_contracts)
        .prover(prover)
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
        .amount(8.0)
        .rollup_fee_amount(4.0)
        .recipient("0x87813A8E81729C0100ce2568b6283772cb31bdb8".to_string())
        .wallet_password("P@ssw0rd".to_string())
        .gas_relayer("test_relayer_1".to_string())
        .build();
    let spend = context.handler.create(create_options).await.unwrap();
    let send_options = SendSpendOptions::builder()
        .spend_id(spend.id)
        .wallet_password("P@ssw0rd".to_string())
        .private_key(private_key)
        .relayer_wait_interval_ms(10_u64)
        .relayer_wait_timeout_ms(100_u64)
        .build();
    let spend = context.handler.send(send_options).await.unwrap();
    assert_eq!(spend.chain_id, chain_id);
    assert_eq!(spend.contract_address, contract_config.address());
    assert_eq!(spend.asset_symbol, contract_config.asset_symbol());
    assert_eq!(spend.asset_decimals, contract_config.asset_decimals());
    assert_eq!(spend.amount, 8.0);
    assert_eq!(spend.decimal_amount, "80000000000000000");
    assert_eq!(spend.recipient, "0x87813A8E81729C0100ce2568b6283772cb31bdb8");
    assert_eq!(spend.wallet_id, context.account.wallet_id);
    assert_eq!(spend.input_commitments, vec!["4".to_string()]);
    assert_eq!(spend.output_commitments.len(), 1);
    assert_eq!(spend.nullifiers.len(), spend.input_commitments.len());
    assert_eq!(spend.signature_public_key_hashes.len(), spend.input_commitments.len());
    assert_eq!(spend.encrypted_auditor_notes.len(), spend.input_commitments.len() * 5);
    assert_eq!(spend.rollup_fee_amount.unwrap(), 4.0);
    assert_eq!(spend.rollup_fee_decimal_amount.clone().unwrap(), "40000000000000000");
    assert_eq!(spend.rollup_fee_total_amount.unwrap(), 4.0);
    assert_eq!(
        spend.rollup_fee_total_decimal_amount.clone().unwrap(),
        "40000000000000000"
    );
    assert_eq!(spend.gas_relayer_fee_amount.unwrap(), 0.018);
    assert_eq!(spend.gas_relayer_fee_decimal_amount.clone().unwrap(), "180000000000000");
    assert_eq!(
        spend.gas_relayer_address.clone().unwrap(),
        "0x357c6Fd2cEE77bA5de49e0bB9d49444781A8f0cc"
    );
    assert_eq!(
        spend.gas_relayer_url.clone().unwrap(),
        "https://test_relayer1.mystiko.network"
    );
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
    context.check_commitments(&spend, 200010000_u64, &[2.0]).await;
}

#[tokio::test]
async fn test_send_transfer_with_gas_relayer() {
    let chain_id = 5_u64;
    let (config, contract_config) = setup_config(chain_id, "0x223903804Ee95e264F74C88B4F8583429524593c").await;
    let (_, private_key) = generate_private_key();
    let merkle_tree = generate_merkle_tree(6_usize);
    let tree_root = biguint_to_u256(&merkle_tree.root());
    let tx_hash = TxHash::from_str("0xa35c998eaf5df995dba638efc114a8f58353784d08a60467fba6ed1e8f0e64a8").unwrap();
    let transact_options = TransactTestOptions::builder()
        .num_inputs(2_usize)
        .num_outputs(2_usize)
        .root_hash(HashSet::from([tree_root]))
        .tx_hash(tx_hash)
        .build();
    let commitment_pool_contracts_options = CommitmentPoolTestOptions::builder()
        .chain_id(chain_id)
        .spent_nullifiers(HashMap::from([(U256::from(2_u64), false), (U256::from(4_u64), false)]))
        .known_root(HashMap::from([(tree_root, true)]))
        .transact_options(transact_options)
        .build();
    let commitment_pool_contracts =
        setup_commitment_pool_contracts(commitment_pool_contracts_options, &contract_config);
    let mut relayer_client = MockRelayerClient::new();
    let gas_relayers = vec![RegisterInfo::builder()
        .chain_id(5_u64)
        .name("test_relayer_1".to_string())
        .url("https://test_relayer1.mystiko.network".to_string())
        .relayer_address("0x357c6Fd2cEE77bA5de49e0bB9d49444781A8f0cc".to_string())
        .relayer_contract_address("0x3f4a3378852887b81dFE593Ee1A68Be4adcd888d".to_string())
        .available(true)
        .support(true)
        .contracts(vec![ContractInfo::builder()
            .asset_symbol("MTT".to_string())
            .relayer_fee_of_ten_thousandth(10_u32)
            .minimum_gas_fee("100000000000000".to_string())
            .build()])
        .build()];
    relayer_client
        .expect_register_info()
        .withf(move |options| {
            let more_options = options.options.as_ref().unwrap();
            options.chain_id == chain_id
                && options.name.is_none()
                && more_options.asset_symbol == "MTT"
                && more_options.circuit_type == mystiko_types::CircuitType::Transaction2x2
                && !more_options.show_unavailable
        })
        .returning(move |_| Ok(gas_relayers.clone()));
    relayer_client
        .expect_relay_transact()
        .withf(move |request| {
            request.relayer_url == "https://test_relayer1.mystiko.network"
                && request.data.chain_id == chain_id
                && request.data.pool_address == "0x223903804Ee95e264F74C88B4F8583429524593c"
                && request.data.asset_symbol == "MTT"
                && request.data.asset_decimals == 16
                && request.data.spend_type == SpendType::Transfer
                && request.data.bridge_type == mystiko_types::BridgeType::Loop
                && request.data.circuit_type == mystiko_types::CircuitType::Transaction2x2
                && request.data.contract_param.root_hash == tree_root
                && request.data.contract_param.public_amount == U256::from_dec_str("0").unwrap()
        })
        .returning(|_| Ok(RelayTransactResponse::builder().uuid("job_1".to_string()).build()));
    relayer_client
        .expect_wait_transaction()
        .withf(|request| {
            request.relayer_url == "https://test_relayer1.mystiko.network"
                && request.uuid == "job_1"
                && request.interval == Some(Duration::from_millis(10_u64))
                && request.timeout == Some(Duration::from_millis(100_u64))
                && request.waiting_status == TransactStatus::Succeeded
        })
        .returning(move |_| {
            Ok(RelayTransactStatusResponse::builder()
                .uuid("job_1".to_string())
                .chain_id(chain_id)
                .spend_type(SpendType::Transfer)
                .transaction_hash(tx_hash.encode_hex())
                .status(TransactStatus::Succeeded)
                .error_msg(None)
                .build())
        });
    let mut mock_data_packer_client = MockDataPackerClient::new();
    mock_data_packer_client
        .expect_query_merkle_tree()
        .times(1)
        .returning(move |_, _| {
            let proto_merkle_tree = ProtoMerkleTree::builder()
                .loaded_block_number(200010000_u64)
                .root_hash(u256_to_bytes(&tree_root))
                .commitment_hashes(merkle_tree.elements().iter().map(biguint_to_bytes).collect::<Vec<_>>())
                .build();
            Ok(Some(proto_merkle_tree))
        });
    let transactions_options = TransactionsTestOptions::builder()
        .chain_id(chain_id)
        .config(&config)
        .tx_hash(tx_hash)
        .block_number(200010000_u64)
        .build();
    let transactions = setup_transactions(transactions_options);
    let static_cache = setup_static_cache(&contract_config, &mystiko_types::CircuitType::Transaction2x2);
    let prover = setup_prover(true);
    let test_options: SendTestOptions = SendTestOptions::builder()
        .chain_id(chain_id)
        .config(config)
        .contract_config(contract_config.clone())
        .relayer_client(relayer_client)
        .data_packer_client(mock_data_packer_client)
        .commitment_pool_contracts(commitment_pool_contracts)
        .prover(prover)
        .static_cache(static_cache)
        .transactions(transactions)
        .build();
    let context = SendTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;
    let create_options = CreateSpendOptions::builder()
        .chain_id(chain_id)
        .asset_symbol("MTT".to_string())
        .bridge_type(BridgeType::Loop as i32)
        .spend_type(SpendType::Transfer as i32)
        .amount(12.0)
        .rollup_fee_amount(4.0)
        .recipient(context.account.shielded_address.clone())
        .wallet_password("P@ssw0rd".to_string())
        .gas_relayer("test_relayer_1".to_string())
        .build();
    let spend = context.handler.create(create_options).await.unwrap();
    let send_options = SendSpendOptions::builder()
        .spend_id(spend.id)
        .wallet_password("P@ssw0rd".to_string())
        .private_key(private_key)
        .relayer_wait_interval_ms(10_u64)
        .relayer_wait_timeout_ms(100_u64)
        .build();
    let mut spend = context.handler.send(send_options).await.unwrap();
    assert_eq!(spend.chain_id, chain_id);
    assert_eq!(spend.contract_address, contract_config.address());
    assert_eq!(spend.asset_symbol, contract_config.asset_symbol());
    assert_eq!(spend.asset_decimals, contract_config.asset_decimals());
    assert_eq!(spend.amount, 12.0);
    assert_eq!(spend.decimal_amount, "120000000000000000");
    assert_eq!(spend.recipient, context.account.shielded_address);
    assert_eq!(spend.wallet_id, context.account.wallet_id);
    spend.input_commitments.sort();
    assert_eq!(spend.input_commitments, vec!["2".to_string(), "4".to_string()]);
    assert_eq!(spend.output_commitments.len(), 2);
    assert_eq!(spend.nullifiers.len(), spend.input_commitments.len());
    assert_eq!(spend.signature_public_key_hashes.len(), spend.input_commitments.len());
    assert_eq!(spend.encrypted_auditor_notes.len(), spend.input_commitments.len() * 5);
    assert_eq!(spend.rollup_fee_amount.unwrap(), 4.0);
    assert_eq!(spend.rollup_fee_decimal_amount.clone().unwrap(), "40000000000000000");
    assert_eq!(spend.rollup_fee_total_amount.unwrap(), 8.0);
    assert_eq!(
        spend.rollup_fee_total_decimal_amount.clone().unwrap(),
        "80000000000000000"
    );
    assert_eq!(spend.gas_relayer_fee_amount.unwrap(), 0.022);
    assert_eq!(spend.gas_relayer_fee_decimal_amount.clone().unwrap(), "220000000000000");
    assert_eq!(
        spend.gas_relayer_address.clone().unwrap(),
        "0x357c6Fd2cEE77bA5de49e0bB9d49444781A8f0cc"
    );
    assert_eq!(
        spend.gas_relayer_url.clone().unwrap(),
        "https://test_relayer1.mystiko.network"
    );
    assert!(spend.signature_public_key.is_some());
    assert_eq!(spend.asset_address(), contract_config.asset_address().unwrap());
    assert!(spend.proof.is_some());
    assert_eq!(spend.root_hash(), tree_root.to_string());
    assert!(spend.signature.is_some());
    assert!(spend.random_auditing_public_key.is_some());
    assert!(spend.error_message.is_none());
    assert_eq!(spend.transaction_hash(), tx_hash.encode_hex());
    assert_eq!(spend.bridge_type, BridgeType::Loop as i32);
    assert_eq!(spend.spend_type, SpendType::Transfer as i32);
    assert_eq!(spend.status, SpendStatus::Succeeded as i32);
    context.check_commitments(&spend, 200010000_u64, &[2.0, 3.978]).await;
}

#[tokio::test]
async fn test_send_withdraw_bridge_assets() {
    let chain_id = 97_u64;
    let (config, contract_config) = setup_config(chain_id, "0xF85679316f1C3998C6387F6F707b31AeEB3a9aBe").await;
    let (_, private_key) = generate_private_key();
    let merkle_tree = generate_merkle_tree(6_usize);
    let tree_root = biguint_to_u256(&merkle_tree.root());
    let tx_hash = TxHash::from_str("0xa35c998eaf5df995dba638efc114a8f58353784d08a60467fba6ed1e8f0e64a8").unwrap();
    let transact_options = TransactTestOptions::builder()
        .num_inputs(1_usize)
        .num_outputs(0_usize)
        .root_hash(HashSet::from([tree_root]))
        .public_amount(HashSet::from([(U256::from_dec_str("3000000000000000000").unwrap())]))
        .public_recipient(ethers_address_from_string("0x87813A8E81729C0100ce2568b6283772cb31bdb8").unwrap())
        .tx_hash(tx_hash)
        .build();
    let commitment_pool_contracts_options = CommitmentPoolTestOptions::builder()
        .chain_id(chain_id)
        .spent_nullifiers(HashMap::from([(U256::from(0_u64), false)]))
        .known_root(HashMap::from([(tree_root, true)]))
        .transact_options(transact_options)
        .query_timeout_ms(100_u64)
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
        .build();
    let transactions = setup_transactions(transactions_options);
    let assets = setup_assets(
        chain_id,
        &contract_config,
        U256::from_dec_str("10000000000000000000").unwrap(),
        Some(100_u64),
    );
    let mut mock_data_packer_client = MockDataPackerClient::new();
    mock_data_packer_client
        .expect_query_merkle_tree()
        .times(1)
        .returning(move |_, _| {
            let proto_merkle_tree = ProtoMerkleTree::builder()
                .loaded_block_number(200010000_u64)
                .root_hash(u256_to_bytes(&tree_root))
                .commitment_hashes(merkle_tree.elements().iter().map(biguint_to_bytes).collect::<Vec<_>>())
                .build();
            Ok(Some(proto_merkle_tree))
        });
    let test_options: SendTestOptions = SendTestOptions::builder()
        .chain_id(chain_id)
        .config(config)
        .contract_config(contract_config.clone())
        .relayer_client(MockRelayerClient::new())
        .data_packer_client(mock_data_packer_client)
        .commitment_pool_contracts(commitment_pool_contracts)
        .prover(prover)
        .static_cache(static_cache)
        .transactions(transactions)
        .assets(assets)
        .build();
    let context = SendTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;
    let create_options = CreateSpendOptions::builder()
        .chain_id(chain_id)
        .asset_symbol("BNB".to_string())
        .bridge_type(BridgeType::Tbridge as i32)
        .spend_type(SpendType::Withdraw as i32)
        .amount(3.0)
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
        .build();
    let spend = context.handler.send(send_options).await.unwrap();
    assert_eq!(spend.chain_id, chain_id);
    assert_eq!(spend.contract_address, contract_config.address());
    assert_eq!(spend.asset_symbol, contract_config.asset_symbol());
    assert_eq!(spend.asset_decimals, contract_config.asset_decimals());
    assert_eq!(spend.amount, 3.0);
    assert_eq!(spend.decimal_amount, "3000000000000000000");
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
    assert!(spend.asset_address.is_none());
    assert!(spend.proof.is_some());
    assert_eq!(spend.root_hash(), tree_root.to_string());
    assert!(spend.signature.is_some());
    assert!(spend.random_auditing_public_key.is_some());
    assert!(spend.error_message.is_none());
    assert_eq!(spend.transaction_hash(), tx_hash.encode_hex());
    assert_eq!(spend.bridge_type, BridgeType::Tbridge as i32);
    assert_eq!(spend.spend_type, SpendType::Withdraw as i32);
    assert_eq!(spend.status, SpendStatus::Succeeded as i32);
    context.check_commitments(&spend, 200010000_u64, &[]).await;
}

#[tokio::test]
async fn test_send_with_wrong_status() {
    let chain_id = 5_u64;
    let (config, contract_config) = setup_config(chain_id, "0x223903804Ee95e264F74C88B4F8583429524593c").await;
    let (_, private_key) = generate_private_key();
    let commitment_pool_contracts_options = CommitmentPoolTestOptions::builder().chain_id(chain_id).build();
    let commitment_pool_contracts =
        setup_commitment_pool_contracts(commitment_pool_contracts_options, &contract_config);
    let test_options: SendTestOptions = SendTestOptions::builder()
        .chain_id(chain_id)
        .config(config)
        .contract_config(contract_config.clone())
        .commitment_pool_contracts(commitment_pool_contracts)
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
        .build();
    let spend = context.handler.create(create_options).await.unwrap();
    context
        .db
        .spends
        .update_by_filter(
            ColumnValues::new().set_value(SpendColumn::Status, SpendStatus::Succeeded as i32),
            SubFilter::equal(DocumentColumn::Id, spend.id.clone()),
        )
        .await
        .unwrap();
    let send_options = SendSpendOptions::builder()
        .spend_id(spend.id.clone())
        .wallet_password("P@ssw0rd".to_string())
        .private_key(private_key)
        .build();
    let result = context.handler.send(send_options).await;
    assert_eq!(
        result.unwrap_err().to_string(),
        "cannot send spend with status=Succeeded"
    );
}

#[tokio::test]
async fn test_send_with_insufficient_pool_balance() {
    let chain_id = 5_u64;
    let (config, contract_config) = setup_config(chain_id, "0x5050F69a9786F081509234F1a7F4684b5E5b76C9").await;
    let (_, private_key) = generate_private_key();
    let merkle_tree = generate_merkle_tree(6_usize);
    let tree_root = biguint_to_u256(&merkle_tree.root());
    let tx_hash = TxHash::from_str("0xa35c998eaf5df995dba638efc114a8f58353784d08a60467fba6ed1e8f0e64a8").unwrap();
    let transact_options = TransactTestOptions::builder()
        .num_inputs(1_usize)
        .num_outputs(0_usize)
        .root_hash(HashSet::from([tree_root]))
        .public_amount(HashSet::from([(U256::from_dec_str("30000000000000000").unwrap())]))
        .public_recipient(ethers_address_from_string("0x87813A8E81729C0100ce2568b6283772cb31bdb8").unwrap())
        .tx_hash(tx_hash)
        .build();
    let commitment_pool_contracts_options = CommitmentPoolTestOptions::builder()
        .chain_id(chain_id)
        .spent_nullifiers(HashMap::from([(U256::from(0_u64), false)]))
        .known_root(HashMap::from([(tree_root, true)]))
        .transact_options(transact_options)
        .build();
    let commitment_pool_contracts =
        setup_commitment_pool_contracts(commitment_pool_contracts_options, &contract_config);
    let static_cache = setup_static_cache(&contract_config, &mystiko_types::CircuitType::Transaction1x0);
    let prover = setup_prover(false);
    let assets = setup_assets(chain_id, &contract_config, U256::from_dec_str("0").unwrap(), None);
    let test_options: SendTestOptions = SendTestOptions::builder()
        .chain_id(chain_id)
        .config(config)
        .contract_config(contract_config.clone())
        .assets(assets)
        .commitment_pool_contracts(commitment_pool_contracts)
        .prover(prover)
        .static_cache(static_cache)
        .build();
    let context = SendTestContext::new(test_options).await;
    context.generate_commitments(&[3.0, 4.0, 10.0]).await;
    let create_options = CreateSpendOptions::builder()
        .chain_id(chain_id)
        .asset_symbol("mBNB".to_string())
        .bridge_type(BridgeType::Tbridge as i32)
        .spend_type(SpendType::Withdraw as i32)
        .amount(3.0)
        .recipient("0x87813A8E81729C0100ce2568b6283772cb31bdb8".to_string())
        .wallet_password("P@ssw0rd".to_string())
        .build();
    let spend = context.handler.create(create_options).await.unwrap();
    let send_options = SendSpendOptions::builder()
        .spend_id(spend.id.clone())
        .wallet_password("P@ssw0rd".to_string())
        .private_key(private_key)
        .build();
    let result = context.handler.send(send_options).await;
    assert_eq!(result.unwrap_err().to_string(), "insufficient pool balance: 0");
    let spend = context.handler.find_by_id(spend.id).await.unwrap().unwrap();
    assert_eq!(spend.status, SpendStatus::Failed as i32);
    assert_eq!(spend.error_message.unwrap(), "insufficient pool balance: 0");
}

#[tokio::test]
async fn test_send_with_double_spending() {
    let chain_id = 5_u64;
    let (config, contract_config) = setup_config(chain_id, "0x223903804Ee95e264F74C88B4F8583429524593c").await;
    let (_, private_key) = generate_private_key();
    let merkle_tree = generate_merkle_tree(6_usize);
    let tree_root = biguint_to_u256(&merkle_tree.root());
    let tx_hash = TxHash::from_str("0xa35c998eaf5df995dba638efc114a8f58353784d08a60467fba6ed1e8f0e64a8").unwrap();
    let transact_options = TransactTestOptions::builder()
        .num_inputs(1_usize)
        .num_outputs(0_usize)
        .root_hash(HashSet::from([tree_root]))
        .public_amount(HashSet::from([(U256::from_dec_str("30000000000000000").unwrap())]))
        .public_recipient(ethers_address_from_string("0x87813A8E81729C0100ce2568b6283772cb31bdb8").unwrap())
        .tx_hash(tx_hash)
        .build();
    let commitment_pool_contracts_options = CommitmentPoolTestOptions::builder()
        .chain_id(chain_id)
        .spent_nullifiers(HashMap::from([(U256::from(0_u64), true)]))
        .known_root(HashMap::from([(tree_root, true)]))
        .transact_options(transact_options)
        .build();
    let commitment_pool_contracts =
        setup_commitment_pool_contracts(commitment_pool_contracts_options, &contract_config);
    let static_cache = setup_static_cache(&contract_config, &mystiko_types::CircuitType::Transaction1x0);
    let prover = setup_prover(true);
    let test_options: SendTestOptions = SendTestOptions::builder()
        .chain_id(chain_id)
        .config(config)
        .contract_config(contract_config.clone())
        .commitment_pool_contracts(commitment_pool_contracts)
        .prover(prover)
        .static_cache(static_cache)
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
        .build();
    let spend = context.handler.create(create_options).await.unwrap();
    let send_options = SendSpendOptions::builder()
        .spend_id(spend.id.clone())
        .wallet_password("P@ssw0rd".to_string())
        .private_key(private_key)
        .build();
    let result = context.handler.send(send_options).await;
    assert_eq!(result.unwrap_err().to_string(), "already spent commitment: 0");
    let spend = context.handler.find_by_id(spend.id).await.unwrap().unwrap();
    assert_eq!(spend.status, SpendStatus::Failed as i32);
    assert_eq!(spend.error_message.unwrap(), "already spent commitment: 0");
}

#[tokio::test]
async fn test_send_with_unknown_root() {
    let chain_id = 5_u64;
    let (config, contract_config) = setup_config(chain_id, "0x223903804Ee95e264F74C88B4F8583429524593c").await;
    let (_, private_key) = generate_private_key();
    let merkle_tree = generate_merkle_tree(6_usize);
    let tree_root = biguint_to_u256(&merkle_tree.root());
    let tx_hash = TxHash::from_str("0xa35c998eaf5df995dba638efc114a8f58353784d08a60467fba6ed1e8f0e64a8").unwrap();
    let transact_options = TransactTestOptions::builder()
        .num_inputs(1_usize)
        .num_outputs(0_usize)
        .root_hash(HashSet::from([tree_root]))
        .public_amount(HashSet::from([(U256::from_dec_str("30000000000000000").unwrap())]))
        .public_recipient(ethers_address_from_string("0x87813A8E81729C0100ce2568b6283772cb31bdb8").unwrap())
        .tx_hash(tx_hash)
        .build();
    let commitment_pool_contracts_options = CommitmentPoolTestOptions::builder()
        .chain_id(chain_id)
        .spent_nullifiers(HashMap::from([(U256::from(0_u64), false)]))
        .known_root(HashMap::from([(tree_root, false)]))
        .transact_options(transact_options)
        .build();
    let commitment_pool_contracts =
        setup_commitment_pool_contracts(commitment_pool_contracts_options, &contract_config);
    let static_cache = setup_static_cache(&contract_config, &mystiko_types::CircuitType::Transaction1x0);
    let prover = setup_prover(true);
    let mut mock_data_packer_client = MockDataPackerClient::new();
    mock_data_packer_client
        .expect_query_merkle_tree()
        .times(1)
        .returning(move |_, _| {
            let proto_merkle_tree = ProtoMerkleTree::builder()
                .loaded_block_number(200010000_u64)
                .root_hash(u256_to_bytes(&tree_root))
                .commitment_hashes(merkle_tree.elements().iter().map(biguint_to_bytes).collect::<Vec<_>>())
                .build();
            Ok(Some(proto_merkle_tree))
        });
    let test_options: SendTestOptions = SendTestOptions::builder()
        .chain_id(chain_id)
        .config(config)
        .contract_config(contract_config.clone())
        .commitment_pool_contracts(commitment_pool_contracts)
        .prover(prover)
        .static_cache(static_cache)
        .data_packer_client(mock_data_packer_client)
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
        .build();
    let spend = context.handler.create(create_options).await.unwrap();
    let send_options = SendSpendOptions::builder()
        .spend_id(spend.id.clone())
        .wallet_password("P@ssw0rd".to_string())
        .private_key(private_key)
        .build();
    let result = context.handler.send(send_options).await;
    assert_eq!(
        result.unwrap_err().to_string(),
        format!("unknown merkle tree root: {}", tree_root)
    );
    let spend = context.handler.find_by_id(spend.id).await.unwrap().unwrap();
    assert_eq!(spend.status, SpendStatus::Failed as i32);
    assert_eq!(
        spend.error_message.unwrap(),
        format!("unknown merkle tree root: {}", tree_root)
    );
}

#[tokio::test]
async fn test_send_with_wrong_proof() {
    let chain_id = 5_u64;
    let (config, contract_config) = setup_config(chain_id, "0x223903804Ee95e264F74C88B4F8583429524593c").await;
    let (_, private_key) = generate_private_key();
    let merkle_tree = generate_merkle_tree(6_usize);
    let tree_root = biguint_to_u256(&merkle_tree.root());
    let tx_hash = TxHash::from_str("0xa35c998eaf5df995dba638efc114a8f58353784d08a60467fba6ed1e8f0e64a8").unwrap();
    let transact_options = TransactTestOptions::builder()
        .num_inputs(1_usize)
        .num_outputs(0_usize)
        .root_hash(HashSet::from([tree_root]))
        .public_amount(HashSet::from([(U256::from_dec_str("30000000000000000").unwrap())]))
        .public_recipient(ethers_address_from_string("0x87813A8E81729C0100ce2568b6283772cb31bdb8").unwrap())
        .tx_hash(tx_hash)
        .build();
    let commitment_pool_contracts_options = CommitmentPoolTestOptions::builder()
        .chain_id(chain_id)
        .spent_nullifiers(HashMap::from([(U256::from(0_u64), false)]))
        .known_root(HashMap::from([(tree_root, true)]))
        .transact_options(transact_options)
        .build();
    let commitment_pool_contracts =
        setup_commitment_pool_contracts(commitment_pool_contracts_options, &contract_config);
    let static_cache = setup_static_cache(&contract_config, &mystiko_types::CircuitType::Transaction1x0);
    let prover = setup_prover(false);
    let mut mock_data_packer_client = MockDataPackerClient::new();
    mock_data_packer_client
        .expect_query_merkle_tree()
        .times(1)
        .returning(move |_, _| {
            let proto_merkle_tree = ProtoMerkleTree::builder()
                .loaded_block_number(200010000_u64)
                .root_hash(u256_to_bytes(&tree_root))
                .commitment_hashes(merkle_tree.elements().iter().map(biguint_to_bytes).collect::<Vec<_>>())
                .build();
            Ok(Some(proto_merkle_tree))
        });
    let test_options: SendTestOptions = SendTestOptions::builder()
        .chain_id(chain_id)
        .config(config)
        .contract_config(contract_config.clone())
        .commitment_pool_contracts(commitment_pool_contracts)
        .prover(prover)
        .static_cache(static_cache)
        .data_packer_client(mock_data_packer_client)
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
        .build();
    let spend = context.handler.create(create_options).await.unwrap();
    let send_options = SendSpendOptions::builder()
        .spend_id(spend.id.clone())
        .wallet_password("P@ssw0rd".to_string())
        .private_key(private_key)
        .build();
    let result = context.handler.send(send_options).await;
    assert_eq!(
        result.unwrap_err().to_string(),
        "failed to verify the generated zk proof"
    );
    let spend = context.handler.find_by_id(spend.id).await.unwrap().unwrap();
    assert_eq!(spend.status, SpendStatus::Failed as i32);
    assert_eq!(spend.error_message.unwrap(), "failed to verify the generated zk proof");
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SendTestOptions {
    chain_id: u64,
    config: MystikoConfig,
    contract_config: PoolContractConfig,
    #[builder(default)]
    assets: Option<MockPublicAssets>,
    #[builder(default)]
    relayer_client: MockRelayerClient,
    #[builder(default)]
    data_packer_client: MockDataPackerClient,
    #[builder(default)]
    commitment_pool_contracts: Option<MockCommitmentPoolContracts>,
    #[builder(default)]
    providers: HashMap<u64, MockProvider>,
    #[builder(default)]
    prover: Option<MockZKProver>,
    #[builder(default)]
    static_cache: Option<MockStaticCache>,
    #[builder(default)]
    transactions: Option<MockTransactions>,
}

#[derive(Debug, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct TransactTestOptions {
    num_inputs: usize,
    num_outputs: usize,
    root_hash: HashSet<U256>,
    public_amount: HashSet<U256>,
    relayer_fee_amount: U256,
    out_rollup_fees: Vec<U256>,
    public_recipient: Address,
    relayer_address: Address,
    tx_hash: TxHash,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CommitmentPoolTestOptions {
    chain_id: u64,
    #[builder(default)]
    spent_nullifiers: HashMap<U256, bool>,
    #[builder(default)]
    known_root: HashMap<U256, bool>,
    #[builder(default)]
    transact_options: TransactTestOptions,
    #[builder(default)]
    query_timeout_ms: Option<u64>,
    #[builder(default)]
    tx_send_timeout_ms: Option<u64>,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TransactionsTestOptions<'a> {
    chain_id: u64,
    config: &'a MystikoConfig,
    tx_hash: TxHash,
    block_number: u64,
    #[builder(default)]
    tx_wait_timeout_ms: Option<u64>,
    #[builder(default)]
    tx_wait_interval_ms: Option<u64>,
    #[builder(default)]
    tx_confirmations: Option<u64>,
}

pub struct SendTestContext<R = MockRelayerClient> {
    db: Arc<DatabaseType>,
    pub handler: SpendsType<R>,
    pub(crate) account: Account,
    chain_id: u64,
    contract_config: PoolContractConfig,
}

impl SendTestContext {
    pub(crate) async fn new(options: SendTestOptions) -> Self {
        let SendTestOptions {
            config,
            chain_id,
            contract_config,
            assets,
            relayer_client,
            data_packer_client,
            commitment_pool_contracts,
            prover,
            static_cache,
            transactions,
            providers,
        } = options;
        let mock_options = MockOptions::<MockRelayerClient, MockDataPackerClient>::builder()
            .config(config)
            .assets(assets.unwrap_or_default())
            .commitment_pool_contracts(commitment_pool_contracts.unwrap_or_default())
            .relayer_client(relayer_client)
            .data_packer_client(data_packer_client)
            .prover(prover.unwrap_or_default())
            .providers(providers)
            .static_cache(static_cache.unwrap_or_default())
            .transactions(transactions.unwrap_or_default())
            .build();
        let (_, db, handler) = setup::<MockRelayerClient, MockDataPackerClient>(mock_options).await;
        let (_, account) = create_wallet(db.clone()).await;
        Self {
            db,
            handler,
            account,
            chain_id,
            contract_config,
        }
    }

    pub async fn generate_commitments(&self, amounts: &[f64]) -> Vec<Document<Commitment>> {
        let mut options1 = amounts
            .iter()
            .enumerate()
            .flat_map(|(index, amount)| {
                vec![
                    CommitmentOptions::builder()
                        .status(CommitmentStatus::Included)
                        .shielded_address(self.account.shielded_address.clone())
                        .amount(
                            number_to_biguint_decimal(*amount, Some(self.contract_config.asset_decimals())).unwrap(),
                        )
                        .leaf_index(2 * (index as u64))
                        .build(),
                    CommitmentOptions::builder()
                        .status(CommitmentStatus::Included)
                        .leaf_index(2 * (index as u64) + 1)
                        .build(),
                ]
            })
            .collect::<Vec<_>>();
        let options2 = amounts
            .iter()
            .map(|_| CommitmentOptions::builder().status(CommitmentStatus::Queued).build())
            .collect::<Vec<_>>();
        options1.extend(options2);
        generate_commitments(self.db.clone(), self.chain_id, &self.contract_config, &options1).await
    }

    pub(crate) async fn check_commitments(
        &self,
        spend: &Spend,
        expected_block_number: u64,
        expected_out_amounts: &[f64],
    ) {
        let input_commitment_hashes = spend
            .input_commitments
            .iter()
            .map(|c| BigUint::from_str(c).unwrap())
            .collect::<Vec<_>>();
        let output_commitment_hashes = spend
            .output_commitments
            .iter()
            .map(|c| BigUint::from_str(c).unwrap())
            .collect::<Vec<_>>();
        let expected_out_amounts = expected_out_amounts
            .iter()
            .map(|out_amount| {
                number_to_biguint_decimal(*out_amount, Some(self.contract_config.asset_decimals())).unwrap()
            })
            .collect::<Vec<_>>();
        let input_commitments_filter = Condition::and(vec![
            SubFilter::equal(CommitmentColumn::ChainId, self.chain_id),
            SubFilter::equal(
                CommitmentColumn::ContractAddress,
                self.contract_config.address().to_string(),
            ),
            SubFilter::in_list(CommitmentColumn::CommitmentHash, input_commitment_hashes.clone()),
        ]);
        let input_commitments = self.db.commitments.find(input_commitments_filter).await.unwrap();
        let mut output_commitments = if !output_commitment_hashes.is_empty() {
            let output_commitments_filter = Condition::and(vec![
                SubFilter::equal(CommitmentColumn::ChainId, self.chain_id),
                SubFilter::equal(
                    CommitmentColumn::ContractAddress,
                    self.contract_config.address().to_string(),
                ),
                SubFilter::in_list(CommitmentColumn::CommitmentHash, output_commitment_hashes.clone()),
            ]);
            self.db.commitments.find(output_commitments_filter).await.unwrap()
        } else {
            vec![]
        };
        output_commitments.sort_by_key(|c| c.data.amount.clone().unwrap_or_default());
        assert_eq!(input_commitments.len(), input_commitment_hashes.len());
        for input_commitment in input_commitments.into_iter() {
            assert!(input_commitment.data.spent);
        }
        assert_eq!(output_commitments.len(), output_commitment_hashes.len());
        for (index, output_commitment) in output_commitments.into_iter().enumerate() {
            assert_eq!(output_commitment.data.chain_id, self.chain_id);
            assert_eq!(output_commitment.data.contract_address, self.contract_config.address());
            assert_eq!(output_commitment.data.bridge_type, spend.bridge_type);
            assert_eq!(output_commitment.data.asset_symbol, self.contract_config.asset_symbol());
            assert_eq!(
                output_commitment.data.asset_decimals,
                self.contract_config.asset_decimals()
            );
            assert_eq!(
                output_commitment.data.asset_address,
                self.contract_config.asset_address().map(|s| s.to_string())
            );
            assert_eq!(output_commitment.data.status, CommitmentStatus::Queued as i32);
            assert!(!output_commitment.data.spent);
            assert_eq!(output_commitment.data.block_number, expected_block_number);
            assert!(output_commitment.data.src_chain_block_number.is_none());
            assert!(output_commitment.data.included_block_number.is_none());
            assert_eq!(output_commitment.data.amount.unwrap(), expected_out_amounts[index]);
            assert_eq!(
                output_commitment.data.rollup_fee_amount,
                spend.rollup_fee_decimal_amount_as_biguint().unwrap()
            );
            assert_eq!(output_commitment.data.queued_transaction_hash, spend.transaction_hash);
            assert!(output_commitment.data.included_transaction_hash.is_none());
            assert!(output_commitment.data.src_chain_transaction_hash.is_none());
        }
    }
}

pub async fn setup_config(chain_id: u64, contract_address: &str) -> (MystikoConfig, PoolContractConfig) {
    let config = MystikoConfig::from_json_file("tests/files/mystiko/config.json")
        .await
        .unwrap();
    let contract_config = config
        .find_pool_contract_by_address(chain_id, contract_address)
        .unwrap()
        .clone();
    (config, contract_config)
}

pub fn setup_commitment_pool_contracts(
    options: CommitmentPoolTestOptions,
    contract_config: &PoolContractConfig,
) -> MockCommitmentPoolContracts {
    let mut commitment_pool_contracts = MockCommitmentPoolContracts::new();
    let CommitmentPoolTestOptions {
        chain_id,
        spent_nullifiers,
        known_root,
        transact_options,
        query_timeout_ms,
        tx_send_timeout_ms,
    } = options;
    let spend_nullifiers_clone = spent_nullifiers.clone();
    let known_root_clone = known_root.clone();
    let contract_address = ethers_address_from_string(contract_config.address()).unwrap();
    commitment_pool_contracts
        .expect_is_spent_nullifier()
        .withf(move |options| {
            options.chain_id == chain_id
                && options.contract_address == contract_address
                && spent_nullifiers.contains_key(&options.nullifier)
        })
        .returning(move |options| {
            if options.timeout_ms == Some(102_u64)
                || (options.timeout_ms == Some(202_u64) && options.nullifier == U256::from(2_u32))
                || options.timeout_ms == Some(302_u64)
                || options.timeout_ms == Some(602_u64)
            {
                Ok(true)
            } else {
                Ok(spend_nullifiers_clone
                    .get(&options.nullifier)
                    .cloned()
                    .unwrap_or_default())
            }
        });
    commitment_pool_contracts
        .expect_is_historic_commitment()
        .returning(move |options| {
            if options.timeout_ms == Some(302_u64) || options.timeout_ms == Some(602_u64) {
                Ok(true)
            } else {
                Ok(false)
            }
        });
    commitment_pool_contracts
        .expect_is_known_root()
        .withf(move |options| {
            options.chain_id == chain_id
                && options.contract_address == contract_address
                && options.timeout_ms == query_timeout_ms
                && known_root.contains_key(&options.root_hash)
        })
        .returning(move |options| Ok(known_root_clone.get(&options.root_hash).cloned().unwrap_or_default()));
    commitment_pool_contracts
        .expect_auditor_public_keys()
        .withf(move |options| {
            options.chain_id == chain_id
                && options.contract_address == contract_address
                && options.timeout_ms == query_timeout_ms
        })
        .returning(|_| {
            Ok(vec![
                U256::from(0_u64),
                U256::from(0_u64),
                U256::from(0_u64),
                U256::from(0_u64),
                U256::from(0_u64),
            ])
        });
    let min_rollup_fee = biguint_to_u256(&contract_config.min_rollup_fee().unwrap());
    commitment_pool_contracts
        .expect_min_rollup_fee()
        .withf(move |options| {
            options.chain_id == chain_id
                && options.contract_address == contract_address
                && options.timeout_ms == query_timeout_ms
        })
        .returning(move |_| Ok(min_rollup_fee));
    let TransactTestOptions {
        num_inputs,
        num_outputs,
        root_hash,
        public_amount,
        relayer_fee_amount,
        out_rollup_fees,
        public_recipient,
        relayer_address,
        tx_hash,
    } = transact_options;
    commitment_pool_contracts
        .expect_transact::<TypedTransaction, PrivateKeySigner<MockProviders>>()
        .withf(move |options| {
            options.chain_id == chain_id
                && options.contract_address == contract_address
                && options.timeout_ms == tx_send_timeout_ms
                && root_hash.contains(&options.request.root_hash)
                && public_amount.contains(&options.request.public_amount)
                && options.request.relayer_fee_amount == relayer_fee_amount
                && options.request.out_rollup_fees == out_rollup_fees
                && options.request.public_recipient == public_recipient
                && options.request.relayer_address == relayer_address
                && options.request.serial_numbers.len() == num_inputs
                && options.request.sig_hashes.len() == num_inputs
                && options.request.out_commitments.len() == num_outputs
                && options.request.out_encrypted_notes.len() == num_outputs
                && options.request.encrypted_auditor_notes.len() == num_inputs * 5
        })
        .returning(move |_| Ok(tx_hash));
    commitment_pool_contracts
}

pub fn setup_assets(
    chain_id: u64,
    contract_config: &PoolContractConfig,
    balance: U256,
    query_timeout_ms: Option<u64>,
) -> MockPublicAssets {
    let mut assets = MockPublicAssets::new();
    if contract_config.bridge_type() != &mystiko_types::BridgeType::Loop {
        let contract_address = ethers_address_from_string(contract_config.address()).unwrap();
        if let Some(asset_address) = contract_config.asset_address() {
            let asset_address = ethers_address_from_string(asset_address).unwrap();
            assets
                .expect_erc20_balance_of()
                .withf(move |options| {
                    options.chain_id == chain_id
                        && options.asset_address == asset_address
                        && options.owner == contract_address
                        && options.timeout_ms == query_timeout_ms
                })
                .returning(move |_| Ok(balance));
        } else {
            assets
                .expect_balance_of()
                .withf(move |options| options.chain_id == chain_id && options.owner == contract_address)
                .returning(move |_| Ok(balance));
        }
    }
    assets
}

pub fn setup_static_cache(
    contract_config: &PoolContractConfig,
    circuit_type: &mystiko_types::CircuitType,
) -> MockStaticCache {
    let mut static_cache = MockStaticCache::new();
    let circuit_config = contract_config.circuit_by_type(circuit_type).unwrap();
    let program_urls = circuit_config.program_file().clone();
    let program_urls_clone = program_urls.clone();
    let proving_key_urls = circuit_config.proving_key_file().clone();
    let proving_key_urls_clone = proving_key_urls.clone();
    let verifying_key_urls = circuit_config.verifying_key_file().clone();
    let verifying_key_urls_clone = verifying_key_urls.clone();
    let abi_file_urls = circuit_config.abi_file().clone();
    let abi_file_urls_clone = abi_file_urls.clone();
    static_cache
        .expect_get_failover()
        .withf(move |urls, _| {
            urls == program_urls || urls == proving_key_urls || urls == verifying_key_urls || urls == abi_file_urls
        })
        .returning(move |urls, _| {
            if urls == program_urls_clone {
                Ok("program file".as_bytes().to_vec())
            } else if urls == proving_key_urls_clone {
                Ok("proving key file".as_bytes().to_vec())
            } else if urls == verifying_key_urls_clone {
                Ok("verifying key file".as_bytes().to_vec())
            } else if urls == abi_file_urls_clone {
                Ok("abi file".as_bytes().to_vec())
            } else {
                Err(anyhow::anyhow!("Unexpected urls: {:?}", urls))
            }
        });
    static_cache
}

pub fn setup_prover(verified: bool) -> MockZKProver {
    let mut prover = MockZKProver::new();
    prover
        .expect_prove()
        .withf(|options| {
            String::from_utf8_lossy(options.program) == "program file"
                && String::from_utf8_lossy(options.proving_key) == "proving key file"
                && String::from_utf8_lossy(options.abi_spec) == "abi file"
        })
        .returning(|_| Ok(Default::default()));
    prover.expect_verify().returning(move |_| Ok(verified));
    prover
}

pub fn setup_transactions(options: TransactionsTestOptions) -> MockTransactions {
    let TransactionsTestOptions {
        config,
        chain_id,
        tx_hash,
        block_number,
        tx_wait_timeout_ms,
        tx_wait_interval_ms,
        tx_confirmations,
    } = options;
    let chain_config = config.find_chain(chain_id).unwrap();
    let expected_tx_type = chain_config.transaction_type().clone();
    let mut transactions = MockTransactions::new();
    transactions
        .expect_create()
        .withf(move |_, tx_type| tx_type == &expected_tx_type)
        .returning(|_, tx_type| match tx_type {
            TransactionType::Legacy => Ok(TypedTransaction::Legacy(TransactionRequest::new())),
            TransactionType::Eip1559 => Ok(TypedTransaction::Eip1559(Eip1559TransactionRequest::new())),
            TransactionType::Eip2930 => Ok(TypedTransaction::Eip2930(Eip2930TransactionRequest::new(
                TransactionRequest::new(),
                AccessList(vec![]),
            ))),
        });
    transactions
        .expect_wait()
        .withf(move |options| {
            options.chain_id == chain_id
                && options.tx_hash == tx_hash
                && options.confirmations == tx_confirmations
                && options.timeout_ms == tx_wait_timeout_ms
                && options.interval_ms == tx_wait_interval_ms
        })
        .returning(move |_| {
            Ok(TransactionReceipt {
                transaction_hash: tx_hash,
                block_number: Some(U64::from(block_number)),
                ..Default::default()
            })
        });
    transactions
}

pub fn generate_merkle_tree(num_leaves: usize) -> MerkleTree {
    let mut leaves = Vec::with_capacity(num_leaves);
    for i in 0..num_leaves {
        leaves.push(BigUint::from(i as u64));
    }
    MerkleTree::new(Some(leaves), None, None).unwrap()
}
