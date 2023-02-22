use mystiko_storage::filter::Filter;
use mystiko_storage::models::deposit::Deposit;
use mystiko_storage::repository::deposit::DepositRepository;
use mystiko_storage::repository::Repository;

pub struct DepositRepositoryInMemoryImpl {}

impl DepositRepository for DepositRepositoryInMemoryImpl {}

impl Repository<Deposit> for DepositRepositoryInMemoryImpl {
    fn create(&self, t: Deposit) -> Deposit {
        todo!()
    }

    fn create_all(&self, t: Vec<Deposit>) -> Vec<Deposit> {
        todo!()
    }

    fn find(&self, filter: Filter) -> Option<Vec<Deposit>> {
        todo!()
    }

    fn find_one(&self, filter: Filter) -> Option<Deposit> {
        todo!()
    }

    fn update(&self, t: Deposit) -> Deposit {
        todo!()
    }

    fn delete(&self, t: Deposit) -> Deposit {
        todo!()
    }
}
