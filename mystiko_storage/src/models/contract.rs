use super::Model;

pub struct Contract {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub contract_type: String,
    pub chain_id: i32,
    pub disabled: i32,
    pub sync_start: i32,
    pub sync_size: i32,
    pub synced_block_number: i32,
    pub checked_leaf_index: i32,
}

impl Model for Contract {}

mod tests {
    #[test]
    fn test_contract_create() {
        use super::Contract;
        let test_contract = Contract {
            id: String::from("i"),
            created_at: String::from("cca"),
            updated_at: String::from("cua"),
            chain_id: 1,
            disabled: 1,
            contract_type: String::from("ct"),
            sync_start: 1874621,
            sync_size: 1000,
            synced_block_number: 1233221,
            checked_leaf_index: 2
        };
        assert_eq!(test_contract.contract_type, String::from("ct"));
        assert_eq!(test_contract.disabled, 1);
    }
}