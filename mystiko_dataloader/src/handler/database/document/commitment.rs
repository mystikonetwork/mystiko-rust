use crate::handler::document::DatabaseCommitment;
use anyhow::Result;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_protos::data::v1::Commitment as ProtoCommitment;
use mystiko_storage::column::{IndexColumns, UniqueColumns};
use mystiko_storage::document::DocumentData;
use mystiko_storage_macros::CollectionBuilder;
use mystiko_utils::convert::{biguint_to_bytes, bytes_to_biguint};
use mystiko_utils::hex::{decode_hex, encode_hex_with_prefix};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(CollectionBuilder, Clone, PartialEq, Debug, Deserialize, Serialize, TypedBuilder)]
#[collection(uniques = uniques(), indexes = indexes())]
#[builder(field_defaults(setter(into)))]
pub struct Commitment {
    pub chain_id: u64,
    #[column(length_limit = 64)]
    pub contract_address: String,
    #[column(length_limit = 128)]
    pub commitment_hash: BigUint,
    pub status: i32,
    pub block_number: u64,
    #[builder(default)]
    pub src_chain_block_number: Option<u64>,
    #[builder(default)]
    pub included_block_number: Option<u64>,
    #[builder(default)]
    pub leaf_index: Option<u64>,
    #[column(length_limit = 128)]
    #[builder(default)]
    pub rollup_fee: Option<BigUint>,
    #[builder(default)]
    pub encrypted_notes: Option<String>,
    #[column(length_limit = 128)]
    #[builder(default)]
    pub queued_transaction_hash: Option<String>,
    #[column(length_limit = 128)]
    #[builder(default)]
    pub included_transaction_hash: Option<String>,
    #[column(length_limit = 128)]
    #[builder(default)]
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
        vec![CommitmentColumn::CommitmentHash].into(),
        vec![CommitmentColumn::BlockNumber].into(),
        vec![CommitmentColumn::SrcChainBlockNumber].into(),
        vec![CommitmentColumn::IncludedBlockNumber].into(),
        vec![CommitmentColumn::LeafIndex].into(),
        vec![CommitmentColumn::Status].into(),
    ]
}

impl DatabaseCommitment for Commitment {
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
        CommitmentColumn::RollupFee.to_string()
    }

    fn column_encrypted_note() -> String {
        CommitmentColumn::EncryptedNotes.to_string()
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

    fn from_proto(_config: Arc<MystikoConfig>, chain_id: u64, address: &str, proto: ProtoCommitment) -> Result<Self> {
        Ok(Self {
            chain_id,
            contract_address: address.to_string(),
            commitment_hash: bytes_to_biguint(&proto.commitment_hash),
            status: proto.status,
            block_number: proto.block_number,
            src_chain_block_number: proto.src_chain_block_number,
            included_block_number: proto.included_block_number,
            leaf_index: proto.leaf_index,
            rollup_fee: proto.rollup_fee.map(|f| bytes_to_biguint(&f)),
            encrypted_notes: proto.encrypted_note.map(|n| encode_hex_with_prefix(&n)),
            queued_transaction_hash: proto.queued_transaction_hash.map(|n| encode_hex_with_prefix(&n)),
            included_transaction_hash: proto.included_transaction_hash.map(|n| encode_hex_with_prefix(&n)),
            src_chain_transaction_hash: proto.src_chain_transaction_hash.map(|n| encode_hex_with_prefix(&n)),
        })
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
        self.rollup_fee.as_ref()
    }

    fn get_encrypted_note(&self) -> Option<&String> {
        self.encrypted_notes.as_ref()
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

    fn update_by_proto(&mut self, _proto: &ProtoCommitment) {
        todo!()
    }

    fn into_proto(self) -> Result<ProtoCommitment> {
        Ok(ProtoCommitment::builder()
            .commitment_hash(biguint_to_bytes(&self.commitment_hash))
            .status(self.status)
            .block_number(self.block_number)
            .src_chain_block_number(self.src_chain_block_number)
            .included_block_number(self.included_block_number)
            .leaf_index(self.leaf_index)
            .rollup_fee(self.rollup_fee.map(|f| biguint_to_bytes(&f)))
            .encrypted_note(self.encrypted_notes.map(|n| decode_hex(&n)).transpose()?)
            .queued_transaction_hash(self.queued_transaction_hash.map(|n| decode_hex(&n)).transpose()?)
            .included_transaction_hash(self.included_transaction_hash.map(|n| decode_hex(&n)).transpose()?)
            .src_chain_transaction_hash(self.src_chain_transaction_hash.map(|n| decode_hex(&n)).transpose()?)
            .build())
    }
}
