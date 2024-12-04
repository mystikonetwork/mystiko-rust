mod handler;

pub use handler::*;
use std::sync::Arc;

use crate::TransactionSigner;
use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, Bytes, TxHash, U256};
use mystiko_abi::commitment_pool::TransactRequest;
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

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct MinRollupFeeOptions {
    pub chain_id: u64,
    pub contract_address: Address,
    #[builder(default)]
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct IncludedCountOptions {
    pub chain_id: u64,
    pub contract_address: Address,
    #[builder(default)]
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct AuditorPublicKeysOptions {
    pub chain_id: u64,
    pub contract_address: Address,
    #[builder(default)]
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TransactOptions<
    T: Into<TypedTransaction> + Clone + Default,
    S: TransactionSigner = Box<dyn TransactionSigner>,
> {
    pub chain_id: u64,
    pub contract_address: Address,
    pub request: TransactRequest,
    pub signature: Bytes,
    pub signer: Arc<S>,
    #[builder(default)]
    pub timeout_ms: Option<u64>,
    #[builder(default)]
    pub tx: T,
}

#[async_trait]
pub trait CommitmentPoolContractHandler: Send + Sync {
    type Error;

    async fn is_historic_commitment(&self, options: IsHistoricCommitmentOptions) -> Result<bool, Self::Error>;

    async fn is_spent_nullifier(&self, options: IsSpentNullifierOptions) -> Result<bool, Self::Error>;

    async fn is_known_root(&self, options: IsKnownRootOptions) -> Result<bool, Self::Error>;

    async fn min_rollup_fee(&self, options: MinRollupFeeOptions) -> Result<U256, Self::Error>;

    async fn get_commitment_included_count(&self, options: IncludedCountOptions) -> Result<U256, Self::Error>;

    async fn auditor_public_keys(&self, options: AuditorPublicKeysOptions) -> Result<Vec<U256>, Self::Error>;

    async fn transact<T, S>(&self, options: TransactOptions<T, S>) -> Result<TxHash, Self::Error>
    where
        T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static,
        S: TransactionSigner + 'static;
}
