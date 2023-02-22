// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
        name -> Text,
        shielded_address -> Text,
        public_key -> Text,
        encrypted_secret_key -> Text,
        status -> Text,
        scan_size -> Integer,
        wallet -> Nullable<Text>,
    }
}

diesel::table! {
    chains (id) {
        id -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
        chain_id -> Integer,
        name -> Text,
        name_override -> Nullable<Integer>,
        providers -> Text,
        provider_override -> Nullable<Integer>,
        event_filter_size -> Integer,
        synced_block_number -> Integer,
    }
}

diesel::table! {
    commitments (id) {
        id -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
        chain_id -> Integer,
        contract_address -> Text,
        commitment_hash -> Text,
        asset_symbol -> Text,
        asset_decimals -> Integer,
        asset_address -> Nullable<Text>,
        status -> Text,
        rollup_fee_amount -> Nullable<Text>,
        encrypted_note -> Nullable<Text>,
        leaf_index -> Nullable<Text>,
        amount -> Nullable<Text>,
        serial_number -> Nullable<Text>,
        shielded_address -> Nullable<Text>,
        creation_transaction_hash -> Nullable<Text>,
        spending_transaction_hash -> Nullable<Text>,
        rollup_transaction_hash -> Nullable<Text>,
    }
}

diesel::table! {
    contracts (id) {
        id -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        chain_id -> Integer,
        contract_address -> Text,
        disabled -> Integer,
        sync_start -> Text,
        sync_size -> Text,
        synced_block_number -> Text,
        checked_leaf_index -> Nullable<Text>,
    }
}

diesel::table! {
    deposits (id) {
        id -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
        chain_id -> Integer,
        contract_address -> Text,
        pool_address -> Text,
        commitment_hash -> Text,
        hash_k -> Text,
        random_s -> Text,
        encrypted_note -> Text,
        asset_symbol -> Text,
        asset_decimals -> Integer,
        asset_address -> Nullable<Text>,
        bridge_type -> Text,
        amount -> Text,
        rollup_fee_amount -> Text,
        bridge_fee_amount -> Text,
        bridge_fee_asset_address -> Nullable<Text>,
        executor_fee_amount -> Text,
        executor_fee_asset_address -> Nullable<Text>,
        service_fee_amount -> Text,
        shielded_recipient_address -> Text,
        status -> Text,
        errormessage -> Nullable<Text>,
        wallet -> Nullable<Text>,
        dst_chain_id -> Integer,
        dst_chain_contract_address -> Text,
        dst_pool_address -> Text,
        asset_approve_transaction_hash -> Nullable<Text>,
        transaction_hash -> Nullable<Text>,
        relay_transaction_hash -> Nullable<Text>,
        rollup_transaction_hash -> Nullable<Text>,
    }
}

diesel::table! {
    nullifiers (id) {
        id -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
        chain_id -> Integer,
        contract_address -> Text,
        serial_number -> Text,
        transaction_hash -> Text,
    }
}

diesel::table! {
    transactions (id) {
        id -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
        chain_id -> Integer,
        contract_address -> Text,
        asset_symbol -> Text,
        asset_decimals -> Integer,
        asset_address -> Nullable<Text>,
        proof -> Nullable<Text>,
        root_hash -> Nullable<Text>,
        input_commitments -> Text,
        output_commitments -> Nullable<Text>,
        serial_numbers -> Nullable<Text>,
        signature_public_key -> Nullable<Text>,
        signature_public_key_hashes -> Nullable<Text>,
        amount -> Text,
        public_amount -> Text,
        rollup_fee_amount -> Text,
        gas_relayer_fee_amount -> Text,
        shielded_address -> Nullable<Text>,
        public_address -> Nullable<Text>,
        gas_relayer_address -> Nullable<Text>,
        signature -> Nullable<Text>,
        random_auditing_public_key -> Nullable<Text>,
        encrypted_auditor_notes -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Text,
        status -> Text,
        error_message -> Nullable<Text>,
        transaction_hash -> Nullable<Text>,
        wallet -> Nullable<Text>,
    }
}

diesel::table! {
    wallets (id) {
        id -> Text,
        created_at -> Text,
        updated_at -> Text,
        encrypted_master_seed -> Text,
        hashed_password -> Text,
        account_nonce -> Integer,
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
