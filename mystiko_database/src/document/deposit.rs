#![forbid(unsafe_code)]

use mystiko_storage::column::{IndexColumns, UniqueColumns};
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
    vec![UniqueColumns::builder()
        .column_names(vec![
            DepositColumn::ChainId.to_string(),
            DepositColumn::ContractAddress.to_string(),
            DepositColumn::CommitmentHash.to_string(),
        ])
        .build()]
}

fn indexes() -> Vec<IndexColumns> {
    vec![
        IndexColumns::builder()
            .column_names(vec![DepositColumn::ChainId.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![DepositColumn::ContractAddress.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![DepositColumn::CommitmentHash.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![DepositColumn::DstChainId.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![DepositColumn::DstChainContractAddress.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![DepositColumn::ShieldedRecipientAddress.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![DepositColumn::AssetApproveTransactionHash.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![DepositColumn::TransactionHash.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![DepositColumn::RelayTransactionHash.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![DepositColumn::RollupTransactionHash.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![DepositColumn::WalletId.to_string()])
            .build(),
    ]
}
