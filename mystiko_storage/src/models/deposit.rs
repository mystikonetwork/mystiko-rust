use super::Model;

pub struct Deposit {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub chain_id: i32,
    pub contract_address: String,
    pub pool_address: String,
    pub commitment_hash: String,
    pub hash_k: String,
    pub random_s: String,
    pub encrypted_note: String,
    pub asset_symbol: String,
    pub asset_decimals: i32,
    pub asset_address: String,
    pub bridge_type: String,
    pub amount: String,
    pub rollup_fee_amount: String,
    pub bridge_fee_amount: String,
    pub bridge_fee_asset_address: String,
    pub executor_fee_amount: String,
    pub executor_fee_asset_address: String,
    pub service_fee_amount: String,
    pub shielded_recipient_address: String,
    pub status: String,
    pub errormessage: String,
    pub wallet: String,
    pub dst_chain_id: i32,
    pub dst_chain_contract_address: String,
    pub dst_pool_address: String,
    pub asset_approve_transaction_hash: String,
    pub transaction_hash: String,
    pub relay_transaction_hash: String,
    pub rollup_transaction_hash: String,
}

impl Model for Deposit {}

mod tests {
    #[test]
    fn test_deposit_create() {
        use super::Deposit;
        let test_deposit = Deposit {
            id: String::from("i"),
            created_at: String::from("cca"),
            updated_at: String::from("cua"),
            chain_id: 1,
            contract_address: String::from("cona"),
            pool_address: String::from("comh"),
            commitment_hash: String::from("ch"),
            hash_k: String::from("hk"),
            random_s: String::from("rs"),
            encrypted_note: String::from("dadqwcqcq"),
            asset_symbol: String::from("assy"),
            asset_decimals: 18,
            asset_address: String::from("0x1346"),
            bridge_type: String::from("bt"),
            amount: String::from("amount"),
            rollup_fee_amount: String::from("1.2233"),
            bridge_fee_amount: String::from("bfa"),
            bridge_fee_asset_address: String::from("bfaa"),
            executor_fee_amount: String::from("efa"),
            executor_fee_asset_address: String::from("efaa"),
            service_fee_amount: String::from("sfa"),
            shielded_recipient_address: String::from("sra"),
            status: String::from("st"),
            errormessage: String::from("em"),
            wallet: String::from("wa"),
            dst_chain_id: 2,
            dst_chain_contract_address: String::from("dcca"),
            dst_pool_address: String::from("dpa"),
            asset_approve_transaction_hash: String::from("aath"),
            transaction_hash: String::from("th"),
            relay_transaction_hash: String::from("rth"),
            rollup_transaction_hash: String::from("rth")
        };
        assert_eq!(test_deposit.asset_approve_transaction_hash, String::from("aath"));
        assert_eq!(test_deposit.dst_chain_id, 2);
    }
}