use super::Repository;
use crate::models::contract::Contract;

pub trait ContractRepository: Repository<Contract> {}