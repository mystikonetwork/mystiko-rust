use mystiko_protos::core::document::v1::Spend as ProtoSpend;
use mystiko_storage::{Document, DocumentData, IndexColumns};
use mystiko_storage_macros::CollectionBuilder;
use serde::{Deserialize, Serialize};

#[derive(CollectionBuilder, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[collection(indexes = indexes())]
pub struct Spend {
    pub chain_id: u64,
    #[column(length_limit = 64)]
    pub contract_address: String,
    #[column(length_limit = 16)]
    pub asset_symbol: String,
    #[column(length_limit = 64)]
    pub asset_address: Option<String>,
    pub proof: Option<String>,
    #[column(length_limit = 128)]
    pub root_hash: String,
    pub input_commitments: Vec<String>,
    pub output_commitments: Option<Vec<String>>,
    pub nullifiers: Option<Vec<String>>,
    #[column(length_limit = 255)]
    pub signature_public_key: Option<String>,
    pub signature_public_key_hashes: Option<Vec<String>>,
    pub amount: f64,
    pub public_amount: f64,
    pub rollup_fee_amount: Option<f64>,
    pub gas_relayer_fee_amount: Option<f64>,
    #[column(length_limit = 128)]
    pub shielded_address: Option<String>,
    #[column(length_limit = 64)]
    pub public_address: Option<String>,
    #[column(length_limit = 64)]
    pub gas_relayer_address: Option<String>,
    #[column(length_limit = 255)]
    pub signature: Option<String>,
    #[column(length_limit = 128)]
    pub random_auditing_public_key: Option<String>,
    pub encrypted_auditor_notes: Option<Vec<String>>,
    pub spend_type: i32,
    pub status: i32,
    pub error_message: Option<String>,
    #[column(length_limit = 128)]
    pub transaction_hash: Option<String>,
    #[column(length_limit = 64)]
    pub wallet_id: String,
}

fn indexes() -> Vec<IndexColumns> {
    vec![
        IndexColumns::builder()
            .column_names(vec![SpendColumn::ChainId.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![SpendColumn::ContractAddress.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![SpendColumn::SignaturePublicKey.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![SpendColumn::RootHash.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![SpendColumn::ShieldedAddress.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![SpendColumn::PublicAddress.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![SpendColumn::TransactionHash.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![SpendColumn::SpendType.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![SpendColumn::WalletId.to_string()])
            .build(),
    ]
}

impl Spend {
    pub fn document_from_proto(proto: ProtoSpend) -> Document<Self> {
        Document::new(
            proto.id,
            proto.created_at,
            proto.updated_at,
            Self {
                chain_id: proto.chain_id,
                contract_address: proto.contract_address,
                asset_symbol: proto.asset_symbol,
                asset_address: proto.asset_address,
                proof: proto.proof,
                root_hash: proto.root_hash,
                input_commitments: proto.input_commitments,
                output_commitments: (!proto.output_commitments.is_empty()).then_some(proto.output_commitments),
                nullifiers: (!proto.nullifiers.is_empty()).then_some(proto.nullifiers),
                signature_public_key: proto.signature_public_key,
                signature_public_key_hashes: (!proto.signature_public_key_hashes.is_empty())
                    .then_some(proto.signature_public_key_hashes),
                amount: proto.amount,
                public_amount: proto.public_amount,
                rollup_fee_amount: proto.rollup_fee_amount,
                gas_relayer_fee_amount: proto.gas_relayer_fee_amount,
                shielded_address: proto.shielded_address,
                public_address: proto.public_address,
                gas_relayer_address: proto.gas_relayer_address,
                signature: proto.signature,
                random_auditing_public_key: proto.random_auditing_public_key,
                encrypted_auditor_notes: (!proto.encrypted_auditor_notes.is_empty())
                    .then_some(proto.encrypted_auditor_notes),
                spend_type: proto.spend_type,
                status: proto.status,
                error_message: proto.error_message,
                transaction_hash: proto.transaction_hash,
                wallet_id: proto.wallet_id,
            },
        )
    }

    pub fn document_into_proto(spend: Document<Self>) -> ProtoSpend {
        ProtoSpend::builder()
            .id(spend.id)
            .created_at(spend.created_at)
            .updated_at(spend.updated_at)
            .chain_id(spend.data.chain_id)
            .contract_address(spend.data.contract_address)
            .asset_symbol(spend.data.asset_symbol)
            .asset_address(spend.data.asset_address)
            .proof(spend.data.proof)
            .root_hash(spend.data.root_hash)
            .input_commitments(spend.data.input_commitments)
            .output_commitments(spend.data.output_commitments.unwrap_or_default())
            .nullifiers(spend.data.nullifiers.unwrap_or_default())
            .signature_public_key(spend.data.signature_public_key)
            .signature_public_key_hashes(spend.data.signature_public_key_hashes.unwrap_or_default())
            .amount(spend.data.amount)
            .public_amount(spend.data.public_amount)
            .rollup_fee_amount(spend.data.rollup_fee_amount)
            .gas_relayer_fee_amount(spend.data.gas_relayer_fee_amount)
            .shielded_address(spend.data.shielded_address)
            .public_address(spend.data.public_address)
            .gas_relayer_address(spend.data.gas_relayer_address)
            .signature(spend.data.signature)
            .random_auditing_public_key(spend.data.random_auditing_public_key)
            .encrypted_auditor_notes(spend.data.encrypted_auditor_notes.unwrap_or_default())
            .spend_type(spend.data.spend_type)
            .status(spend.data.status)
            .error_message(spend.data.error_message)
            .transaction_hash(spend.data.transaction_hash)
            .wallet_id(spend.data.wallet_id)
            .build()
    }
}
