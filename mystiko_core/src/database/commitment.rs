use mystiko_protos::common::v1::BridgeType;
use mystiko_storage::{DocumentData, IndexColumns, UniqueColumns};
use mystiko_storage_macros::CollectionBuilder;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

#[derive(CollectionBuilder, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[collection(uniques = uniques(), indexes = indexes())]
pub struct Commitment {
    pub chain_id: u64,
    #[column(length_limit = 64)]
    pub contract_address: String,
    pub bridge_type: i32,
    #[column(length_limit = 128)]
    pub commitment_hash: BigUint,
    #[column(length_limit = 16)]
    pub asset_symbol: String,
    pub asset_decimals: u32,
    #[column(length_limit = 64)]
    pub asset_address: Option<String>,
    pub status: i32,
    pub spent: bool,
    pub block_number: u64,
    pub src_chain_block_number: Option<u64>,
    pub included_block_number: Option<u64>,
    #[column(length_limit = 128)]
    pub rollup_fee_amount: Option<BigUint>,
    pub encrypted_note: Option<String>,
    pub leaf_index: Option<u64>,
    #[column(length_limit = 128)]
    pub amount: Option<BigUint>,
    #[column(length_limit = 128)]
    pub nullifier: Option<BigUint>,
    #[column(length_limit = 128)]
    pub shielded_address: Option<String>,
    #[column(length_limit = 128)]
    pub queued_transaction_hash: Option<String>,
    #[column(length_limit = 128)]
    pub included_transaction_hash: Option<String>,
    #[column(length_limit = 128)]
    pub src_chain_transaction_hash: Option<String>,
}

fn uniques() -> Vec<UniqueColumns> {
    vec![
        vec![
            CommitmentColumn::ChainId,
            CommitmentColumn::ContractAddress,
            CommitmentColumn::CommitmentHash,
        ]
        .into(),
        vec![
            CommitmentColumn::ChainId,
            CommitmentColumn::ContractAddress,
            CommitmentColumn::LeafIndex,
        ]
        .into(),
    ]
}

fn indexes() -> Vec<IndexColumns> {
    vec![
        vec![CommitmentColumn::ChainId].into(),
        vec![CommitmentColumn::ContractAddress].into(),
        vec![CommitmentColumn::BridgeType].into(),
        vec![CommitmentColumn::CommitmentHash].into(),
        vec![CommitmentColumn::LeafIndex].into(),
        vec![CommitmentColumn::Status].into(),
        vec![CommitmentColumn::Spent].into(),
        vec![CommitmentColumn::BlockNumber].into(),
        vec![CommitmentColumn::SrcChainBlockNumber].into(),
        vec![CommitmentColumn::IncludedBlockNumber].into(),
        vec![CommitmentColumn::AssetSymbol].into(),
        vec![CommitmentColumn::AssetAddress].into(),
        vec![CommitmentColumn::Nullifier].into(),
        vec![CommitmentColumn::ShieldedAddress].into(),
        vec![CommitmentColumn::QueuedTransactionHash].into(),
        vec![CommitmentColumn::IncludedTransactionHash].into(),
        vec![CommitmentColumn::SrcChainTransactionHash].into(),
    ]
}

impl mystiko_dataloader::handler::document::DatabaseCommitment for Commitment {
    fn column_chain_id() -> String {
        CommitmentColumn::ChainId.to_string()
    }

    fn column_contract_address() -> String {
        CommitmentColumn::ContractAddress.to_string()
    }

    fn column_commitment_hash() -> String {
        CommitmentColumn::CommitmentHash.to_string()
    }

    fn column_status() -> String {
        CommitmentColumn::Status.to_string()
    }

    fn column_block_number() -> String {
        CommitmentColumn::BlockNumber.to_string()
    }

    fn column_src_block_number() -> String {
        CommitmentColumn::SrcChainBlockNumber.to_string()
    }

    fn column_included_block_number() -> String {
        CommitmentColumn::IncludedBlockNumber.to_string()
    }

    fn column_leaf_index() -> String {
        CommitmentColumn::LeafIndex.to_string()
    }

    fn column_rollup_fee() -> String {
        CommitmentColumn::RollupFeeAmount.to_string()
    }

    fn column_encrypted_note() -> String {
        CommitmentColumn::EncryptedNote.to_string()
    }

    fn column_queued_transaction_hash() -> String {
        CommitmentColumn::QueuedTransactionHash.to_string()
    }

    fn column_included_transaction_hash() -> String {
        CommitmentColumn::IncludedTransactionHash.to_string()
    }

    fn column_src_transaction_hash() -> String {
        CommitmentColumn::SrcChainTransactionHash.to_string()
    }

