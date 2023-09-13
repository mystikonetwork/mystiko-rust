use mystiko_lib::wallet::create;
use mystiko_lib::{initialize, is_initialized};
use mystiko_protos::common::v1::ConfigOptions;
use mystiko_protos::core::handler::v1::CreateWalletOptions;
use mystiko_protos::core::v1::MystikoOptions;
use prost::Message;

#[test]
fn test_create() {
    let options = MystikoOptions::builder()
        .config_options(
            ConfigOptions::builder()
                .file_path(String::from("tests/files/config.json"))
                .is_testnet(true)
                .build(),
        )
        .build();
    let result = initialize(&options.encode_to_vec());
    assert!(result.is_ok());
    assert!(is_initialized());

    let options = CreateWalletOptions::builder()
        .password("PASSword_1!".to_string())
        .build();
    let result = create(&options.encode_to_vec());
    assert!(result.is_ok());
}
