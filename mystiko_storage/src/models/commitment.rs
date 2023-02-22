use super::Model;

pub struct Commitment {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub chain_id: i32,
    pub contract_address: String,
    pub commitment_hash: String,
    pub asset_symbol: String,
    pub asset_decimals: i32,
    pub asset_address: String,
    pub status: String,
    pub rollup_fee_amount: String,
    pub encrypted_note: String,
    pub leaf_index: String,
    pub amount: String,
    pub serial_number: String,
    pub shielded_address: String,
    pub creation_transaction_hash: String,
    pub spending_transaction_hash: String,
    pub rollup_transaction_hash: String
}

impl Model for Commitment {}

mod tests {
    #[test]
    fn test_commitment_create() {
        use super::Commitment;
        let test_commitment = Commitment {
            id: String::from("i"),
            created_at: String::from("cca"),
            updated_at: String::from("cua"),
            chain_id: 1,
            contract_address: String::from("cona"),
            commitment_hash: String::from("comh"),
            asset_symbol: String::from("assy"),
            asset_decimals: 18,
            asset_address: String::from("0x1346"),
            status: String::from("success"),
            rollup_fee_amount: String::from("1.2233"),
            encrypted_note: String::from("dadqwcqcq"),
            leaf_index: String::from("1"),
            amount: String::from("2.3123"),
            serial_number: String::from("sn"),
            shielded_address: String::from("sa"),
            creation_transaction_hash: String::from("cth"),
            spending_transaction_hash: String::from("sth"),
            rollup_transaction_hash: String::from("rth")
        };
        assert_eq!(test_commitment.creation_transaction_hash, String::from("cth"));
        assert_eq!(test_commitment.asset_decimals, 18);
    }
}