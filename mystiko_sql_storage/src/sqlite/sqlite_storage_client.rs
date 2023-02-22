pub use sqlite_storage_client::*;

pub mod sqlite_storage_client{
  use mystiko_storage::client::storage_client::StorageClient;
  use mystiko_storage::repository::repository::Repository;
  use crate::sqlite::repository::wallet_repository::WalletRepository;

  pub struct SqliteStorageClient{
    pub url: String,
  }

  impl StorageClient for SqliteStorageClient{
    fn get_wallet_repository(&self)->Box<dyn Repository>{
      return Box::new(WalletRepository{url: String::from(&self.url)});
    }
  }
}