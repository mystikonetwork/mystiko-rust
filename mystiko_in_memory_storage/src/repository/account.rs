use mystiko_storage::filter::Filter;
use mystiko_storage::models::account::Account;
use mystiko_storage::repository::account::AccountRepository;
use mystiko_storage::repository::Repository;
use crate::C;

pub struct AccountRepositoryInMemoryImpl<'a> {
    pub connection: &'a C,
}

impl <'a> AccountRepositoryInMemoryImpl<'a> {
    // pub fn new(connection: &'a C) -> &'a AccountRepositoryInMemoryImpl {
    //     let a = &AccountRepositoryInMemoryImpl { connection };
    //     a
    // }
}

impl <'a> AccountRepository for AccountRepositoryInMemoryImpl<'a> {
}

impl <'a> Repository<Account> for AccountRepositoryInMemoryImpl<'a> {
    fn create(&self, t: Account) -> Account {
        // use connection to save and update database
        todo!()
    }

    fn create_all(&self, t: Vec<Account>) -> Vec<Account> {
        // use connection to save and update database
        todo!()
    }

    fn find(&self, filter: Filter) -> Option<Vec<Account>> {
        // use connection to save and update database
        todo!()
    }

    fn find_one(&self, filter: Filter) -> Option<Account> {
        // use connection to save and update database
        let test_account = Account {
            id: String::from("i"),
            created_at: String::from("ca"),
            updated_at: String::from("ua"),
            name: String::from("n"),
            shielded_address: String::from("sa"),
            public_key: String::from("pk"),
            encrypted_secret_key: String::from("esk"),
            status: String::from("s"),
            scan_size: 1,
            wallet: String::from("w"),
        };
        Some(test_account)
    }

    fn update(&self, t: Account) -> Account {
        // use connection to save and update database
        todo!()
    }

    fn delete(&self, t: Account) -> Account {
        // use connection to save and update database
        todo!()
    }
}
