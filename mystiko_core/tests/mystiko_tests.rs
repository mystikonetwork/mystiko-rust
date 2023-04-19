use crate::common::create_database;
use mockito::Server;
use mystiko_core::mystiko::{Mystiko, MystikoOptions};
use mystiko_ethers::provider::factory::DefaultProviderFactory;

mod common;

#[tokio::test]
async fn test_create_with_config_file() {
    let database = create_database().await;
    let mystiko_options = MystikoOptions::builder()
        .config_file_path(String::from("tests/files/mystiko/config.json"))
        .build();
    let mystiko = Mystiko::new(database, Some(mystiko_options)).await.unwrap();
    assert_eq!(mystiko.config.version(), "0.1.0");
}

#[tokio::test]
async fn test_create_with_config_options() {
    let mut server = Server::new_async().await;
    let path = server
        .mock("GET", "/config/staging/testnet/8de5858/config.json")
        .with_status(200)
        .with_body("{\"version\": \"0.2.0\"}")
        .create_async()
        .await;
    let database = create_database().await;
    let mystiko_options = MystikoOptions::builder()
        .is_testnet(true)
        .is_staging(true)
        .config_remote_base_url(format!("{}/config", server.url()))
        .config_git_revision(String::from("8de5858"))
        .build();
    let mystiko = Mystiko::new(database, Some(mystiko_options)).await.unwrap();
    path.assert_async().await;
    assert_eq!(mystiko.config.version(), "0.2.0");
}

#[tokio::test]
async fn test_crete_with_provider_factory() {
    let database = create_database().await;
    let mystiko_options = MystikoOptions::builder()
        .config_file_path(String::from("tests/files/mystiko/config.json"))
        .provider_factory(Box::new(DefaultProviderFactory::new()))
        .build();
    let mystiko = Mystiko::new(database, Some(mystiko_options)).await.unwrap();
    assert_eq!(mystiko.config.version(), "0.1.0");
}
