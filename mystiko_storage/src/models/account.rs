use super::Model;
pub struct Account {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
    pub shielded_address: String,
    pub public_key: String,
    pub encrypted_secret_key: String,
    pub status: String,
    pub scan_size: i32,
    pub wallet: String,
}

impl Model for Account {}

mod tests {
    #[test]
    fn test_account_create() {
        use super::Account;
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
        assert_eq!(test_account.encrypted_secret_key, String::from("esk"));
        assert_eq!(test_account.public_key, String::from("pk"));
    }
}



