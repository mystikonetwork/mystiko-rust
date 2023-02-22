use super::Repository;
use crate::models::wallet::Wallet;

pub trait WalletRepository: Repository<Wallet> {}