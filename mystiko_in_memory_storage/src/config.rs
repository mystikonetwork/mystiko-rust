use mystiko_storage::config::StorageConfig;

pub struct InMemoryStorageConfig {
    pub database_url: String,
}

impl InMemoryStorageConfig {
    pub fn new(database_url: String) -> InMemoryStorageConfig {
        InMemoryStorageConfig {
            database_url
        }
    }
}

impl StorageConfig for InMemoryStorageConfig {
    fn get_database_url(&self) -> &String {
        &self.database_url
    }
}
