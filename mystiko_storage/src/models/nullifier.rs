use super::Model;

pub struct Nullifier {
    pub id: String,
    pub updated_at: String,
    pub created_at: String,
    pub chain_id: i32,
    pub contract_address: String,
    pub serial_number: String,
    pub transaction_hash: String,
}

impl Model for Nullifier {}

mod tests {
    #[test]
    fn test_nullifier_create() {
        use super::Nullifier;
        let test_nullifier = Nullifier {
            id: String::from("i"),
            created_at: String::from("cca"),
            updated_at: String::from("cua"),
            chain_id: 1,
            contract_address: String::from("ca"),
            serial_number: String::from("sn"),
            transaction_hash: String::from("th"),
        };
        assert_eq!(test_nullifier.contract_address, String::from("ca"));
        assert_eq!(test_nullifier.serial_number, String::from("sn"));
    }
}