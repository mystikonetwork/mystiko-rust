use anyhow::bail;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use validator::Validate;

pub trait Validator {
    fn validation(&self) -> Result<(), anyhow::Error>;
}

#[derive(Validate, Deserialize, Serialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct RawConfig;

impl RawConfig {
    pub fn validate_object<T>(&self, object: &T) -> anyhow::Result<()>
    where
        T: Validate + Debug,
    {
        let validate = object.validate();
        match validate {
            Ok(_) => Ok(()),
            Err(validation) => bail!(
                "failed to validate config object: \n {:?}",
                validation.errors()
            ),
        }
    }

    pub fn from_object<T>(plain: T) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned + Serialize + Validator + Debug,
    {
        match plain.validation() {
            Ok(_) => Ok(plain),
            Err(err) => Err(err),
        }
    }

    pub fn from_json_string<T>(json_str: &str) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned + Serialize + Validator + Debug,
    {
        let object: T = from_str(json_str).unwrap();
        RawConfig::from_object::<T>(object)
    }

    pub async fn from_file<T>(json_file: &str) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned + Serialize + Validator + Debug,
    {
        let mut file = File::open(json_file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let result: T = from_str(&contents)?;
        RawConfig::from_object::<T>(result)
    }
}

impl Validator for RawConfig {
    fn validation(&self) -> Result<(), anyhow::Error> {
        self.validate_object(self)
    }
}
