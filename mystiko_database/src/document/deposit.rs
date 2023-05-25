#![forbid(unsafe_code)]

use mystiko_storage::column::{IndexColumns, UniqueColumns};
use mystiko_storage::document::DocumentData;
use mystiko_storage_macros::CollectionBuilder;
use mystiko_types::{BridgeType, DepositStatus};
use num_bigint::BigInt;

#[derive(CollectionBuilder, Clone, PartialEq, Debug)]
#[collection(uniques = uniques(), indexes = indexes())]
pub struct Deposit {
    pub chain_id: u64,
    #[column(length_limit = 64)]
    pub contract_address: String,
    #[column(length_limit = 64)]
    pub pool_address: String,
    #[column(length_limit = 128)]
    pub commitment_hash: BigInt,
    #[column(length_limit = 128)]
    pub hash_k: BigInt,
    #[column(length_limit = 128)]
    pub random_s: BigInt,
    pub encrypted_note: String,
    #[column(length_limit = 16)]
    pub asset_symbol: String,
    pub asset_decimals: u32,
    #[column(length_limit = 64)]
    pub asset_address: Option<String>,
    #[column(length_limit = 32)]
    pub bridge_type: BridgeType,
    #[column(length_limit = 128)]
    pub amount: BigInt,
    #[column(length_limit = 128)]
    pub rollup_fee_amount: BigInt,
    #[column(length_limit = 128)]
    pub bridge_fee_amount: BigInt,
    #[column(length_limit = 64)]
    pub bridge_fee_asset_address: Option<String>,
    #[column(length_limit = 128)]
    pub executor_fee_amount: BigInt,
    #[column(length_limit = 64)]
    pub executor_fee_asset_address: Option<String>,
    #[column(length_limit = 128)]
    pub service_fee_amount: BigInt,
    #[column(length_limit = 128)]
    pub shielded_recipient_address: String,
    #[column(length_limit = 32)]
    pub status: DepositStatus,
    pub error_message: Option<String>,
    #[column(length_limit = 64)]
    pub wallet_id: String,
    pub dst_chain_id: u64,
    #[column(length_limit = 64)]
    pub dst_chain_contract_address: String,
    #[column(length_limit = 64)]
    pub dst_pool_address: String,
    #[column(length_limit = 128)]
    pub asset_approve_transaction_hash: Option<String>,
    #[column(length_limit = 128)]
    pub transaction_hash: Option<String>,
    #[column(length_limit = 128)]
    pub relay_transaction_hash: Option<String>,
    #[column(length_limit = 128)]
    pub rollup_transaction_hash: Option<String>,
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
        vec![DepositColumn::AssetApproveTransactionHash].into(),
        vec![DepositColumn::TransactionHash].into(),
        vec![DepositColumn::RelayTransactionHash].into(),
        vec![DepositColumn::RollupTransactionHash].into(),
        vec![DepositColumn::WalletId].into(),
    ]
}
