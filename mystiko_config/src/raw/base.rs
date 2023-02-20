use std::fmt::Debug;
use std::fs::{File};
use std::io::Read;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::{from_str};
use validator::Validate;
use crate::common::validate_object;

pub trait Validator {
    fn validation(&self);
}

#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct RawConfig;

impl RawConfig {
    pub fn validate_object<T>(&self, object: T) where
        T: Validate + Debug
    {
        let result = validate_object(object);
        if result.is_err() {
            panic!("{:?}", result.unwrap_err());
        }
    }

    pub async fn create_from_object<T>(plain: T) -> T
        where T: DeserializeOwned + Serialize + Validator
    {
        plain.validation();
        plain
    }

    pub async fn create_from_file<T>(json_file: &str) -> T
        where T: DeserializeOwned + Serialize + Validator
    {
        let mut file = File::open(json_file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let object: T = from_str(&contents).unwrap();
        RawConfig::create_from_object::<T>(object).await
    }

    pub async fn create_from_json_string<T>(json_str: &str) -> T
        where T: DeserializeOwned + Serialize + Validator
    {
        let object: T = from_str(json_str).unwrap();
        RawConfig::create_from_object::<T>(object).await
    }
}
