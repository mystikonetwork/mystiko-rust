mod v1;

pub use v1::*;

use async_trait::async_trait;
use mystiko_protos::storage::v1::QueryFilter;

#[async_trait]
pub trait AccountHandler<A, C, U>: Send + Sync {
    type Error;

    async fn create(&self, options: &C) -> Result<A, Self::Error>;

    async fn find<Q>(&self, filter: Q) -> Result<Vec<A>, Self::Error>
    where
        Q: Into<QueryFilter> + Send + Sync;

    async fn find_all(&self) -> Result<Vec<A>, Self::Error>;

    async fn find_by_id(&self, id: &str) -> Result<Option<A>, Self::Error>;

    async fn find_by_shielded_address(&self, shielded_address: &str) -> Result<Option<A>, Self::Error>;

    async fn find_by_public_key(&self, public_key: &str) -> Result<Option<A>, Self::Error>;

    async fn count<Q>(&self, filter: Q) -> Result<u64, Self::Error>
    where
        Q: Into<QueryFilter> + Send + Sync;

    async fn count_all(&self) -> Result<u64, Self::Error>;

    async fn update_by_id(&self, id: &str, options: &U) -> Result<A, Self::Error>;

    async fn update_by_shielded_address(&self, shielded_address: &str, options: &U) -> Result<A, Self::Error>;

    async fn update_by_public_key(&self, public_key: &str, options: &U) -> Result<A, Self::Error>;

    async fn update_encryption(
        &self,
        old_wallet_password: &str,
        new_wallet_password: &str,
    ) -> Result<Vec<A>, Self::Error>;

    async fn export_secret_key_by_id(&self, wallet_password: &str, id: &str) -> Result<String, Self::Error>;

    async fn export_secret_key_by_public_key(
        &self,
        wallet_password: &str,
        public_key: &str,
    ) -> Result<String, Self::Error>;

    async fn export_secret_key_by_shielded_address(
        &self,
        wallet_password: &str,
        shielded_address: &str,
    ) -> Result<String, Self::Error>;
}
