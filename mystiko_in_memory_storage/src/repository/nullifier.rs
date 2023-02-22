use mystiko_storage::filter::Filter;
use mystiko_storage::models::nullifier::Nullifier;
use mystiko_storage::repository::nullifier::NullifierRepository;
use mystiko_storage::repository::Repository;

pub struct NullifierRepositoryInMemoryImpl {}

impl NullifierRepository for NullifierRepositoryInMemoryImpl {}

impl Repository<Nullifier> for NullifierRepositoryInMemoryImpl {
    fn create(&self, t: Nullifier) -> Nullifier {
        todo!()
    }

    fn create_all(&self, t: Vec<Nullifier>) -> Vec<Nullifier> {
        todo!()
    }

    fn find(&self, filter: Filter) -> Option<Vec<Nullifier>> {
        todo!()
    }

    fn find_one(&self, filter: Filter) -> Option<Nullifier> {
        todo!()
    }

    fn update(&self, t: Nullifier) -> Nullifier {
        todo!()
    }

    fn delete(&self, t: Nullifier) -> Nullifier {
        todo!()
    }
}
