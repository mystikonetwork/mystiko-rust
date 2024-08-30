mod handler;

pub use handler::*;

use crate::TransactionSigner;
use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, TxHash, U256};
use mystiko_abi::mystiko_v2_bridge::BridgeDepositRequest as CrossChainDepositRequest;
use mystiko_abi::mystiko_v2_loop::LoopDepositRequest;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DepositQuoteOptions {
    pub chain_id: u64,
    pub contract_address: Address,
    #[builder(default)]
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct DepositQuote {
    pub min_amount: U256,
    pub max_amount: U256,
    pub min_rollup_fee_amount: U256,
    pub min_bridge_fee_amount: Option<U256>,
    pub min_executor_fee_amount: Option<U256>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ScreeningOptions {
    #[builder(default)]
    pub deadline: u64,
    #[builder(default)]
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DepositOptions<
    T: Into<TypedTransaction> + Clone + Default,
    S: TransactionSigner = Box<dyn TransactionSigner>,
> {
    pub chain_id: u64,
    pub contract_address: Address,
    pub request: LoopDepositRequest,
    #[builder(default)]
    pub screening: Option<ScreeningOptions>,
    pub signer: Arc<S>,
    #[builder(default)]
    pub timeout_ms: Option<u64>,
    #[builder(default)]
    pub tx: T,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct CrossChainDepositOptions<
    T: Into<TypedTransaction> + Clone + Default,
    S: TransactionSigner = Box<dyn TransactionSigner>,
> {
    pub chain_id: u64,
    pub contract_address: Address,
    pub request: CrossChainDepositRequest,
    #[builder(default)]
    pub screening: Option<ScreeningOptions>,
    pub signer: Arc<S>,
    #[builder(default)]
    pub timeout_ms: Option<u64>,
    #[builder(default)]
    pub tx: T,
}

#[async_trait]
pub trait DepositContractHandler: Send + Sync {
    type Error;

    async fn quote(&self, options: DepositQuoteOptions) -> Result<DepositQuote, Self::Error>;

    async fn deposit<T, S>(&self, options: DepositOptions<T, S>) -> Result<TxHash, Self::Error>
    where
        T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static,
        S: TransactionSigner + 'static;

    async fn cross_chain_deposit<T, S>(&self, options: CrossChainDepositOptions<T, S>) -> Result<TxHash, Self::Error>
    where
        T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static,
        S: TransactionSigner + 'static;
}
