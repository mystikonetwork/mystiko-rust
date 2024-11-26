mod error;
mod handler;
mod import;

pub use error::*;
pub use handler::*;

use async_trait::async_trait;

#[async_trait]
pub trait ScannerHandler<SO, SR, RO, RR, IO, IR, BO, BR, AO, AR>: Send + Sync {
    type Error;

    async fn scan(&self, options: SO) -> Result<SR, Self::Error>;

    async fn reset(&self, options: RO) -> Result<RR, Self::Error>;

    async fn import(&self, options: IO) -> Result<IR, Self::Error>;

    async fn balance(&self, options: BO) -> Result<BR, Self::Error>;

    async fn assets(&self, options: AO) -> Result<Vec<AR>, Self::Error>;

    async fn chain_assets(&self, chain_id: u64, options: AO) -> Result<Option<AR>, Self::Error>;
}
