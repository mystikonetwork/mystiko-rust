use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::{Address, TxHash};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder, Deserialize, Serialize)]
#[builder(field_defaults(setter(into)))]
pub struct ContractLoadedBlock {
    pub address: String,
    pub loaded_block: u64,
}

#[derive(Debug, Clone, TypedBuilder, Deserialize, Serialize)]
#[builder(field_defaults(setter(into)))]
pub struct ChainLoadedBlock {
    pub loaded_block: u64,
    #[builder(default)]
    pub contracts: Vec<ContractLoadedBlock>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CommitmentsWithContract<C> {
    pub chain_id: u64,
    pub contract_address: Address,
    pub commitments: Vec<C>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct NullifiersWithContract<N> {
    pub chain_id: u64,
    pub contract_address: Address,
    pub nullifiers: Vec<N>,
}

#[async_trait]
pub trait SequencerClient<D, R, C, N>: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn chain_loaded_block(&self, chain_id: u64, with_contracts: bool) -> Result<ChainLoadedBlock, Self::Error>;

    async fn contract_loaded_block(&self, chain_id: u64, contract_address: &Address) -> Result<u64, Self::Error>;

    async fn fetch_chain(&self, request: D) -> Result<R, Self::Error>;

    async fn get_commitments(
        &self,
        chain_id: u64,
        contract_address: &Address,
        commitment_hashes: &[BigUint],
    ) -> Result<Vec<C>, Self::Error>;

    async fn get_commitments_by_tx_hash(
        &self,
        chain_id: u64,
        tx_hash: &TxHash,
    ) -> Result<CommitmentsWithContract<C>, Self::Error>;

    async fn get_nullifiers(
        &self,
        chain_id: u64,
        contract_address: &Address,
        nullifier_hashes: &[BigUint],
    ) -> Result<Vec<N>, Self::Error>;

    async fn get_nullifiers_by_tx_hash(
        &self,
        chain_id: u64,
        tx_hash: &TxHash,
    ) -> Result<NullifiersWithContract<N>, Self::Error>;

    async fn health_check(&self) -> Result<(), Self::Error>;
}
