mod handler;

pub use handler::*;

use async_trait::async_trait;
use bip39::Mnemonic;

#[async_trait]
pub trait WalletHandler<W, C>: Send + Sync {
    type Error;

    async fn current(&self) -> Result<Option<W>, Self::Error>;

    async fn check_current(&self) -> Result<W, Self::Error>;

    async fn create(&self, options: &C) -> Result<W, Self::Error>;

    async fn check_password(&self, password: &str) -> Result<W, Self::Error>;

    async fn update_password(&self, old_password: &str, new_password: &str) -> Result<W, Self::Error>;

    async fn export_mnemonic(&self, password: &str) -> Result<Mnemonic, Self::Error>;

    async fn export_mnemonic_phrase(&self, password: &str) -> Result<String, Self::Error>;
}
