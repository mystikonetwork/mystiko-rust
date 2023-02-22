use super::Repository;
use crate::models::chain::Chain;

pub trait ChainRepository: Repository<Chain> {}
