use mystiko_protos::core::document::v1::Spend as ProtoSpend;
use mystiko_storage::{Document, DocumentData, IndexColumns};
use mystiko_storage_macros::CollectionBuilder;
use num_bigint::{BigUint, ParseBigIntError};
use serde::{Deserialize, Serialize};

#[derive(CollectionBuilder, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[collection(indexes = indexes())]
pub struct Spend {
    pub chain_id: u64,
    #[column(length_limit = 64)]
    pub contract_address: String,
    pub bridge_type: i32,
    #[column(length_limit = 16)]
    pub asset_symbol: String,
    pub asset_decimals: u32,
    #[column(length_limit = 64)]
    pub asset_address: Option<String>,
    pub proof: Option<String>,
    #[column(length_limit = 128)]
    pub root_hash: Option<BigUint>,
    pub input_commitments: Vec<BigUint>,
    pub output_commitments: Option<Vec<BigUint>>,
    pub nullifiers: Option<Vec<BigUint>>,
    #[column(length_limit = 255)]
    pub signature_public_key: Option<String>,
    pub signature_public_key_hashes: Option<Vec<BigUint>>,
    pub amount: f64,
    #[column(length_limit = 128)]
    pub decimal_amount: BigUint,
    #[column(length_limit = 128)]
    pub recipient: String,
    pub rollup_fee_amount: Option<f64>,
    #[column(length_limit = 128)]
    pub rollup_fee_decimal_amount: Option<BigUint>,
    pub rollup_fee_total_amount: Option<f64>,
    #[column(length_limit = 128)]
    pub rollup_fee_total_decimal_amount: Option<BigUint>,
    pub gas_relayer_fee_amount: Option<f64>,
    #[column(length_limit = 128)]
    pub gas_relayer_fee_decimal_amount: Option<BigUint>,
    #[column(length_limit = 64)]
    pub gas_relayer_address: Option<String>,
    pub gas_relayer_url: Option<String>,
    #[column(length_limit = 255)]
    pub signature: Option<String>,
    #[column(length_limit = 128)]
    pub random_auditing_public_key: Option<BigUint>,
    pub encrypted_auditor_notes: Option<Vec<BigUint>>,
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
            .column_names(vec![SpendColumn::BridgeType.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![SpendColumn::SignaturePublicKey.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![SpendColumn::RootHash.to_string()])
            .build(),
        IndexColumns::builder()
            .column_names(vec![SpendColumn::Recipient.to_string()])
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
    pub fn document_from_proto(proto: ProtoSpend) -> Result<Document<Self>, ParseBigIntError> {
        let input_commitments = proto.input_commitments_as_biguint()?;
        let output_commitments = proto.output_commitments_as_biguint()?;
        let nullifiers = proto.nullifiers_as_biguint()?;
        let decimal_amount = proto.decimal_amount_as_biguint()?;
        let rollup_fee_decimal_amount = proto.rollup_fee_decimal_amount_as_biguint()?;
        let rollup_fee_total_decimal_amount = proto.rollup_fee_total_decimal_amount_as_biguint()?;
        let gas_relayer_fee_decimal_amount = proto.gas_relayer_fee_decimal_amount_as_biguint()?;
        let signature_public_key_hashes = proto.signature_public_key_hashes_as_biguint()?;
        let random_auditing_public_key = proto.random_auditing_public_key_as_biguint()?;
        let encrypted_auditor_notes = proto.encrypted_auditor_notes_as_biguint()?;
        let root_hash = proto.root_hash_as_biguint()?;
        Ok(Document::new(
            proto.id,
            proto.created_at,
            proto.updated_at,
            Self {
                chain_id: proto.chain_id,
                contract_address: proto.contract_address,
                bridge_type: proto.bridge_type,
                asset_symbol: proto.asset_symbol,
                asset_decimals: proto.asset_decimals,
                asset_address: proto.asset_address,
                proof: proto.proof,
                root_hash,
                input_commitments,
                output_commitments,
                nullifiers,
                signature_public_key: proto.signature_public_key,
                signature_public_key_hashes,
                amount: proto.amount,
                decimal_amount,
                rollup_fee_amount: proto.rollup_fee_amount,
                rollup_fee_decimal_amount,
                rollup_fee_total_amount: proto.rollup_fee_total_amount,
                rollup_fee_total_decimal_amount,
                gas_relayer_fee_amount: proto.gas_relayer_fee_amount,
                gas_relayer_fee_decimal_amount,
                recipient: proto.recipient,
                gas_relayer_address: proto.gas_relayer_address,
                gas_relayer_url: proto.gas_relayer_url,
                signature: proto.signature,
                random_auditing_public_key,
                encrypted_auditor_notes,
                spend_type: proto.spend_type,
                status: proto.status,
                error_message: proto.error_message,
                transaction_hash: proto.transaction_hash,
                wallet_id: proto.wallet_id,
            },
        ))
    }

    pub fn document_into_proto(spend: Document<Self>) -> ProtoSpend {
        ProtoSpend::builder()
            .id(spend.id)
            .created_at(spend.created_at)
            .updated_at(spend.updated_at)
            .chain_id(spend.data.chain_id)
            .contract_address(spend.data.contract_address)
            .bridge_type(spend.data.bridge_type)
            .asset_symbol(spend.data.asset_symbol)
            .asset_decimals(spend.data.asset_decimals)
            .asset_address(spend.data.asset_address)
            .proof(spend.data.proof)
            .root_hash(spend.data.root_hash.map(|n| n.to_string()))
            .input_commitments(
                spend
                    .data
                    .input_commitments
                    .into_iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>(),
            )
            .output_commitments(
                spend
                    .data
                    .output_commitments
                    .unwrap_or_default()
                    .into_iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>(),
            )
            .nullifiers(
                spend
                    .data
                    .nullifiers
                    .unwrap_or_default()
                    .into_iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>(),
            )
            .signature_public_key(spend.data.signature_public_key)
            .signature_public_key_hashes(
                spend
                    .data
                    .signature_public_key_hashes
                    .unwrap_or_default()
                    .into_iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>(),
            )
            .amount(spend.data.amount)
            .decimal_amount(spend.data.decimal_amount.to_string())
            .rollup_fee_amount(spend.data.rollup_fee_amount)
            .rollup_fee_decimal_amount(spend.data.rollup_fee_decimal_amount.map(|n| n.to_string()))
            .rollup_fee_total_amount(spend.data.rollup_fee_total_amount)
            .rollup_fee_total_decimal_amount(spend.data.rollup_fee_total_decimal_amount.map(|n| n.to_string()))
            .gas_relayer_fee_amount(spend.data.gas_relayer_fee_amount)
            .gas_relayer_fee_decimal_amount(spend.data.gas_relayer_fee_decimal_amount.map(|n| n.to_string()))
            .recipient(spend.data.recipient)
            .gas_relayer_address(spend.data.gas_relayer_address)
            .gas_relayer_url(spend.data.gas_relayer_url)
            .signature(spend.data.signature)
            .random_auditing_public_key(spend.data.random_auditing_public_key.map(|n| n.to_string()))
            .encrypted_auditor_notes(
                spend
                    .data
                    .encrypted_auditor_notes
                    .unwrap_or_default()
                    .into_iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>(),
            )
            .spend_type(spend.data.spend_type)
            .status(spend.data.status)
            .error_message(spend.data.error_message)
            .transaction_hash(spend.data.transaction_hash)
            .wallet_id(spend.data.wallet_id)
            .build()
    }
}
