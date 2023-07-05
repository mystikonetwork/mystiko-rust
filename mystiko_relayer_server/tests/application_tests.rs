mod common;

use crate::common::{TESTNET_CONFIG_PATH, TEST_MYSTIKO_CONFIG_PATH, TEST_RELAYER_CONFIG_PATH};
use mystiko_relayer_server::application::{Application, ApplicationOptions};
use mystiko_relayer_server::common::AppStateOptions;
use std::fs::remove_file;

#[actix_rt::test]
async fn test_create_application() {
    let app_state_options = AppStateOptions::builder()
        .server_config_path(TESTNET_CONFIG_PATH)
        .relayer_config_path(Some(TEST_RELAYER_CONFIG_PATH))
        .mystiko_config_path(Some(TEST_MYSTIKO_CONFIG_PATH))
        .log_level("debug")
        .build();
    let options = ApplicationOptions::builder()
        .host("127.0.0.1")
        .port(8081)
        .array_queue_capacity(10)
        .app_state_options(app_state_options)
        .build();
    let app = Application::new(options).await;
    assert!(app.is_ok());
    // delete db.sqlite
    remove_file("./tests/files/db.sqlite").unwrap();
}
