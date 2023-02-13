use std::fmt::Debug;
use std::fs::{File, read_to_string};
use std::io::Read;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::{from_str, json};
use validator::Validate;
use crate::common::validate_object;

pub trait RawConfigTrait {
    fn validate(&self);
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
        where T: DeserializeOwned + Serialize + RawConfigTrait
    {
        plain.validate();
        plain
    }

    pub async fn create_from_file<T>(json_file: &str) -> T
        where T: DeserializeOwned + Serialize + RawConfigTrait
    {
        let mut file = File::open(json_file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let object: T = from_str(&contents).unwrap();
        RawConfig::create_from_object::<T>(object).await
    }
}

#[cfg(test)]
mod tests {}
