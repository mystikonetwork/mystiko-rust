use super::Model;

pub struct Transaction {
    pub id: String,
    pub updated_at: String,
    pub created_at: String,
    pub chain_id: i32,
    pub contract_address: String,
    pub asset_symbol: String,
    pub asset_decimals: i32,
    pub asset_address: String,
    pub proof: String,
    pub root_hash: String,
    pub input_commitments: String,
    pub output_commitments: String,
    pub serial_numbers: String,
    pub signature_public_key: String,
    pub signature_public_key_hashes: String,
    pub amount: String,
    pub public_amount: String,
    pub rollup_fee_amount: String,
    pub gas_relayer_fee_amount: String,
    pub shielded_address: String,
    pub public_address: String,
    pub gas_relayer_address: String,
    pub signature: String,
    pub random_auditing_public_key: String,
    pub encrypted_auditor_notes: String,
    pub transaction_type: String,
    pub status: String,
    pub error_message: String,
    pub transaction_hash: String,
    pub wallet: String,
}

impl Model for Transaction {}

mod tests {
    #[test]
    fn test_transaction_create() {
        use super::Transaction;
        let test_transaction = Transaction {
            id: String::from("i"),
            created_at: String::from("cca"),
            updated_at: String::from("cua"),
            chain_id: 1,
            contract_address: String::from("cona"),
            asset_symbol: String::from("assy"),
            asset_decimals: 18,
            asset_address: String::from("0x1346"),
            proof: String::from("proof"),
            root_hash: String::from("rh"),
            input_commitments: String::from("ic"),
            output_commitments: String::from("oc"),
            serial_numbers: String::from("sn"),
            signature_public_key: String::from("spk"),
            signature_public_key_hashes: String::from("spkh"),
            amount: String::from("amount"),
            public_amount: String::from("pa"),
            rollup_fee_amount: String::from("1.2233"),
            gas_relayer_fee_amount: String::from("grfa"),
            shielded_address: String::from("sa"),
            public_address: String::from("pa"),
            gas_relayer_address: String::from("gra"),
            signature: String::from("signature"),
            random_auditing_public_key: String::from("rapk"),
            encrypted_auditor_notes: String::from("ean"),
            transaction_type: String::from("tt"),
            status: String::from("st"),
            error_message: String::from("em"),
            wallet: String::from("wa"),
            transaction_hash: String::from("th")
        };
        assert_eq!(test_transaction.signature_public_key_hashes, String::from("spkh"));
        assert_eq!(test_transaction.proof, String::from("proof"));
    }
}