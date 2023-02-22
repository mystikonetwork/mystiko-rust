use mystiko_sql_storage::*;
use mystiko_storage::factory::storage_factory::StorageFactory;
use crate::sqlite::sqlite_storage_factory::SqliteStorageFactory;
fn main(){
  let url = "/tmp/diesel_test.sqlite";
  let s = SqliteStorageFactory::create_storage_client(String::from(url));
  s.get_wallet_repository().find_by_id(String::from("1"));
}