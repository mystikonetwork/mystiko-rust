use crate::repository::repository::Repository;

pub trait StorageClient {
  fn get_wallet_repository(&self)->Box<dyn Repository>;
}