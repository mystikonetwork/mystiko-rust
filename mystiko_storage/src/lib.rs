use crate::config::StorageConfig;
use crate::repository::account::AccountRepository;
use crate::repository::chain::ChainRepository;
use crate::repository::commitment::CommitmentRepository;
use crate::repository::contract::ContractRepository;
use crate::repository::deposit::DepositRepository;
use crate::repository::nullifier::NullifierRepository;
use crate::repository::transaction::TransactionRepository;
use crate::repository::wallet::WalletRepository;

pub mod config;
pub mod filter;
pub mod models;
pub mod repository;

pub trait MystikoStorage {
    // fn new(&self, storage_config: Box<dyn StorageConfig>) -> Box<dyn MystikoStorage>;
    // fn get_storage_config(&self) -> &Box<dyn StorageConfig>;
    fn accounts(&self) -> Box<dyn AccountRepository>;
    fn chains(&self) -> Box<dyn ChainRepository>;
    fn commitments(&self) -> Box<dyn CommitmentRepository>;
    fn contracts(&self) -> Box<dyn ContractRepository>;
    fn deposits(&self) -> Box<dyn DepositRepository>;
    fn nullifiers(&self) -> Box<dyn NullifierRepository>;
    fn transactions(&self) -> Box<dyn TransactionRepository>;
    fn wallets(&self) -> Box<dyn WalletRepository>;
}


#[test]
    fn it_works() {
        struct TestBorw<'a>{
            pub connection: &'a String
        }
        impl <'a> TestBorw<'a> {
            fn test1(&self) -> &String {
                self.connection
            }
            fn test2(&self) {
                println!("conn is {}", self.connection);
            }
        }
        let b = TestBorw{connection: &String::from("conn")};
        b.test1();
        b.test2();
        assert_eq!(*b.test1(), String::from("hello"));
    }