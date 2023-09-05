use crate::handler::document::DatabaseNullifier;
use anyhow::Result;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_protos::data::v1::Nullifier as ProtoNullifier;
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
pub struct Nullifier {
    pub chain_id: u64,
    #[column(length_limit = 64)]
    pub contract_address: String,
    #[column(length_limit = 128)]
    pub nullifier: BigUint,
    pub block_number: u64,
    #[column(length_limit = 128)]
    pub transaction_hash: String,
}

fn uniques() -> Vec<UniqueColumns> {
    vec![vec![
        NullifierColumn::ChainId,
        NullifierColumn::ContractAddress,
        NullifierColumn::Nullifier,
    ]
    .into()]
}

fn indexes() -> Vec<IndexColumns> {
    vec![
        vec![NullifierColumn::ChainId].into(),
        vec![NullifierColumn::ContractAddress].into(),
        vec![NullifierColumn::BlockNumber].into(),
        vec![NullifierColumn::Nullifier].into(),
        vec![NullifierColumn::TransactionHash].into(),
    ]
}

impl DatabaseNullifier for Nullifier {
    fn column_chain_id() -> String {
        NullifierColumn::ChainId.to_string()
    }

    fn column_contract_address() -> String {
        NullifierColumn::ContractAddress.to_string()
    }

    fn column_nullifier() -> String {
        NullifierColumn::Nullifier.to_string()
    }

    fn column_block_number() -> String {
        NullifierColumn::BlockNumber.to_string()
    }

    fn column_transaction_hash() -> String {
        NullifierColumn::TransactionHash.to_string()
    }

    fn from_proto(_config: Arc<MystikoConfig>, chain_id: u64, address: &str, proto: ProtoNullifier) -> Result<Self> {
        Ok(Self {
            chain_id,
            contract_address: address.to_string(),
            nullifier: bytes_to_biguint(&proto.nullifier),
            block_number: proto.block_number,
            transaction_hash: encode_hex_with_prefix(&proto.transaction_hash),
        })
    }

    fn get_chain_id(&self) -> u64 {
        self.chain_id
    }

    fn get_contract_address(&self) -> &String {
        &self.contract_address
    }

    fn get_nullifier(&self) -> &BigUint {
        &self.nullifier
    }

    fn get_block_number(&self) -> u64 {
        self.block_number
    }

    fn get_transaction_hash(&self) -> &String {
        &self.transaction_hash
    }

    fn update_by_proto(&mut self, _config: Arc<MystikoConfig>, proto: &ProtoNullifier) -> Result<()> {
        self.block_number = proto.block_number;
        self.transaction_hash = encode_hex_with_prefix(&proto.transaction_hash);
        Ok(())
    }

    fn into_proto(self) -> Result<ProtoNullifier> {
        Ok(ProtoNullifier::builder()
            .nullifier(biguint_to_bytes(&self.nullifier))
            .block_number(self.block_number)
            .transaction_hash(decode_hex(&self.transaction_hash)?)
            .build())
    }
}
