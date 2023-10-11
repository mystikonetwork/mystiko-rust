use mystiko_storage::{DocumentData, IndexColumns, UniqueColumns};
use mystiko_storage_macros::CollectionBuilder;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

#[derive(CollectionBuilder, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[collection(uniques = uniques(), indexes = indexes())]
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

impl mystiko_dataloader::handler::document::DatabaseNullifier for Nullifier {
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

    fn from_proto(
        _config: std::sync::Arc<mystiko_config::MystikoConfig>,
        chain_id: u64,
        address: &str,
        proto: mystiko_protos::data::v1::Nullifier,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            chain_id,
            contract_address: address.to_string(),
            nullifier: proto.nullifier_as_biguint(),
            block_number: proto.block_number,
            transaction_hash: proto.transaction_hash_as_hex(),
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

    fn update_by_proto(
        &mut self,
        _config: std::sync::Arc<mystiko_config::MystikoConfig>,
        proto: &mystiko_protos::data::v1::Nullifier,
    ) -> anyhow::Result<()> {
        self.nullifier = proto.nullifier_as_biguint();
        self.block_number = proto.block_number;
        self.transaction_hash = proto.transaction_hash_as_hex();
        Ok(())
    }

    fn into_proto(self) -> anyhow::Result<mystiko_protos::data::v1::Nullifier> {
        Ok(mystiko_protos::data::v1::Nullifier::builder()
            .nullifier(mystiko_utils::convert::biguint_to_bytes(&self.nullifier))
            .block_number(self.block_number)
            .transaction_hash(mystiko_utils::hex::decode_hex(self.transaction_hash)?)
            .build())
    }
}
