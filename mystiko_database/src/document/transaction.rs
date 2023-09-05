#![forbid(unsafe_code)]

use mystiko_storage::{DocumentData, IndexColumns};
use mystiko_storage_macros::CollectionBuilder;
use mystiko_types::{TransactionStatus, TransactionType};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

#[derive(CollectionBuilder, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[collection(indexes = indexes())]
pub struct Transaction {
    pub chain_id: u64,
    #[column(length_limit = 64)]
    pub contract_address: String,
    #[column(length_limit = 16)]
    pub asset_symbol: String,
    pub asset_decimals: u32,
    #[column(length_limit = 64)]
    pub asset_address: Option<String>,
    pub proof: Option<String>,
    #[column(length_limit = 128)]
    pub root_hash: BigUint,
    pub input_commitments: Vec<BigUint>,
    pub output_commitments: Option<Vec<BigUint>>,
    pub nullifiers: Option<Vec<BigUint>>,
    #[column(length_limit = 255)]
    pub signature_public_key: Option<String>,
    pub signature_public_key_hashes: Option<Vec<String>>,
    #[column(length_limit = 128)]
    pub amount: BigUint,
    #[column(length_limit = 128)]
    pub public_amount: BigUint,
    #[column(length_limit = 128)]
    pub rollup_fee_amount: BigUint,
    #[column(length_limit = 128)]
    pub gas_relayer_fee_amount: BigUint,
    #[column(length_limit = 128)]
    pub shielded_address: Option<String>,
    #[column(length_limit = 64)]
    pub public_address: Option<String>,
    #[column(length_limit = 64)]
    pub gas_relayer_address: Option<String>,
    #[column(length_limit = 255)]
    pub signature: Option<String>,
    #[column(length_limit = 128)]
    pub random_auditing_public_key: Option<BigUint>,
    pub encrypted_auditor_notes: Option<Vec<String>>,
    #[column(length_limit = 16)]
    pub transaction_type: TransactionType,
    #[column(length_limit = 32)]
    pub status: TransactionStatus,
    pub error_message: Option<String>,
    #[column(length_limit = 128)]
    pub transaction_hash: Option<String>,
    #[column(length_limit = 64)]
    pub wallet_id: String,
}

fn indexes() -> Vec<IndexColumns> {
    vec![
        IndexColumns::builder()
            .column_names(vec![TransactionColumn::ChainId.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![TransactionColumn::ContractAddress.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![TransactionColumn::SignaturePublicKey.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![TransactionColumn::RootHash.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![TransactionColumn::ShieldedAddress.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![TransactionColumn::PublicAddress.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![TransactionColumn::TransactionHash.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![TransactionColumn::TransactionType.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![TransactionColumn::WalletId.to_string()])
            .build(),
    ]
}
