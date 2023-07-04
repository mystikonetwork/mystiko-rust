use mystiko_relayer_types::TransactStatus;
use mystiko_storage::column::IndexColumns;
use mystiko_storage::document::DocumentData;
use mystiko_storage_macros::CollectionBuilder;
use mystiko_types::{BridgeType, CircuitType, TransactionType};
use num_bigint::BigInt;

#[derive(CollectionBuilder, Clone, Debug, PartialEq)]
#[collection(indexes = indexes())]
pub struct Transaction {
    pub chain_id: u64,
    #[column(length_limit = 32)]
    pub transaction_type: TransactionType,
    #[column(length_limit = 64)]
    pub bridge_type: BridgeType,
    #[column(length_limit = 64)]
    pub status: TransactStatus,
    #[column(length_limit = 64)]
    pub pool_address: String,
    #[column(length_limit = 16)]
    pub asset_symbol: String,
    pub asset_decimals: u32,
    #[column(length_limit = 64)]
    pub circuit_type: CircuitType,
    pub proof: String,
    #[column(length_limit = 128)]
    pub root_hash: BigInt,
    pub output_commitments: Option<Vec<BigInt>>,
    #[column(length_limit = 255)]
    pub signature: String,
    pub serial_numbers: Option<Vec<BigInt>>,
    pub sig_hashes: Option<Vec<BigInt>>,
    #[column(length_limit = 255)]
    pub sig_pk: String,
    #[column(length_limit = 128)]
    pub public_amount: BigInt,
    #[column(length_limit = 128)]
    pub gas_relayer_fee_amount: BigInt,
    pub out_rollup_fees: Option<Vec<BigInt>>,
    #[column(length_limit = 64)]
    pub public_recipient: String,
    #[column(length_limit = 64)]
    pub relayer_recipient_address: String,
    pub out_encrypted_notes: Option<Vec<String>>,
    #[column(length_limit = 255)]
    pub random_auditing_public_key: BigInt,
    pub error_message: Option<String>,
    #[column(length_limit = 128)]
    pub transaction_hash: Option<String>,
}

fn indexes() -> Vec<IndexColumns> {
    vec![
        IndexColumns::builder()
            .column_names(vec![TransactionColumn::ChainId.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![TransactionColumn::Signature.to_string()])
            .build(),
    ]
}
