use mystiko_storage::filter::Filter;
use mystiko_storage::models::transaction::Transaction;
use mystiko_storage::repository::transaction::TransactionRepository;
use mystiko_storage::repository::Repository;

pub struct TransactionRepositoryInMemoryImpl {}

impl TransactionRepository for TransactionRepositoryInMemoryImpl {}

impl Repository<Transaction> for TransactionRepositoryInMemoryImpl {
    fn create(&self, t: Transaction) -> Transaction {
        todo!()
    }

    fn create_all(&self, t: Vec<Transaction>) -> Vec<Transaction> {
        todo!()
    }

    fn find(&self, filter: Filter) -> Option<Vec<Transaction>> {
        todo!()
    }

    fn find_one(&self, filter: Filter) -> Option<Transaction> {
        todo!()
    }

    fn update(&self, t: Transaction) -> Transaction {
        todo!()
    }

    fn delete(&self, t: Transaction) -> Transaction {
        todo!()
    }
}
