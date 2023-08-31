use mystiko_lib::core::{mystiko_initialize, mystiko_is_initialized};
use mystiko_lib::handler::wallet::wallet_create;
use mystiko_protos::core::handler::v1::CreateWalletOptions;
use mystiko_protos::core::v1::MystikoOptions;
use prost::Message;

#[test]
fn test_wallet_create() {
    let options = MystikoOptions::builder()
        .is_testnet(true)
        .config_file_path(String::from("tests/files/config.json"))
        .build();
    let result = mystiko_initialize(&options.encode_to_vec());
    assert!(result.is_ok());
    assert!(mystiko_is_initialized());

    let options = CreateWalletOptions::builder()
        .password("PASSword_1!".to_string())
        .build();
    let result = wallet_create(&options.encode_to_vec());
    assert!(result.is_ok());
}
