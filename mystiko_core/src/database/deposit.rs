use mystiko_protos::core::document::v1::Deposit as ProtoDeposit;
use mystiko_storage::{Document, DocumentData, IndexColumns, UniqueColumns};
use mystiko_storage_macros::CollectionBuilder;
use serde::{Deserialize, Serialize};

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
    pub commitment_hash: String,
    #[column(length_limit = 128)]
    pub hash_k: String,
    #[column(length_limit = 128)]
    pub random_s: String,
    pub encrypted_note: String,
    #[column(length_limit = 16)]
    pub asset_symbol: String,
    #[column(length_limit = 64)]
    pub asset_address: Option<String>,
    pub bridge_type: i32,
    pub amount: f64,
    pub rollup_fee_amount: f64,
    pub bridge_fee_amount: Option<f64>,
    #[column(length_limit = 64)]
    pub bridge_fee_asset_address: Option<String>,
    #[column(length_limit = 16)]
    pub bridge_fee_asset_symbol: Option<String>,
    pub executor_fee_amount: Option<f64>,
    #[column(length_limit = 64)]
    pub executor_fee_asset_address: Option<String>,
    #[column(length_limit = 16)]
    pub executor_fee_asset_symbol: Option<String>,
    #[column(length_limit = 128)]
    pub shielded_recipient_address: String,
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
        vec![DepositColumn::ShieldedRecipientAddress].into(),
        vec![DepositColumn::SrcChainTransactionHash].into(),
        vec![DepositColumn::QueuedTransactionHash].into(),
        vec![DepositColumn::IncludedTransactionHash].into(),
        vec![DepositColumn::WalletId].into(),
    ]
}

impl Deposit {
    pub fn document_from_proto(proto: ProtoDeposit) -> Document<Self> {
        Document::new(
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
                commitment_hash: proto.commitment_hash,
                hash_k: proto.hash_k,
                random_s: proto.random_s,
                encrypted_note: proto.encrypted_note,
                asset_symbol: proto.asset_symbol,
                asset_address: proto.asset_address,
                bridge_type: proto.bridge_type,
                amount: proto.amount,
                rollup_fee_amount: proto.rollup_fee_amount,
                bridge_fee_amount: proto.bridge_fee_amount,
                bridge_fee_asset_address: proto.bridge_fee_asset_address,
                bridge_fee_asset_symbol: proto.bridge_fee_asset_symbol,
                executor_fee_amount: proto.executor_fee_amount,
                executor_fee_asset_address: proto.executor_fee_asset_address,
                executor_fee_asset_symbol: proto.executor_fee_asset_symbol,
                shielded_recipient_address: proto.shielded_recipient_address,
                status: proto.status,
                error_message: proto.error_message,
                wallet_id: proto.wallet_id,
                asset_approve_transaction_hash: (!proto.asset_approve_transaction_hash.is_empty())
                    .then_some(proto.asset_approve_transaction_hash),
                src_chain_transaction_hash: proto.src_chain_transaction_hash,
                queued_transaction_hash: proto.queued_transaction_hash,
                included_transaction_hash: proto.included_transaction_hash,
            },
        )
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
            .commitment_hash(deposit.data.commitment_hash)
            .hash_k(deposit.data.hash_k)
            .random_s(deposit.data.random_s)
            .encrypted_note(deposit.data.encrypted_note)
            .asset_symbol(deposit.data.asset_symbol)
            .asset_address(deposit.data.asset_address)
            .bridge_type(deposit.data.bridge_type)
            .amount(deposit.data.amount)
            .rollup_fee_amount(deposit.data.rollup_fee_amount)
            .bridge_fee_amount(deposit.data.bridge_fee_amount)
            .bridge_fee_asset_address(deposit.data.bridge_fee_asset_address)
            .bridge_fee_asset_symbol(deposit.data.bridge_fee_asset_symbol)
            .executor_fee_amount(deposit.data.executor_fee_amount)
            .executor_fee_asset_address(deposit.data.executor_fee_asset_address)
            .executor_fee_asset_symbol(deposit.data.executor_fee_asset_symbol)
            .shielded_recipient_address(deposit.data.shielded_recipient_address)
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
