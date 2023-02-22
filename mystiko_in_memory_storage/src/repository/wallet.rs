use mystiko_storage::filter::Filter;
use mystiko_storage::models::wallet::Wallet;
use mystiko_storage::repository::wallet::WalletRepository;
use mystiko_storage::repository::Repository;

pub struct WalletRepositoryInMemoryImpl {}

impl WalletRepository for WalletRepositoryInMemoryImpl {}

impl Repository<Wallet> for WalletRepositoryInMemoryImpl {
    fn create(&self, t: Wallet) -> Wallet {
        todo!()
    }

    fn create_all(&self, t: Vec<Wallet>) -> Vec<Wallet> {
        todo!()
    }

    fn find(&self, filter: Filter) -> Option<Vec<Wallet>> {
        todo!()
    }

    fn find_one(&self, filter: Filter) -> Option<Wallet> {
        todo!()
    }

    fn update(&self, t: Wallet) -> Wallet {
        todo!()
    }

    fn delete(&self, t: Wallet) -> Wallet {
        todo!()
    }
}
