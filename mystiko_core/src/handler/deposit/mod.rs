mod asset;
mod context;
mod create;
mod error;
mod fix;
mod handler;
mod quote;
mod screening;
mod send;
mod summary;
mod utils;

pub use error::*;
pub use handler::*;

pub(crate) use asset::*;
pub(crate) use context::*;
pub(crate) use utils::*;

use crate::TransactionSigner;
use async_trait::async_trait;
use mystiko_protos::storage::v1::QueryFilter;
use mystiko_storage::ColumnValues;
use std::sync::Arc;

#[async_trait]
pub trait DepositHandler<D, QO, Q, C, DS, S, F>: Send + Sync {
    type Error;

    async fn quote(&self, options: QO) -> Result<Q, Self::Error>;

    async fn summary(&self, options: C) -> Result<DS, Self::Error>;

    async fn create(&self, options: C) -> Result<D, Self::Error>;

    async fn send(&self, options: S) -> Result<D, Self::Error>;

    async fn send_with_signer<Signer>(&self, options: S, signer: Arc<Signer>) -> Result<D, Self::Error>
    where
        Signer: TransactionSigner + 'static;

    async fn fix_status(&self, options: F) -> Result<D, Self::Error>;

    async fn find<Filter>(&self, filter: Filter) -> Result<Vec<D>, Self::Error>
    where
        Filter: Into<QueryFilter> + Send + Sync + 'static;

    async fn find_all(&self) -> Result<Vec<D>, Self::Error>;

    async fn find_one<Filter>(&self, filter: Filter) -> Result<Option<D>, Self::Error>
    where
        Filter: Into<QueryFilter> + Send + Sync + 'static;

    async fn find_by_id(&self, id: String) -> Result<Option<D>, Self::Error>;

    async fn count<Filter>(&self, filter: Filter) -> Result<u64, Self::Error>
    where
        Filter: Into<QueryFilter> + Send + Sync + 'static;

    async fn count_all(&self) -> Result<u64, Self::Error>;

    async fn update(&self, deposit: D) -> Result<D, Self::Error>;

    async fn update_batch(&self, deposits: Vec<D>) -> Result<Vec<D>, Self::Error>;

    async fn update_by_filter<Filter, Values>(&self, column_values: Values, filter: Filter) -> Result<(), Self::Error>
    where
        Filter: Into<QueryFilter> + Send + Sync + 'static,
        Values: Into<ColumnValues> + Send + Sync + 'static;

    async fn update_all<Values>(&self, column_values: Values) -> Result<(), Self::Error>
    where
        Values: Into<ColumnValues> + Send + Sync + 'static;

    async fn delete(&self, deposit: D) -> Result<(), Self::Error>;

    async fn delete_batch(&self, deposits: Vec<D>) -> Result<(), Self::Error>;

    async fn delete_by_filter<Filter>(&self, filter: Filter) -> Result<(), Self::Error>
    where
        Filter: Into<QueryFilter> + Send + Sync + 'static;

    async fn delete_all(&self) -> Result<(), Self::Error>;
}
