use crate::client::storage_client::*;

pub trait StorageFactory {
  fn create_storage_client(url:String)-> Box<dyn StorageClient>;
}
