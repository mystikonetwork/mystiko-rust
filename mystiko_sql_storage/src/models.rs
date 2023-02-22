use diesel::prelude::*;

#[derive(Queryable, Clone)]
pub struct Wallet{
  pub id:String,
  pub created_at:String,
  pub updated_at:String,
  pub encrypted_master_seed:String,
  pub hashed_password:String,
  pub account_nonce:i32,
}

use mystiko_storage::Model;
impl Model for Wallet{}

