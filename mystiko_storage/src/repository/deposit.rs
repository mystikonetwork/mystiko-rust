use super::Repository;
use crate::models::deposit::Deposit;

pub trait DepositRepository: Repository<Deposit> {}