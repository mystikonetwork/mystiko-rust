use mystiko_storage::repository::repository::Repository;
use crate::models::*;

pub struct WalletRepository{
  pub url: String,
}

use mystiko_storage::Model;
impl Repository for WalletRepository {
  fn find_by_id(&self, _id: String) -> Box<dyn Model> {
    let conn = &mut establish_connection(&self.url);

    use crate::schema::wallets::dsl::*;

    let result = wallets
        .find(_id)
        .first::<Wallet>(conn)
        .expect("Error loading wallets");
    return Box::new(result.clone());
  }
}


use diesel::prelude::*;

pub fn establish_connection(database_url:&str) -> SqliteConnection {
    SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}