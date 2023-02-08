// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
        name -> Varchar,
        shielded_address -> Varchar,
        public_key -> Varchar,
        encrypted_secret_key -> Varchar,
        status -> Varchar,
        scan_size -> Int4,
        wallet -> Nullable<Varchar>,
    }
}

diesel::table! {
    chains (id) {
        id -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
        chain_id -> Int4,
        name -> Varchar,
        name_override -> Nullable<Int4>,
        providers -> Json,
        provider_override -> Nullable<Int4>,
        event_filter_size -> Int4,
        synced_block_number -> Int4,
    }
}

diesel::table! {
    commitments (id) {
        id -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
        chain_id -> Int4,
        contract_address -> Varchar,
        commitment_hash -> Varchar,
        asset_symbol -> Varchar,
        asset_decimals -> Int4,
        asset_address -> Nullable<Varchar>,
        status -> Varchar,
        rollup_fee_amount -> Nullable<Varchar>,
        encrypted_note -> Nullable<Varchar>,
        leaf_index -> Nullable<Varchar>,
        amount -> Nullable<Varchar>,
        serial_number -> Nullable<Varchar>,
        shielded_address -> Nullable<Varchar>,
        creation_transaction_hash -> Nullable<Varchar>,
        spending_transaction_hash -> Nullable<Varchar>,
        rollup_transaction_hash -> Nullable<Varchar>,
    }
}

diesel::table! {
    contracts (id) {
        id -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        chain_id -> Int4,
        contract_address -> Varchar,
        disabled -> Int4,
        sync_start -> Varchar,
        sync_size -> Varchar,
        synced_block_number -> Varchar,
        checked_leaf_index -> Nullable<Varchar>,
    }
}

diesel::table! {
    deposits (id) {
        id -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
        chain_id -> Int4,
        contract_address -> Varchar,
        pool_address -> Varchar,
        commitment_hash -> Varchar,
        hash_k -> Varchar,
        random_s -> Varchar,
        encrypted_note -> Varchar,
        asset_symbol -> Varchar,
        asset_decimals -> Int4,
        asset_address -> Nullable<Varchar>,
        bridge_type -> Varchar,
        amount -> Varchar,
        rollup_fee_amount -> Varchar,
        bridge_fee_amount -> Varchar,
        bridge_fee_asset_address -> Nullable<Varchar>,
        executor_fee_amount -> Varchar,
        executor_fee_asset_address -> Nullable<Varchar>,
        service_fee_amount -> Varchar,
        shielded_recipient_address -> Varchar,
        status -> Varchar,
        errormessage -> Nullable<Varchar>,
        wallet -> Nullable<Varchar>,
        dst_chain_id -> Int4,
        dst_chain_contract_address -> Varchar,
        dst_pool_address -> Varchar,
        asset_approve_transaction_hash -> Nullable<Varchar>,
        transaction_hash -> Nullable<Varchar>,
        relay_transaction_hash -> Nullable<Varchar>,
        rollup_transaction_hash -> Nullable<Varchar>,
    }
}

diesel::table! {
    nullifiers (id) {
        id -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
        chain_id -> Int4,
        contract_address -> Varchar,
        serial_number -> Varchar,
        transaction_hash -> Varchar,
    }
}

diesel::table! {
    transactions (id) {
        id -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
        chain_id -> Int4,
        contract_address -> Varchar,
        asset_symbol -> Varchar,
        asset_decimals -> Int4,
        asset_address -> Nullable<Varchar>,
        proof -> Nullable<Varchar>,
        root_hash -> Nullable<Varchar>,
        input_commitments -> Json,
        output_commitments -> Nullable<Json>,
        serial_numbers -> Nullable<Json>,
        signature_public_key -> Nullable<Varchar>,
        signature_public_key_hashes -> Nullable<Json>,
        amount -> Varchar,
        public_amount -> Varchar,
        rollup_fee_amount -> Varchar,
        gas_relayer_fee_amount -> Varchar,
        shielded_address -> Nullable<Varchar>,
        public_address -> Nullable<Varchar>,
        gas_relayer_address -> Nullable<Varchar>,
        signature -> Nullable<Varchar>,
        random_auditing_public_key -> Nullable<Varchar>,
        encrypted_auditor_notes -> Nullable<Json>,
        #[sql_name = "type"]
        type_ -> Varchar,
        status -> Varchar,
        error_message -> Nullable<Varchar>,
        transaction_hash -> Nullable<Varchar>,
        wallet -> Nullable<Varchar>,
    }
}

diesel::table! {
    wallets (id) {
        id -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
        encrypted_master_seed -> Varchar,
        hashed_password -> Varchar,
        account_nonce -> Int4,
    }
}

diesel::joinable!(accounts -> wallets (wallet));
diesel::joinable!(deposits -> wallets (wallet));
diesel::joinable!(transactions -> wallets (wallet));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    chains,
    commitments,
    contracts,
    deposits,
    nullifiers,
    transactions,
    wallets,
);
