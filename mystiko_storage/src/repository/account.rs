use super::Repository;
use crate::models::account::Account;

pub trait AccountRepository: Repository<Account> {}
