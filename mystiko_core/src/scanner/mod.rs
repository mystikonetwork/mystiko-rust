mod error;
mod handler;

pub use error::*;
pub use handler::*;

use async_trait::async_trait;

#[async_trait]
pub trait ScannerHandler<SO, SR, RO, RR, BO, BR>: Send + Sync {
    type Error;

    async fn scan(&self, options: SO) -> Result<SR, Self::Error>;

    async fn reset(&self, options: RO) -> Result<RR, Self::Error>;

    async fn balance(&self, options: BO) -> Result<BR, Self::Error>;
}
