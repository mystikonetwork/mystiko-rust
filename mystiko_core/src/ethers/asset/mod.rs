mod handler;

pub use handler::*;

use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, TxHash, U256};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct BalanceOptions {
    pub chain_id: u64,
    pub owner: Address,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct TransferOptions<T: Into<TypedTransaction> + Clone + Default> {
    pub chain_id: u64,
    pub owner: Address,
    pub recipient: Address,
    pub amount: U256,
    #[builder(default)]
    pub tx: T,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Erc20BalanceOptions {
    pub chain_id: u64,
    pub asset_address: Address,
    pub owner: Address,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Erc20ApproveOptions<T: Into<TypedTransaction> + Clone + Default> {
    pub chain_id: u64,
    pub asset_address: Address,
    pub owner: Address,
    pub recipient: Address,
    pub amount: U256,
    #[builder(default)]
    pub tx: T,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Erc20TransferOptions<T: Into<TypedTransaction> + Clone + Default> {
    pub chain_id: u64,
    pub asset_address: Address,
    pub owner: Address,
    pub recipient: Address,
    pub amount: U256,
    #[builder(default)]
    pub tx: T,
}

#[async_trait]
pub trait PublicAssetHandler: Send + Sync {
    type Error;

    async fn balance_of(&self, options: BalanceOptions) -> Result<U256, Self::Error>;

    async fn transfer<T>(&self, options: TransferOptions<T>) -> Result<TxHash, Self::Error>
    where
        T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static;

    async fn erc20_balance_of(&self, options: Erc20BalanceOptions) -> Result<U256, Self::Error>;

    async fn erc20_approve<T>(&self, options: Erc20ApproveOptions<T>) -> Result<Option<TxHash>, Self::Error>
    where
        T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static;

    async fn erc20_transfer<T>(&self, options: Erc20TransferOptions<T>) -> Result<TxHash, Self::Error>
    where
        T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static;
}

impl<T> From<TransferOptions<T>> for BalanceOptions
where
    T: Into<TypedTransaction> + Clone + Default,
{
    fn from(options: TransferOptions<T>) -> Self {
        Self::builder().chain_id(options.chain_id).owner(options.owner).build()
    }
}
