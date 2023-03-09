use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mystiko_config::common::AssetType;
use mystiko_config::raw::asset::RawAssetConfig;
use mystiko_config::raw::base::RawConfig;
use mystiko_config::wrapper::asset::{AssetConfig, MAIN_ASSET_ADDRESS};

async fn default_raw_config() -> RawAssetConfig {
    RawConfig::create_from_file::<RawAssetConfig>("tests/files/asset.valid.json").await.unwrap()
}

async fn default_asset_config() -> AssetConfig {
    AssetConfig::new(default_raw_config().await)
}

async fn default_config() -> (RawAssetConfig, AssetConfig) {
    (default_raw_config().await, default_asset_config().await)
}

lazy_static! {
    static ref CONFIG_CREATER: AsyncOnce<AssetConfig> = AsyncOnce::new(async {
        default_asset_config().await
    });
}

lazy_static! {
    static ref RAW_CONFIG_CREATER: AsyncOnce<RawAssetConfig> = AsyncOnce::new(async {
       default_raw_config().await
    });
}

#[tokio::test]
async fn test_equality() {
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let config = CONFIG_CREATER.get().await;
    assert_eq!(config.asset_address(), raw_config.asset_address);
    assert_eq!(config.asset_type(), &raw_config.asset_type);
    assert_eq!(config.asset_decimals(), raw_config.asset_decimals);
    assert_eq!(config.asset_symbol(), raw_config.asset_symbol);
    let recommended_amounts: Vec<String> =
        config.recommended_amounts().iter().map(|a| a.to_string()).collect();
    assert_eq!(recommended_amounts, raw_config.recommended_amounts);
    assert_eq!(config.recommended_amounts_number(), vec![1f64, 10f64]);
}

#[tokio::test]
#[should_panic(expected = "wrong asset address=0x0000000000000000000000000000000000000000 and type=Erc20")]
async fn test_wrong_address_or_type_0() {
    let mut raw_config = default_raw_config().await;
    raw_config.asset_address = MAIN_ASSET_ADDRESS.to_string();
    raw_config.asset_type = AssetType::Erc20;
    AssetConfig::new(raw_config);
}

#[tokio::test]
#[should_panic(expected = "wrong asset address=0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a and type=Main")]
async fn test_wrong_address_or_type_1() {
    let mut raw_config = default_raw_config().await;
    raw_config.asset_address = String::from("0xEC1d5CfB0bf18925aB722EeeBCB53Dc636834e8a");
    raw_config.asset_type = AssetType::Main;
    AssetConfig::new(raw_config);
}

#[tokio::test]
async fn test_copy() {
    let config = CONFIG_CREATER.get().await;
    assert_eq!(
        &AssetConfig::new(config.base.copy_data()),
        config
    );
}

#[tokio::test]
async fn test_mutate() {
    let (mut raw_config, config) = default_config().await;
    assert_eq!(config.mutate(None), config);
    raw_config.asset_decimals = 6;
    let new_config = config.mutate(Some(raw_config));
    assert_eq!(new_config.asset_decimals(), 6);
}

#[tokio::test]
async fn test_to_json_string() {
    let raw_config = RAW_CONFIG_CREATER.get().await;
    let config = CONFIG_CREATER.get().await;
    let json_string = config.base.to_json_string();
    let loaded_raw_config =
        RawConfig::create_from_json_string::<RawAssetConfig>(json_string.as_str()).await.unwrap();
    assert_eq!(&loaded_raw_config, raw_config);
}

