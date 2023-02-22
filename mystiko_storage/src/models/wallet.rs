use super::Model;

pub struct Wallet {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub encrypted_master_seed: String,
    pub hashed_password: String,
    pub account_nonce: i32,
}

impl Model for Wallet {}

mod tests {
    #[test]
    fn test_wallet_create() {
        use super::Wallet;
        let test_wallet = Wallet {
            id: String::from("i"),
            created_at: String::from("wca"),
            updated_at: String::from("wua"),
            encrypted_master_seed: String::from("ems"),
            hashed_password: String::from("hp"),
            account_nonce: 1000,
        };
        assert_eq!(test_wallet.encrypted_master_seed, String::from("ems"));
        assert_eq!(test_wallet.account_nonce, 1000);
    }
}