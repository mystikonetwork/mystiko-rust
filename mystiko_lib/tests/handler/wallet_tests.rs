use crate::setup;
use mystiko_lib::wallet::create;
use mystiko_protos::core::handler::v1::CreateWalletOptions;
use prost::Message;
use serial_test::serial;

#[test]
#[serial]
fn test_create() {
    assert!(setup(false).is_ok());
    let options = CreateWalletOptions::builder()
        .password("PASSword_1!".to_string())
        .build();
    let result = create(&options.encode_to_vec());
    assert!(result.is_ok());
}
