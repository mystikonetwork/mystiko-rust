mod handler;

pub use handler::*;
use std::sync::Arc;

use crate::TransactionSigner;
use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, TxHash, U256};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct BalanceOptions {
    pub chain_id: u64,
    pub owner: Address,
    #[builder(default)]
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TransferOptions<
    T: Into<TypedTransaction> + Clone + Default,
    S: TransactionSigner = Box<dyn TransactionSigner>,
> {
    pub chain_id: u64,
    pub owner: Address,
    pub recipient: Address,
    pub amount: U256,
    pub signer: Arc<S>,
    #[builder(default)]
    pub timeout_ms: Option<u64>,
    #[builder(default)]
    pub tx: T,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Erc20BalanceOptions {
    pub chain_id: u64,
    pub asset_address: Address,
    pub owner: Address,
    #[builder(default)]
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Erc20AllowanceOptions {
    pub chain_id: u64,
    pub asset_address: Address,
    pub owner: Address,
    pub recipient: Address,
    #[builder(default)]
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Erc20ApproveOptions<
    T: Into<TypedTransaction> + Clone + Default,
    S: TransactionSigner = Box<dyn TransactionSigner>,
> {
    pub chain_id: u64,
    pub asset_address: Address,
    pub owner: Address,
    pub recipient: Address,
    pub amount: U256,
    pub signer: Arc<S>,
    #[builder(default)]
    pub timeout_ms: Option<u64>,
    #[builder(default)]
    pub tx: T,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Erc20TransferOptions<
    T: Into<TypedTransaction> + Clone + Default,
    S: TransactionSigner = Box<dyn TransactionSigner>,
> {
    pub chain_id: u64,
    pub asset_address: Address,
    pub owner: Address,
    pub recipient: Address,
    pub amount: U256,
    pub signer: Arc<S>,
    #[builder(default)]
    pub timeout_ms: Option<u64>,
    #[builder(default)]
    pub tx: T,
}

#[async_trait]
pub trait PublicAssetHandler: Send + Sync {
    type Error;

    async fn balance_of(&self, options: BalanceOptions) -> Result<U256, Self::Error>;

    async fn transfer<T, S>(&self, options: TransferOptions<T, S>) -> Result<TxHash, Self::Error>
    where
        S: TransactionSigner + 'static,
        T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static;

    async fn erc20_balance_of(&self, options: Erc20BalanceOptions) -> Result<U256, Self::Error>;

    async fn erc20_allowance(&self, options: Erc20AllowanceOptions) -> Result<U256, Self::Error>;

    async fn erc20_approve<T, S>(&self, options: Erc20ApproveOptions<T, S>) -> Result<Option<TxHash>, Self::Error>
    where
        S: TransactionSigner + 'static,
        T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static;

    async fn erc20_transfer<T, S>(&self, options: Erc20TransferOptions<T, S>) -> Result<TxHash, Self::Error>
    where
        S: TransactionSigner + 'static,
        T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static;
}