    fn from_proto(
        config: std::sync::Arc<mystiko_config::MystikoConfig>,
        chain_id: u64,
        address: &str,
        proto: mystiko_protos::data::v1::Commitment,
    ) -> anyhow::Result<Self> {
        if let Some(contract_config) = config.find_contract_by_address(chain_id, address) {
            let asset = contract_config.asset();
            let asset_address = if asset.asset_type() == &mystiko_types::AssetType::Main {
                None
            } else {
                Some(asset.asset_address().to_string())
            };
            let bridge_type: BridgeType = contract_config.bridge_type().into();
            Ok(Self {
                chain_id,
                contract_address: address.to_string(),
                bridge_type: bridge_type as i32,
                commitment_hash: mystiko_utils::convert::bytes_to_biguint(&proto.commitment_hash),
                asset_symbol: asset.asset_symbol().to_string(),
                asset_decimals: asset.asset_decimals(),
                asset_address,
                status: proto.status,
                spent: false,
                block_number: proto.block_number,
                src_chain_block_number: proto.src_chain_block_number,
                included_block_number: proto.included_block_number,
                rollup_fee_amount: proto.rollup_fee.as_ref().map(mystiko_utils::convert::bytes_to_biguint),
                encrypted_note: proto.encrypted_note.as_ref().map(mystiko_utils::hex::encode_hex),
                leaf_index: proto.leaf_index,
                amount: None,
                nullifier: None,
                shielded_address: None,
                queued_transaction_hash: proto.queued_transaction_hash_as_hex(),
                included_transaction_hash: proto.included_transaction_hash_as_hex(),
                src_chain_transaction_hash: proto.src_chain_transaction_hash_as_hex(),
            })
        } else {
            Err(anyhow::anyhow!(
                "chain_id {} contract {} not found in config",
                chain_id,
                address
            ))
        }
    }

    fn get_chain_id(&self) -> u64 {
        self.chain_id
    }

    fn get_contract_address(&self) -> &String {
        &self.contract_address
    }

    fn get_commitment_hash(&self) -> &BigUint {
        &self.commitment_hash
    }

    fn get_status(&self) -> i32 {
        self.status
    }

    fn get_block_number(&self) -> u64 {
        self.block_number
    }

    fn get_src_block_number(&self) -> Option<u64> {
        self.src_chain_block_number
    }

    fn get_included_block_number(&self) -> Option<u64> {
        self.included_block_number
    }

    fn get_leaf_index(&self) -> Option<u64> {
        self.leaf_index
    }

    fn get_rollup_fee(&self) -> Option<&BigUint> {
        self.rollup_fee_amount.as_ref()
    }

    fn get_encrypted_note(&self) -> Option<&String> {
        self.encrypted_note.as_ref()
    }

    fn get_queued_transaction_hash(&self) -> Option<&String> {
        self.queued_transaction_hash.as_ref()
    }

    fn get_included_transaction_hash(&self) -> Option<&String> {
        self.included_transaction_hash.as_ref()
    }

    fn get_src_transaction_hash(&self) -> Option<&String> {
        self.src_chain_transaction_hash.as_ref()
    }

    fn update_by_proto(
        &mut self,
        _config: std::sync::Arc<mystiko_config::MystikoConfig>,
        proto: &mystiko_protos::data::v1::Commitment,
    ) -> anyhow::Result<()> {
        if self.block_number > proto.block_number {
            self.block_number = proto.block_number;
        } else {
            self.status = proto.status;
        }
        if self.src_chain_block_number.is_none() {
            self.src_chain_block_number = proto.src_chain_block_number;
        }
        if self.included_block_number.is_none() {
            self.included_block_number = proto.included_block_number;
        }
        if self.rollup_fee_amount.is_none() {
            self.rollup_fee_amount = proto.rollup_fee.as_ref().map(mystiko_utils::convert::bytes_to_biguint);
        }
        if self.encrypted_note.is_none() {
            self.encrypted_note = proto.encrypted_note.as_ref().map(mystiko_utils::hex::encode_hex);
        }
        if self.leaf_index.is_none() {
            self.leaf_index = proto.leaf_index;
        }
        if self.queued_transaction_hash.is_none() {
            self.queued_transaction_hash = proto.queued_transaction_hash_as_hex();
        }
        if self.included_transaction_hash.is_none() {
            self.included_transaction_hash = proto.included_transaction_hash_as_hex();
        }
        if self.src_chain_transaction_hash.is_none() {
            self.src_chain_transaction_hash = proto.src_chain_transaction_hash_as_hex();
        }
        Ok(())
    }

    fn into_proto(self) -> anyhow::Result<mystiko_protos::data::v1::Commitment> {
        Ok(mystiko_protos::data::v1::Commitment::builder()
            .commitment_hash(mystiko_utils::convert::biguint_to_bytes(&self.commitment_hash))
            .status(self.status)
            .block_number(self.block_number)
            .src_chain_block_number(self.src_chain_block_number)
            .included_block_number(self.included_block_number)
            .rollup_fee(
                self.rollup_fee_amount
                    .map(|f| mystiko_utils::convert::biguint_to_bytes(&f)),
            )
            .encrypted_note(self.encrypted_note.map(mystiko_utils::hex::decode_hex).transpose()?)
            .leaf_index(self.leaf_index)
            .queued_transaction_hash(
                self.queued_transaction_hash
                    .map(mystiko_utils::hex::decode_hex)
                    .transpose()?,
            )
            .included_transaction_hash(
                self.included_transaction_hash
                    .map(mystiko_utils::hex::decode_hex)
                    .transpose()?,
            )
            .src_chain_transaction_hash(
                self.src_chain_transaction_hash
                    .map(mystiko_utils::hex::decode_hex)
                    .transpose()?,
            )
            .build())
    }
}
