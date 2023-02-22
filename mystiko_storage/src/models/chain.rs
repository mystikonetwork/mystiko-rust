use super::Model;

pub struct Chain {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub chain_id: i32,
    pub name: String,
    pub name_override: i32,
    pub providers: String,
    pub provider_override: i32,
    pub event_filter_size: i32,
    pub synced_block_number: i32,
}

impl Model for Chain {}

mod tests {
    #[test]
    fn test_chain_create() {
        use super::Chain;
        let test_chain = Chain {
            id: String::from("i"),
            created_at: String::from("cca"),
            updated_at: String::from("cua"),
            chain_id: 1,
            name: String::from("n"),
            name_override: 1,
            providers: String::from("pro"),
            provider_override: 1,
            event_filter_size: 1000,
            synced_block_number: 10000
        };
        assert_eq!(test_chain.event_filter_size, 1000);
        assert_eq!(test_chain.updated_at, String::from("cua"));
    }
}