use crate::common::validate_object;
use crate::errors::ValidationError;
use flamer::flame;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use validator::Validate;

pub trait Validator {
    fn validation(&self) -> Result<(), ValidationError>;
}

#[derive(Validate, Clone, Debug, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct RawConfig;

impl RawConfig {
    pub fn validate_object<T>(&self, object: T) -> Result<(), ValidationError>
    where
        T: Validate + Debug,
    {
        let result = validate_object(object);
        if let Err(errs) = result {
            Err(ValidationError::new(errs))
        } else {
            Ok(())
        }
    }

    pub async fn create_from_object<T>(plain: T) -> Result<T, ValidationError>
    where
        T: DeserializeOwned + Serialize + Validator + Debug,
    {
        match plain.validation() {
            Ok(_) => Ok(plain),
            Err(err) => Err(err),
        }
    }

    #[flame]
    pub async fn create_from_file<T>(json_file: &str) -> Result<T, ValidationError>
    where
        T: DeserializeOwned + Serialize + Validator + Debug,
    {
        let mut file = File::open(json_file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let result = from_str(&contents);
        if result.is_err() {
            return Err(ValidationError::new(vec![result.unwrap_err().to_string()]));
        }
        let object: T = result.unwrap();
        RawConfig::create_from_object::<T>(object).await
    }

    pub async fn create_from_json_string<T>(json_str: &str) -> Result<T, ValidationError>
    where
        T: DeserializeOwned + Serialize + Validator + Debug,
    {
        let object: T = from_str(json_str).unwrap();
        RawConfig::create_from_object::<T>(object).await
    }
}

impl Validator for RawConfig {
    fn validation(&self) -> Result<(), ValidationError> {
        self.validate_object(self)
    }
}
