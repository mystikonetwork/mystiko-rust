use mystiko_storage::factory::storage_factory::StorageFactory;
use mystiko_storage::client::storage_client::StorageClient;
use crate::sqlite::sqlite_storage_client::SqliteStorageClient;

pub struct SqliteStorageFactory;
impl StorageFactory for SqliteStorageFactory {
  fn create_storage_client(url:String)-> Box<dyn StorageClient>{
    return Box::new(SqliteStorageClient{url});
  }
}