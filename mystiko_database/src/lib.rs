use mystiko_storage::MystikoStorage;
pub struct MystikoDatabase {
    storage: Box<dyn MystikoStorage>
}

impl MystikoDatabase {
    pub fn init(storage: Box<dyn MystikoStorage>) -> MystikoDatabase {
        Self {
            storage,
        }
    }

    pub fn get_storage(self) -> Box<dyn MystikoStorage> {
        self.storage
    }

}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        use mystiko_storage::filter::Filter;
        use crate::MystikoDatabase;
        use mystiko_in_memory_storage::config::InMemoryStorageConfig;
        use mystiko_in_memory_storage::MystikoInmemoryStorage;
        use mystiko_in_memory_storage::repository::account::AccountRepositoryInMemoryImpl;
        use mystiko_in_memory_storage::C;
        let storage_config = Box::new(InMemoryStorageConfig::new(String::from("http://localhost:8080")));
        let database_url = storage_config.database_url;
        let connection: C = C {};
        let storage = Box::new(MystikoInmemoryStorage {
            storage_config,
            connection,
        });

        let db2 = MystikoDatabase {
            storage
        };
        let account = db2.get_storage().find_one(Filter::new()).unwrap();
        dbg!(&account);
        assert_eq!(account.public_key, "pk");
        assert_eq!(account.encrypted_secret_key, "esk");
    }
}
