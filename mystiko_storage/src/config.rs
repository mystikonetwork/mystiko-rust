pub trait StorageConfig {
    fn get_database_url(&self) -> &String;
}
