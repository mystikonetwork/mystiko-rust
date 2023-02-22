use mystiko_storage::filter::Filter;
use mystiko_storage::models::commitment::Commitment;
use mystiko_storage::repository::commitment::CommitmentRepository;
use mystiko_storage::repository::Repository;

pub struct CommitmentRepositoryInMemoryImpl {}

impl CommitmentRepository for CommitmentRepositoryInMemoryImpl {}

impl Repository<Commitment> for CommitmentRepositoryInMemoryImpl {
    fn create(&self, t: Commitment) -> Commitment {
        todo!()
    }

    fn create_all(&self, t: Vec<Commitment>) -> Vec<Commitment> {
        todo!()
    }

    fn find(&self, filter: Filter) -> Option<Vec<Commitment>> {
        todo!()
    }

    fn find_one(&self, filter: Filter) -> Option<Commitment> {
        todo!()
    }

    fn update(&self, t: Commitment) -> Commitment {
        todo!()
    }

    fn delete(&self, t: Commitment) -> Commitment {
        todo!()
    }
}
