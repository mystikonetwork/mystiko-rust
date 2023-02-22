use mystiko_storage::filter::Filter;
use mystiko_storage::models::contract::Contract;
use mystiko_storage::repository::contract::ContractRepository;
use mystiko_storage::repository::Repository;

pub struct ContractRepositoryInMemoryImpl {}

impl ContractRepository for ContractRepositoryInMemoryImpl {}

impl Repository<Contract> for ContractRepositoryInMemoryImpl {
    fn create(&self, t: Contract) -> Contract {
        todo!()
    }

    fn create_all(&self, t: Vec<Contract>) -> Vec<Contract> {
        todo!()
    }

    fn find(&self, filter: Filter) -> Option<Vec<Contract>> {
        todo!()
    }

    fn find_one(&self, filter: Filter) -> Option<Contract> {
        todo!()
    }

    fn update(&self, t: Contract) -> Contract {
        todo!()
    }

    fn delete(&self, t: Contract) -> Contract {
        todo!()
    }
}
