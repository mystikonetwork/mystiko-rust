use mystiko_protos::core::document::v1::Deposit as ProtoDeposit;
use mystiko_storage::{Document, DocumentData, IndexColumns, UniqueColumns};
use mystiko_storage_macros::CollectionBuilder;
use num_bigint::{BigUint, ParseBigIntError};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(CollectionBuilder, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[collection(uniques = uniques(), indexes = indexes())]
pub struct Deposit {
    pub chain_id: u64,
    #[column(length_limit = 64)]
    pub contract_address: String,
    #[column(length_limit = 64)]
    pub pool_address: String,
    pub dst_chain_id: u64,
    #[column(length_limit = 64)]
    pub dst_chain_contract_address: String,
    #[column(length_limit = 64)]
    pub dst_pool_address: String,
    #[column(length_limit = 128)]
    pub commitment_hash: BigUint,
    #[column(length_limit = 128)]
    pub hash_k: BigUint,
    #[column(length_limit = 128)]
    pub random_s: BigUint,
    pub encrypted_note: String,
    #[column(length_limit = 16)]
    pub asset_symbol: String,
    pub asset_decimals: u32,
    #[column(length_limit = 64)]
    pub asset_address: Option<String>,
    pub bridge_type: i32,
    pub amount: f64,
    #[column(length_limit = 128)]
    pub decimal_amount: BigUint,
    pub rollup_fee_amount: f64,
    #[column(length_limit = 128)]
    pub rollup_fee_decimal_amount: BigUint,
    pub bridge_fee_amount: Option<f64>,
    #[column(length_limit = 128)]
    pub bridge_fee_decimal_amount: Option<BigUint>,
    #[column(length_limit = 64)]
    pub bridge_fee_asset_address: Option<String>,
    #[column(length_limit = 16)]
    pub bridge_fee_asset_symbol: Option<String>,
    pub bridge_fee_asset_decimals: Option<u32>,
    pub executor_fee_amount: Option<f64>,
    #[column(length_limit = 128)]
    pub executor_fee_decimal_amount: Option<BigUint>,
    #[column(length_limit = 64)]
    pub executor_fee_asset_address: Option<String>,
    #[column(length_limit = 16)]
    pub executor_fee_asset_symbol: Option<String>,
    pub executor_fee_asset_decimals: Option<u32>,
    #[column(length_limit = 128)]
    pub shielded_address: String,
    pub status: i32,
    pub error_message: Option<String>,
    #[column(length_limit = 64)]
    pub wallet_id: String,
    pub asset_approve_transaction_hash: Option<Vec<String>>,
    #[column(length_limit = 128)]
    pub src_chain_transaction_hash: Option<String>,
    #[column(length_limit = 128)]
    pub queued_transaction_hash: Option<String>,
    #[column(length_limit = 128)]
    pub included_transaction_hash: Option<String>,
}

fn uniques() -> Vec<UniqueColumns> {
    vec![vec![
        DepositColumn::ChainId,
        DepositColumn::ContractAddress,
        DepositColumn::CommitmentHash,
    ]
    .into()]
}

fn indexes() -> Vec<IndexColumns> {
    vec![
        vec![DepositColumn::ChainId].into(),
        vec![DepositColumn::ContractAddress].into(),
        vec![DepositColumn::CommitmentHash].into(),
        vec![DepositColumn::DstChainId].into(),
        vec![DepositColumn::DstChainContractAddress].into(),
        vec![DepositColumn::ShieldedAddress].into(),
        vec![DepositColumn::SrcChainTransactionHash].into(),
        vec![DepositColumn::QueuedTransactionHash].into(),
        vec![DepositColumn::IncludedTransactionHash].into(),
        vec![DepositColumn::WalletId].into(),
    ]
}

impl Deposit {
    pub fn document_from_proto(proto: ProtoDeposit) -> Result<Document<Self>, ParseBigIntError> {
        let decimal_amount = proto.decimal_amount_as_biguint()?;
        let rollup_fee_decimal_amount = proto.rollup_fee_decimal_amount_as_biguint()?;
        let bridge_fee_decimal_amount = proto.bridge_fee_decimal_amount_as_biguint()?;
        let executor_fee_decimal_amount = proto.executor_fee_decimal_amount_as_biguint()?;
        Ok(Document::new(
            proto.id,
            proto.created_at,
            proto.updated_at,
            Self {
                chain_id: proto.chain_id,
                contract_address: proto.contract_address,
                pool_address: proto.pool_address,
                dst_chain_id: proto.dst_chain_id,
                dst_chain_contract_address: proto.dst_chain_contract_address,
                dst_pool_address: proto.dst_pool_address,
                commitment_hash: BigUint::from_str(&proto.commitment_hash)?,
                hash_k: BigUint::from_str(&proto.hash_k)?,
                random_s: BigUint::from_str(&proto.random_s)?,
                encrypted_note: proto.encrypted_note,
                asset_symbol: proto.asset_symbol,
                asset_decimals: proto.asset_decimals,
                asset_address: proto.asset_address,
                bridge_type: proto.bridge_type,
                amount: proto.amount,
                decimal_amount,
                rollup_fee_amount: proto.rollup_fee_amount,
                rollup_fee_decimal_amount,
                bridge_fee_amount: proto.bridge_fee_amount,
                bridge_fee_decimal_amount,
                bridge_fee_asset_address: proto.bridge_fee_asset_address,
                bridge_fee_asset_symbol: proto.bridge_fee_asset_symbol,
                bridge_fee_asset_decimals: proto.bridge_fee_asset_decimals,
                executor_fee_amount: proto.executor_fee_amount,
                executor_fee_decimal_amount,
                executor_fee_asset_address: proto.executor_fee_asset_address,
                executor_fee_asset_symbol: proto.executor_fee_asset_symbol,
                executor_fee_asset_decimals: proto.executor_fee_asset_decimals,
                shielded_address: proto.shielded_address,
                status: proto.status,
                error_message: proto.error_message,
                wallet_id: proto.wallet_id,
                asset_approve_transaction_hash: (!proto.asset_approve_transaction_hash.is_empty())
                    .then_some(proto.asset_approve_transaction_hash),
                src_chain_transaction_hash: proto.src_chain_transaction_hash,
                queued_transaction_hash: proto.queued_transaction_hash,
                included_transaction_hash: proto.included_transaction_hash,
            },
        ))
    }

    pub fn document_into_proto(deposit: Document<Self>) -> ProtoDeposit {
        ProtoDeposit::builder()
            .id(deposit.id)
            .created_at(deposit.created_at)
            .updated_at(deposit.updated_at)
            .chain_id(deposit.data.chain_id)
            .contract_address(deposit.data.contract_address)
            .pool_address(deposit.data.pool_address)
            .dst_chain_id(deposit.data.dst_chain_id)
            .dst_chain_contract_address(deposit.data.dst_chain_contract_address)
            .dst_pool_address(deposit.data.dst_pool_address)
            .commitment_hash(deposit.data.commitment_hash.to_string())
            .hash_k(deposit.data.hash_k.to_string())
            .random_s(deposit.data.random_s.to_string())
            .encrypted_note(deposit.data.encrypted_note)
            .asset_symbol(deposit.data.asset_symbol)
            .asset_decimals(deposit.data.asset_decimals)
            .asset_address(deposit.data.asset_address)
            .bridge_type(deposit.data.bridge_type)
            .amount(deposit.data.amount)
            .decimal_amount(deposit.data.decimal_amount.to_string())
            .rollup_fee_amount(deposit.data.rollup_fee_amount)
            .rollup_fee_decimal_amount(deposit.data.rollup_fee_decimal_amount.to_string())
            .bridge_fee_amount(deposit.data.bridge_fee_amount)
            .bridge_fee_decimal_amount(deposit.data.bridge_fee_decimal_amount.map(|n| n.to_string()))
            .bridge_fee_asset_address(deposit.data.bridge_fee_asset_address)
            .bridge_fee_asset_symbol(deposit.data.bridge_fee_asset_symbol)
            .bridge_fee_asset_decimals(deposit.data.bridge_fee_asset_decimals)
            .executor_fee_amount(deposit.data.executor_fee_amount)
            .executor_fee_decimal_amount(deposit.data.executor_fee_decimal_amount.map(|n| n.to_string()))
            .executor_fee_asset_address(deposit.data.executor_fee_asset_address)
            .executor_fee_asset_symbol(deposit.data.executor_fee_asset_symbol)
            .executor_fee_asset_decimals(deposit.data.executor_fee_asset_decimals)
            .shielded_address(deposit.data.shielded_address)
            .status(deposit.data.status)
            .error_message(deposit.data.error_message)
            .wallet_id(deposit.data.wallet_id)
            .asset_approve_transaction_hash(deposit.data.asset_approve_transaction_hash.unwrap_or_default())
            .src_chain_transaction_hash(deposit.data.src_chain_transaction_hash)
            .queued_transaction_hash(deposit.data.queued_transaction_hash)
            .included_transaction_hash(deposit.data.included_transaction_hash)
            .build()
    }
}
