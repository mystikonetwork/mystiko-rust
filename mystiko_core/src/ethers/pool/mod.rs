mod handler;

pub use handler::*;

use async_trait::async_trait;
use ethers_core::types::{Address, U256};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct IsHistoricCommitmentOptions {
    pub chain_id: u64,
    pub contract_address: Address,
    pub commitment_hash: U256,
    #[builder(default)]
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct IsSpentNullifierOptions {
    pub chain_id: u64,
    pub contract_address: Address,
    pub nullifier: U256,
    #[builder(default)]
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct IsKnownRootOptions {
    pub chain_id: u64,
    pub contract_address: Address,
    pub root_hash: U256,
    #[builder(default)]
    pub timeout_ms: Option<u64>,
}

#[async_trait]
pub trait CommitmentPoolContractHandler: Send + Sync {
    type Error;

    async fn is_historic_commitment(&self, options: IsHistoricCommitmentOptions) -> Result<bool, Self::Error>;

    async fn is_spent_nullifier(&self, options: IsSpentNullifierOptions) -> Result<bool, Self::Error>;

    async fn is_known_root(&self, options: IsKnownRootOptions) -> Result<bool, Self::Error>;
}
