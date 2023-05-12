use crate::common::error::{Result, RollerError};
use dotenv::dotenv;
use std::env;

const EVN_ROLLER_PRIVATE_KEY: &str = "ROLLER_PRIVATE_KEY";
const EVN_X_SCAN_API_KEY: &str = "X_SCAN_API_KEY";
const EVN_COIN_MARKET_CAP_API_KEY: &str = "COIN_MARKET_CAP_API_KEY";
const EVN_ROLLER_HOME_PATH: &str = "ROLLER_HOME_PATH";

pub fn load_roller_private_key() -> Result<String> {
    dotenv().ok();
    match env::var(EVN_ROLLER_PRIVATE_KEY) {
        Ok(value) => Ok(value),
        Err(_) => Err(RollerError::EnvNotConfig(EVN_ROLLER_PRIVATE_KEY.to_string())),
    }
}

pub fn load_x_scan_api_key() -> Result<String> {
    dotenv().ok();
    match env::var(EVN_X_SCAN_API_KEY) {
        Ok(value) => Ok(value),
        Err(_) => Err(RollerError::EnvNotConfig(EVN_X_SCAN_API_KEY.to_string())),
    }
}

pub fn load_coin_market_api_key() -> Result<String> {
    dotenv().ok();
    match env::var(EVN_COIN_MARKET_CAP_API_KEY) {
        Ok(value) => Ok(value),
        Err(_) => Err(RollerError::EnvNotConfig(EVN_COIN_MARKET_CAP_API_KEY.to_string())),
    }
}

pub fn load_roller_home_path() -> String {
    dotenv().ok();
    match env::var(EVN_ROLLER_HOME_PATH) {
        Ok(value) => value,
        Err(_) => "/home/mystiko-miner/roller".to_string(),
    }
}

pub fn load_roller_config_path() -> String {
    load_roller_home_path() + "/config"
}

pub fn load_roller_db_path() -> String {
    load_roller_home_path() + "/data"
}

pub fn load_roller_circuits_path() -> String {
    load_roller_home_path() + "/circuits"
}
