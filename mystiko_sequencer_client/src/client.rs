use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::Address;
use num_bigint::BigUint;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ContractLoadedBlock {
    pub address: String,
    pub loaded_block: u64,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ChainLoadedBlock {
    pub loaded_block: u64,
    #[builder(default)]
    pub contracts: Vec<ContractLoadedBlock>,
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
        commitment_hashes: Vec<BigUint>,
    ) -> Result<Vec<C>, Self::Error>;

    async fn get_nullifiers(
        &self,
        chain_id: u64,
        contract_address: &Address,
        nullifier_hashes: Vec<BigUint>,
    ) -> Result<Vec<N>, Self::Error>;

    async fn health_check(&self) -> Result<(), Self::Error>;
}
