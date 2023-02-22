pub mod config;
pub mod repository;
use crate::repository::account::AccountRepositoryInMemoryImpl;
use crate::repository::chain::ChainRepositoryInMemoryImpl;
use crate::repository::commitment::CommitmentRepositoryInMemoryImpl;
use crate::repository::contract::ContractRepositoryInMemoryImpl;
use crate::repository::deposit::DepositRepositoryInMemoryImpl;
use crate::repository::nullifier::NullifierRepositoryInMemoryImpl;
use crate::repository::transaction::TransactionRepositoryInMemoryImpl;
use crate::repository::wallet::WalletRepositoryInMemoryImpl;
use mystiko_storage::config::StorageConfig;
use mystiko_storage::repository::Repository;
use mystiko_storage::repository::account::AccountRepository;
use mystiko_storage::repository::chain::ChainRepository;
use mystiko_storage::repository::commitment::CommitmentRepository;
use mystiko_storage::repository::contract::ContractRepository;
use mystiko_storage::repository::deposit::DepositRepository;
use mystiko_storage::repository::nullifier::NullifierRepository;
use mystiko_storage::repository::transaction::TransactionRepository;
use mystiko_storage::repository::wallet::WalletRepository;
use mystiko_storage::MystikoStorage;

// example struct, will remove in next commit
pub struct C {

}
pub struct MystikoInmemoryStorage {
    storage_config: Box<dyn StorageConfig>,
    connection: C,
}

impl MystikoInmemoryStorage {
    pub fn get_connection(&self, storage_config: & Box<dyn StorageConfig>) -> C {
        // 使用database_url建立连接
        let database_url = storage_config.get_database_url();
        let connection: C = C {};
        connection
    }
}


impl MystikoStorage for MystikoInmemoryStorage {
    // fn new(&self, storage_config: Box<dyn StorageConfig>) -> Box<dyn MystikoStorage> {
        // let connection: C = self.get_connection(&storage_config);
        // Box::new(MystikoInmemoryStorage {
        //     storage_config,
        //     connection,
        // })
    // }
    
    fn accounts(&self) -> Box<dyn AccountRepository> {
        Box::new(AccountRepositoryInMemoryImpl{connection: &self.connection})
    }

    fn chains(&self) -> Box<dyn ChainRepository> {
        Box::new(ChainRepositoryInMemoryImpl {})
    }

    fn commitments(&self) -> Box<dyn CommitmentRepository> {
        Box::new(CommitmentRepositoryInMemoryImpl {})
    }

    fn contracts(&self) -> Box<dyn ContractRepository> {
        Box::new(ContractRepositoryInMemoryImpl {})
    }

    fn deposits(&self) -> Box<dyn DepositRepository> {
        Box::new(DepositRepositoryInMemoryImpl {})
    }

    fn nullifiers(&self) -> Box<dyn NullifierRepository> {
        Box::new(NullifierRepositoryInMemoryImpl {})
    }

    fn transactions(&self) -> Box<dyn TransactionRepository> {
        Box::new(TransactionRepositoryInMemoryImpl {})
    }

    fn wallets(&self) -> Box<dyn WalletRepository> {
        Box::new(WalletRepositoryInMemoryImpl {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        struct TestBorw{
            pub connection: String
        }
        impl TestBorw {
            fn test1(self) -> String {
                self.connection
            }
            fn test2(self) {
                println!("conn is {}", self.connection);
            }
        }
        let b = TestBorw{connection: String::from("conn")};
        // b.test1();
        // b.test2();
        // assert_eq!(b.test1(), String::from("hello"));
    }
}
