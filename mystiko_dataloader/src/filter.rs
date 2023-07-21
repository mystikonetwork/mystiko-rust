use mystiko_config::wrapper::contract::ContractConfig;
use std::fmt::Debug;

pub trait ContractFilter: Debug + Send + Sync {
    fn filter(&self, chain_id: u64, contract_config: &ContractConfig) -> bool;
}
