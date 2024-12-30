mod context;
mod create;
mod error;
mod fix;
mod handler;
mod merkle;
mod proof;
mod quote;
mod send;
mod status;
mod summary;
mod utils;

pub use error::*;
pub use handler::*;
pub use utils::*;

pub(crate) use context::*;

use crate::TransactionSigner;
use async_trait::async_trait;
use mystiko_protos::storage::v1::QueryFilter;
use mystiko_storage::ColumnValues;
use std::sync::Arc;

#[async_trait]
pub trait SpendHandler<S, QO, QR, CO, SS, SO, FO>: Send + Sync {
    type Error;

    async fn quote(&self, options: QO) -> Result<QR, Self::Error>;

    async fn summary(&self, options: CO) -> Result<SS, Self::Error>;

    async fn create(&self, options: CO) -> Result<S, Self::Error>;

    async fn send(&self, options: SO) -> Result<S, Self::Error>;

    async fn send_with_signer<Signer>(&self, options: SO, signer: Arc<Signer>) -> Result<S, Self::Error>
    where
        Signer: TransactionSigner + 'static;
    async fn fix_status(&self, options: FO) -> Result<S, Self::Error>;

    async fn find<Filter>(&self, filter: Filter) -> Result<Vec<S>, Self::Error>
    where
        Filter: Into<QueryFilter> + Send + Sync + 'static;

    async fn find_all(&self) -> Result<Vec<S>, Self::Error>;

    async fn find_one<Filter>(&self, filter: Filter) -> Result<Option<S>, Self::Error>
    where
        Filter: Into<QueryFilter> + Send + Sync + 'static;

    async fn find_by_id(&self, id: String) -> Result<Option<S>, Self::Error>;

    async fn count<Filter>(&self, filter: Filter) -> Result<u64, Self::Error>
    where
        Filter: Into<QueryFilter> + Send + Sync + 'static;

    async fn count_all(&self) -> Result<u64, Self::Error>;

    async fn update(&self, spend: S) -> Result<S, Self::Error>;

    async fn update_batch(&self, spends: Vec<S>) -> Result<Vec<S>, Self::Error>;

    async fn update_by_filter<Filter, Values>(&self, column_values: Values, filter: Filter) -> Result<(), Self::Error>
    where
        Filter: Into<QueryFilter> + Send + Sync + 'static,
        Values: Into<ColumnValues> + Send + Sync + 'static;

    async fn update_all<Values>(&self, column_values: Values) -> Result<(), Self::Error>
    where
        Values: Into<ColumnValues> + Send + Sync + 'static;

    async fn delete(&self, spend: S) -> Result<(), Self::Error>;

    async fn delete_batch(&self, spends: Vec<S>) -> Result<(), Self::Error>;

    async fn delete_by_filter<Filter>(&self, filter: Filter) -> Result<(), Self::Error>
    where
        Filter: Into<QueryFilter> + Send + Sync + 'static;

    async fn delete_all(&self) -> Result<(), Self::Error>;
}
