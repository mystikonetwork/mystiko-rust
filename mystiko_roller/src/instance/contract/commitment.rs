use ethers_core::abi::Address;
use mystiko_abi::commitment_pool::CommitmentPool;
use mystiko_ethers::provider::factory::Provider;

struct ContractInstance {
    pool: CommitmentPool<Provider>,
}

impl ContractInstance {
    fn new(pool_address: Address, provider: &Provider) -> Self {
        let pool = CommitmentPool::new(pool_address, provider);
        ContractInstance { pool }
    }
}
