use mystiko_storage::filter::Filter;
use mystiko_storage::models::chain::Chain;
use mystiko_storage::repository::chain::ChainRepository;
use mystiko_storage::repository::Repository;

pub struct ChainRepositoryInMemoryImpl {}

impl ChainRepository for ChainRepositoryInMemoryImpl {}

impl Repository<Chain> for ChainRepositoryInMemoryImpl {
    fn create(&self, t: Chain) -> Chain {
        todo!()
    }

    fn create_all(&self, t: Vec<Chain>) -> Vec<Chain> {
        todo!()
    }

    fn find(&self, filter: Filter) -> Option<Vec<Chain>> {
        todo!()
    }

    fn find_one(&self, filter: Filter) -> Option<Chain> {
        todo!()
    }

    fn update(&self, t: Chain) -> Chain {
        todo!()
    }

    fn delete(&self, t: Chain) -> Chain {
        todo!()
    }
}
