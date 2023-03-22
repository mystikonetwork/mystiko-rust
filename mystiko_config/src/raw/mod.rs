pub mod asset;
pub mod bridge;
pub mod chain;
pub mod circuit;
pub mod contract;
pub mod indexer;
pub mod mystiko;
pub mod provider;
pub mod validator;

use ::validator::Validate;
use anyhow::bail;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::from_str;
use std::fmt::Debug;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub trait Validator {
    fn validation(&self) -> anyhow::Result<()>;
}

pub fn validate_raw<T>(raw: &T) -> anyhow::Result<()>
where
    T: Validate + Debug,
{
    let validate = raw.validate();
    match validate {
        Ok(_) => Ok(()),
        Err(validation) => bail!(
            "failed to validate config object: \n {:?}",
            validation.errors()
        ),
    }
}

pub fn create_raw<T>(plain: T) -> anyhow::Result<T>
where
    T: Validator + Serialize + DeserializeOwned + Debug,
{
    match plain.validation() {
        Ok(_) => Ok(plain),
        Err(err) => Err(err),
    }
}

pub fn create_raw_from_json<T>(json_str: &str) -> anyhow::Result<T>
where
    T: Validator + Serialize + DeserializeOwned + Debug,
{
    let object: T = from_str(json_str)?;
    create_raw::<T>(object)
}

pub async fn create_raw_from_file<T>(json_file: &str) -> anyhow::Result<T>
where
    T: Validator + Serialize + DeserializeOwned + Debug,
{
    let mut file = File::open(json_file).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let result: T = from_str(&contents)?;
    create_raw::<T>(result)
}
