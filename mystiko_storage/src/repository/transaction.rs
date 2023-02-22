use super::Repository;
use crate::models::transaction::Transaction;

pub trait TransactionRepository: Repository<Transaction> {}