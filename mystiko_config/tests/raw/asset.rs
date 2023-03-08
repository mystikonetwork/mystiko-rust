use mystiko_config::common::AssetType;
use mystiko_config::raw::asset::RawAssetConfig;
use mystiko_config::raw::base::{RawConfig, Validator};

async fn default_config() -> RawAssetConfig {
    RawConfig::create_from_object::<RawAssetConfig>(
        RawAssetConfig::new(
            AssetType::Erc20,
            String::from("MTT"),
            16,
            "0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a".to_string(),
            vec![
                String::from("10000000000000000"),
                String::from("100000000000000000"),
            ],
        )
    ).await
}

#[tokio::test]
#[should_panic]
async fn test_invalid_asset_symbol() {
    let mut config = default_config().await;
    config.asset_symbol = "".to_string();
    config.validation();
}

#[tokio::test]
#[should_panic]
async fn test_invalid_asset_address_0() {
    let mut config = default_config().await;
    config.asset_address = String::from("");
    config.validation();
}

#[tokio::test]
#[should_panic]
async fn test_invalid_asset_address_1() {
    let mut config = default_config().await;
    config.asset_address = String::from("0xdeadbeef");
    config.validation();
}

#[tokio::test]
#[should_panic]
async fn test_invalid_recommended_amounts_0() {
    let mut config = default_config().await;
    config.recommended_amounts = vec![String::from("")];
    config.validation();
}

#[tokio::test]
#[should_panic]
async fn test_invalid_recommended_amounts_1() {
    let mut config = default_config().await;
    config.recommended_amounts = vec![String::from("abcd")];
    config.validation();
}

#[tokio::test]
#[should_panic]
async fn test_invalid_recommended_amounts_2() {
    let mut config = default_config().await;
    config.recommended_amounts = vec![String::from("1"), String::from("1")];
    config.validation();
}

#[tokio::test]
async fn test_import_valid_json_file() {
    let file_config =
        RawConfig::create_from_file::<RawAssetConfig>("tests/files/asset.valid.json").await;
    assert_eq!(file_config, default_config().await);
}

#[tokio::test]
#[should_panic]
async fn test_import_invalid_json_file() {
    let _file_config =
        RawConfig::create_from_file::<RawAssetConfig>("tests/files/asset.invalid.json").await;
}